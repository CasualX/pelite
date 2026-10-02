use pelite::pattern;

use super::*;

#[derive(serde::Serialize)]
struct PatternMatches<'a> {
	pattern: &'a str,
	matches: Vec<Vec<u32>>,
}

struct SectionFilter<'a> {
	selected: Option<&'a pelite::image::IMAGE_SECTION_HEADER>,
}

impl<'a> SectionFilter<'a> {
	fn new(pe: pelite::PeFile<'a>, source: Option<&str>) -> Result<SectionFilter<'a>> {
		let selected = match source {
			Some(value) => {
				if let Some(section) = pe.section_headers().by_name(value) {
					Some(section)
				}
				else if let Ok(index) = value.parse::<usize>() && let Some(section) = pe.section_headers().image().get(index) {
					Some(section)
				}
				else {
					return Err(err(format!("section '{value}' not found")));
				}
			}
			None => None,
		};
		Ok(SectionFilter { selected })
	}

	fn includes(&self, section: &pelite::image::IMAGE_SECTION_HEADER) -> bool {
		match self.selected {
			Some(selected) => std::ptr::eq(selected, section),
			None => section.Characteristics & pelite::image::IMAGE_SCN_MEM_EXECUTE != 0,
		}
	}
}

pub fn command() -> clap::Command {
	clap::Command::new("findsig")
		.about("Find byte patterns in a PE image")
		.after_help(include_str!("../docs/re-findsig.md"))
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("pattern")
			.value_name("PATTERN")
			.help("Pattern to scan for"))
		.arg(clap::Arg::new("section")
			.long("section")
			.value_name("SECTION")
			.help("Scan one section by 0-based index or name"))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");

	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let sections = SectionFilter::new(pe, matches.get_one::<String>("section").map(String::as_str))?;
	let file_name = path.file_name().and_then(|name| name.to_str()).unwrap_or("<input>");

	if let Some(source) = matches.get_one::<String>("pattern") {
		return print_matches(find_pattern(pe, source, &sections)?, file_name, format);
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
		match find_pattern(pe, source, &sections) {
			Ok(result) => print_matches(result, file_name, format)?,
			Err(error) => eprintln!("pelite-cli: {error}"),
		}
	}
	Ok(())
}

fn print_matches(result: PatternMatches, file_name: &str, format: OutputFormat) -> Result {
	match format {
		OutputFormat::Nul => Ok(()),
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

fn find_pattern<'a>(pe: pelite::PeFile<'_>, source: &'a str, sections: &SectionFilter<'_>) -> Result<PatternMatches<'a>> {
	let parsed = pattern::parse(&source, pattern::ParseOptions::default())
		.map_err(|error| err(format!("pattern '{source}': {error}")))?;
	let captures_len = pattern::captures_len(&parsed);
	let mut save = vec![0; pattern::save_len(&parsed)];
	let mut scanner = pe.scanner().sections(|section| sections.includes(section)).matches(&parsed);
	let mut matches = Vec::new();
	while scanner.next(&mut save).is_some() {
		matches.push(save[..captures_len].to_vec());
	}
	Ok(PatternMatches { pattern: source, matches })
}
