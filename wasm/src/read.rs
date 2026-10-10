//! Typed RVA reads, adapted from pelite-cli/src/re/read.rs.

use super::*;
use binfact::ty;
use std::fmt;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn err(message: impl Into<String>) -> Box<dyn std::error::Error> {
	message.into().into()
}

#[derive(serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ReadOptions {
	pub zerofill: bool,
	pub string_preview_length: usize,
	pub max_dynamic_array_length: u32,
}

impl Default for ReadOptions {
	fn default() -> Self {
		Self { zerofill: false, string_preview_length: 256, max_dynamic_array_length: 1024 }
	}
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ReadArgs {
	#[serde(rename = "type")]
	ty: String,
	#[serde(default)]
	options: ReadOptions,
}

fn interpret(pe: pelite::PeFile<'_>, rva: u32, args: &[u8]) -> Result<serde_json::Value> {
	let args: ReadArgs = serde_json::from_slice(args)?;
	let width = match pe {
		pelite::Wrap::T32(_) => ty::PointerWidth::Bits32,
		pelite::Wrap::T64(_) => ty::PointerWidth::Bits64,
	};
	let ty = ty::Type::parse(&args.ty, width)?;
	Ok(read_at(pe, rva, &ty, &args.options))
}

#[unsafe(export_name = "pefileRead")]
pub unsafe fn read(pefile: *mut PeFile, rva: u32, args: *const [u8]) {
	let pe = match pelite::PeFile::from_bytes(unsafe { &*pefile }.as_ref()) {
		Ok(pe) => pe,
		Err(err) => return return_error(err),
	};
	match interpret(pe, rva, unsafe { &*args }) {
		Ok(value) => {
			// A failed root read follows the usual wasm Error convention. Nested
			// failures stay inline so callers can inspect successful fields too.
			if let Some(error) = value.get("$error").and_then(|v| v.as_str()) {
				return_error(error);
			}
			else {
				return_json(value);
			}
		},
		Err(err) => return_error(err),
	}
}

struct StructContext<'a> {
	/// RVA of the struct containing the field being read.
	rva: u32,
	fields: &'a [ty::Field],
}

fn error_value(rva: Option<u32>, error: impl fmt::Display) -> serde_json::Value {
	serde_json::json!({ "$error": error.to_string(), "$rva": rva })
}

pub fn read_at(pe: pelite::PeFile<'_>, rva: u32, ty: &ty::Type, options: &ReadOptions) -> serde_json::Value {
	read_value(pe, rva, ty, options, None)
}

fn read_value(pe: pelite::PeFile<'_>, rva: u32, ty: &ty::Type, options: &ReadOptions, context: Option<&StructContext<'_>>) -> serde_json::Value {
	try_read_value(pe, rva, ty, options, context).unwrap_or_else(|error| error_value(Some(rva), error))
}

fn try_read_value(pe: pelite::PeFile<'_>, rva: u32, ty: &ty::Type, options: &ReadOptions, context: Option<&StructContext<'_>>) -> Result<serde_json::Value> {
	macro_rules! read {
		($ty:ty) => { pe.derva_copy::<$ty>(rva, options.zerofill)? };
	}
	let value = match ty {
		ty::Type::U8 => serde_json::to_value(read!(u8)),
		ty::Type::U16 => serde_json::to_value(read!(u16)),
		ty::Type::U32 => serde_json::to_value(read!(u32)),
		ty::Type::U64 => serde_json::to_value(read!(u64)),
		ty::Type::I8 => serde_json::to_value(read!(i8)),
		ty::Type::I16 => serde_json::to_value(read!(i16)),
		ty::Type::I32 => serde_json::to_value(read!(i32)),
		ty::Type::I64 => serde_json::to_value(read!(i64)),
		ty::Type::F32 => serde_json::to_value(read!(f32)),
		ty::Type::F64 => serde_json::to_value(read!(f64)),
		ty::Type::Code | ty::Type::Fn | ty::Type::Unknown | ty::Type::Ptr32(_) | ty::Type::Ptr64(_) => {
			let va = match ty {
				ty::Type::Ptr32(_) => u64::from(read!(u32)),
				ty::Type::Ptr64(_) => read!(u64),
				_ => return Err(err(format!("cannot read opaque type '{ty}' directly"))),
			};
			if va == 0 {
				return Ok(serde_json::Value::Null);
			}
			let target = pe.va_to_rva(va)?;
			if let ty::Type::Ptr32(pointee) | ty::Type::Ptr64(pointee) = ty && !pointee.is_opaque() {
				return Ok(read_value(pe, target, pointee, options, context));
			}
			serde_json::to_value(target)
		},
		ty::Type::CStr => serde_json::to_value(read_cstr(pe.slice_bytes(rva)?, options.string_preview_length)?),
		ty::Type::Utf16LEZ => serde_json::to_value(read_utf16lez(pe.slice_bytes(rva)?, options.string_preview_length)?),
		ty::Type::Array(array) => {
			let (stride, _) = array.ty.layout().map_err(err)?;
			let len = match &array.len {
				ty::ArrayLen::Fixed(len) => *len,
				ty::ArrayLen::Dyn(name) => {
					let context = context.ok_or_else(|| err("dynamic array requires a containing struct"))?;
					let field = context.fields.iter().find(|field| matches!(&field.name, ty::FieldName::Named(field_name) if field_name == name))
						.ok_or_else(|| err(format!("array length field '{name}' is not in the containing struct")))?;
					let length_rva = context.rva.checked_add(field.offset).ok_or_else(|| err("array length RVA overflow"))?;
					let len = match field.ty {
						ty::Type::U8 => u64::from(pe.derva_copy::<u8>(length_rva, options.zerofill)?),
						ty::Type::U16 => u64::from(pe.derva_copy::<u16>(length_rva, options.zerofill)?),
						ty::Type::U32 => u64::from(pe.derva_copy::<u32>(length_rva, options.zerofill)?),
						ty::Type::U64 => pe.derva_copy::<u64>(length_rva, options.zerofill)?,
						_ => return Err(err(format!("array length field '{name}' must be u8, u16, u32, or u64"))),
					};
					if len > options.max_dynamic_array_length as u64 {
						return Err(err(format!("dynamic array length {len} exceeds maximum of {}", options.max_dynamic_array_length)));
					}
					len as u32
				},
			};
			stride.checked_mul(len).ok_or_else(|| err("array size overflow"))?;
			let mut values = Vec::new();
			for index in 0..len {
				let value = match index.checked_mul(stride).and_then(|offset| rva.checked_add(offset)) {
					Some(element_rva) => read_value(pe, element_rva, &array.ty, options, context),
					None => error_value(None, "read RVA overflow"),
				};
				values.push(value);
			}
			return Ok(serde_json::Value::Array(values));
		},
		ty::Type::Struct(structure) => {
			let context = StructContext { rva, fields: &structure.fields };
			let mut fields = serde_json::Map::new();
			for (index, field) in structure.fields.iter().enumerate() {
				let name = match &field.name {
					ty::FieldName::Named(name) => name.clone(),
					ty::FieldName::Discarded => continue,
					ty::FieldName::Unnamed => index.to_string(),
				};
				let value = match rva.checked_add(field.offset) {
					Some(field_rva) => read_value(pe, field_rva, &field.ty, options, Some(&context)),
					None => error_value(None, "read RVA overflow"),
				};
				fields.insert(name, value);
			}
			return Ok(serde_json::Value::Object(fields));
		},
	};
	Ok(value?)
}

