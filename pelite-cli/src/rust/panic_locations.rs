use pelite::{FileMap, PeFile, Wrap, image};

use crate::*;

const MAX_PATH_LENGTH: u64 = 4096;
const MAX_LINE: u32 = 10_000_000;
const MAX_COLUMN: u32 = 1_000_000;

#[derive(serde::Serialize)]
struct LocationOutput<'a> {
	address: u32,
	file: &'a str,
	line: u32,
	column: u32,
}

pub fn command() -> clap::Command {
	clap::Command::new("panic-locations")
		.about("Find candidate Rust panic locations in a PE image")
		.after_help(include_str!("../docs/rust-panic-locations.md"))
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let map = FileMap::open(path)?;
	let file = PeFile::from_bytes(&map)?;
	let output = analyze(file);
	match format {
		OutputFormat::Json => print_json(&output, false),
		OutputFormat::JsonPretty => print_json(&output, true),
		OutputFormat::Text => {
			let name = path.file_name().and_then(|name| name.to_str()).unwrap_or("<input>");
			let mut writer = io::stdout().lock();
			for item in output {
				writeln!(writer, "{name}!{:#010x} {}:{}:{}", item.address, item.file, item.line, item.column)?;
			}
			Ok(())
		},
	}
}

fn is_readonly_data(section: &image::IMAGE_SECTION_HEADER) -> bool {
	let flags = section.Characteristics;
	flags & image::IMAGE_SCN_MEM_READ != 0 && flags & (image::IMAGE_SCN_MEM_WRITE | image::IMAGE_SCN_MEM_EXECUTE) == 0
}

fn source_path(file: PeFile<'_>, pointer: u64, length: u64) -> Option<&'_ str> {
	if length == 0 || length > MAX_PATH_LENGTH {
		return None;
	}
	let rva = file.va_to_rva(pointer).ok()?;
	if !file.section_headers().by_rva(rva).is_some_and(is_readonly_data) {
		return None;
	}
	let bytes = file.slice_bytes(rva).ok()?.get(..length as usize)?;
	let path = valid_source_path(bytes)?;
	Some(path)
}

fn valid_source_path(bytes: &[u8]) -> Option<&str> {
	let path = str::from_utf8(bytes).ok()?;
	(path.contains('/') && path.ends_with(".rs") && path.chars().all(|ch| !ch.is_control())).then_some(path)
}

fn valid_position(line: u32, column: u32) -> bool {
	line >= 1 && line <= MAX_LINE && column >= 1 && column <= MAX_COLUMN
}

fn location(file: PeFile<'_>, address: u32, pointer: u64, length: u64, line: u32, column: u32) -> Option<LocationOutput<'_>> {
	if !valid_position(line, column) {
		return None;
	}
	Some(LocationOutput {
		address,
		file: source_path(file, pointer, length)?,
		line,
		column,
	})
}

pub fn is_location64(file: pelite::pe64::PeFile<'_>, rva: u32) -> bool {
	if !file.section_headers().by_rva(rva).is_some_and(is_readonly_data) {
		return false;
	}
	let Ok(record) = file.derva_slice::<u8>(rva, 24) else { return false };
	let pointer = u64::from_le_bytes(record[0..8].try_into().unwrap());
	let length = u64::from_le_bytes(record[8..16].try_into().unwrap());
	let line = u32::from_le_bytes(record[16..20].try_into().unwrap());
	let column = u32::from_le_bytes(record[20..24].try_into().unwrap());
	location(Wrap::T64(file), rva, pointer, length, line, column).is_some()
}

fn analyze(file: PeFile<'_>) -> Vec<LocationOutput<'_>> {
	match file {
		Wrap::T32(file) => analyze32(file),
		Wrap::T64(file) => analyze64(file),
	}
}

fn analyze32(file: pelite::pe32::PeFile<'_>) -> Vec<LocationOutput<'_>> {
	let mut output = Vec::new();
	for section in file.section_headers() {
		if !is_readonly_data(section) {
			continue;
		}
		let Ok(bytes) = file.get_section_bytes(section) else { continue };
		let Some(words) = dataview::DataView::from(bytes).try_slice::<u32>(0, bytes.len() / 4) else { continue };
		for (index, record) in words.windows(4).enumerate() {
			let Some(address) = u32::try_from(index * 4).ok().and_then(|offset| section.VirtualAddress.checked_add(offset)) else { continue };
			if let Some(candidate) = location(Wrap::T32(file), address, record[0] as u64, record[1] as u64, record[2], record[3]) {
				output.push(candidate);
			}
		}
	}
	output
}

fn analyze64(file: pelite::pe64::PeFile<'_>) -> Vec<LocationOutput<'_>> {
	let mut output = Vec::new();
	for section in file.section_headers() {
		if !is_readonly_data(section) {
			continue;
		}
		let Ok(bytes) = file.get_section_bytes(section) else { continue };
		let Some(words) = dataview::DataView::from(bytes).try_slice::<u64>(0, bytes.len() / 8) else { continue };
		for (index, record) in words.windows(3).enumerate() {
			let Some(address) = u32::try_from(index * 8).ok().and_then(|offset| section.VirtualAddress.checked_add(offset)) else { continue };
			let line = record[2] as u32;
			let column = (record[2] >> 32) as u32;
			if let Some(candidate) = location(Wrap::T64(file), address, record[0], record[1], line, column) {
				output.push(candidate);
			}
		}
	}
	output
}

#[test]
fn source_paths_require_valid_utf8_and_rust_suffix() {
	assert_eq!(valid_source_path(b"src/main.rs"), Some("src/main.rs"));
	assert_eq!(valid_source_path(b"C:\\work/src/lib.rs"), Some("C:\\work/src/lib.rs"));
	assert_eq!(valid_source_path(b"main.rs"), None);
	assert_eq!(valid_source_path(b"src/main.rs.bak"), None);
	assert_eq!(valid_source_path(b"src/main.rs\n"), None);
	assert_eq!(valid_source_path(b"src/\xff.rs"), None);
}

#[test]
fn positions_have_reasonable_bounds() {
	assert!(valid_position(1, 1));
	assert!(valid_position(MAX_LINE, MAX_COLUMN));
	assert!(!valid_position(0, 1));
	assert!(!valid_position(1, 0));
	assert!(!valid_position(MAX_LINE + 1, 1));
	assert!(!valid_position(1, MAX_COLUMN + 1));
}
