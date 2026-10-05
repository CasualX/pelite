use pelite::{FileMap, PeFile, Wrap, base_relocs::BaseRelocationDirectory, image};

use crate::*;

const MAX_ALIGN: usize = 4096;
const MAX_SIZE: usize = 1024 * 1024; // 1 MiB

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct VTableOutput {
	address: u32,
	size: usize,
	align: usize,
	functions: usize,
	comments: Vec<String>,
}

#[allow(dead_code)]
trait VtableExample {
	fn name(&self) -> &'static str;
	fn value(&self) -> u64;
}

struct VtableExampleType;

impl VtableExample for VtableExampleType {
	fn name(&self) -> &'static str {
		"pelite-cli vtable example"
	}

	fn value(&self) -> u64 {
		0x123456789abcdef
	}
}

pub fn command() -> clap::Command {
	let _example: &dyn VtableExample = std::hint::black_box(&VtableExampleType);
	clap::Command::new("vtables")
		.about("Find candidate Rust trait vtables in a PE image")
		.after_help(include_str!("docs/vtables.md"))
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
		OutputFormat::Nul => Ok(()),
		OutputFormat::Json => print_json(&output, false),
		OutputFormat::JsonPretty => print_json(&output, true),
		OutputFormat::Text => {
			let name = path.file_name().and_then(|name| name.to_str()).unwrap_or("<input>");
			let mut writer = io::stdout().lock();
			for item in output {
				writeln!(writer, "{name}!{:#010x} size={} align={} functions={}", item.address, item.size, item.align, item.functions)?;
				for comment in item.comments {
					writeln!(writer, "  // {comment}")?;
				}
			}
			Ok(())
		},
	}
}

fn analyze(file: PeFile<'_>) -> Vec<VTableOutput> {
	match file {
		Wrap::T32(file) => analyze32(file),
		Wrap::T64(file) => analyze64(file),
	}
}

fn is_readonly_data(section: &image::IMAGE_SECTION_HEADER) -> bool {
	let flags = section.Characteristics;
	flags & image::IMAGE_SCN_MEM_READ != 0 && flags & (image::IMAGE_SCN_MEM_WRITE | image::IMAGE_SCN_MEM_EXECUTE) == 0
}

fn is_executable(section: &image::IMAGE_SECTION_HEADER) -> bool {
	section.Characteristics & image::IMAGE_SCN_MEM_EXECUTE != 0
}

fn constant_return(code: &[u8], is_64: bool) -> Option<u64> {
	let (offset, width) = match code.first()? {
		0xb0 => (1, 1), // mov al, imm8
		0x66 if code.get(1) == Some(&0xb8) => (2, 2), // mov ax, imm16
		0xb8 => (1, 4), // mov eax, imm32
		0x48 if is_64 && code.get(1) == Some(&0xb8) => (2, 8), // mov rax, imm64
		_ => return None,
	};
	let immediate = code.get(offset..offset + width)?;
	if code.get(offset + width) != Some(&0xc3) {
		return None;
	}
	let mut value = [0; 8];
	value[..width].copy_from_slice(immediate);
	Some(u64::from_le_bytes(value))
}

fn valid_string(bytes: &[u8], length: usize) -> Option<&str> {
	if length == 0 || length > 64 * 1024 {
		return None;
	}
	let string = str::from_utf8(bytes.get(..length)?).ok()?;
	string.chars().all(|ch| !ch.is_control()).then_some(string)
}

//----------------------------------------------------------------

#[repr(C)]
struct VtableHeader32 {
	drop_fn: u32,
	size: u32,
	align: u32,
}

unsafe impl dataview::Pod for VtableHeader32 {}

fn pointers32(file: pelite::pe32::PeFile<'_>) -> Vec<u32> {
	let mut pointers = match file.base_relocs() {
		Ok(relocs) => relocated_pointers32(file, relocs),
		Err(_) => scanned_pointers32(file),
	};
	pointers.sort_unstable();
	pointers.dedup();
	pointers
}

