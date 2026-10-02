use iced_x86::{Decoder, DecoderOptions, Mnemonic, OpKind, Register};
use pelite::{image, FileMap, PeFile, Wrap};

use super::*;

mod throws32;
mod throws64;

pub fn command() -> clap::Command {
	clap::Command::new("throws")
		.about("Extract Microsoft C++ thrown types and exception conversion metadata")
		.after_help(include_str!("../docs/msvc-throws.md"))
		.arg(clap::Arg::new("demangle").long("demangle")
			.help("Demangle C++ exception type names").action(clap::ArgAction::SetTrue))
		.arg(clap::Arg::new("file").value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf)).required(true))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let map = FileMap::open(matches.get_one::<PathBuf>("file").expect("required by clap"))?;
	let mut output = analyze(PeFile::from_bytes(&map)?)?;
	if matches.get_flag("demangle") {
		for item in &mut output {
			for ty in &mut item.catchable_types {
				ty.name = demangle_name(&ty.name);
			}
		}
	}
	if !matches!(format, OutputFormat::Text) {
		return print_json(&output, matches!(format, OutputFormat::JsonPretty));
	}
	let mut out = io::stdout().lock();
	for item in output {
		writeln!(out, "{:#010X}: throw {} (attributes {:#X})", item.rva, item.catchable_types[0].name, item.attributes)?;
		writeln!(out, "  destructor: {}, forward compatibility: {}", display_rva(item.destructor_rva), display_rva(item.forward_compat_rva))?;
		for reference in item.references {
			writeln!(out, "  metadata reference: {reference:#010X}")?;
		}
		for ty in item.catchable_types {
			writeln!(out, "  catch {}: size/offset {}, PMD ({}, {}, {}), properties {:#X}, copy {}",
				ty.name, ty.size_or_offset, ty.displacement.mdisp, ty.displacement.pdisp,
				ty.displacement.vdisp, ty.properties, display_rva(ty.copy_function_rva))?;
		}
		writeln!(out)?;
	}
	Ok(())
}

fn display_rva(rva: Option<u32>) -> impl fmt::Display {
	fmt::from_fn(move |f| match rva {
		Some(rva) => write!(f, "{rva:#010X}"),
		None => f.write_str("none"),
	})
}

#[derive(Debug, serde::Serialize)]
struct ThrowOutput {
	rva: u32,
	attributes: u32,
	destructor_rva: Option<u32>,
	forward_compat_rva: Option<u32>,
	catchable_type_array_rva: u32,
	/// Instructions loading this metadata, not verified throw calls.
	references: Vec<u32>,
	catchable_types: Vec<CatchableOutput>,
}

#[derive(Debug, serde::Serialize)]
struct CatchableOutput {
	rva: u32,
	type_descriptor_rva: u32,
	name: String,
	properties: u32,
	simple_type: bool,
	by_reference_only: bool,
	has_virtual_bases: bool,
	displacement: Displacement,
	size_or_offset: i32,
	copy_function_rva: Option<u32>,
}

#[derive(Debug, serde::Serialize)]
struct Displacement {
	mdisp: i32,
	pdisp: i32,
	vdisp: i32,
}

fn analyze(file: PeFile<'_>) -> Result<Vec<ThrowOutput>> {
	let machine = file.file_header().Machine;
	let bits = match machine {
		image::IMAGE_FILE_MACHINE_I386 => 32,
		image::IMAGE_FILE_MACHINE_AMD64 => 64,
		_ => return Err(err("MSVC throw analysis supports x86 and x64 only")),
	};
	let mut candidates = std::collections::BTreeMap::<u32, Vec<u32>>::new();
	for section in file.section_headers() {
		if section.Characteristics & image::IMAGE_SCN_MEM_EXECUTE == 0 {
			continue;
		}
		let Ok(bytes) = file.get_section_bytes(section) else { continue };
		let base = file.rva_to_va(section.VirtualAddress)?;
		let mut decoder = Decoder::with_ip(bits, bytes, base, DecoderOptions::NONE);
		while decoder.can_decode() {
			let instruction = decoder.decode();
			let target = if bits == 64 && instruction.mnemonic() == Mnemonic::Lea
				&& instruction.op0_register() == Register::RDX && instruction.is_ip_rel_memory_operand() {
				Some(instruction.ip_rel_memory_address())
			}
			else if bits == 32 && instruction.mnemonic() == Mnemonic::Push && instruction.op0_kind() == OpKind::Immediate32 {
				Some(instruction.immediate32() as u64)
			}
			else if instruction.mnemonic() == Mnemonic::Mov && instruction.op1_kind() == OpKind::Immediate32
				&& (bits == 32 || instruction.op0_register() == Register::EDX) {
				Some(instruction.immediate32() as u64)
			}
			else { None };
			if let Some(target) = target {
				if let (Ok(rva), Ok(reference)) = (file.va_to_rva(target), file.va_to_rva(instruction.ip())) {
					if rva != 0 && rva % 4 == 0 {
						candidates.entry(rva).or_default().push(reference);
					}
				}
			}
		}
	}
	let mut output = Vec::new();
	for (rva, references) in candidates {
		let parsed = match file {
			Wrap::T32(file) => throws32::parse(file, rva),
			Wrap::T64(file) => throws64::parse(file, rva),
		};
		if let Ok(mut item) = parsed {
			item.references = references;
			output.push(item);
		}
	}
	Ok(output)
}

