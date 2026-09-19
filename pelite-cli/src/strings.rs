use super::*;

#[derive(serde::Serialize)]
struct FoundString<'a> {
	section: &'a str,
	address: u32,
	nul_terminated: bool,
	value: &'a str,
}

pub fn command() -> clap::Command {
	clap::Command::new("strings")
		.about("Find printable strings in PE sections")
		.arg(clap::Arg::new("file").value_name("FILE").value_parser(clap::value_parser!(PathBuf)).required(true))
		.arg(clap::Arg::new("min-length")
			.long("min-length")
			.value_name("N")
			.value_parser(clap::value_parser!(u8))
			.default_value("6")
			.help("Minimum unterminated string length"))
		.arg(clap::Arg::new("min-length-nul")
			.long("min-length-nul")
			.value_name("N")
			.value_parser(clap::value_parser!(u8))
			.default_value("3")
			.help("Minimum nul-terminated string length"))
		.arg(clap::Arg::new("strict-nul")
			.long("strict-nul")
			.value_name("BOOL")
			.value_parser(clap::value_parser!(bool))
			.default_value("true")
			.help("Require strings to have a nul terminator"))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let config = pelite::strings::Config {
		min_length: *matches.get_one("min-length").expect("defaulted by clap"),
		min_length_nul: *matches.get_one("min-length-nul").expect("defaulted by clap"),
		strict_nul: *matches.get_one("strict-nul").expect("defaulted by clap"),
		..pelite::strings::Config::default()
	};
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	analyze(path, config, format)
}

fn analyze(path: &Path, config: pelite::strings::Config, format: OutputFormat) -> Result {
	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let file_name = path.file_name().and_then(|name| name.to_str()).unwrap_or("<input>");
	let mut found = Vec::new();
	for section in pe.section_headers() {
		let Ok(bytes) = pe.get_section_bytes(section)
		else {
			continue;
		};
		let section_name = section.name().unwrap_or("<invalid>");
		for item in config.clone().enumerate(section.VirtualAddress, bytes) {
			found.push(FoundString {
				section: section_name,
				address: item.address,
				nul_terminated: item.has_nul,
				value: str::from_utf8(item.string)?,
			});
		}
	}
	match format {
		OutputFormat::Json => print_json(&found, false),
		OutputFormat::JsonPretty => print_json(&found, true),
		OutputFormat::Text => {
			for item in found {
				println!("{file_name}!{}:{:#x} {:?} {}", item.section, item.address, item.value, if item.nul_terminated { "0" } else { "?" });
			}
			Ok(())
		},
	}
}
