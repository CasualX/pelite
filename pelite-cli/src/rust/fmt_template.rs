use pelite::{FileMap, PeFile, image};

use crate::*;

#[derive(serde::Serialize)]
struct TemplateOutput {
	template_rva: u32,
	encoded_len: usize,
	format: String,
	argument_count: u16,
	placeholders: Vec<Placeholder>,
}

pub fn command() -> clap::Command {
	clap::Command::new("fmt-template")
		.about("Find Rust format_args! templates in read-only PE data")
		.after_help(include_str!("../docs/rust-fmt-template.md"))
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let map = FileMap::open(path)?;
	let file = PeFile::from_bytes(&map)?;
	let output = analyze(file);
	match format {
		OutputFormat::Json => print_json(&output, false),
		OutputFormat::JsonPretty => print_json(&output, true),
		OutputFormat::Text => {
			let name = path.file_name().and_then(|name| name.to_str()).unwrap_or("<input>");
			let mut writer = io::stdout().lock();
			for item in output {
				writeln!(writer, "{name}!{:#010x} {:?} (arguments={})", item.template_rva, item.format, item.argument_count)?;
			}
			Ok(())
		},
	}
}

fn is_readonly_data(section: &image::IMAGE_SECTION_HEADER) -> bool {
	let flags = section.Characteristics;
	flags & image::IMAGE_SCN_MEM_READ != 0
		&& flags & (image::IMAGE_SCN_MEM_WRITE | image::IMAGE_SCN_MEM_EXECUTE | image::IMAGE_SCN_MEM_DISCARDABLE) == 0
}

fn template_length(bytes: &[u8]) -> Option<usize> {
	const MAX_TEMPLATE: usize = 64 * 1024;
	let bytes = bytes.get(..bytes.len().min(MAX_TEMPLATE))?;
	let mut cursor = 0;
	let mut placeholders = 0;
	let mut literal_bytes = 0;
	loop {
		let tag = *bytes.get(cursor)?;
		cursor += 1;
		match tag {
			0 => return (placeholders > 0 && literal_bytes >= 2).then_some(cursor),
			1..=0x80 => {
				let length = if tag == 0x80 {
					let length = u16::from_le_bytes(bytes.get(cursor..cursor + 2)?.try_into().ok()?) as usize;
					cursor += 2;
					length
				}
				else {
					tag as usize
				};
				let piece = str::from_utf8(bytes.get(cursor..cursor.checked_add(length)?)?).ok()?;
				if piece.chars().any(|ch| ch != '\n' && ch != '\r' && ch != '\t' && ch.is_control()) {
					return None;
				}
				cursor += length;
				literal_bytes += length;
			},
			0x81..=0xbf => return None,
			_ => {
				placeholders += 1;
				if placeholders > 4096 {
					return None;
				}
				let skip = usize::from(tag & 1 != 0) * 4
					+ usize::from(tag & 2 != 0) * 2
					+ usize::from(tag & 4 != 0) * 2
					+ usize::from(tag & 8 != 0) * 2;
				cursor = cursor.checked_add(skip)?;
			},
		}
	}
}

fn analyze(file: PeFile<'_>) -> Vec<TemplateOutput> {
	let mut output = Vec::new();
	for section in file.section_headers() {
		if !is_readonly_data(section) {
			continue;
		}
		let Ok(bytes) = file.get_section_bytes(section) else { continue };
		let mut candidates = Vec::new();
		for offset in 0..bytes.len() {
			let Some(template_rva) = u32::try_from(offset).ok().and_then(|offset| section.VirtualAddress.checked_add(offset)) else { break };
			let Some(length) = template_length(&bytes[offset..]) else { continue };
			let Some(template) = parse_format_template(&bytes[offset..offset + length]) else { continue };
			candidates.push((offset, TemplateOutput {
				template_rva,
				encoded_len: template.encoded_len,
				format: template.rendered,
				argument_count: template.argument_count,
				placeholders: template.placeholders,
			}));
		}
		// A valid template can contain bytes that also parse as a shorter template.
		// Prefer the longest candidate where byte ranges overlap.
		candidates.sort_unstable_by_key(|(offset, item)| (std::cmp::Reverse(item.encoded_len), *offset));
		let mut accepted = BTreeMap::<usize, usize>::new();
		for (start, item) in candidates {
			let end = start + item.encoded_len;
			if accepted.range(..end).next_back().is_some_and(|(_, &other_end)| other_end > start) {
				continue;
			}
			accepted.insert(start, end);
			output.push(item);
		}
	}
	output.sort_unstable_by_key(|item| item.template_rva);
	output
}

