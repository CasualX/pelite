use std::f64::consts::LN_2;

use pelite::{PeFile, Wrap, image, stringify};
use sha2::{Digest, Sha256};

use super::*;

#[derive(serde::Serialize)]
struct Summary {
	file: String,
	file_size: usize,
	hashes: Hashes,
	image: Image,
	mitigations: Mitigations,
	sections: Vec<Section>,
	imports: Imports,
	#[serde(skip_serializing_if = "Option::is_none")]
	pdb_path: Option<String>,
	findings: Vec<Finding>,
}

#[derive(serde::Serialize)]
struct Hashes {
	md5: String,
	sha256: String,
	imphash: String,
}

#[derive(serde::Serialize)]
struct Image {
	format: &'static str,
	machine: String,
	kind: &'static str,
	subsystem: String,
	entry_point_rva: u32,
	entry_point_section: Option<String>,
	image_base: u64,
	image_size: u32,
	compile_timestamp: u32,
	compile_time_utc: Option<String>,
	checksum: u32,
	checksum_valid: bool,
	certificate_table_present: bool,
	tls_callbacks: usize,
	overlay_offset: usize,
	overlay_size: usize,
}

#[derive(serde::Serialize)]
struct Mitigations {
	aslr: bool,
	dep: bool,
	cfg: bool,
	high_entropy_va: bool,
}

#[derive(serde::Serialize)]
struct Section {
	name: String,
	virtual_address: u32,
	virtual_size: u32,
	raw_size: u32,
	entropy: Option<f64>,
	permissions: String,
	entry_point: bool,
}

#[derive(serde::Serialize)]
struct Imports {
	libraries: usize,
	functions: usize,
	by_library: Vec<LibraryImports>,
	notable: BTreeMap<&'static str, Vec<String>>,
}

#[derive(serde::Serialize)]
struct LibraryImports {
	name: String,
	functions: usize,
}

#[derive(serde::Serialize)]
struct Finding {
	level: &'static str,
	message: String,
}

pub fn command() -> clap::Command {
	clap::Command::new("summary")
		.about("Give a first-look summary of a possibly suspicious PE binary")
		.after_help("Highlights facts and triage signals; it does not determine whether a file is malicious. The compile timestamp is self-reported, and signature presence is not signature validation.")
		.arg(file_arg().required(true))
}

pub fn file_arg() -> clap::Arg {
	clap::Arg::new("file")
		.value_name("FILE")
		.value_parser(clap::value_parser!(PathBuf))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let map = pelite::FileMap::open(path)?;
	let pe = PeFile::from_bytes(&map)?;
	let output = summarize(path, map.as_ref(), pe)?;

	match format {
		OutputFormat::Text => print_text(&output),
		OutputFormat::Json => print_json(&output, false),
		OutputFormat::JsonPretty => print_json(&output, true),
	}
}

