use pelite::image;

use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AddressLayout {
	None,
	Indent,
	Rva,
	Va,
}

impl AddressLayout {
	fn parse(value: &str) -> std::result::Result<Self, String> {
		match value {
			"none" => Ok(Self::None),
			"indent" => Ok(Self::Indent),
			"rva" => Ok(Self::Rva),
			"va" => Ok(Self::Va),
			_ => Err("expected none, indent, rva, or va".to_owned()),
		}
	}
}

#[derive(serde::Serialize)]
struct DisassembledInstruction<'a> {
	address: u32,
	bytes: &'a [u8],
	instruction: String,
}

pub fn command() -> clap::Command {
	clap::Command::new("disasm")
		.about("Disassemble an address range using iced-x86")
		.after_help(include_str!("docs/disasm.md"))
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("range")
			.value_name("RANGE")
			.value_parser(AddressRange::parse)
			.required(true))
		.arg(clap::Arg::new("arch")
			.long("arch")
			.value_name("ARCH")
			.value_parser(Arch::parse)
			.help("Override the PE machine header (x86_16, x86_32 [alias x86], or x86_64)"))
		.arg(clap::Arg::new("hex")
			.long("hex")
			.action(clap::ArgAction::SetTrue)
			.help("Show instruction opcode bytes in text output"))
		.arg(clap::Arg::new("layout")
			.long("layout")
			.value_name("MODE")
			.value_parser(AddressLayout::parse)
			.default_value("indent")
			.help("Choose plain, indented, RVA, or VA text layout"))
		.arg(clap::Arg::new("lookback")
			.long("lookback")
			.value_name("BYTES")
			.value_parser(clap::value_parser!(u32))
			.default_value("0")
			.help("Decode up to BYTES earlier (decimal); include any instruction overlapping the requested start. Correct alignment requires starting at an instruction boundary"))
		.arg(symbols::arg())
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let range = *matches.get_one::<AddressRange>("range").expect("required by clap");
	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let range = range.to_rva(pe)?;
	let bitness = if let Some(arch) = matches.get_one::<Arch>("arch") {
		arch.bitness()
	}
	else {
		match pe.file_header().Machine {
			image::IMAGE_FILE_MACHINE_I386 => Arch::X86_32.bitness(),
			image::IMAGE_FILE_MACHINE_AMD64 => Arch::X86_64.bitness(),
			machine => return Err(err(format!("unsupported machine type {machine:#06x}; expected i386 or AMD64"))),
		}
	};
	let len = usize::try_from(range.end - range.start)?;
	let bytes = pe.slice(range.start, len, 1)?;
	let lookback = *matches.get_one::<u32>("lookback").expect("defaulted by clap");
	let (decode_start, bytes) = pe.section_headers().by_rva(range.start)
		.filter(|_| lookback != 0)
		.and_then(|section| {
			let start = range.start.saturating_sub(lookback).max(section.VirtualAddress);
			let len = usize::try_from(range.end - start).ok()?;
			pe.slice(start, len, 1).ok().map(|bytes| (start, bytes))
		})
		.unwrap_or((range.start, bytes));
	let image_base = pe.image_base();
	let start_ip = image_base + range.start as u64;
	let decode_ip = image_base + decode_start as u64;
	let end_ip = image_base + range.end as u64;

	let color = format == OutputFormat::Text && io::stdout().is_terminal();
	let facts = symbols::load(matches, ty::PointerWidth::from(pe), image_base, Some(pe))?;
	let symbols = Arc::new(facts.symbols);
	let mut decoded = iced::decode_bytes(bytes, bitness, decode_ip, start_ip, end_ip, color, Arc::clone(&symbols));
	iced::append_comments(&mut decoded, &facts.comments);
	match format {
		OutputFormat::Nul => Ok(()),
		OutputFormat::Json | OutputFormat::JsonPretty => {
			let mut instructions = Vec::with_capacity(decoded.len());
			for item in decoded {
				instructions.push(DisassembledInstruction {
					address: u32::try_from(item.ip - image_base)?,
					bytes: item.bytes,
					instruction: item.instruction,
				});
			}
			print_json(&instructions, format == OutputFormat::JsonPretty)
		},
		OutputFormat::Text => {
			let layout = *matches.get_one::<AddressLayout>("layout").expect("defaulted by clap");
			let stdout = io::stdout();
			iced::print_text(&mut stdout.lock(), &decoded, &symbols, color, matches.get_flag("hex"), |ip| {
				let rva = (ip - image_base) as u32;
				match layout {
					AddressLayout::None => iced::TextPrefix::None,
					AddressLayout::Indent => iced::TextPrefix::Indent,
					AddressLayout::Rva => iced::TextPrefix::Address { label: get_section_name_by_rva(pe, rva), value: u64::from(rva) },
					AddressLayout::Va => iced::TextPrefix::Address { label: get_section_name_by_rva(pe, rva), value: ip },
				}
			})?;
			Ok(())
		},
	}
}