#[test]
fn template_filter_needs_literal_text_and_complete_encoding() {
	assert_eq!(template_length(b"\x02hi\xc0\0"), Some(5));
	assert_eq!(template_length(b"\xc0\0"), None);
	assert_eq!(template_length(b"\x02hi\xc0"), None);
	assert_eq!(template_length(b"\x02\xffi\xc0\0"), None);
}

//----------------------------------------------------------------

#[derive(Clone, Debug)]
#[derive(serde::Serialize)]
pub struct Placeholder {
	pub argument: u16,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub flags: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub width: Option<u16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub precision: Option<u16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub width_argument: Option<u16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub precision_argument: Option<u16>,
}

#[derive(Clone, Debug)]
pub struct FormatTemplate {
	pub rendered: String,
	pub detailed: String,
	pub placeholders: Vec<Placeholder>,
	pub argument_count: u16,
	pub encoded_len: usize,
}

pub fn parse_format_template(bytes: &[u8]) -> Option<FormatTemplate> {
	const MAX_TEMPLATE: usize = 64 * 1024;
	const MAX_ARGUMENTS: u16 = 4096;

	let bytes = bytes.get(..bytes.len().min(MAX_TEMPLATE))?;
	let mut cursor = 0usize;
	let mut next_argument = 0u16;
	let mut required_arguments = 0u16;
	let mut rendered = String::new();
	let mut detailed = String::new();
	let mut placeholders = Vec::new();
	loop {
		let tag = *bytes.get(cursor)?;
		cursor += 1;
		match tag {
			0 => break,
			1..=0x7f => {
				let len = tag as usize;
				let piece = str::from_utf8(bytes.get(cursor..cursor.checked_add(len)?)?).ok()?;
				push_escaped_literal(&mut rendered, piece);
				push_escaped_literal(&mut detailed, piece);
				cursor += len;
			},
			0x80 => {
				let len = read_u16(bytes, &mut cursor)? as usize;
				let piece = str::from_utf8(bytes.get(cursor..cursor.checked_add(len)?)?).ok()?;
				push_escaped_literal(&mut rendered, piece);
				push_escaped_literal(&mut detailed, piece);
				cursor += len;
			},
			0x81..=0xbf => return None,
			0xc0..=0xff => {
				let flags = transpose_option((tag & 1 != 0).then(|| read_u32(bytes, &mut cursor)))?;
				if let Some(flags) = flags {
					let fill = flags & 0x1f_ffff;
					let valid_fill = char::from_u32(fill).is_some();
					let width_matches = (flags & (1 << 27) != 0) == (tag & 2 != 0);
					let precision_matches = (flags & (1 << 28) != 0) == (tag & 4 != 0);
					if flags & 0x8000_0000 != 0 || !valid_fill || !width_matches || !precision_matches {
						return None;
					}
				}
				else if tag & 6 != 0 {
					return None;
				}
				let width = transpose_option((tag & 2 != 0).then(|| read_u16(bytes, &mut cursor)))?;
				let precision = transpose_option((tag & 4 != 0).then(|| read_u16(bytes, &mut cursor)))?;
				let explicit = transpose_option((tag & 8 != 0).then(|| read_u16(bytes, &mut cursor)))?;
				let argument = explicit.unwrap_or(next_argument);
				next_argument = argument.checked_add(1)?;
				let width_argument = if tag & 0x10 != 0 { Some(width?) } else { None };
				let precision_argument = if tag & 0x20 != 0 { Some(precision?) } else { None };
				required_arguments = required_arguments.max(argument.checked_add(1)?);
				if let Some(index) = width_argument {
					required_arguments = required_arguments.max(index.checked_add(1)?);
				}
				if let Some(index) = precision_argument {
					required_arguments = required_arguments.max(index.checked_add(1)?);
				}
				if required_arguments > MAX_ARGUMENTS {
					return None;
				}
				let placeholder = Placeholder {
					argument,
					flags,
					width: if width_argument.is_none() { width } else { None },
					precision: if precision_argument.is_none() { precision } else { None },
					width_argument,
					precision_argument,
				};
				push_placeholder(&mut rendered, &placeholder, false);
				push_placeholder(&mut detailed, &placeholder, true);
				placeholders.push(placeholder);
			},
		}
	}
	if placeholders.is_empty() {
		return None;
	}
	Some(FormatTemplate {
		rendered,
		detailed,
		placeholders,
		argument_count: required_arguments,
		encoded_len: cursor,
	})
}