fn data_address(file: PeFile<'_>, rva: u32) -> pelite::Result<()> {
	if rva != 0 && file.section_headers().iter().any(|s|
		rva >= s.VirtualAddress && rva < s.VirtualAddress.saturating_add(s.VirtualSize)
		&& s.Characteristics & image::IMAGE_SCN_MEM_READ != 0
		&& s.Characteristics & image::IMAGE_SCN_MEM_EXECUTE == 0) {
		Ok(())
	} else { Err(pelite::Error::Invalid) }
}

fn code_address(file: PeFile<'_>, rva: u32) -> pelite::Result<Option<u32>> {
	if rva == 0 { return Ok(None) }
	if file.section_headers().iter().any(|s|
		rva >= s.VirtualAddress && rva < s.VirtualAddress.saturating_add(s.VirtualSize)
		&& s.Characteristics & image::IMAGE_SCN_MEM_EXECUTE != 0) {
		file.derva_copy::<u8>(rva)?;
		Ok(Some(rva))
	} else { Err(pelite::Error::Invalid) }
}

fn type_name(file: PeFile<'_>, rva: u32, header: u32) -> pelite::Result<String> {
	data_address(file, rva)?;
	let name = file.derva_c_str(rva.checked_add(header).ok_or(pelite::Error::Overflow)?)?.to_str()?;
	// Exception descriptors include primitive and pointer types, not just .?A classes.
	if !name.starts_with('.') || name.len() < 2 || name.len() > 4096 || !name.is_ascii() {
		return Err(pelite::Error::Invalid);
	}
	Ok(name.to_owned())
}

#[cfg(test)]
mod tests {
	use super::*;

	const X86: &[u8] = include_bytes!("../../../fixtures/bin/throws-x86.exe");
	const X64: &[u8] = include_bytes!("../../../fixtures/bin/throws-x64.exe");

	fn check_fixture(bytes: &[u8], pointer_size: i32) {
		let file = PeFile::from_bytes(bytes).unwrap();
		let output = analyze(file).unwrap();
		let find = |name: &str| output.iter().find(|item| item.catchable_types[0].name == name).unwrap();
		let integer = find(".H");
		assert!(integer.catchable_types[0].simple_type);
		assert_eq!(integer.catchable_types[0].size_or_offset, 4);
		assert!(integer.destructor_rva.is_none());
		let plain = find(".?AUPlainException@fixture@@");
		assert_eq!(plain.catchable_types.len(), 1);
		assert_eq!(plain.catchable_types[0].size_or_offset, 4);
		let derived = find(".?AUDerivedException@fixture@@");
		assert_eq!(derived.catchable_types.len(), 3);
		assert_eq!(derived.catchable_types[1].name, ".?AUBaseException@fixture@@");
		assert_eq!(derived.catchable_types[2].name, ".?AUOtherException@fixture@@");
		assert_eq!(derived.catchable_types[2].displacement.mdisp, 4);
		let virtual_type = find(".?AUVirtualException@fixture@@");
		assert!(virtual_type.catchable_types[0].has_virtual_bases);
		assert!(virtual_type.catchable_types[0].copy_function_rva.is_some());
		assert_eq!(virtual_type.catchable_types[1].displacement.pdisp, 0);
		assert_eq!(virtual_type.catchable_types[1].displacement.vdisp, 4);
		let owned = find(".?AUOwnedException@fixture@@");
		assert!(owned.destructor_rva.is_some());
		assert!(owned.catchable_types[0].copy_function_rva.is_some());
		let pointers: Vec<_> = output.iter().filter(|item| item.catchable_types[0].name.starts_with(".P")
			&& item.catchable_types[0].name.contains("PlainException")).collect();
		assert_eq!(pointers.len(), 2);
		assert!(pointers.iter().any(|item| item.attributes & 1 != 0));
		assert!(pointers.iter().any(|item| item.attributes == 0));
		for item in pointers {
			assert_eq!(item.catchable_types.len(), 2);
			assert_eq!(item.catchable_types[0].size_or_offset, pointer_size);
			assert!(item.catchable_types[0].simple_type);
		}
		// Seven fixture records; CRT exceptions are allowed separately.
		assert_eq!(output.iter().filter(|item| item.catchable_types[0].name == ".H"
			|| item.catchable_types[0].name.contains("@fixture@@")).count(), 7);
		for item in output {
			assert!(!item.references.is_empty());
			assert!(item.references.windows(2).all(|pair| pair[0] < pair[1]));
		}
	}

