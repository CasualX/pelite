use std::io::Read;

use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AddressLayout {
	None,
	Indent,
	FileOffset,
	Va,
}

impl AddressLayout {
	fn parse(value: &str) -> std::result::Result<Self, String> {
		match value {
			"none" => Ok(Self::None),
			"indent" => Ok(Self::Indent),
			"fo" => Ok(Self::FileOffset),
			"va" => Ok(Self::Va),
			_ => Err("expected none, indent, fo, or va".to_owned()),
		}
	}
}

#[derive(serde::Serialize)]
struct RawInstruction<'a> {
	offset: usize,
	ip: u64,
	bytes: &'a [u8],
	instruction: String,
}

fn parse_number(value: &str) -> std::result::Result<u64, String> {
	if let Some(hex) = value.strip_prefix("0x").or_else(|| value.strip_prefix("0X")) {
		u64::from_str_radix(hex, 16).map_err(|error| error.to_string())
	}
	else {
		value.parse::<u64>().map_err(|error| error.to_string())
	}
}

pub fn command() -> clap::Command {
	clap::Command::new("disasm-raw")
		.about("Disassemble raw x86 bytes without parsing a PE file")
		.after_help(include_str!("../docs/re-disasm-raw.md"))
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.required(true)
			.help("Raw binary file, or - for standard input"))
		.arg(clap::Arg::new("arch")
			.long("arch")
			.value_name("ARCH")
			.value_parser(Arch::parse)
			.required(true)
			.help("Instruction set and mode (x86_16, x86_32 [alias x86], or x86_64)"))
		.arg(clap::Arg::new("offset")
			.long("offset")
			.value_name("BYTES")
			.value_parser(parse_number)
			.default_value("0")
			.help("Start decoding at this file offset"))
		.arg(clap::Arg::new("length")
			.long("length")
			.value_name("BYTES")
			.value_parser(parse_number)
			.help("Number of bytes to disassemble (default: through end of file)"))
		.arg(clap::Arg::new("base")
			.long("base")
			.value_name("ADDRESS")
			.value_parser(parse_number)
			.default_value("0")
			.help("Instruction address corresponding to file offset zero"))
		.arg(clap::Arg::new("lookback")
			.long("lookback")
			.value_name("BYTES")
			.value_parser(clap::value_parser!(usize))
			.default_value("0")
			.help("Decode up to BYTES before --offset; include an instruction overlapping the start"))
		.arg(clap::Arg::new("hex")
			.long("hex")
			.action(clap::ArgAction::SetTrue)
			.help("Show instruction opcode bytes in text output"))
		.arg(clap::Arg::new("layout")
			.long("layout")
			.value_name("MODE")
			.value_parser(AddressLayout::parse)
			.default_value("indent")
			.help("Choose plain, indented, file offset, or virtual address text layout"))
		.arg(symbols::arg())
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<String>("file").expect("required by clap");
	let mut bytes = Vec::new();
	if path == "-" {
		io::stdin().read_to_end(&mut bytes)?;
	}
	else {
		bytes = fs::read(path)?;
	}
	let bitness = matches.get_one::<Arch>("arch").expect("required by clap").bitness();
	let offset = usize::try_from(*matches.get_one::<u64>("offset").expect("defaulted by clap"))?;
	if offset > bytes.len() {
		return Err(err(format!("offset {offset:#x} exceeds input length {:#x}", bytes.len())));
	}
	let end = if let Some(length) = matches.get_one::<u64>("length") {
		offset.checked_add(usize::try_from(*length)?).ok_or_else(|| err("offset plus length overflows"))?
	}
	else {
		bytes.len()
	};
	if end > bytes.len() {
		return Err(err(format!("range ends at {end:#x}, beyond input length {:#x}", bytes.len())));
	}
	let lookback = *matches.get_one::<usize>("lookback").expect("defaulted by clap");
	let decode_start = offset.saturating_sub(lookback);
	let base = *matches.get_one::<u64>("base").expect("defaulted by clap");
	let decode_ip = base.checked_add(decode_start as u64).ok_or_else(|| err("base plus decode offset overflows"))?;
	let start_ip = base.checked_add(offset as u64).ok_or_else(|| err("base plus offset overflows"))?;
	let end_ip = base.checked_add(end as u64).ok_or_else(|| err("base plus end offset overflows"))?;
	let color = format == OutputFormat::Text && io::stdout().is_terminal();
	let pointer_width = if bitness == 64 { ty::PointerWidth::Bits64 } else { ty::PointerWidth::Bits32 };
	let symbols = Arc::new(symbols::load(matches, pointer_width, base)?);
	let decoded = iced::decode_bytes(&bytes[decode_start..end], bitness, decode_ip, start_ip, end_ip, color, Arc::clone(&symbols));
	match format {
		OutputFormat::Nul => Ok(()),
		OutputFormat::Json | OutputFormat::JsonPretty => {
			let instructions: Vec<_> = decoded.into_iter().map(|item| RawInstruction {
				offset: decode_start + (item.ip - decode_ip) as usize,
				ip: item.ip,
				bytes: item.bytes,
				instruction: item.instruction,
			}).collect();
			print_json(&instructions, format == OutputFormat::JsonPretty)
		},
		OutputFormat::Text => {
			let layout = *matches.get_one::<AddressLayout>("layout").expect("defaulted by clap");
			let stdout = io::stdout();
			iced::print_text(&mut stdout.lock(), &decoded, &symbols, color, matches.get_flag("hex"), |ip| match layout {
				AddressLayout::None => iced::TextPrefix::None,
				AddressLayout::Indent => iced::TextPrefix::Indent,
				AddressLayout::FileOffset => iced::TextPrefix::Address { label: "fo", value: ip - base },
				AddressLayout::Va => iced::TextPrefix::Address { label: "va", value: ip },
			})?;
			Ok(())
		},
	}
}