fn relocated_pointers32(file: pelite::pe32::PeFile<'_>, relocs: BaseRelocationDirectory<'_>) -> Vec<u32> {
	let mut pointers = Vec::new();
	relocs.for_each(|rva, ty| {
		// Looking for aligned pointers
		if ty != image::IMAGE_REL_BASED_HIGHLOW || rva % 4 != 0 {
			return;
		}
		// Inside a readonly data section
		if !file.section_headers().by_rva(rva).is_some_and(is_readonly_data) {
			return;
		}
		// Read the pointer
		let Ok(target_va) = file.derva_copy::<u32>(rva, false) else { return };
		let Ok(target_rva) = file.va_to_rva(target_va) else { return };
		// Check the pointer goes from rdata section to text section
		if !file.section_headers().by_rva(target_rva).is_some_and(is_executable) {
			return;
		}
		pointers.push(rva);
	});
	pointers
}

fn scanned_pointers32(file: pelite::pe32::PeFile<'_>) -> Vec<u32> {
	let mut pointers = Vec::new();
	for section in file.section_headers() {
		// Look inside readonly data sections
		if !is_readonly_data(section) {
			continue;
		}
		let Ok(bytes) = file.get_section_bytes(section) else {
			continue;
		};
		// Looking for aligned pointers
		let Some(words) = dataview::DataView::from(bytes).try_slice::<u32>(0, bytes.len() / 4) else {
			continue;
		};
		for (index, &target_va) in words.iter().enumerate() {
			let rva = section.VirtualAddress.wrapping_add((index * 4) as u32);
			// Read the pointer
			let Ok(target_rva) = file.va_to_rva(target_va) else {
				continue;
			};
			// Check the pointer goes from rdata section to text section
			if !file.section_headers().by_rva(target_rva).is_some_and(is_executable) {
				continue;
			}
			pointers.push(rva);
		}
	}
	pointers
}

