use super::*;

pub fn run(bytes: &mut pelite::PeMemory) -> Result {
	let pe = pelite::PeFile::from_bytes(&*bytes)?;
	let headers = pe.section_headers().image();
	let mut sections = headers.to_vec();

	let mut used = HashSet::new();
	let mut counts = HashMap::new();
	for section in &mut sections {
		let base = section_name(section, pe.data_directory());
		section.Name = unique_section_name(base, &mut counts, &mut used);
	}
	let offset = pe.offset_of(headers);
	bytes.view_mut().write(offset, sections.as_slice());
	Ok(())
}

fn section_name(section: &pelite::image::IMAGE_SECTION_HEADER, directories: &[pelite::image::IMAGE_DATA_DIRECTORY]) -> &'static str {
	use pelite::image::*;

	let size = if section.VirtualSize != 0 { section.VirtualSize } else { section.SizeOfRawData };
	// A directory embedded in a larger section is not enough to identify that section.
	for (index, name) in [
		(IMAGE_DIRECTORY_ENTRY_BASERELOC, ".reloc"),
		(IMAGE_DIRECTORY_ENTRY_EXCEPTION, ".pdata"),
		(IMAGE_DIRECTORY_ENTRY_RESOURCE, ".rsrc"),
		(IMAGE_DIRECTORY_ENTRY_IMPORT, ".idata"),
		(IMAGE_DIRECTORY_ENTRY_EXPORT, ".edata"),
	] {
		let Some(directory) = directories.get(index) else {
			continue;
		};
		if directory.VirtualAddress != 0 && directory.VirtualAddress == section.VirtualAddress && directory.Size != 0 && directory.Size <= size {
			return name;
		}
	}

	let flags = section.Characteristics;
	if flags & IMAGE_SCN_MEM_EXECUTE != 0 {
		if flags & IMAGE_SCN_MEM_WRITE != 0 { ".rwe" } else { ".text" }
	}
	else if section.SizeOfRawData == 0 && flags & IMAGE_SCN_CNT_UNINITIALIZED_DATA != 0 {
		".bss"
	}
	else if flags & IMAGE_SCN_MEM_WRITE != 0 {
		".data"
	}
	else if flags & IMAGE_SCN_MEM_READ != 0 {
		".rdata"
	}
	else {
		".sect"
	}
}

fn unique_section_name(base: &'static str, counts: &mut HashMap<&'static str, usize>, used: &mut HashSet<[u8; 8]>) -> [u8; 8] {
	let count = counts.entry(base).or_default();
	loop {
		let suffix = if *count == 0 { String::new() } else { count.to_string() };
		*count += 1;
		// Keep even large numeric suffixes inside the eight-byte name field.
		let prefix = &base.as_bytes()[..base.len().min(8 - suffix.len())];
		let mut name = [0; 8];
		name[..prefix.len()].copy_from_slice(prefix);
		name[prefix.len()..prefix.len() + suffix.len()].copy_from_slice(suffix.as_bytes());
		if used.insert(name) {
			return name;
		}
	}
}

#[test]
fn section_name_heuristics() {
	use pelite::image::*;

	let mut section = IMAGE_SECTION_HEADER {
		Name: [0xff; 8], VirtualSize: 0x100, VirtualAddress: 0x1000,
		SizeOfRawData: 0x200, PointerToRawData: 0,
		PointerToRelocations: 0, PointerToLinenumbers: 0,
		NumberOfRelocations: 0, NumberOfLinenumbers: 0, Characteristics: 0,
	};
	for (flags, name) in [
		(IMAGE_SCN_MEM_READ | IMAGE_SCN_MEM_EXECUTE, ".text"),
		(IMAGE_SCN_MEM_READ | IMAGE_SCN_MEM_WRITE, ".data"),
		(IMAGE_SCN_MEM_READ, ".rdata"),
		(IMAGE_SCN_MEM_READ | IMAGE_SCN_MEM_WRITE | IMAGE_SCN_MEM_EXECUTE, ".rwe"),
		(0, ".sect"),
	] {
		section.Characteristics = flags;
		assert_eq!(section_name(&section, &[]), name);
	}
	section.Characteristics = IMAGE_SCN_MEM_READ;
	let mut dirs = [IMAGE_DATA_DIRECTORY { VirtualAddress: 0, Size: 0 }; 16];
	for (index, name) in [
		(IMAGE_DIRECTORY_ENTRY_BASERELOC, ".reloc"),
		(IMAGE_DIRECTORY_ENTRY_EXCEPTION, ".pdata"),
		(IMAGE_DIRECTORY_ENTRY_RESOURCE, ".rsrc"),
		(IMAGE_DIRECTORY_ENTRY_IMPORT, ".idata"),
		(IMAGE_DIRECTORY_ENTRY_EXPORT, ".edata"),
	] {
		dirs[index] = IMAGE_DATA_DIRECTORY { VirtualAddress: 0x1000, Size: 0x100 };
		assert_eq!(section_name(&section, &dirs), name);
		dirs[index].VirtualAddress += 4;
		assert_eq!(section_name(&section, &dirs), ".rdata");
		dirs[index] = IMAGE_DATA_DIRECTORY { VirtualAddress: 0x1000, Size: 0x101 };
		assert_eq!(section_name(&section, &dirs), ".rdata");
		dirs[index] = IMAGE_DATA_DIRECTORY { VirtualAddress: 0, Size: 0 };
	}
	section.SizeOfRawData = 0;
	section.Characteristics |= IMAGE_SCN_CNT_UNINITIALIZED_DATA;
	assert_eq!(section_name(&section, &dirs), ".bss");
}

#[test]
fn section_names_fit_and_remain_unique() {
	let mut counts = HashMap::new();
	let mut used = HashSet::new();
	assert_eq!(unique_section_name(".rdata", &mut counts, &mut used), *b".rdata\0\0");
	assert_eq!(unique_section_name(".rdata", &mut counts, &mut used), *b".rdata1\0");
	for _ in 2..u16::MAX {
		let name = unique_section_name(".rdata", &mut counts, &mut used);
		assert!(std::str::from_utf8(&name).is_ok());
	}
	assert_eq!(used.len(), u16::MAX as usize);
}
