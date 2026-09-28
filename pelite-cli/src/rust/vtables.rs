use pelite::{FileMap, PeFile, Wrap, base_relocs::BaseRelocationDirectory, image};

use crate::*;

const MAX_ALIGN: usize = 4096;
const MAX_SIZE: usize = 1 << 31;

#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct VTableOutput {
	address: u32,
	size: u64,
	align: u64,
	functions: usize,
}

pub fn command() -> clap::Command {
	clap::Command::new("vtables")
		.about("Find candidate Rust trait vtables in a PE image")
		.after_help("Scans readable, non-writable data sections for a drop pointer, size, alignment, and one or more function pointers. Uses base relocations when present; otherwise scans aligned pointer-sized words. Results are heuristic and may include unrelated pointer tables. The vtable RVA points to the drop-pointer slot.")
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
				writeln!(writer, "{name}!{:#010x} size={} align={} fnptrs={}", item.address, item.size, item.align, item.functions)?;
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
		let Ok(target_va) = file.derva_copy::<u32>(rva) else { return };
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
			size: header.size as u64,
			align: header.align as u64,
			functions: 1,
		};
		index += 1;
		while index < pointers.len() && pointers[index - 1].wrapping_add(4) == pointers[index] {
			// The next pointer may be the drop slot of an adjacent vtable.
			if parse_vtable_header32(file, pointers[index]).is_some() {
				break;
			}
			table.functions += 1;
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
		let Ok(target_va) = file.derva_copy::<u64>(rva) else { return };
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
			size: header.size,
			align: header.align,
			functions: 1,
		};
		index += 1;
		while index < pointers.len() && pointers[index - 1].wrapping_add(8) == pointers[index] {
			// The next pointer may be the drop slot of an adjacent vtable.
			if parse_vtable_header64(file, pointers[index]).is_some() {
				break;
			}
			table.functions += 1;
			index += 1;
		}
		output.push(table);
	}
	output
}
