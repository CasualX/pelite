use super::*;

mod read_type;
use read_type::{PointerWidth, ReadArrayLen, ReadType};

const DEFAULT_MAX_STRING_BYTES: &str = "256";
const DEFAULT_MAX_DYNAMIC_ARRAY_LENGTH: &str = "1024";

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
	max_dynamic_array_length: u32,
}

struct StructContext {
	/// Address of the struct containing the field being read.
	address: u32,
}

pub fn command() -> clap::Command {
	clap::Command::new("read")
		.about("Read typed data at a PE address")
		.after_help(include_str!("docs/read.md"))
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
		.arg(clap::Arg::new("max-dynamic-array-length")
			.long("max-dynamic-array-length")
			.value_name("LENGTH")
			.value_parser(clap::value_parser!(u64))
			.default_value(DEFAULT_MAX_DYNAMIC_ARRAY_LENGTH)
			.help("Maximum element count for a field-length array"))
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
		max_dynamic_array_length: *matches.get_one::<u32>("max-dynamic-array-length").expect("defaulted by clap"),
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
		Ok(rva) => read_value(pe, rva, &ty, &options, None),
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

fn read_value(pe: pelite::PeFile<'_>, rva: u32, ty: &ReadType, options: &ReadOptions, context: Option<&StructContext>) -> serde_json::Value {
	try_read_value(pe, rva, ty, options, context).unwrap_or_else(|error| error_value(Some(rva), error))
}

fn try_read_value(pe: pelite::PeFile<'_>, rva: u32, ty: &ReadType, options: &ReadOptions, context: Option<&StructContext>) -> Result<serde_json::Value> {
	macro_rules! read {
		($ty:ty) => { pe.derva_copy::<$ty>(rva)? };
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
				let value = read_value(pe, target, ty, options, context);
				return Ok(value);
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
			let (stride, _) = array.ty.layout(pointer_width).map_err(err)?;
			let len = match array.len {
				ReadArrayLen::Const(len) => len,
				ReadArrayLen::DynU8(offset) | ReadArrayLen::DynU16(offset) | ReadArrayLen::DynU32(offset) | ReadArrayLen::DynU64(offset) => {
					let context = context.ok_or_else(|| err("dynamic array requires a containing struct"))?;
					let address = context.address.checked_add(offset).ok_or_else(|| err("array length address overflow"))?;
					let len = match array.len {
						ReadArrayLen::DynU8(_) => u64::from(pe.derva_copy::<u8>(address)?),
						ReadArrayLen::DynU16(_) => u64::from(pe.derva_copy::<u16>(address)?),
						ReadArrayLen::DynU32(_) => u64::from(pe.derva_copy::<u32>(address)?),
						ReadArrayLen::DynU64(_) => pe.derva_copy::<u64>(address)?,
						ReadArrayLen::Const(_) => unreachable!(),
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
					Some(address) => read_value(pe, address, &array.ty, options, context),
					None => error_value(None, "read address overflow"),
				};
				values.push(value);
			}
			return Ok(serde_json::Value::Array(values));
		},
		ReadType::Struct(structure) => {
			let context = StructContext { address: rva };
			let mut fields = serde_json::Map::new();
			for field in &structure.fields {
				let value = match rva.checked_add(field.offset) {
					Some(address) => read_value(pe, address, &field.ty, options, Some(&context)),
					None => error_value(None, "read address overflow"),
				};
				fields.insert(field.name.clone(), value);
			}
			return Ok(serde_json::Value::Object(fields));
		},
	};
	Ok(value?)
}