fn summarize(path: &Path, bytes: &[u8], pe: PeFile<'_>) -> Result<Summary> {
	let file_header = pe.file_header();
	let (format, entry_point, image_base, image_size, subsystem, dll_chars, checksum) = match pe.optional_header() {
		Wrap::T32(header) => ("PE32", header.AddressOfEntryPoint, header.ImageBase as u64, header.SizeOfImage, header.Subsystem, header.DllCharacteristics, header.CheckSum),
		Wrap::T64(header) => ("PE32+", header.AddressOfEntryPoint, header.ImageBase.get(), header.SizeOfImage, header.Subsystem, header.DllCharacteristics, header.CheckSum),
	};
	let is_dll = file_header.Characteristics & image::IMAGE_FILE_DLL != 0;
	let is_driver = file_header.Characteristics & image::IMAGE_FILE_SYSTEM != 0 || dll_chars & image::IMAGE_DLLCHARACTERISTICS_WDM_DRIVER != 0;
	let kind = if is_driver { "driver" } else if is_dll { "DLL" } else { "executable" };
	let machine = stringify::Machine(file_header.Machine).description().map(str::to_owned).unwrap_or_else(|| format!("unknown ({:#x})", file_header.Machine));
	let subsystem = stringify::Subsystem(subsystem).description().map(str::to_owned).unwrap_or_else(|| format!("unknown ({subsystem:#x})"));

	let entry_section = pe.section_headers().by_rva(entry_point);
	let entry_point_section = entry_section.map(|section| section.name_fmt().to_string());
	let mut findings = Vec::new();
	if entry_point != 0 && entry_section.is_none() {
		findings.push(Finding { level: "warning", message: "Entry point is outside the declared sections".to_owned() });
	}
	else if let Some(section) = entry_section
		&& section.Characteristics & image::IMAGE_SCN_MEM_EXECUTE == 0 {
		findings.push(Finding { level: "warning", message: format!("Entry point is in non-executable section {}", section.name_fmt()) });
	}

	let mut sections = Vec::new();
	let mut last_raw_end = pe.optional_header().into_size_of_headers() as usize;
	for section in pe.section_headers() {
		let name = section.name_fmt().to_string();
		let raw_end = (section.PointerToRawData as usize).saturating_add(section.SizeOfRawData as usize);
		last_raw_end = last_raw_end.max(raw_end.min(bytes.len()));
		let section_bytes = pe.get_section_bytes(section).ok();
		let entropy = section_bytes.filter(|data| !data.is_empty()).map(shannon_entropy);
		let executable = section.Characteristics & image::IMAGE_SCN_MEM_EXECUTE != 0;
		let writable = section.Characteristics & image::IMAGE_SCN_MEM_WRITE != 0;
		if executable && writable {
			findings.push(Finding { level: "warning", message: format!("Section {name} is both writable and executable") });
		}
		if entropy.is_some_and(|value| value >= 7.2) && section.SizeOfRawData >= 256 {
			findings.push(Finding { level: "note", message: format!("Section {name} has high entropy ({:.2}); it may be compressed or encrypted", entropy.unwrap()) });
		}
		if section_bytes.is_none() && section.SizeOfRawData != 0 {
			findings.push(Finding { level: "warning", message: format!("Section {name} has an invalid raw-data range") });
		}
		sections.push(Section {
			name,
			virtual_address: section.VirtualAddress,
			virtual_size: section.VirtualSize,
			raw_size: section.SizeOfRawData,
			entropy: entropy.map(|value| (value * 100.0).round() / 100.0),
			permissions: permissions(section.Characteristics),
			entry_point: entry_section.is_some_and(|candidate| std::ptr::eq(candidate, section)),
		});
	}

	let overlay_offset = last_raw_end.min(bytes.len());
	let overlay_size = bytes.len().saturating_sub(overlay_offset);
	let security = pe.data_directory().get(image::IMAGE_DIRECTORY_ENTRY_SECURITY);
	let certificate_table_present = security.is_some_and(|entry| entry.VirtualAddress != 0 && entry.Size != 0);
	let certificate_overlap = security.map(|entry| {
		let start = entry.VirtualAddress as usize;
		let end = start.saturating_add(entry.Size as usize).min(bytes.len());
		end.saturating_sub(start.max(overlay_offset))
	}).unwrap_or(0);
	let unexplained_overlay = overlay_size.saturating_sub(certificate_overlap.min(overlay_size));
	if unexplained_overlay != 0 {
		findings.push(Finding { level: "note", message: format!("{unexplained_overlay} bytes follow the last section outside the certificate table") });
	}

	let (imports, imphash) = summarize_imports(pe, &mut findings)?;
	if !imports.notable.is_empty() {
		findings.push(Finding { level: "note", message: "Notable imports suggest capabilities worth reviewing; imports alone do not prove behavior".to_owned() });
	}
	let tls_callbacks = match pe.tls() {
		Ok(tls) => tls.callbacks().map(|callbacks| match callbacks { Wrap::T32(values) => values.len(), Wrap::T64(values) => values.len() }).unwrap_or(0),
		Err(error) if error.is_null() => 0,
		Err(_) => {
			findings.push(Finding { level: "warning", message: "TLS directory could not be parsed".to_owned() });
			0
		},
	};
	if tls_callbacks != 0 {
		findings.push(Finding { level: "note", message: format!("Contains {tls_callbacks} TLS callback(s) that run before the normal entry point") });
	}

	let pdb_path = pe.debug().ok()
		.and_then(|debug| debug.pdb_file_name().ok().flatten())
		.and_then(|name| name.to_str().ok())
		.map(str::to_owned);
	let checksum_valid = checksum != 0 && checksum == pe.headers().check_sum();
	let mut sha256 = Sha256::new();
	sha256.update(bytes);
	let sha256 = sha256.finalize();

	Ok(Summary {
		file: path.to_string_lossy().into_owned(),
		file_size: bytes.len(),
		hashes: Hashes {
			md5: format!("{:x}", md5::compute(bytes)),
			sha256: encode_lower_hex(sha256.as_ref()),
			imphash,
		},
		image: Image {
			format,
			machine,
			kind,
			subsystem,
			entry_point_rva: entry_point,
			entry_point_section,
			image_base,
			image_size,
			compile_timestamp: file_header.TimeDateStamp,
			compile_time_utc: format_timestamp(file_header.TimeDateStamp),
			checksum,
			checksum_valid,
			certificate_table_present,
			tls_callbacks,
			overlay_offset,
			overlay_size,
		},
		mitigations: Mitigations {
			aslr: dll_chars & image::IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE != 0,
			dep: dll_chars & image::IMAGE_DLLCHARACTERISTICS_NX_COMPAT != 0,
			cfg: dll_chars & image::IMAGE_DLLCHARACTERISTICS_GUARD_CF != 0,
			high_entropy_va: dll_chars & image::IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA != 0,
		},
		sections,
		imports,
		pdb_path,
		findings,
	})
}

