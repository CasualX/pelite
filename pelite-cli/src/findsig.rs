use pelite::pattern;

use super::*;

#[derive(serde::Serialize)]
struct PatternMatches {
	pattern: String,
	matches: Vec<Vec<u32>>,
}

pub fn command() -> clap::Command {
	clap::Command::new("findsig")
		.about("Find byte patterns in a PE image")
		.after_help("If no patterns are supplied, patterns are read one per line from standard input.\nPattern syntax: https://docs.rs/pelite/latest/pelite/pattern/fn.parse.html")
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("patterns")
			.value_name("PATTERN")
			.num_args(0..)
			.action(clap::ArgAction::Append)
			.help("Pattern to scan for"))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let mut patterns: Vec<String> = matches.get_many::<String>("patterns").map(|values| values.cloned().collect()).unwrap_or_default();
	if patterns.is_empty() {
		let interactive = format == OutputFormat::Text && io::stdin().is_terminal();
		if interactive {
			println!("Interactive mode; enter one pattern per line. Press Ctrl-Z/Ctrl-D to quit.");
		}
		let stdin = io::stdin();
		let mut lines = stdin.lock().lines();
		loop {
			if interactive {
				print!(">>> ");
				io::stdout().flush()?;
			}
			let Some(line) = lines.next() else {
				break;
			};
			let line = line?;
			if !line.trim().is_empty() {
				patterns.push(line.trim().to_owned());
			}
		}
	}

	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let file_name = path.file_name().and_then(|name| name.to_str()).unwrap_or("<input>");
	let mut results = Vec::new();
	for source in patterns {
		let opts = pattern::ParseOptions::default();
		let parsed = pattern::parse(&source, opts).map_err(|error| err(format!("pattern '{source}': {error}")))?;
		let captures_len = pattern::captures_len(&parsed);
		let mut save = vec![0; pattern::save_len(&parsed)];
		let mut scanner = pe.scanner().sections(|_| true).matches(&parsed);
		let mut matches = Vec::new();
		while scanner.next(&mut save).is_some() {
			matches.push(save[..captures_len].to_vec());
		}
		results.push(PatternMatches { pattern: source, matches });
	}
	match format {
		OutputFormat::Json => print_json(&results, false),
		OutputFormat::JsonPretty => print_json(&results, true),
		OutputFormat::Text => {
			for result in results {
				println!("Pattern {:?} matches:", result.pattern);
				for captures in result.matches {
					if let Some((address, captures)) = captures.split_first() {
						print!("  {file_name}!{address:#010x}");
						if !captures.is_empty() {
							print!("  [");
							for (index, capture) in captures.iter().enumerate() {
								print!("{}/{capture:#010x} ", index + 1);
							}
							print!("]");
						}
						println!();
					}
				}
			}
			Ok(())
		},
	}
}
