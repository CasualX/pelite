use super::*;
use pelite::pattern;

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
				else if let Ok(index) = value_parser::parse_usize(value) && let Some(section) = pe.section_headers().image().get(index) {
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
			None => !section.is_code(),
		}
	}
}

pub fn command() -> clap::Command {
	clap::Command::new("scan")
		.about("Find patterns and read typed data at their matches")
		.after_help(include_str!("docs/scan.md"))
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
		.arg(clap::Arg::new("zerofill")
			.long("zerofill")
			.action(clap::ArgAction::SetTrue)
			.help("Allow typed reads from zero-filled section data"))
		.arg(clap::Arg::new("string-preview-length")
			.long("string-preview-length")
			.value_name("STRING_PREVIEW_LENGTH")
			.value_parser(value_parser::parse_usize)
			.default_value(read::DEFAULT_STRING_PREVIEW_LENGTH)
			.help("Maximum string preview length (bytes for cstr, u16 units for utf16lez)"))
		.arg(clap::Arg::new("max-dynamic-array-length")
			.long("max-dynamic-array-length")
			.value_name("MAX_DYNAMIC_ARRAY_LENGTH")
			.value_parser(value_parser::parse_u32)
			.default_value(read::DEFAULT_MAX_DYNAMIC_ARRAY_LENGTH)
			.help("Maximum element count for a field-length array"))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let source = matches.get_one::<String>("type").expect("required by clap");
	let options = read::ReadOptions {
		zerofill: matches.get_flag("zerofill"),
		string_preview_length: *matches.get_one::<usize>("string-preview-length").expect("defaulted by clap"),
		max_dynamic_array_length: *matches.get_one::<u32>("max-dynamic-array-length").expect("defaulted by clap"),
	};
	let map = pelite::FileMap::open(path).file_context("open", path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let sections = SectionFilter::new(pe, matches.get_one::<String>("section").map(String::as_str))?;
	let pointer_width = pointer_width(pe);
	let ty = ty::Type::parse(source, pointer_width)
		.map_err(|error| err(format!("failed to parse type {source:?}: {error}")))?;
	validate_type(&ty, None)
		.map_err(|error| err(format!("cannot scan type {source:?}: {error}")))?;

	let pattern = matches.get_one::<String>("pattern").map(String::as_str).unwrap_or("?");
	let result = scan_pattern(pe, &ty, &options, pattern, &sections)?;
	print("Scan", &result, format)
}

// Reject definitions that cannot be read regardless of the bytes at a match.
// Pointer dereferences retain the containing struct, just as read_value does.
fn validate_type(ty: &ty::Type, fields: Option<&[ty::Field]>) -> Result {
	match ty {
		ty::Type::Code | ty::Type::Fn | ty::Type::Unknown => return Err(err(format!("cannot read opaque type '{ty}' directly"))),
		ty::Type::Ptr32(pointee) | ty::Type::Ptr64(pointee) => {
			if !pointee.is_opaque() { validate_type(pointee, fields)?; }
		},
		ty::Type::Array(array) => {
			if let ty::ArrayLen::Dyn(name) = &array.len {
				let fields = fields.ok_or_else(|| err("dynamic array requires a containing struct"))?;
				let field = fields.iter().find(|field| matches!(&field.name, ty::FieldName::Named(field_name) if field_name == name))
					.ok_or_else(|| err(format!("array length field '{name}' is not in the containing struct")))?;
				if !matches!(field.ty, ty::Type::U8 | ty::Type::U16 | ty::Type::U32 | ty::Type::U64) {
					return Err(err(format!("array length field '{name}' must be u8, u16, u32, or u64")));
				}
			}
			if !matches!(array.len, ty::ArrayLen::Fixed(0)) { validate_type(&array.ty, fields)?; }
		},
		ty::Type::Struct(structure) => {
			for (index, field) in structure.fields.iter().enumerate() {
				let name = match &field.name {
					ty::FieldName::Discarded => continue,
					ty::FieldName::Named(name) => name.to_string(),
					ty::FieldName::Unnamed => index.to_string(),
				};
				validate_type(&field.ty, Some(&structure.fields))
					.map_err(|error| err(format!("field {name:?}: {error}")))?;
			}
		},
		_ => {},
	}
	Ok(())
}

fn scan_pattern(pe: pelite::PeFile<'_>, ty: &ty::Type, options: &read::ReadOptions, source: &str, sections: &SectionFilter<'_>) -> Result<Vec<ScanResult>> {
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

fn scan_matches(pe: pelite::PeFile<'_>, ty: &ty::Type, options: &read::ReadOptions, matches: Vec<Vec<u32>>) -> Vec<ScanResult> {
	let alignment = ty.alignment();
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