fn encode_lower_hex(bytes: &[u8]) -> String {
	const DIGITS: &[u8; 16] = b"0123456789abcdef";
	let mut output = String::with_capacity(bytes.len() * 2);
	for &byte in bytes {
		output.push(char::from(DIGITS[usize::from(byte >> 4)]));
		output.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
	}
	output
}

trait OptionalHeaderSize {
	fn into_size_of_headers(self) -> u32;
}

impl OptionalHeaderSize for Wrap<&image::IMAGE_OPTIONAL_HEADER32, &image::IMAGE_OPTIONAL_HEADER64> {
	fn into_size_of_headers(self) -> u32 {
		match self { Wrap::T32(header) => header.SizeOfHeaders, Wrap::T64(header) => header.SizeOfHeaders }
	}
}

fn summarize_imports(pe: PeFile<'_>, findings: &mut Vec<Finding>) -> Result<(Imports, String)> {
	let mut libraries = Vec::new();
	let mut normalized = Vec::new();
	let mut notable = BTreeMap::<&'static str, Vec<String>>::new();
	match pe.imports() {
		Ok(directory) => for descriptor in directory {
			let dll = descriptor.dll_name()?.to_str()?.to_owned();
			let dll_hash = strip_library_extension(&dll.to_ascii_lowercase()).to_owned();
			let mut count = 0;
			for import in descriptor.int()? {
				let symbol = match import? {
					pelite::Import::ByName { name, .. } => name.to_str()?.to_owned(),
					pelite::Import::ByOrdinal { ord } => format!("ord{ord}"),
				};
				normalized.push(format!("{dll_hash}.{}", symbol.to_ascii_lowercase()));
				if let Some(category) = import_category(&symbol) {
					notable.entry(category).or_default().push(format!("{dll}!{symbol}"));
				}
				count += 1;
			}
			libraries.push(LibraryImports { name: dll, functions: count });
		},
		Err(error) if error.is_null() => (),
		Err(_) => findings.push(Finding { level: "warning", message: "Import directory could not be parsed".to_owned() }),
	}
	let input = normalized.join(",");
	let functions = libraries.iter().map(|library| library.functions).sum();
	let imphash = format!("{:x}", md5::compute(input.as_bytes()));
	Ok((Imports { libraries: libraries.len(), functions, by_library: libraries, notable }, imphash))
}

