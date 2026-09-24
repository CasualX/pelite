//! Helpers for recognizing data emitted by the current Rust x64 MSVC toolchain.

use pelite::pe64::{image, PeFile, Rva};

use super::*;

#[derive(Clone, Debug)]
pub struct Xref {
	pub code_rva: Rva,
	pub target_rva: Rva,
}

#[derive(Clone, Debug)]
pub struct Immediate {
	pub code_rva: Rva,
	pub value: u32,
}

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

pub fn open_x64<'a>(bytes: &'a [u8]) -> Result<PeFile<'a>> {
	let file = PeFile::from_bytes(bytes).map_err(|error| err(format!("input is not a PE32+ image: {error}")))?;
	if file.file_header().Machine != image::IMAGE_FILE_MACHINE_AMD64 {
		return Err(err("input is not an x86-64 PE image"));
	}
	Ok(file)
}

/// Finds candidate relative references made by `lea` instructions.
///
/// The pattern intentionally leaves the REX and ModR/M details to semantic
/// filtering of the referenced Rust data structure.
pub fn candidate_relative_xrefs(file: PeFile<'_>) -> Vec<Xref> {
	let pattern = pelite::pattern!("8D ? $'");
	let mut output = Vec::new();
	let mut save = [0; 3];
	let mut matches = file.scanner().code().matches(pattern);
	while matches.next(&mut save).is_some() {
		if !file.section_headers().by_rva(save[1]).is_some_and(|section| section.Characteristics & image::IMAGE_SCN_MEM_EXECUTE != 0) && file.slice_bytes(save[1]).is_ok() {
			output.push(Xref {
				code_rva: save[0],
				target_rva: save[1],
			});
		}
	}
	output.sort_unstable_by_key(|xref| (xref.target_rva, xref.code_rva));
	output.dedup_by_key(|xref| (xref.target_rva, xref.code_rva));
	output
}

pub fn relative_call_xrefs(file: PeFile<'_>) -> Vec<Xref> {
	let pattern = pelite::pattern!("E8 $'");
	let mut output = Vec::new();
	let mut save = [0; 3];
	let mut matches = file.scanner().code().matches(pattern);
	while matches.next(&mut save).is_some() {
		if file.section_headers().by_rva(save[1]).is_some_and(|section| section.Characteristics & image::IMAGE_SCN_MEM_EXECUTE != 0) {
			output.push(Xref {
				code_rva: save[0],
				target_rva: save[1],
			});
		}
	}
	output
}

pub fn edx_immediates(file: PeFile<'_>) -> Vec<Immediate> {
	let pattern = pelite::pattern!("BA u4[1]");
	let mut output = Vec::new();
	let mut save = [0; 2];
	let mut matches = file.scanner().code().matches(pattern);
	while matches.next(&mut save).is_some() {
		output.push(Immediate { code_rva: save[0], value: save[1] });
	}
	output
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

pub fn read_format_template(file: PeFile<'_>, rva: Rva) -> Option<FormatTemplate> {
	parse_format_template(file.slice_bytes(rva).ok()?)
}

pub fn is_rust_location(file: PeFile<'_>, rva: Rva) -> bool {
	let Ok(record) = file.derva_slice::<u8>(rva, 24) else {
		return false;
	};
	let pointer = u64::from_le_bytes(record[0..8].try_into().unwrap());
	let length64 = u64::from_le_bytes(record[8..16].try_into().unwrap());
	let line = u32::from_le_bytes(record[16..20].try_into().unwrap());
	let column = u32::from_le_bytes(record[20..24].try_into().unwrap());
	let Ok(path_rva) = file.va_to_rva(pointer) else {
		return false;
	};
	let Ok(length) = usize::try_from(length64) else {
		return false;
	};
	line != 0 && line <= 10_000_000 && column != 0 && column <= 1_000_000 && read_utf8(file, path_rva, length).is_some_and(|path| path.contains(".rs") && path.chars().all(|ch| !ch.is_control()))
}

pub fn read_utf8(file: PeFile<'_>, rva: Rva, len: usize) -> Option<&str> {
	if len == 0 || len > 64 * 1024 {
		return None;
	}
	str::from_utf8(file.slice_bytes(rva).ok()?.get(..len)?).ok()
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

#[cfg(test)]
mod tests {
	use super::*;

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
}
