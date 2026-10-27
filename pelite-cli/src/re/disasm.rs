use super::*;

pub fn command() -> clap::Command {
	clap::Command::new("disasm")
		.about("Disassemble bytes or trace to the first control transfer using iced-x86")
		.after_help(include_str!("docs/disasm.md"))
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("address")
			.value_name("ADDRESS")
			.value_parser(Address::parse)
			.required(true))
		.arg(clap::Arg::new("length")
			.value_name("BYTES")
			.value_parser(value_parser::parse_usize)
			.required_unless_present("trace")
			.conflicts_with("trace")
			.help("Number of bytes to disassemble (decimal or 0xhex)"))
		.arg(clap::Arg::new("trace")
			.long("trace")
			.action(clap::ArgAction::SetTrue)
			.conflicts_with("lookback")
			.help("Disassemble through the first control transfer, including that instruction"))
		.arg(clap::Arg::new("trace-limit")
			.long("trace-limit")
			.value_name("N")
			.value_parser(value_parser::parse_usize)
			.default_value("256")
			.requires("trace")
			.conflicts_with("length")
			.help("With --trace, stop after N instructions; 0 disables the limit"))
		.arg(clap::Arg::new("arch")
			.long("arch")
			.value_name("ARCH")
			.value_parser(clap::value_parser!(Arch))
			.help("Override the PE machine header (x86_16, x86_32 [alias x86], or x86_64)"))
		.arg(clap::Arg::new("lookback")
			.long("lookback")
			.value_name("BYTES")
			.value_parser(value_parser::parse_u32)
			.default_value("0")
			.help("Decode up to BYTES earlier (decimal or 0x hex); include any instruction overlapping the requested start. Correct alignment requires starting at an instruction boundary"))
		.arg(clap::Arg::new("hex")
			.long("hex")
			.action(clap::ArgAction::SetTrue)
			.help("Include instruction opcode bytes in text and JSON output"))
		.arg(iced::layout_arg(false))
		.arg(symbols::arg())
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let address = matches.get_one::<Address>("address").expect("required by clap");
	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let facts = symbols::load(matches, pointer_width(pe), pe.image_base(), Some(pe))?;
	let start = address.to_rva(pe, &facts)?;
	let arch = get_arch(matches, pe)?;
	let length = if matches.get_flag("trace") {
		let limit = *matches.get_one::<usize>("trace-limit").expect("defaulted by clap");
		let ip = pe.image_base() + start as u64;
		let bytes = pe.slice_bytes(start)?;
		iced::trace_bytes(bytes, arch.bitness(), ip, limit)?
	}
	else {
		*matches.get_one::<usize>("length").expect("required by clap")
	};
	let lookback = *matches.get_one::<u32>("lookback").expect("defaulted by clap");
	disassemble(matches, format, pe, start, length, arch, lookback, facts)
}

fn disassemble(
	matches: &clap::ArgMatches, format: OutputFormat, pe: pelite::PeFile<'_>,
	rva: u32, len: usize, arch: Arch, lookback: u32,
	facts: symbols::IndexedFacts,
) -> Result {
	let end = u32::try_from(len).ok().and_then(|len| rva.checked_add(len))
		.ok_or_else(|| err("RVA plus length overflows"))?;
	let bytes = pe.slice(rva, len, 1)?;
	let (decode_start, bytes) = pe.section_headers().by_rva(rva)
		.filter(|_| lookback != 0 && len != 0)
		.and_then(|section| {
			let start = rva.saturating_sub(lookback).max(section.VirtualAddress);
			let len = usize::try_from(end - start).ok()?;
			pe.slice(start, len, 1).ok().map(|bytes| (start, bytes))
		})
		.unwrap_or((rva, bytes));
	// An x86 instruction starting before end can consume up to 14 bytes beyond it.
	let decode_len = if len == 0 { 0 } else { ((end - decode_start) as usize).saturating_add(14) };
	let bytes = &bytes[..bytes.len().min(decode_len)];
	let image_base = pe.image_base();
	let start_ip = image_base + rva as u64;
	let decode_ip = image_base + decode_start as u64;
	let end_ip = image_base + end as u64;

	let prefix = get_section_name_by_rva(pe, rva);
	let mut options = iced::DecodeOptions::new(format, prefix, facts);
	let (relocations, relocation_width) = match pe {
		pelite::Wrap::T32(pe) => (load_relocs32(pe, decode_start, bytes.len()), 4),
		pelite::Wrap::T64(pe) => (load_relocs64(pe, decode_start, bytes.len()), 8),
	};
	options.relocations = relocations;
	options.relocation_width = relocation_width;
	let mut decoded = iced::decode_bytes(bytes, arch.bitness(), decode_ip, start_ip, end_ip, options);
	if !matches.get_flag("hex") {
		iced::omit_bytes(&mut decoded);
	}
	let layout = if format == OutputFormat::Text && !decoded.is_empty() {
		match matches.get_one::<iced::AddressLayout>("layout").expect("defaulted by clap") {
			iced::AddressLayout::None => iced::TextLayout::None,
			iced::AddressLayout::Indent => iced::TextLayout::Indent,
			iced::AddressLayout::Rva => iced::TextLayout::Address { prefix, offset: image_base },
			iced::AddressLayout::Va => iced::TextLayout::Address { prefix, offset: 0 },
			iced::AddressLayout::FileOffset => iced::TextLayout::Address {
				prefix: "fo", offset: start_ip.wrapping_sub(pe.headers().rva_to_file_offset(rva)? as u64),
			},
		}
	}
	else {
		iced::TextLayout::None
	};
	iced::print(&decoded, format, layout)
}

/// Collect offsets of complete HIGHLOW fields within the decode slice.
fn load_relocs32(pe: pelite::pe32::PeFile<'_>, start: u32, len: usize) -> BTreeSet<usize> {
	let mut result = BTreeSet::new();
	let Ok(relocs) = pe.base_relocs() else { return result };
	relocs.for_each(|rva, ty| {
		if ty != pelite::image::IMAGE_REL_BASED_HIGHLOW {
			return;
		}
		let Some(offset) = rva.checked_sub(start) else { return };
		let offset = offset as usize;
		if offset.checked_add(4).is_some_and(|end| end <= len) {
			result.insert(offset);
		}
	});
	result
}

/// Collect offsets of complete DIR64 fields within the decode slice.
fn load_relocs64(pe: pelite::pe64::PeFile<'_>, start: u32, len: usize) -> BTreeSet<usize> {
	let mut result = BTreeSet::new();
	let Ok(relocs) = pe.base_relocs() else { return result };
	relocs.for_each(|rva, ty| {
		if ty != pelite::image::IMAGE_REL_BASED_DIR64 {
			return;
		}
		let Some(offset) = rva.checked_sub(start) else { return };
		let offset = offset as usize;
		if offset.checked_add(8).is_some_and(|end| end <= len) {
			result.insert(offset);
		}
	});
	result
}