	#[test]
	fn fixture_x86() { check_fixture(X86, 4); }

	#[test]
	fn fixture_x64() { check_fixture(X64, 8); }

	fn parse_candidate(bytes: &[u8], rva: u32) -> pelite::Result<ThrowOutput> {
		match PeFile::from_bytes(bytes).unwrap() {
			Wrap::T32(file) => throws32::parse(file, rva),
			Wrap::T64(file) => throws64::parse(file, rva),
		}
	}

	fn overwrite(bytes: &mut [u8], rva: u32, value: u32) {
		let file = PeFile::from_bytes(bytes).unwrap();
		let offset = file.headers().rva_to_file_offset(rva).unwrap();
		bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
	}

	#[test]
	fn malformed_metadata_is_rejected() {
		for source in [X86, X64] {
			let result = analyze(PeFile::from_bytes(source).unwrap()).unwrap();
			let item = &result[0];
			for (rva, value) in [(item.rva, u32::MAX), (item.rva + 12, 0),
				(item.catchable_type_array_rva, 0), (item.catchable_type_array_rva, u32::MAX),
				(item.catchable_type_array_rva, 4097), (item.catchable_type_array_rva + 4, u32::MAX),
				(item.catchable_types[0].rva + 4, u32::MAX), (item.catchable_types[0].rva + 24, u32::MAX)] {
				let mut bytes = source.to_vec();
				overwrite(&mut bytes, rva, value);
				assert!(parse_candidate(&bytes, item.rva).is_err(), "accepted invalid value at {rva:#x}");
				// Candidate rejection must also be safe during whole-image analysis.
				let output = analyze(PeFile::from_bytes(&bytes).unwrap()).unwrap();
				assert!(!output.iter().any(|entry| entry.rva == item.rva));
			}
		}
	}

	#[test]
	fn independent_of_relocations_and_section_names() {
		for source in [X86, X64] {
			let mut bytes = source.to_vec();
			let file = PeFile::from_bytes(source).unwrap();
			let expected = serde_json::to_value(analyze(file).unwrap()).unwrap();
			let nt = u32::from_le_bytes(source[0x3c..0x40].try_into().unwrap()) as usize;
			let directories = nt + 24 + if file.file_header().Machine == image::IMAGE_FILE_MACHINE_I386 { 96 } else { 112 };
			let reloc = directories + image::IMAGE_DIRECTORY_ENTRY_BASERELOC * 8;
			bytes[reloc..reloc + 8].fill(0);
			let sections = nt + 24 + file.file_header().SizeOfOptionalHeader as usize;
			for index in 0..file.file_header().NumberOfSections as usize {
				bytes[sections + index * 40..sections + index * 40 + 8].copy_from_slice(b"renamed\0");
			}
			assert_eq!(serde_json::to_value(analyze(PeFile::from_bytes(&bytes).unwrap()).unwrap()).unwrap(), expected);
		}
	}

	#[test]
	fn unsupported_machine_is_rejected() {
		let mut bytes = X64.to_vec();
		let nt = u32::from_le_bytes(bytes[0x3c..0x40].try_into().unwrap()) as usize;
		bytes[nt + 4..nt + 6].copy_from_slice(&image::IMAGE_FILE_MACHINE_ARM64.to_le_bytes());
		assert!(analyze(PeFile::from_bytes(&bytes).unwrap()).is_err());
	}
}