fn parse_vtable_header32(file: pelite::pe32::PeFile<'_>, address: u32) -> Option<&'_ VtableHeader32> {
	let header = file.derva::<VtableHeader32>(address).ok()?;
	// Validate align
	let align = header.align;
	if align == 0 || align > MAX_ALIGN as u32 || !align.is_power_of_two() {
		return None;
	}
	// Validate size
	if header.size >= MAX_SIZE as u32 || header.size % align != 0 {
		return None;
	}
	// Validate drop function
	if header.drop_fn != 0 {
		let target_rva = file.va_to_rva(header.drop_fn).ok()?;
		if !file.section_headers().by_rva(target_rva).is_some_and(is_executable) {
			return None;
		}
	}
	Some(header)
}

fn static_string32(code: &[u8]) -> Option<(u32, usize)> {
	// mov eax, pointer; mov edx, length; ret (or the two moves reversed)
	let code = code.get(..11)?;
	if code[10] != 0xc3 {
		return None;
	}
	let (pointer, length) = match (code[0], code[5]) {
		(0xb8, 0xba) => (&code[1..5], &code[6..10]),
		(0xba, 0xb8) => (&code[6..10], &code[1..5]),
		_ => return None,
	};
	Some((u32::from_le_bytes(pointer.try_into().ok()?), u32::from_le_bytes(length.try_into().ok()?) as usize))
}

fn function_comment32(file: pelite::pe32::PeFile<'_>, slot: u32, index: usize) -> Option<String> {
	let function_rva = file.va_to_rva(file.derva_copy::<u32>(slot, false).ok()?).ok()?;
	let code = file.slice_bytes(function_rva).ok()?;
	if let Some(value) = constant_return(code, false) {
		return Some(format!("fn[{index}]: return {value:#x}"));
	}
	let (pointer, length) = static_string32(code)?;
	let string_rva = file.va_to_rva(pointer).ok()?;
	if !file.section_headers().by_rva(string_rva).is_some_and(is_readonly_data) {
		return None;
	}
	let string = valid_string(file.slice_bytes(string_rva).ok()?, length)?;
	Some(format!("fn[{index}]: return {string:?}"))
}

fn analyze32(file: pelite::pe32::PeFile<'_>) -> Vec<VTableOutput> {
	let pointers = pointers32(file);

	let mut output = Vec::new();
	let mut index = 0;
	while index < pointers.len() {
		let address = pointers[index].wrapping_sub(12);
		let Some(header) = parse_vtable_header32(file, address) else {
			index += 1;
			continue;
		};
		let mut table = VTableOutput {
			address,
			size: header.size as usize,
			align: header.align as usize,
			functions: 1,
			comments: function_comment32(file, pointers[index], 0).into_iter().collect(),
		};
		index += 1;
		while index < pointers.len() && pointers[index - 1].wrapping_add(4) == pointers[index] {
			// The next pointer may be the drop slot of an adjacent vtable.
			if parse_vtable_header32(file, pointers[index]).is_some() {
				break;
			}
			table.functions += 1;
			if let Some(comment) = function_comment32(file, pointers[index], table.functions - 1) {
				table.comments.push(comment);
			}
			index += 1;
		}
		output.push(table);
	}
	output
}

//----------------------------------------------------------------

#[repr(C)]
struct VtableHeader64 {
	drop_fn: u64,
	size: u64,
	align: u64,
}

unsafe impl dataview::Pod for VtableHeader64 {}

fn pointers64(file: pelite::pe64::PeFile<'_>) -> Vec<u32> {
	let mut pointers = match file.base_relocs() {
		Ok(relocs) => relocated_pointers64(file, relocs),
		Err(_) => scanned_pointers64(file),
	};
	pointers.sort_unstable();
	pointers.dedup();
	pointers
}

fn relocated_pointers64(file: pelite::pe64::PeFile<'_>, relocs: BaseRelocationDirectory<'_>) -> Vec<u32> {
	let mut pointers = Vec::new();
	relocs.for_each(|rva, ty| {
		// Looking for aligned pointers
		if ty != image::IMAGE_REL_BASED_DIR64 || rva % 8 != 0 {
			return;
		}
		// Inside a readonly data section
		if !file.section_headers().by_rva(rva).is_some_and(is_readonly_data) {
			return;
		}
		// Read the pointer
		let Ok(target_va) = file.derva_copy::<u64>(rva, false) else { return };
		let Ok(target_rva) = file.va_to_rva(target_va) else { return };
		// Check the pointer goes from rdata section to text section
		if !file.section_headers().by_rva(target_rva).is_some_and(is_executable) {
			return;
		}
		pointers.push(rva);
	});
	pointers
}

fn scanned_pointers64(file: pelite::pe64::PeFile<'_>) -> Vec<u32> {
	let mut pointers = Vec::new();
	for section in file.section_headers() {
		// Look inside readonly data sections
		if !is_readonly_data(section) {
			continue;
		}
		let Ok(bytes) = file.get_section_bytes(section) else {
			continue;
		};
		// Looking for aligned pointers
		let Some(words) = dataview::DataView::from(bytes).try_slice::<u64>(0, bytes.len() / 8) else {
			continue;
		};
		for (index, &target_va) in words.iter().enumerate() {
			let rva = section.VirtualAddress.wrapping_add((index * 8) as u32);
			// Read the pointer
			let Ok(target_rva) = file.va_to_rva(target_va) else {
				continue;
			};
			// Check the pointer goes from rdata section to text section
			if !file.section_headers().by_rva(target_rva).is_some_and(is_executable) {
				continue;
			}
			pointers.push(rva);
		}
	}
	pointers
}

fn parse_vtable_header64(file: pelite::pe64::PeFile<'_>, address: u32) -> Option<&'_ VtableHeader64> {
	let header = file.derva::<VtableHeader64>(address).ok()?;
	// Validate align
	let align = header.align;
	if align == 0 || align > MAX_ALIGN as u64 || !align.is_power_of_two() {
		return None;
	}
	// Validate size
	if header.size >= MAX_SIZE as u64 || header.size % align != 0 {
		return None;
	}
	// Validate drop function
	if header.drop_fn != 0 {
		let target_rva = file.va_to_rva(header.drop_fn).ok()?;
		if !file.section_headers().by_rva(target_rva).is_some_and(is_executable) {
			return None;
		}
	}
	Some(header)
}

fn static_string64(code: &[u8], function_rva: u32) -> Option<(u32, usize)> {
	// lea rax, [rip+disp32]; mov edx, length; ret
	for (lea, mov) in [(0, 7), (5, 0)] {
		if code.get(lea..lea + 3)? != [0x48, 0x8d, 0x05] || code.get(mov)? != &0xba || code.get(12)? != &0xc3 {
			continue;
		}
		let displacement = i32::from_le_bytes(code.get(lea + 3..lea + 7)?.try_into().ok()?);
		let target_rva = (function_rva as i64).checked_add(lea as i64 + 7)?.checked_add(displacement as i64)?;
		let length = u32::from_le_bytes(code.get(mov + 1..mov + 5)?.try_into().ok()?) as usize;
		return Some((target_rva.try_into().ok()?, length));
	}
	None
}

fn function_comment64(file: pelite::pe64::PeFile<'_>, slot: u32, index: usize) -> Option<String> {
	let function_rva = file.va_to_rva(file.derva_copy::<u64>(slot, false).ok()?).ok()?;
	let code = file.slice_bytes(function_rva).ok()?;
	if let Some(value) = constant_return(code, true) {
		return Some(format!("fn[{index}]: return {value:#x}"));
	}
	let (string_rva, length) = static_string64(code, function_rva)?;
	if !file.section_headers().by_rva(string_rva).is_some_and(is_readonly_data) {
		return None;
	}
	let string = valid_string(file.slice_bytes(string_rva).ok()?, length)?;
	Some(format!("fn[{index}]: return {string:?}"))
}

fn analyze64(file: pelite::pe64::PeFile<'_>) -> Vec<VTableOutput> {
	let pointers = pointers64(file);

	let mut output = Vec::new();
	let mut index = 0;
	while index < pointers.len() {
		let address = pointers[index].wrapping_sub(24);
		let Some(header) = parse_vtable_header64(file, address) else {
			index += 1;
			continue;
		};
		let mut table = VTableOutput {
			address,
			size: header.size as usize,
			align: header.align as usize,
			functions: 1,
			comments: function_comment64(file, pointers[index], 0).into_iter().collect(),
		};
		index += 1;
		while index < pointers.len() && pointers[index - 1].wrapping_add(8) == pointers[index] {
			// The next pointer may be the drop slot of an adjacent vtable.
			if parse_vtable_header64(file, pointers[index]).is_some() {
				break;
			}
			table.functions += 1;
			if let Some(comment) = function_comment64(file, pointers[index], table.functions - 1) {
				table.comments.push(comment);
			}
			index += 1;
		}
		output.push(table);
	}
	output
}

//----------------------------------------------------------------

#[test]
fn static_string32_moves() {
	assert_eq!(static_string32(&[0xb8, 0x34, 0x12, 0, 0, 0xba, 3, 0, 0, 0, 0xc3]), Some((0x1234, 3)));
	assert_eq!(static_string32(&[0xba, 3, 0, 0, 0, 0xb8, 0x34, 0x12, 0, 0, 0xc3]), Some((0x1234, 3)));
}

#[test]
fn static_string64_lea() {
	assert_eq!(static_string64(&[0x48, 0x8d, 0x05, 0x19, 0, 0, 0, 0xba, 3, 0, 0, 0, 0xc3], 0x1000), Some((0x1020, 3)));
	assert_eq!(static_string64(&[0xba, 3, 0, 0, 0, 0x48, 0x8d, 0x05, 0x14, 0, 0, 0, 0xc3], 0x1000), Some((0x1020, 3)));
}

#[test]
fn reject_invalid_strings() {
	assert_eq!(valid_string(b"hello", 6), None);
	assert_eq!(valid_string(b"\xff", 1), None);
	assert_eq!(valid_string(b"a\nb", 3), None);
}

#[test]
fn immediate_returns() {
	assert_eq!(constant_return(&[0xb0, 0xab, 0xc3], false), Some(0xab));
	assert_eq!(constant_return(&[0x66, 0xb8, 0x34, 0x12, 0xc3], false), Some(0x1234));
	assert_eq!(constant_return(&[0xb8, 0x78, 0x56, 0x34, 0x12, 0xc3], true), Some(0x1234_5678));
	assert_eq!(constant_return(&[0x48, 0xb8, 0xf0, 0xde, 0xbc, 0x9a, 0x78, 0x56, 0x34, 0x12, 0xc3], true), Some(0x1234_5678_9abc_def0));
	assert_eq!(constant_return(&[0x48, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0xc3], false), None);
	assert_eq!(constant_return(&[0xb8, 1, 0, 0, 0, 0x90, 0xc3], true), None);
}
