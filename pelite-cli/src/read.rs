use super::*;

mod read_type;
use read_type::{PointerWidth, ReadType};

const DEFAULT_MAX_STRING_BYTES: &str = "256";

impl From<pelite::PeFile<'_>> for PointerWidth {
	fn from(pe: pelite::PeFile<'_>) -> PointerWidth {
		match pe {
			pelite::Wrap::T32(_) => PointerWidth::Bits32,
			pelite::Wrap::T64(_) => PointerWidth::Bits64,
		}
	}
}

struct ReadOptions {
	max_string_bytes: usize,
}

pub fn command() -> clap::Command {
	clap::Command::new("read")
		.about("Read typed data at a PE address")
		.after_help(include_str!("../docs/read.md"))
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("address")
			.value_name("ADDRESS")
			.value_parser(Address::parse)
			.required(true))
		.arg(clap::Arg::new("max-string-bytes")
			.long("max-string-bytes")
			.value_name("MAX_STRING_BYTES")
			.value_parser(clap::value_parser!(usize))
			.default_value(DEFAULT_MAX_STRING_BYTES)
			.help("Maximum bytes to inspect for a string"))
		.arg(clap::Arg::new("type")
			.value_name("TYPE")
			.required(true))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let address = *matches.get_one::<Address>("address").expect("required by clap");
	let source = matches.get_one::<String>("type").expect("required by clap");
	let options = ReadOptions {
		max_string_bytes: *matches.get_one::<usize>("max-string-bytes").expect("defaulted by clap"),
	};
	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let ty = ReadType::parse(source, PointerWidth::from(pe)).map_err(err)?;
	let rva = match address {
		Address::Rva(rva) => Ok(rva),
		Address::Va(va) => pe.va_to_rva(va),
		Address::Fo(fo) => pe.headers().file_offset_to_rva(fo),
	};
	let value = match rva {
		Ok(rva) => read_value(pe, rva, &ty, &options),
		Err(error) => error_value(None, error),
	};
	print("Read", &value, format)?;
	if contains_read_errors(&value) {
		return Err(err("one or more reads failed; see $error values in the output"));
	}
	Ok(())
}

fn contains_read_errors(value: &serde_json::Value) -> bool {
	match value {
		// Type syntax cannot declare a field named "$error", so it is reserved.
		serde_json::Value::Object(fields) => fields.contains_key("$error") || fields.values().any(contains_read_errors),
		serde_json::Value::Array(values) => values.iter().any(contains_read_errors),
		_ => false,
	}
}

fn error_value(rva: Option<u32>, error: impl fmt::Display) -> serde_json::Value {
	serde_json::json!({ "$error": error.to_string(), "$address": rva })
}

fn read_value(pe: pelite::PeFile<'_>, rva: u32, ty: &ReadType, options: &ReadOptions) -> serde_json::Value {
	try_read_value(pe, rva, ty, options).unwrap_or_else(|error| error_value(Some(rva), error))
}

fn read_offset(pe: pelite::PeFile<'_>, rva: u32, offset: Option<u32>, ty: &ReadType, options: &ReadOptions) -> serde_json::Value {
	match offset.and_then(|offset| rva.checked_add(offset)) {
		Some(address) => read_value(pe, address, ty, options),
		None => error_value(None, "read address exceeds RVA range"),
	}
}

fn try_read_value(pe: pelite::PeFile<'_>, rva: u32, ty: &ReadType, options: &ReadOptions) -> Result<serde_json::Value> {
	macro_rules! read {
		($ty:ty) => { <$ty>::from_le_bytes(pe.derva_copy(rva)?) };
	}
	let value = match ty {
		ReadType::U8 => serde_json::to_value(read!(u8)),
		ReadType::U16 => serde_json::to_value(read!(u16)),
		ReadType::U32 => serde_json::to_value(read!(u32)),
		ReadType::U64 => serde_json::to_value(read!(u64)),
		ReadType::I8 => serde_json::to_value(read!(i8)),
		ReadType::I16 => serde_json::to_value(read!(i16)),
		ReadType::I32 => serde_json::to_value(read!(i32)),
		ReadType::I64 => serde_json::to_value(read!(i64)),
		ReadType::F32 => serde_json::to_value(read!(f32)),
		ReadType::F64 => serde_json::to_value(read!(f64)),
		ReadType::Va | ReadType::Ptr(_) => {
			let va = match pe {
				pelite::Wrap::T32(_) => u64::from(read!(u32)),
				pelite::Wrap::T64(_) => read!(u64),
			};
			if va == 0 {
				return Ok(serde_json::Value::Null);
			}
			let target = pe.va_to_rva(va)?;
			if let ReadType::Ptr(ty) = ty {
				return Ok(read_value(pe, target, ty, options));
			}
			else {
				serde_json::to_value(target)
			}
		},
		ReadType::CStr => {
			let bytes = pe.slice_bytes(rva)?;
			let len = bytes.len().min(options.max_string_bytes);
			let string = pelite::util::CStr::from_bytes(&bytes[..len]).ok_or_else(|| {
				if bytes.len() > options.max_string_bytes {
					err(format!("C string exceeds maximum of {} bytes", options.max_string_bytes))
				}
				else {
					err("unterminated C string")
				}
			})?;
			serde_json::to_value(string)
		},
		ReadType::Array(array) => {
			let pointer_width = PointerWidth::from(pe);
			ty.layout(pointer_width).map_err(err)?;
			let (stride, _) = array.ty.layout(pointer_width).map_err(err)?;
			let mut values = Vec::new();
			for index in 0..array.len {
				let offset = index.checked_mul(stride);
				values.push(read_offset(pe, rva, offset, &array.ty, options));
			}
			return Ok(serde_json::Value::Array(values));
		},
		ReadType::Struct(structure) => {
			let mut fields = serde_json::Map::new();
			for field in &structure.fields {
				fields.insert(field.name.clone(), read_offset(pe, rva, Some(field.offset), &field.ty, options));
			}
			return Ok(serde_json::Value::Object(fields));
		},
	};
	Ok(value?)
}
