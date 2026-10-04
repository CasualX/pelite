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
			.help("Number of bytes to disassemble (decimal or 0x-prefixed hexadecimal)"))
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
			.help("Show instruction opcode bytes in text output"))
		.arg(clap::Arg::new("layout")
			.long("layout")
			.value_name("MODE")
			.value_parser(AddressLayout::parse)
			.default_value("indent")
			.help("Choose plain, indented, RVA, or VA text layout"))
		.arg(symbols::arg())
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let address = *matches.get_one::<Address>("address").expect("required by clap");
	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let start = address.to_rva(pe)?;
	let arch = get_arch(matches, pe)?;
	let length = if matches.get_flag("trace") {
		let limit = *matches.get_one::<usize>("trace-limit").expect("defaulted by clap");
		trace_bytes(pe.slice_bytes(start)?, arch.bitness(), start, pe.image_base(), limit)?
	}
	else {
		*matches.get_one::<usize>("length").expect("required by clap")
	};
	let lookback = *matches.get_one::<u32>("lookback").expect("defaulted by clap");
	disassemble(matches, format, pe, start, length, arch, lookback)
}

fn disassemble(
	matches: &clap::ArgMatches, format: OutputFormat, pe: pelite::PeFile<'_>,
	rva: u32, len: usize, arch: Arch, lookback: u32,
) -> Result {
	let end = u32::try_from(len).ok().and_then(|len| rva.checked_add(len))
		.ok_or_else(|| err("address plus length overflows RVA"))?;
	let bytes = pe.slice(rva, len, 1)?;
	let (decode_start, bytes) = pe.section_headers().by_rva(rva)
		.filter(|_| lookback != 0 && len != 0)
		.and_then(|section| {
			let start = rva.saturating_sub(lookback).max(section.VirtualAddress);
			let len = usize::try_from(end - start).ok()?;
			pe.slice(start, len, 1).ok().map(|bytes| (start, bytes))
		})
		.unwrap_or((rva, bytes));
	let image_base = pe.image_base();
	let start_ip = image_base + rva as u64;
	let decode_ip = image_base + decode_start as u64;
	let end_ip = image_base + end as u64;

	let color = format == OutputFormat::Text && io::stdout().is_terminal();
	let facts = symbols::load(matches, ty::PointerWidth::from(pe), image_base, Some(pe))?;
	let symbols = Arc::new(facts.symbols);
	let mut decoded = iced::decode_bytes(bytes, arch.bitness(), decode_ip, start_ip, end_ip, color, Arc::clone(&symbols));
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

fn is_control_flow(instr: &iced_x86::Instruction) -> bool {
	instr.flow_control() != iced_x86::FlowControl::Next || instr.mnemonic() == iced_x86::Mnemonic::Hlt
}

fn trace_bytes(bytes: &[u8], bitness: u32, rva: u32, image_base: u64, trace_limit: usize) -> Result<usize> {
	let ip = image_base + u64::from(rva);
	let mut decoder = iced_x86::Decoder::with_ip(bitness, bytes, ip, iced_x86::DecoderOptions::NONE);
	let mut count = 0;
	while decoder.can_decode() {
		let offset = decoder.position();
		let address = u64::from(rva) + offset as u64;
		let instr = decoder.decode();
		if instr.is_invalid() {
			return Err(err(format!("(bad): unable to decode instruction at rva:{address:#x} after {count} instructions, {offset} bytes")));
		}
		count += 1;
		if is_control_flow(&instr) || (trace_limit != 0 && count >= trace_limit) {
			return Ok(decoder.position());
		}
	}
	Err(err(format!("no control-transfer instruction before end of available data at rva:{:#x} after {count} instructions, {} bytes", u64::from(rva) + decoder.position() as u64, decoder.position())))
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
		assert!(error.contains("rva:0x1001"));
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
