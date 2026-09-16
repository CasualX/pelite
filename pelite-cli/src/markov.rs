use std::path::PathBuf;

use clap::{Arg, ArgMatches, Command};
use pelite::image::IMAGE_SCN_MEM_EXECUTE;
use serde::Serialize;

use crate::{OutputFormat, Result, err, print_json};

type Buckets = Vec<[u64; 256]>;

#[derive(Serialize)]
struct Source {
	file: String,
	sections: usize,
	bytes: usize,
}

#[derive(Serialize)]
struct Generated {
	seed: u64,
	bytes: Vec<u8>,
	hex: String,
	sources: Vec<Source>,
	#[serde(skip_serializing_if = "Option::is_none")]
	output: Option<String>,
}

pub fn command() -> Command {
	Command::new("markov")
		.visible_alias("markovbin")
		.about("Generate bytes from executable PE sections using a Markov chain")
		.arg(Arg::new("count")
			.value_name("COUNT")
			.value_parser(clap::value_parser!(usize))
			.required(true)
			.help("Number of bytes to generate"))
		.arg(Arg::new("files")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.num_args(1..)
			.required(true)
			.help("PE files used to train the chain"))
		.arg(Arg::new("seed")
			.long("seed")
			.value_name("SEED")
			.value_parser(clap::value_parser!(u64))
			.help("Use a deterministic random seed"))
		.arg(Arg::new("output")
			.long("output")
			.short('o')
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.help("Also write the generated raw bytes to a file"))
}

pub fn run(matches: &ArgMatches, format: OutputFormat) -> Result {
	let count = *matches.get_one::<usize>("count").expect("required by clap");
	let files = matches.get_many::<PathBuf>("files").expect("required by clap");
	let seed = matches
		.get_one::<u64>("seed")
		.copied()
		.unwrap_or_else(|| urandom::new().random());
	let mut buckets = vec![[0u64; 256]; 256];
	let mut sources = Vec::new();

	for path in files {
		let map = pelite::FileMap::open(path)?;
		let pe = pelite::PeFile::from_bytes(&map)?;
		let mut source = Source {
			file: path.to_string_lossy().into_owned(),
			sections: 0,
			bytes: 0,
		};
		for section in pe.section_headers() {
			if section.Characteristics & IMAGE_SCN_MEM_EXECUTE == 0 {
				continue;
			}
			let bytes = pe.get_section_bytes(section)?;
			analyze(bytes, &mut buckets);
			source.sections += 1;
			source.bytes += bytes.len();
		}
		sources.push(source);
	}

	let bytes = generate(&buckets, count, seed)?;
	let output = matches.get_one::<PathBuf>("output").map(|path| {
		std::fs::write(path, &bytes)?;
		Ok::<_, std::io::Error>(path.to_string_lossy().into_owned())
	}).transpose()?;
	let generated = Generated {
		hex: bytes.iter().map(|byte| format!("{byte:02X}")).collect::<Vec<_>>().join(" "),
		seed,
		bytes,
		sources,
		output,
	};

	match format {
		OutputFormat::Text => {
			println!("{}", generated.hex);
			Ok(())
		},
		OutputFormat::Json => print_json(&generated, false),
		OutputFormat::JsonPretty => print_json(&generated, true),
	}
}

fn analyze(bytes: &[u8], buckets: &mut Buckets) {
	for pair in bytes.windows(2) {
		let count = &mut buckets[pair[0] as usize][pair[1] as usize];
		*count = count.saturating_add(1);
	}
}

fn generate(buckets: &Buckets, count: usize, seed: u64) -> Result<Vec<u8>> {
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
	let mut byte = active[random.uniform(0..active.len())];
	let mut output = Vec::with_capacity(count);
	for _ in 0..count {
		output.push(byte);
		let bucket = &buckets[byte as usize];
		let total: u64 = bucket.iter().sum();
		if total == 0 {
			byte = active[random.uniform(0..active.len())];
			continue;
		}
		let mut pick = random.uniform(0..total);
		for (next, &weight) in bucket.iter().enumerate() {
			if pick < weight {
				byte = next as u8;
				break;
			}
			pick -= weight;
		}
	}
	Ok(output)
}

#[cfg(test)]
mod tests {
	use super::{analyze, generate};

	#[test]
	fn seeded_generation_is_exact_and_reproducible() {
		let mut buckets = vec![[0u64; 256]; 256];
		analyze(&[1, 2, 1, 3, 1, 2], &mut buckets);
		let first = generate(&buckets, 32, 7).unwrap();
		let second = generate(&buckets, 32, 7).unwrap();
		assert_eq!(first.len(), 32);
		assert_eq!(first, second);
	}
}
