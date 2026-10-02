use super::*;
use pelite::pattern;
use super::ty::{PointerWidth, Type};

#[derive(serde::Serialize)]
struct ScanResult {
	captures: Vec<u32>,
	value: serde_json::Value,
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
			None => section.Characteristics & pelite::image::IMAGE_SCN_MEM_EXECUTE == 0,
		}
	}
}

pub fn command() -> clap::Command {
	clap::Command::new("scan")
		.about("Find patterns and read typed data at their matches")
		.after_help(include_str!("../docs/re-scan.md"))
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("type")
			.value_name("TYPE")
			.required(true))
		.arg(clap::Arg::new("pattern")
			.value_name("PATTERN")
			.help("Pattern to scan for"))
		.arg(clap::Arg::new("section")
			.long("section")
			.value_name("SECTION")
			.help("Scan one section by 0-based index or name"))
		.arg(clap::Arg::new("max-string-bytes")
			.long("max-string-bytes")
			.value_name("MAX_STRING_BYTES")
			.value_parser(clap::value_parser!(usize))
			.default_value(read::DEFAULT_MAX_STRING_BYTES)
			.help("Maximum bytes to inspect for a string"))
		.arg(clap::Arg::new("max-dynamic-array-length")
			.long("max-dynamic-array-length")
			.value_name("MAX_DYNAMIC_ARRAY_LENGTH")
			.value_parser(clap::value_parser!(u32))
			.default_value(read::DEFAULT_MAX_DYNAMIC_ARRAY_LENGTH)
			.help("Maximum element count for a field-length array"))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let source = matches.get_one::<String>("type").expect("required by clap");
	let options = read::ReadOptions {
		max_string_bytes: *matches.get_one::<usize>("max-string-bytes").expect("defaulted by clap"),
		max_dynamic_array_length: *matches.get_one::<u32>("max-dynamic-array-length").expect("defaulted by clap"),
	};
	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let sections = SectionFilter::new(pe, matches.get_one::<String>("section").map(String::as_str))?;
	let pointer_width = PointerWidth::from(pe);
	let ty = Type::parse(source, pointer_width).map_err(err)?;

	let pattern = matches.get_one::<String>("pattern").map(String::as_str).unwrap_or("?");
	let result = scan_pattern(pe, &ty, &options, pattern, &sections)?;
	print("Scan", &result, format)
}

fn scan_pattern(pe: pelite::PeFile<'_>, ty: &Type, options: &read::ReadOptions, source: &str, sections: &SectionFilter<'_>) -> Result<Vec<ScanResult>> {
	let parsed = pattern::parse(source, pattern::ParseOptions::default())
		.map_err(|error| err(format!("pattern '{source}': {error}")))?;
	let captures_len = pattern::captures_len(&parsed);
	let mut save = vec![0; pattern::save_len(&parsed)];
	let mut scanner = pe.scanner().sections(|section| sections.includes(section)).matches(&parsed);
	let mut matches = Vec::new();
	while scanner.next(&mut save).is_some() {
		matches.push(save[..captures_len].to_vec());
	}
	Ok(scan_matches(pe, ty, options, matches))
}

fn scan_matches(pe: pelite::PeFile<'_>, ty: &Type, options: &read::ReadOptions, matches: Vec<Vec<u32>>) -> Vec<ScanResult> {
	let alignment = ty.alignment(PointerWidth::from(pe));
	matches.into_iter().filter_map(|captures| {
		let &rva = captures.first()?;
		if rva % alignment != 0 {
			return None;
		}
		let value = read::read_at(pe, rva, ty, options);
		if read::contains_read_errors(&value) {
			return None;
		}
		Some(ScanResult { captures, value })
	}).collect()
}
