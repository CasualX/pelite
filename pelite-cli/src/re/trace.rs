use super::*;

#[derive(Debug, serde::Serialize)]
struct Trace {
	instructions: usize,
	bytes: usize,
	stop_address: u32,
	instruction: String,
}

pub fn command() -> clap::Command {
	disasm::output_args(clap::Command::new("trace")
		.about("Count instructions and bytes up to the first control transfer")
		.after_help(include_str!("docs/trace.md"))
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("address")
			.value_name("ADDRESS")
			.value_parser(Address::parse)
			.required(true))
		.arg(clap::Arg::new("arch")
			.long("arch")
			.value_name("ARCH")
			.value_parser(Arch::parse)
			.help("Override the PE machine header (x86_16, x86_32 [alias x86], or x86_64)"))
		.arg(clap::Arg::new("disasm")
			.long("disasm")
			.action(clap::ArgAction::SetTrue)
			.help("Disassemble the traced instructions instead of printing counts"))
		.arg(clap::Arg::new("disasm-limit")
			.long("disasm-limit")
			.value_name("N")
			.value_parser(clap::value_parser!(usize))
			.default_value("256")
			.help("With --disasm, stop after N instructions; 0 disables the limit")))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let address = *matches.get_one::<Address>("address").expect("required by clap");
	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let rva = address.to_rva(pe)?;
	let arch = get_arch(matches, pe)?;

	// Disassembly arguments
	let disasm = matches.get_flag("disasm");
	let disasm_limit = if !disasm { 0 }
		else { *matches.get_one::<usize>("disasm-limit").expect("defaulted by clap") };

	let trace = trace_bytes(pe.slice_bytes(rva)?, arch.bitness(), rva, pe.image_base(), disasm_limit)?;

	if disasm {
		let range = RvaRange { start: rva, end: rva + trace.bytes as u32 };
		disasm::disassemble(matches, format, pe, range, arch, 0)
	}
	else {
		print("Trace", &trace, format)
	}
}

fn is_control_flow(instr: &iced_x86::Instruction) -> bool {
	instr.flow_control() != iced_x86::FlowControl::Next || instr.mnemonic() == iced_x86::Mnemonic::Hlt
}

fn trace_bytes(bytes: &[u8], bitness: u32, rva: u32, image_base: u64, disasm_limit: usize) -> Result<Trace> {
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
		if is_control_flow(&instr) || (disasm_limit != 0 && count >= disasm_limit) {
			let text = iced::Formatter::new(false, None).format(&instr).plain;
			return Ok(Trace { instructions: count, bytes: decoder.position(), stop_address: u32::try_from(address)?, instruction: text });
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
		assert_eq!(trace.instructions, 3, "{ending:02x?}");
		assert_eq!(trace.bytes, 4 + ending.len(), "{ending:02x?}");
		assert_eq!(trace.stop_address, 0x1004);
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
	assert_eq!(trace.instructions, 2);
	assert_eq!(trace.bytes, 4);
	assert_eq!(trace.stop_address, 0x1001);
	assert_eq!(trace.instruction, "mov rbp,rsp");
	assert!(trace_bytes(&bytes, 64, 0x1000, 0x180000000, 3).is_err());
}

#[test]
fn zero_limit_traces_beyond_default_and_control_flow_stops_early() {
	let mut bytes = vec![0x90; 300];
	bytes.push(0xc3);
	for (limit, count, instruction) in [(256, 256, "nop"), (0, 301, "ret"), (400, 301, "ret")] {
		let trace = trace_bytes(&bytes, 64, 0x1000, 0x180000000, limit).unwrap();
		assert_eq!(trace.instructions, count);
		assert_eq!(trace.bytes, count);
		assert_eq!(trace.stop_address, 0x1000 + count as u32 - 1);
		assert_eq!(trace.instruction, instruction);
	}
	assert!(trace_bytes(&bytes[..300], 64, 0x1000, 0x180000000, 0).is_err());
}