fn read_cstr(bytes: &[u8], string_preview_length: usize) -> Result<String> {
	let string = pelite::util::CStr::from_bytes(bytes).ok_or_else(|| err("unterminated C string"))?;
	if string.len() <= string_preview_length {
		return Ok(string.to_string());
	}
	let bytes = string.as_ref();
	let len = bytes.len().min(string_preview_length);
	// Match CStr's display of non-ASCII bytes.
	let mut preview = String::new();
	for &byte in &bytes[..len] {
		if byte.is_ascii() {
			preview.push(char::from(byte));
		}
		else {
			use std::fmt::Write;
			write!(preview, "\\x{byte:02X}")?;
		}
	}
	preview.push('…');
	Ok(preview)
}

fn read_utf16lez(bytes: &[u8], string_preview_length: usize) -> Result<String> {
	let mut words = Vec::new();
	for (index, pair) in bytes.chunks_exact(2).enumerate() {
		let word = u16::from_le_bytes([pair[0], pair[1]]);
		if word == 0 {
			let mut preview = String::from_utf16_lossy(&words);
			if index > string_preview_length {
				preview.push('…');
			}
			return Ok(preview);
		}
		if index < string_preview_length {
			words.push(word);
		}
	}
	Err(err("unterminated UTF-16LE string"))
}

#[test]
fn bounded_cstr_previews() {
	assert_eq!(read_cstr(b"abc\0ignored", 4).unwrap(), "abc");
	assert_eq!(read_cstr(b"abc\0", 3).unwrap(), "abc");
	assert_eq!(read_cstr(b"abcdef\0", 3).unwrap(), "abc…");
	assert_eq!(read_cstr(b"abc\0", 0).unwrap(), "…");
	assert_eq!(read_cstr(b"\0", 0).unwrap(), "");
	assert_eq!(read_cstr(b"a\xffb\0", 2).unwrap(), "a\\xFF…");
	assert_eq!(read_cstr(b"a\xff\0", 3).unwrap(), "a\\xFF");
	for bytes in [b"".as_slice(), b"abc", b"abcdef"] {
		assert!(read_cstr(bytes, 3).unwrap_err().to_string().contains("unterminated C string"));
	}
}

#[test]
fn bounded_utf16lez_previews() {
	let bytes = [b'a', 0, b'b', 0, 0, 0];
	assert_eq!(read_utf16lez(&bytes, 3).unwrap(), "ab");
	assert_eq!(read_utf16lez(&bytes, 2).unwrap(), "ab");
	assert_eq!(read_utf16lez(&bytes, 1).unwrap(), "a…");
	assert_eq!(read_utf16lez(&bytes, 0).unwrap(), "…");
	assert_eq!(read_utf16lez(&[0, 0], 0).unwrap(), "");
	assert_eq!(read_utf16lez(&[0x3d, 0xd8, 0, 0xde, 0, 0], 2).unwrap(), "😀");
	for bytes in [b"".as_slice(), &bytes[..4], &bytes[..3]] {
		for limit in [0, 2, 6] {
			assert!(read_utf16lez(bytes, limit).unwrap_err().to_string().contains("unterminated UTF-16LE string"));
		}
	}
}
