use pelite::image;

use super::*;

#[derive(Debug, serde::Serialize)]
struct Trace {
	instructions: usize,
	bytes: usize,
	stop_address: u32,
	instruction: String,
}

pub fn command() -> clap::Command {
	clap::Command::new("trace")
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
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let address = *matches.get_one::<Address>("address").expect("required by clap");
	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let rva = address.to_rva(pe)?;
	let bitness = if let Some(arch) = matches.get_one::<Arch>("arch") {
		arch.bitness()
	}
	else {
		match pe.file_header().Machine {
			image::IMAGE_FILE_MACHINE_I386 => 32,
			image::IMAGE_FILE_MACHINE_AMD64 => 64,
			machine => return Err(err(format!("unsupported machine type {machine:#06x}; expected i386 or AMD64"))),
		}
	};
	let trace = trace_bytes(pe.slice_bytes(rva)?, bitness, rva, pe.image_base())?;
	match format {
		OutputFormat::Nul => Ok(()),
		OutputFormat::Json | OutputFormat::JsonPretty => print_json(&trace, format == OutputFormat::JsonPretty),
		OutputFormat::Text => {
			writeln!(io::stdout().lock(), "{} instructions, {} bytes; stopped at rva:{:#x}: {}",
				trace.instructions, trace.bytes, trace.stop_address, trace.instruction)?;
			Ok(())
		},
	}
}

fn is_control_flow(instr: &iced_x86::Instruction) -> bool {
	instr.flow_control() != iced_x86::FlowControl::Next || instr.mnemonic() == iced_x86::Mnemonic::Hlt
}

fn trace_bytes(bytes: &[u8], bitness: u32, rva: u32, image_base: u64) -> Result<Trace> {
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
		if is_control_flow(&instr) {
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
		let trace = trace_bytes(&bytes, 64, 0x1000, 0x180000000).unwrap();
		assert_eq!(trace.instructions, 3, "{ending:02x?}");
		assert_eq!(trace.bytes, 4 + ending.len(), "{ending:02x?}");
		assert_eq!(trace.stop_address, 0x1004);
	}
}

#[test]
fn invalid_and_truncated_instructions_fail_with_successful_counts() {
	for bytes in [&[0x90, 0x06][..], &[0x90, 0xe8, 0][..]] {
		let error = trace_bytes(bytes, 64, 0x1000, 0x180000000).unwrap_err().to_string();
		assert!(error.contains("(bad)"));
		assert!(error.contains("rva:0x1001"));
		assert!(error.contains("after 1 instructions, 1 bytes"));
	}
}

#[test]
fn exhaustion_does_not_report_success() {
	for bytes in [&[][..], &[0x90][..]] {
		let error = trace_bytes(bytes, 64, 0x1000, 0x180000000).unwrap_err().to_string();
		assert!(error.contains("no control-transfer instruction before end of available data"));
	}
}
