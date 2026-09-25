use pelite::{base_relocs::BaseRelocationDirectory, image, PeFile, Wrap};

use super::*;

const fn is_zero(&value: &u8) -> bool {
	value == 0
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, serde::Serialize)]
enum ReferenceKind {
	PointerRelocation,
	PointerRawScan,
	RelativeDisp32,
}

#[derive(serde::Serialize)]
struct Reference<'a> {
	rva: u32,
	file_offset: Option<usize>,
	section: Option<&'a str>,
	kind: ReferenceKind,
	#[serde(skip_serializing_if = "is_zero")]
	trailing_bytes: u8,
}

#[derive(serde::Serialize)]
struct Xrefs<'a> {
	target_rva: u32,
	target_va: u64,
	references: Vec<Reference<'a>>,
}

pub fn command() -> clap::Command {
	clap::Command::new("xref")
		.about("Find absolute pointers and relative code references to an address")
		.after_help("Addresses are hexadecimal RVAs, VAs, or file offsets (for example, rva:1000). Relative matches are candidates: four-byte displacements with 0, 1, or 4 trailing bytes are checked without decoding instructions.")
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("address")
			.value_name("rva:HEX|va:HEX|fo:HEX")
			.value_parser(Address::parse)
			.required(true))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let address = *matches.get_one::<Address>("address").expect("required by clap");
	let map = pelite::FileMap::open(path)?;
	let pe = PeFile::from_bytes(&map)?;
	let target_rva = match address {
		Address::Rva(rva) => rva,
		Address::Va(va) => pe.va_to_rva(va)?,
		Address::Fo(fo) => pe.headers().file_offset_to_rva(fo)?,
	};
	let target_va = pe.rva_to_va(target_rva)?;
	let mut references = Vec::new();
	match pe {
		Wrap::T32(_) => search32(pe, target_va as u32, &mut references),
		Wrap::T64(_) => search64(pe, target_va, &mut references),
	}
	search_relative_displacements(pe, target_rva, &mut references);
	references.sort_unstable_by_key(|item| (item.rva, item.file_offset, item.kind, item.trailing_bytes));
	references.dedup_by(|a, b| (a.rva, a.file_offset, a.kind, a.trailing_bytes) == (b.rva, b.file_offset, b.kind, b.trailing_bytes));
	printer::print("Cross references", &Xrefs { target_rva, target_va, references }, format)
}

fn search32<'a>(pe: PeFile<'a>, target_va: u32, references: &mut Vec<Reference<'a>>) {
	if let Ok(relocs) = pe.base_relocs() {
		search_relocations32(pe, relocs, target_va, references);
	}
	else {
		search_pointers32(pe, target_va, references);
	}
}

fn search64<'a>(pe: PeFile<'a>, target_va: u64, references: &mut Vec<Reference<'a>>) {
	if let Ok(relocs) = pe.base_relocs() {
		search_relocations64(pe, relocs, target_va, references);
	}
	else {
		search_pointers64(pe, target_va, references);
	}
}

fn search_relocations32<'a>(pe: PeFile<'a>, relocs: BaseRelocationDirectory<'_>, target_va: u32, references: &mut Vec<Reference<'a>>) {
	relocs.for_each(|rva, ty| {
		if ty != image::IMAGE_REL_BASED_HIGHLOW {
			return;
		}
		if let Ok(candidate_va) = pe.derva_copy::<u32>(rva) {
			if candidate_va == target_va {
				let offset = pe.headers().rva_to_file_offset(rva).ok();
				references.push(reference(pe, rva, offset, ReferenceKind::PointerRelocation, 0));
			}
		}
	});
}

fn search_relocations64<'a>(pe: PeFile<'a>, relocs: BaseRelocationDirectory<'_>, target_va: u64, references: &mut Vec<Reference<'a>>) {
	relocs.for_each(|rva, ty| {
		if ty != image::IMAGE_REL_BASED_DIR64 {
			return;
		}
		if let Ok(candidate_va) = pe.derva_copy::<u64>(rva) {
			if candidate_va == target_va {
				let offset = pe.headers().rva_to_file_offset(rva).ok();
				references.push(reference(pe, rva, offset, ReferenceKind::PointerRelocation, 0));
			}
		}
	});
}

fn search_pointers32<'a>(pe: PeFile<'a>, target_va: u32, references: &mut Vec<Reference<'a>>) {
	let needle = target_va.to_le_bytes();
	for (offset, bytes) in pe.image().windows(4).enumerate() {
		if bytes == needle {
			if let Some(rva) = mapped_rva(pe, offset, 4) {
				references.push(reference(pe, rva, Some(offset), ReferenceKind::PointerRawScan, 0));
			}
		}
	}
}

fn search_pointers64<'a>(pe: PeFile<'a>, target_va: u64, references: &mut Vec<Reference<'a>>) {
	let needle = target_va.to_le_bytes();
	for (offset, bytes) in pe.image().windows(8).enumerate() {
		if bytes == needle {
			if let Some(rva) = mapped_rva(pe, offset, 8) {
				references.push(reference(pe, rva, Some(offset), ReferenceKind::PointerRawScan, 0));
			}
		}
	}
}

fn mapped_rva(pe: PeFile<'_>, offset: usize, pointer_width: usize) -> Option<u32> {
	let last = offset.checked_add(pointer_width.checked_sub(1)?)?;
	let rva = pe.headers().file_offset_to_rva(offset).ok()?;
	let last_rva = pe.headers().file_offset_to_rva(last).ok()?;
	let expected_last = u32::try_from(pointer_width - 1).ok().and_then(|len| rva.checked_add(len))?;
	(last_rva == expected_last && pe.headers().rva_to_file_offset(rva).ok() == Some(offset)).then_some(rva)
}

fn search_relative_displacements<'a>(pe: PeFile<'a>, target_rva: u32, references: &mut Vec<Reference<'a>>) {
	for section in pe.section_headers() {
		if section.Characteristics & (image::IMAGE_SCN_CNT_CODE | image::IMAGE_SCN_MEM_EXECUTE) == 0 {
			continue;
		}
		let Ok(bytes) = pe.get_section_bytes(section) else {
			continue;
		};
		// let bytes = &bytes[..bytes.len().min(section.VirtualSize as usize)];
		relative_matches(pe, bytes, section.VirtualAddress, target_rva, references);
	}
}

fn relative_matches<'a>(pe: PeFile<'a>, bytes: &[u8], section_rva: u32, target_rva: u32, references: &mut Vec<Reference<'a>>) {
	for (offset, field) in bytes.windows(4).enumerate() {
		let Some(field_rva) = u32::try_from(offset).ok().and_then(|offset| section_rva.checked_add(offset)) else {
			continue;
		};
		let displacement = i32::from_le_bytes(field.try_into().unwrap()) as i64;
		for trailing in [0u8, 1, 4] {
			if offset + 4 + trailing as usize > bytes.len() {
				continue;
			}
			let end = field_rva as i64 + 4 + trailing as i64;
			if end + displacement == target_rva as i64 {
				references.push(reference(pe, field_rva, pe.headers().rva_to_file_offset(field_rva).ok(), ReferenceKind::RelativeDisp32, trailing));
			}
		}
	}
}

fn reference<'a>(pe: PeFile<'a>, rva: u32, file_offset: Option<usize>, kind: ReferenceKind, trailing_bytes: u8) -> Reference<'a> {
	let section = pe.section_headers().by_rva(rva).and_then(|section| section.name().ok());
	Reference { rva, file_offset, section, kind, trailing_bytes }
}