fn strip_library_extension(name: &str) -> &str {
	match name.rsplit_once('.') {
		Some((stem, "dll" | "sys" | "ocx")) => stem,
		_ => name,
	}
}

fn import_category(name: &str) -> Option<&'static str> {
	let name = name.to_ascii_lowercase();
	let category = match name.as_str() {
		"createprocessa" | "createprocessw" | "shellexecutea" | "shellexecutew" | "winexec" => "process execution",
		"virtualallocex" | "writeprocessmemory" | "readprocessmemory" | "createremotethread" | "ntmapviewofsection" | "ntunmapviewofsection" | "openprocess" => "process access / injection",
		"internetopena" | "internetopenw" | "internetconnecta" | "internetconnectw" | "httpopenrequesta" | "httpopenrequestw" | "httpsendrequesta" | "httpsendrequestw" | "urldownloadtofilea" | "urldownloadtofilew" | "wsastartup" | "connect" | "recv" | "send" => "networking",
		"createservicea" | "createservicew" | "startservicea" | "startservicew" | "regsetvalueexa" | "regsetvalueexw" => "services / registry",
		"cryptdecrypt" | "cryptencrypt" | "cryptacquirecontexta" | "cryptacquirecontextw" | "bcryptdecrypt" | "bcryptencrypt" => "cryptography",
		"isdebuggerpresent" | "checkremotedebuggerpresent" | "ntqueryinformationprocess" | "gettickcount" | "queryperformancecounter" => "anti-analysis",
		"loadlibrarya" | "loadlibraryw" | "loadlibraryexa" | "loadlibraryexw" | "getprocaddress" => "dynamic loading",
		_ => return None,
	};
	Some(category)
}

fn permissions(characteristics: u32) -> String {
	[
		(image::IMAGE_SCN_MEM_READ, 'R'),
		(image::IMAGE_SCN_MEM_WRITE, 'W'),
		(image::IMAGE_SCN_MEM_EXECUTE, 'X'),
	].into_iter().map(|(flag, letter)| if characteristics & flag != 0 { letter } else { '-' }).collect()
}

fn shannon_entropy(bytes: &[u8]) -> f64 {
	let mut counts = [0usize; 256];
	for &byte in bytes { counts[byte as usize] += 1; }
	let entropy = counts.into_iter().filter(|&count| count != 0).map(|count| {
		let probability = count as f64 / bytes.len() as f64;
		-probability * (probability.ln() / LN_2)
	}).sum::<f64>();
	if entropy == 0.0 { 0.0 } else { entropy }
}

