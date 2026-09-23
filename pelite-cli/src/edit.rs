use pelite::{PeMemory, PeFile, image};

use super::*;

pub fn command() -> clap::Command {
	clap::Command::new("edit")
		.about("Edit a PE file in place")
		.arg(summary::file_arg().required(true))
		.arg(clap::Arg::new("fix-baserelocs")
			.long("fix-baserelocs")
			.action(clap::ArgAction::SetTrue)
			.help("Set the base relocation directory to the .reloc section"))
		.arg(clap::Arg::new("image")
			.long("image")
			.visible_alias("raw")
			.action(clap::ArgAction::SetTrue)
			.help("Convert a memory image to file layout by using section virtual offsets and sizes"))
}

pub fn run(matches: &clap::ArgMatches) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let mut bytes = PeMemory::open(path)?;

	if matches.get_flag("image") {
		convert_image(&mut bytes)?;
	}
	if matches.get_flag("fix-baserelocs") {
		fix_baserelocs(&mut bytes)?;
	}

	fs::write(path, &bytes)?;
	Ok(())
}

fn convert_image(bytes: &mut PeMemory) -> Result {
	let pe = PeFile::from_bytes(&*bytes)?;
	let mut sections = pe.section_headers().iter().map(|section| **section).collect::<Vec<_>>();
	let sections = sections.as_mut_slice();

	for section in &mut *sections {
		section.SizeOfRawData = section.VirtualSize;
		section.PointerToRawData = section.VirtualAddress;
	}

	let offset = pe.offset_of(sections).unwrap();
	bytes.view_mut().write(offset, sections);
	Ok(())
}

fn fix_baserelocs(bytes: &mut PeMemory) -> Result {
	let pe = PeFile::from_bytes(&bytes)?;

	let section = pe.section_headers().by_name(".reloc")
		.ok_or_else(|| err("no .reloc section found"))?;

	let directory = pe.data_directory().get(image::IMAGE_DIRECTORY_ENTRY_BASERELOC)
		.ok_or_else(|| err("base relocation directory entry is missing"))?;

	let size = if section.VirtualSize != 0 { section.VirtualSize } else { section.SizeOfRawData };
	if size == 0 {
		return Err(err(".reloc section is empty"));
	}

	pe.get_section_bytes(section)
		.map_err(|_| err(".reloc section data is outside the file"))?;

	let offset = pe.offset_of(directory)
		.ok_or_else(|| err("invalid offset"))?;
	let reloc_dir = pelite::image::IMAGE_DATA_DIRECTORY {
		VirtualAddress: section.VirtualAddress,
		Size: size,
	};

	bytes.view_mut().write(offset, &reloc_dir);
	Ok(())
}
