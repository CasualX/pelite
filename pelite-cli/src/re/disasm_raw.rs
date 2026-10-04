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
struct DisassembledInstruction<'a> {
	offset: usize,
	ip: u64,
	bytes: &'a [u8],
	instruction: String,
}

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
			.value_parser(Arch::parse)
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
	let arch = *matches.get_one::<Arch>("arch").expect("required by clap");
	let offset = usize::try_from(*matches.get_one::<u64>("offset").expect("defaulted by clap"))?;
	if offset > bytes.len() {
		return Err(err(format!("offset {offset:#x} exceeds input length {:#x}", bytes.len())));
	}
	let base = *matches.get_one::<u64>("base").expect("defaulted by clap");
	let length = if matches.get_flag("trace") {
		let limit = *matches.get_one::<usize>("trace-limit").expect("defaulted by clap");
		trace_bytes(&bytes[offset..], arch.bitness(), offset, base, limit)?
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

	let color = format == OutputFormat::Text && io::stdout().is_terminal();
	let pointer_width = if arch.bitness() == 64 { ty::PointerWidth::Bits64 } else { ty::PointerWidth::Bits32 };
	let facts = symbols::load(matches, pointer_width, base, None)?;
	let symbols = Arc::new(facts.symbols);
	let mut decoded = iced::decode_bytes(&bytes[decode_start..end], arch.bitness(), decode_ip, start_ip, end_ip, color, Arc::clone(&symbols));
	iced::append_comments(&mut decoded, &facts.comments);
	match format {
		OutputFormat::Nul => Ok(()),
		OutputFormat::Json | OutputFormat::JsonPretty => {
			let mut instructions = Vec::with_capacity(decoded.len());
			for item in decoded {
				instructions.push(DisassembledInstruction {
					offset: usize::try_from(item.ip - base)?,
					ip: item.ip,
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
				let offset = ip - base;
				match layout {
					AddressLayout::None => iced::TextPrefix::None,
					AddressLayout::Indent => iced::TextPrefix::Indent,
					AddressLayout::FileOffset => iced::TextPrefix::Address { label: "fo", value: offset },
					AddressLayout::Va => iced::TextPrefix::Address { label: "va", value: ip },
				}
			})?;
			Ok(())
		},
	}
}

fn is_control_flow(instr: &iced_x86::Instruction) -> bool {
	instr.flow_control() != iced_x86::FlowControl::Next || instr.mnemonic() == iced_x86::Mnemonic::Hlt
}

fn trace_bytes(bytes: &[u8], bitness: u32, offset: usize, base: u64, trace_limit: usize) -> Result<usize> {
	let ip = base.checked_add(offset as u64).ok_or_else(|| err("base plus offset overflows"))?;
	let mut decoder = iced_x86::Decoder::with_ip(bitness, bytes, ip, iced_x86::DecoderOptions::NONE);
	let mut count = 0;
	while decoder.can_decode() {
		let position = decoder.position();
		let address = offset + position;
		let instr = decoder.decode();
		if instr.is_invalid() {
			return Err(err(format!("(bad): unable to decode instruction at fo:{address:#x} after {count} instructions, {position} bytes")));
		}
		count += 1;
		if is_control_flow(&instr) || (trace_limit != 0 && count >= trace_limit) {
			return Ok(decoder.position());
		}
	}
	Err(err(format!("no control-transfer instruction before end of available data at fo:{:#x} after {count} instructions, {} bytes", offset + decoder.position(), decoder.position())))
}

#[test]
fn stops_at_control_transfers_including_their_bytes() {
	let endings: &[&[u8]] = &[
		&[0xe8, 0, 0, 0, 0], // direct call
		&[0xff, 0xd0], // indirect call
		&[0xe9, 0, 0, 0, 0], // direct jump
		&[0xff, 0xe0], // indirect jump
		&[0x75, 0], // conditional jump
		&[0xe2, 0], // loop
		&[0xe3, 0], // jrcxz
		&[0xcd, 0x80], // int
		&[0xcc], // int3
		&[0xc3], // ret
		&[0xc2, 8, 0], // ret with stack adjustment
		&[0x48, 0xcf], // iretq
		&[0x0f, 0x05], // syscall
		&[0x0f, 0x0b], // ud2: valid decoding, traps when executed
		&[0xf4], // hlt
		&[0xc7, 0xf8, 0, 0, 0, 0], // xbegin
	];
	for ending in endings {
		let mut bytes = vec![0x90, 0x48, 0x89, 0xe5];
		bytes.extend_from_slice(ending);
		bytes.push(0x90);
		let trace = trace_bytes(&bytes, 64, 0x1000, 0x180000000, 0).unwrap();
		assert_eq!(trace, 4 + ending.len(), "{ending:02x?}");
	}
}

#[test]
fn invalid_and_truncated_instructions_fail_with_successful_counts() {
	for bytes in [&[0x90, 0x06][..], &[0x90, 0xe8, 0][..]] {
		let error = trace_bytes(bytes, 64, 0x1000, 0x180000000, 0).unwrap_err().to_string();
		assert!(error.contains("(bad)"));
		assert!(error.contains("fo:0x1001"));
		assert!(error.contains("after 1 instructions, 1 bytes"));
	}
}

#[test]
fn exhaustion_does_not_report_success() {
	for bytes in [&[][..], &[0x90][..]] {
		let error = trace_bytes(bytes, 64, 0x1000, 0x180000000, 0).unwrap_err().to_string();
		assert!(error.contains("no control-transfer instruction before end of available data"));
	}
}

#[test]
fn instruction_limit_stops_on_complete_instructions() {
	// The invalid instruction after the limit must not be decoded.
	let bytes = [0x90, 0x48, 0x89, 0xe5, 0x06];
	let trace = trace_bytes(&bytes, 64, 0x1000, 0x180000000, 2).unwrap();
	assert_eq!(trace, 4);
	assert!(trace_bytes(&bytes, 64, 0x1000, 0x180000000, 3).is_err());
}

#[test]
fn zero_limit_traces_beyond_default_and_control_flow_stops_early() {
	let mut bytes = vec![0x90; 300];
	bytes.push(0xc3);
	for (limit, count) in [(256, 256), (0, 301), (400, 301)] {
		let trace = trace_bytes(&bytes, 64, 0x1000, 0x180000000, limit).unwrap();
		assert_eq!(trace, count);
	}
	assert!(trace_bytes(&bytes[..300], 64, 0x1000, 0x180000000, 0).is_err());
}

#[test]
fn traces_all_raw_architectures() {
	for bitness in [16, 32, 64] {
		assert_eq!(trace_bytes(&[0x90, 0xc3, 0x90], bitness, 0x100, 0x1000, 0).unwrap(), 2);
	}
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