fn format_timestamp(timestamp: u32) -> Option<String> {
	if timestamp == 0 { return None; }
	let days = timestamp as i64 / 86_400;
	let seconds = timestamp % 86_400;
	let z = days + 719_468;
	let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
	let day_of_era = z - era * 146_097;
	let year_of_era = (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
	let mut year = year_of_era + era * 400;
	let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
	let month_prime = (5 * day_of_year + 2) / 153;
	let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
	let month = month_prime + if month_prime < 10 { 3 } else { -9 };
	year += i64::from(month <= 2);
	Some(format!("{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z", seconds / 3_600, seconds / 60 % 60, seconds % 60))
}

fn print_text(summary: &Summary) -> Result {
	println!("PE summary");
	println!("==========");
	println!("File       : {} ({} bytes)", summary.file, summary.file_size);
	println!("SHA-256    : {}", summary.hashes.sha256);
	println!("MD5        : {}", summary.hashes.md5);
	println!("Import hash: {}", summary.hashes.imphash);
	println!();
	println!("Image");
	println!("  Type       : {} {} {} ({})", summary.image.format, summary.image.machine, summary.image.kind, summary.image.subsystem);
	println!("  Entry point: RVA {:#x} ({})", summary.image.entry_point_rva, summary.image.entry_point_section.as_deref().unwrap_or("no section"));
	println!("  Image base : {:#x}", summary.image.image_base);
	println!("  Image size : {} bytes", summary.image.image_size);
	println!("  Timestamp  : {} (self-reported, not trusted)", summary.image.compile_time_utc.as_deref().unwrap_or("not set"));
	println!("  Checksum   : {:#x} ({})", summary.image.checksum, if summary.image.checksum_valid { "valid" } else if summary.image.checksum == 0 { "not set" } else { "does not match" });
	println!("  Certificate: {}", if summary.image.certificate_table_present { "present (not validated)" } else { "not present" });
	println!("  TLS callbacks: {}", summary.image.tls_callbacks);
	println!("  Overlay    : {} bytes at file offset {:#x}", summary.image.overlay_size, summary.image.overlay_offset);
	println!();
	println!("Mitigations");
	println!("  ASLR: {:<3}  DEP/NX: {:<3}  CFG: {:<3}  High-entropy VA: {}", yes_no(summary.mitigations.aslr), yes_no(summary.mitigations.dep), yes_no(summary.mitigations.cfg), yes_no(summary.mitigations.high_entropy_va));
	println!();
	println!("Sections");
	println!("  {:<9} {:>10} {:>10} {:>8} {:>7}  Entry", "Name", "RVA", "Raw size", "Entropy", "Perms");
	for section in &summary.sections {
		println!("  {:<9} {:#010x} {:>10} {:>8} {:>7}  {}", section.name, section.virtual_address, section.raw_size, section.entropy.map(|value| format!("{value:.2}")).unwrap_or_else(|| "-".to_owned()), section.permissions, if section.entry_point { "<- entry point" } else { "" });
	}
	println!();
	println!("Imports: {} functions from {} libraries", summary.imports.functions, summary.imports.libraries);
	if !summary.imports.by_library.is_empty() {
		println!("  {}", summary.imports.by_library.iter().map(|library| format!("{} ({})", library.name, library.functions)).collect::<Vec<_>>().join(", "));
	}
	if !summary.imports.notable.is_empty() {
		println!("  Notable imports (review in context):");
		for (category, imports) in &summary.imports.notable {
			println!("    {category}: {}", imports.join(", "));
		}
	}
	if let Some(pdb) = &summary.pdb_path { println!("\nPDB path: {pdb}"); }
	println!("\nSignals to review");
	if summary.findings.is_empty() {
		println!("  None from these lightweight checks.");
	}
	else {
		for finding in &summary.findings { println!("  [{}] {}", finding.level, finding.message); }
	}
	println!("\nThese are triage clues, not a malware verdict. Do not execute an untrusted sample on your host.");
	Ok(())
}

fn yes_no(value: bool) -> &'static str {
	if value { "yes" } else { "no" }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn entropy_extremes() {
		assert_eq!(shannon_entropy(&[0; 512]), 0.0);
		let every_byte = (0..=255).collect::<Vec<u8>>();
		assert!((shannon_entropy(&every_byte) - 8.0).abs() < f64::EPSILON);
	}

	#[test]
	fn timestamps_are_utc() {
		assert_eq!(format_timestamp(0), None);
		assert_eq!(format_timestamp(1), Some("1970-01-01T00:00:01Z".to_owned()));
		assert_eq!(format_timestamp(951_827_696), Some("2000-02-29T12:34:56Z".to_owned()));
	}
}
