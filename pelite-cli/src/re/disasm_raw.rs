use std::io::Read;

use super::*;

pub fn command() -> clap::Command {
	clap::Command::new("disasm-raw")
		.about("Disassemble raw bytes or trace to the first control transfer using iced-x86")
		.after_help(include_str!("docs/disasm-raw.md"))
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.required(true)
			.help("Raw binary file, or - for standard input"))
		.arg(clap::Arg::new("offset")
			.long("offset")
			.value_name("BYTES")
			.value_parser(value_parser::parse_u64)
			.default_value("0")
			.help("Start decoding at this file offset"))
		.arg(clap::Arg::new("length")
			.long("length")
			.value_name("BYTES")
			.value_parser(value_parser::parse_u64)
			.conflicts_with("trace")
			.help("Number of bytes to disassemble (default: through end of file)"))
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
			.required(true)
			.help("Instruction set and mode (x86_16, x86_32 [alias x86], or x86_64)"))
		.arg(clap::Arg::new("base")
			.long("base")
			.value_name("ADDRESS")
			.value_parser(value_parser::parse_u64)
			.default_value("0")
			.help("Instruction address corresponding to file offset zero"))
		.arg(clap::Arg::new("lookback")
			.long("lookback")
			.value_name("BYTES")
			.value_parser(value_parser::parse_usize)
			.default_value("0")
			.help("Decode up to BYTES before --offset; include an instruction overlapping the start"))
		.arg(clap::Arg::new("hex")
			.long("hex")
			.action(clap::ArgAction::SetTrue)
			.help("Include instruction opcode bytes in text and JSON output"))
		.arg(iced::layout_arg(true))
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
	let arch = *matches.get_one::<Arch>("arch").expect("required by clap");
	let offset = usize::try_from(*matches.get_one::<u64>("offset").expect("defaulted by clap"))?;
	if offset > bytes.len() {
		return Err(err(format!("offset {offset:#x} exceeds input length {:#x}", bytes.len())));
	}
	let base = *matches.get_one::<u64>("base").expect("defaulted by clap");
	let length = if matches.get_flag("trace") {
		let limit = *matches.get_one::<usize>("trace-limit").expect("defaulted by clap");
		let ip = base.checked_add(offset as u64).ok_or_else(|| err("base plus offset overflows"))?;
		iced::trace_bytes(&bytes[offset..], arch.bitness(), ip, limit)?
	}
	else if let Some(length) = matches.get_one::<u64>("length") {
		usize::try_from(*length)?
	}
	else {
		bytes.len() - offset
	};
	let lookback = *matches.get_one::<usize>("lookback").expect("defaulted by clap");
	disassemble(matches, format, &bytes, offset, length, arch, base, lookback)
}

fn disassemble(
	matches: &clap::ArgMatches, format: OutputFormat, bytes: &[u8],
	offset: usize, len: usize, arch: Arch, base: u64, lookback: usize,
) -> Result {
	let end = offset.checked_add(len).ok_or_else(|| err("offset plus length overflows"))?;
	if end > bytes.len() {
		return Err(err(format!("range ends at {end:#x}, beyond input length {:#x}", bytes.len())));
	}
	let decode_start = if len != 0 { offset.saturating_sub(lookback) } else { offset };
	let decode_ip = base.checked_add(decode_start as u64).ok_or_else(|| err("base plus decode offset overflows"))?;
	let start_ip = base.checked_add(offset as u64).ok_or_else(|| err("base plus offset overflows"))?;
	let end_ip = base.checked_add(end as u64).ok_or_else(|| err("base plus end offset overflows"))?;

	let pointer_width = if arch.bitness() == 64 { ty::PointerWidth::X64 } else { ty::PointerWidth::X32 };
	let facts = symbols::load(matches, pointer_width, base, None)?;
	let options = iced::DecodeOptions::new(format, "fo", facts);
	let mut decoded = iced::decode_bytes(&bytes[decode_start..end], arch.bitness(), decode_ip, start_ip, end_ip, options);
	if !matches.get_flag("hex") {
		iced::omit_bytes(&mut decoded);
	}
	let layout = match matches.get_one::<iced::AddressLayout>("layout").expect("defaulted by clap") {
		iced::AddressLayout::None => iced::TextLayout::None,
		iced::AddressLayout::Indent => iced::TextLayout::Indent,
		iced::AddressLayout::FileOffset => iced::TextLayout::Address { prefix: "fo", offset: base },
		iced::AddressLayout::Va => iced::TextLayout::Address { prefix: "va", offset: 0 },
		iced::AddressLayout::Rva => unreachable!("RVA layout is not accepted for raw input"),
	};
	iced::print(&decoded, format, layout)
}

#[test]
fn trace_options_require_trace_and_conflict_with_byte_windows() {
	for options in [
		vec!["--trace", "--length", "4"],
		vec!["--trace", "--lookback", "0"],
		vec!["--trace-limit", "4"],
	] {
		let mut args = vec!["disasm-raw", "code.bin", "--arch", "x86_64"];
		args.extend(options);
		assert!(command().try_get_matches_from(args).is_err());
	}
	for options in [vec![], vec!["--length", "4"], vec!["--trace"], vec!["--trace", "--trace-limit", "0"]] {
		let mut args = vec!["disasm-raw", "-", "--arch", "x86"];
		args.extend(options);
		assert!(command().try_get_matches_from(args).is_ok());
	}
}