fn push_placeholder(output: &mut String, placeholder: &Placeholder, detailed: bool) {
	use std::fmt::Write;

	let _ = write!(output, "{{{}", placeholder.argument);
	if detailed {
		if let Some(flags) = placeholder.flags {
			let _ = write!(output, ", flags={flags:#010x}");
		}
		if let Some(width) = placeholder.width {
			let _ = write!(output, ", width={width}");
		}
		if let Some(argument) = placeholder.width_argument {
			let _ = write!(output, ", width=arg[{argument}]");
		}
		if let Some(precision) = placeholder.precision {
			let _ = write!(output, ", precision={precision}");
		}
		if let Some(argument) = placeholder.precision_argument {
			let _ = write!(output, ", precision=arg[{argument}]");
		}
	}
	output.push('}');
}

fn push_escaped_literal(output: &mut String, piece: &str) {
	for ch in piece.chars() {
		match ch {
			'{' => output.push_str("{{"),
			'}' => output.push_str("}}"),
			_ => output.push(ch),
		}
	}
}

fn read_u16(bytes: &[u8], cursor: &mut usize) -> Option<u16> {
	let value = u16::from_le_bytes(bytes.get(*cursor..cursor.checked_add(2)?)?.try_into().ok()?);
	*cursor += 2;
	Some(value)
}

fn read_u32(bytes: &[u8], cursor: &mut usize) -> Option<u32> {
	let value = u32::from_le_bytes(bytes.get(*cursor..cursor.checked_add(4)?)?.try_into().ok()?);
	*cursor += 4;
	Some(value)
}

fn transpose_option<T>(value: Option<Option<T>>) -> Option<Option<T>> {
	match value {
		Some(Some(value)) => Some(Some(value)),
		Some(None) => None,
		None => Some(None),
	}
}

#[test]
fn parses_current_rust_format_template() {
	let template = b"\x06hello \xc0\x01!\0";
	let parsed = parse_format_template(template).unwrap();
	assert_eq!(parsed.rendered, "hello {0}!");
	assert_eq!(parsed.detailed, "hello {0}!");
	assert_eq!(parsed.argument_count, 1);
	assert_eq!(parsed.encoded_len, template.len());
}

#[test]
fn parses_explicit_and_dynamic_arguments() {
	let template = [0xdbu8, 0x20, 0, 0, 0x68, 2, 0, 3, 0, 0];
	let parsed = parse_format_template(&template).unwrap();
	assert_eq!(parsed.placeholders[0].argument, 3);
	assert_eq!(parsed.placeholders[0].width_argument, Some(2));
	assert_eq!(parsed.detailed, "{3, flags=0x68000020, width=arg[2]}");
	assert_eq!(parsed.argument_count, 4);
}

#[test]
fn rejects_reserved_tags_and_plain_strings() {
	assert!(parse_format_template(b"hello\0").is_none());
	assert!(parse_format_template(b"\x81\0").is_none());
}
