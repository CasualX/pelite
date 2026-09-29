use pelite::pattern;

use super::*;

#[derive(serde::Serialize)]
struct PatternMatches<'a> {
	pattern: &'a str,
	matches: Vec<Vec<u32>>,
}

pub fn command() -> clap::Command {
	clap::Command::new("findsig")
		.about("Find byte patterns in a PE image")
		.after_help("If no pattern is supplied, patterns are read one per line from standard input.\nPattern syntax: https://docs.rs/pelite/latest/pelite/pattern/fn.parse.html")
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("pattern")
			.value_name("PATTERN")
			.help("Pattern to scan for"))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");

	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let file_name = path.file_name().and_then(|name| name.to_str()).unwrap_or("<input>");

	if let Some(source) = matches.get_one::<String>("pattern") {
		return print_matches(find_pattern(pe, source)?, file_name, format);
	}

	let stdin = io::stdin();
	let interactive = stdin.is_terminal();
	if interactive {
		println!("Interactive mode; enter one pattern per line. Press Ctrl-Z/Ctrl-D to quit.");
	}
	let mut lines = stdin.lock().lines();
	loop {
		if interactive {
			print!(">>> ");
			io::stdout().flush()?;
		}
		let Some(line) = lines.next() else {
			break;
		};
		let source = line?;
		let source = source.trim();
		if source.is_empty() {
			continue;
		}
		match find_pattern(pe, source) {
			Ok(result) => print_matches(result, file_name, format)?,
			Err(error) => eprintln!("pelite-cli: {error}"),
		}
	}
	Ok(())
}

fn print_matches(result: PatternMatches, file_name: &str, format: OutputFormat) -> Result {
	match format {
		OutputFormat::Json => print_json(&result, false),
		OutputFormat::JsonPretty => print_json(&result, true),
		OutputFormat::Text => {
			let mut output = io::stdout().lock();
			writeln!(output, "Pattern {:?} matches:", result.pattern)?;
			for captures in result.matches {
				if let Some((address, captures)) = captures.split_first() {
					write!(output, "  {file_name}!{address:#010x}")?;
					if !captures.is_empty() {
						write!(output, "  [")?;
						for (index, capture) in captures.iter().enumerate() {
							write!(output, "{}/{capture:#010x} ", index + 1)?;
						}
						write!(output, "]")?;
					}
					writeln!(output)?;
				}
			}
			Ok(())
		},
	}
}

fn find_pattern<'a>(pe: pelite::PeFile<'_>, source: &'a str) -> Result<PatternMatches<'a>> {
	let parsed = pattern::parse(&source, pattern::ParseOptions::default())
		.map_err(|error| err(format!("pattern '{source}': {error}")))?;
	let captures_len = pattern::captures_len(&parsed);
	let mut save = vec![0; pattern::save_len(&parsed)];
	let mut scanner = pe.scanner().sections(|_| true).matches(&parsed);
	let mut matches = Vec::new();
	while scanner.next(&mut save).is_some() {
		matches.push(save[..captures_len].to_vec());
	}
	Ok(PatternMatches { pattern: source, matches })
}
