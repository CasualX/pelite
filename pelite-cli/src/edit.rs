use super::*;

mod fix_section_names;

pub fn command() -> clap::Command {
	clap::Command::new("edit")
		.about("Edit a PE file in place")
		.arg(summary::file_arg().required(true))
		.arg(clap::Arg::new("fix-baserelocs")
			.long("fix-baserelocs")
			.action(clap::ArgAction::SetTrue)
			.help("Set the base relocation directory to the .reloc section"))
		.arg(clap::Arg::new("fix-section-names")
			.long("fix-section-names")
			.action(clap::ArgAction::SetTrue)
			.help("Replace section names with unique names inferred from their contents and permissions"))
		.arg(clap::Arg::new("image")
			.long("image")
			.visible_alias("raw")
			.action(clap::ArgAction::SetTrue)
			.help("Convert a memory image to file layout by using section virtual offsets and sizes"))
}

pub fn run(matches: &clap::ArgMatches) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let mut bytes = pelite::PeMemory::open(path)?;

	if matches.get_flag("image") {
		convert_image(&mut bytes)?;
	}
	if matches.get_flag("fix-baserelocs") {
		fix_baserelocs(&mut bytes)?;
	}

	if matches.get_flag("fix-section-names") {
		fix_section_names::run(&mut bytes)?;
	}

	fs::write(path, &bytes)?;
	Ok(())
}

fn align_to(value: u32, align: u32) -> u32 {
	value.wrapping_add(align - 1) & (align - 1)
}

fn convert_image(bytes: &mut pelite::PeMemory) -> Result {
	let pe = pelite::PeFile::from_bytes(&*bytes)?;
	let pe_sections = pe.section_headers().image();
	let mut sections_mut = pe_sections.to_vec();
	let sections_mut = sections_mut.as_mut_slice();

	let file_alignment = match pe.optional_header() {
		pelite::Wrap::T32(h) => h.FileAlignment,
		pelite::Wrap::T64(h) => h.FileAlignment,
	};
	eprintln!("FileAlignment={file_alignment:#x}");
	if !file_alignment.is_power_of_two() {
		return Err(err("invalid file alignment"));
	}

	for (index, section) in sections_mut.iter_mut().enumerate() {
		let raw_size = align_to(section.VirtualSize, file_alignment);
		let end = section.VirtualAddress as usize + raw_size as usize;
		if end > bytes.len() {
			eprintln!("Invalid: Name={:?} VirtualAddress={:#x}, VirtualSize={:#x}", pe_sections[index].name(), section.VirtualAddress, section.VirtualSize)
		}
		section.SizeOfRawData = raw_size;
		section.PointerToRawData = section.VirtualAddress;
	}

	let offset = pe.offset_of(pe_sections);
	bytes.view_mut().write(offset, sections_mut);
	Ok(())
}

fn fix_baserelocs(bytes: &mut pelite::PeMemory) -> Result {
	let pe = pelite::PeFile::from_bytes(&bytes)?;

	let section = pe.section_headers().by_name(".reloc")
		.ok_or_else(|| err("no .reloc section found"))?;

	let directory = pe.data_directory().get(pelite::image::IMAGE_DIRECTORY_ENTRY_BASERELOC)
		.ok_or_else(|| err("base relocation directory entry is missing"))?;

	let size = if section.VirtualSize != 0 { section.VirtualSize } else { section.SizeOfRawData };
	if size == 0 {
		return Err(err(".reloc section is empty"));
	}

	pe.get_section_bytes(section)
		.map_err(|_| err(".reloc section data is outside the file"))?;

	let offset = pe.offset_of(directory);
	let reloc_dir = pelite::image::IMAGE_DATA_DIRECTORY {
		VirtualAddress: section.VirtualAddress,
		Size: size,
	};

	bytes.view_mut().write(offset, &reloc_dir);
	Ok(())
}
