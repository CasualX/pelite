use super::*;

type Buckets = Vec<[u64; 256]>;

pub fn command() -> clap::Command {
	clap::Command::new("markov")
		.about("Generate decodable x86 code from PE sections using a Markov chain")
		.after_help(include_str!("docs/markov.md"))
		.arg(clap::Arg::new("files")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.num_args(1..)
			.required(true)
			.help("PE files used to train the chain"))
		.arg(clap::Arg::new("count")
			.value_name("COUNT")
			.value_parser(value_parser::parse_usize)
			.required(true)
			.help("Number of bytes to generate"))
		.arg(clap::Arg::new("seed")
			.long("seed")
			.value_name("SEED")
			.value_parser(value_parser::parse_u64)
			.help("Use a deterministic random seed"))
		.arg(clap::Arg::new("raw")
			.long("raw")
			.action(clap::ArgAction::SetTrue)
			.help("Write raw bytes to standard output with --format=text"))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let count = *matches.get_one::<usize>("count").expect("required by clap");
	let files = matches.get_many::<PathBuf>("files").expect("required by clap");
	let seed = matches
		.get_one::<u64>("seed")
		.copied()
		.unwrap_or_else(|| urandom::new().random());
	let mut buckets = vec![[0u64; 256]; 256];
	let mut machine = None;

	for path in files {
		let map = pelite::FileMap::open(path)?;
		let pe = pelite::PeFile::from_bytes(&map)?;
		let input_machine = pe.file_header().Machine;
		machine_bitness(input_machine)?;
		if machine.is_some_and(|machine| machine != input_machine) {
			return Err(err(format!("{}: input PE files have different Machine values; expected a common architecture", path.display())));
		}
		machine = Some(input_machine);
		for section in pe.section_headers() {
			if !section.is_code() {
				continue;
			}
			let bytes = pe.get_section_bytes(section)?;
			analyze(bytes, &mut buckets);
		}
	}

	let bitness = machine_bitness(machine.expect("at least one file is required by clap"))?;
	let bytes = generate(&buckets, count, seed, bitness)?;

	match format {
		OutputFormat::Nul => Ok(()),
		OutputFormat::Text => {
			if matches.get_flag("raw") {
				io::stdout().lock().write_all(&bytes)?;
				return Ok(());
			}
			let hex = bytes.iter().map(|byte| format!("{byte:02X}")).collect::<Vec<_>>().join(" ");
			writeln!(io::stdout().lock(), "{hex}")?;
			Ok(())
		},
		OutputFormat::Json => print_json(&bytes, false),
		OutputFormat::JsonPretty => print_json(&bytes, true),
	}
}

fn analyze(bytes: &[u8], buckets: &mut Buckets) {
	for pair in bytes.windows(2) {
		let count = &mut buckets[pair[0] as usize][pair[1] as usize];
		*count = count.saturating_add(1);
	}
}

fn machine_bitness(machine: u16) -> Result<u32> {
	match machine {
		pelite::image::IMAGE_FILE_MACHINE_I386 => Ok(32),
		pelite::image::IMAGE_FILE_MACHINE_AMD64 => Ok(64),
		_ => Err(err(format!("unsupported machine type {machine:#06x}; expected i386 or AMD64"))),
	}
}

fn generate(buckets: &Buckets, count: usize, seed: u64, bitness: u32) -> Result<Vec<u8>> {
	let active: Vec<u8> = buckets
		.iter()
		.enumerate()
		.filter(|(_, bucket)| bucket.iter().any(|&weight| weight != 0))
		.map(|(byte, _)| byte as u8)
		.collect();
	if active.is_empty() {
		return Err(err("the input files contain no executable byte transitions"));
	}

	let mut random = urandom::seeded(seed);
	let totals: Vec<u64> = buckets.iter().map(|bucket| bucket.iter().sum()).collect();
	let mut sample = |previous: Option<u8>| {
		if let Some(previous) = previous.filter(|&byte| totals[byte as usize] != 0) {
			let mut pick = random.uniform(0..totals[previous as usize]);
			for (next, &weight) in buckets[previous as usize].iter().enumerate() {
				if pick < weight {
					return next as u8;
				}
				pick -= weight;
			}
		}
		active[random.uniform(0..active.len())]
	};

	let mut output = Vec::with_capacity(count);
	while output.len() < count {
		let mut accepted = false;
		// Reject only the current instruction, preserving the committed prefix.
		// Bound attempts because some chains cannot produce a valid instruction
		// (or cannot fit one into the remaining byte count).
		for _ in 0..1024 {
			let mut candidate = [0u8; 15];
			let mut previous = output.last().copied();
			for len in 1..=candidate.len().min(count - output.len()) {
				let byte = sample(previous);
				candidate[len - 1] = byte;
				previous = Some(byte);
				let mut decoder = iced_x86::Decoder::new(bitness, &candidate[..len], iced_x86::DecoderOptions::NONE);
				let instruction = decoder.decode();
				if !instruction.is_invalid() {
					output.extend_from_slice(&candidate[..len]);
					accepted = true;
					break;
				}
				if decoder.last_error() != iced_x86::DecoderError::NoMoreBytes {
					break;
				}
			}
			if accepted {
				break;
			}
		}
		if !accepted {
			return Err(err(format!("could not generate a valid {bitness}-bit instruction at byte offset {} after 1024 attempts ({} bytes remaining); try another seed, byte count, or training input", output.len(), count - output.len())));
		}
	}
	Ok(output)
}
