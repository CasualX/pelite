use std::collections::HashSet;
use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PointerWidth {
	Bits32,
	Bits64,
}

impl PointerWidth {
	#[inline]
	pub const fn bytes(self) -> u32 {
		match self {
			Self::Bits32 => 4,
			Self::Bits64 => 8,
		}
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReadArrayLen {
	Const(u32),
	DynU8(u32),
	DynU16(u32),
	DynU32(u32),
	DynU64(u32),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadArrayType {
	pub ty: ReadType,
	pub len: ReadArrayLen,
	/// Fixed layout computed when parsed; size is zero for a dynamic array.
	pub size: u32,
	pub align: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReadType {
	U8, U16, U32, U64,
	I8, I16, I32, I64,
	F32, F64,

	Va,
	CStr,

	Array(Box<ReadArrayType>),
	Ptr(Box<ReadType>),
	Struct(Box<ReadStructType>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadStructType {
	pub name: Option<String>,
	pub fields: Vec<ReadStructFieldType>,
	/// Layout computed when the struct or union is parsed for the PE pointer width.
	pub size: u32,
	pub align: u32,
	/// True if the last field is dynamic.
	pub is_dst: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadStructFieldType {
	pub name: String,
	pub offset: u32,
	pub ty: ReadType,
}

impl ReadType {
	/// Alignment of the starting address, including dynamically sized types.
	pub fn alignment(&self, pointer_width: PointerWidth) -> u32 {
		match self {
			Self::CStr => 1,
			Self::Array(array) => array.align,
			Self::Struct(structure) => structure.align,
			_ => self.layout(pointer_width).expect("fixed-size type").1,
		}
	}

	pub fn is_dst(&self) -> bool {
		match self {
			Self::CStr => true,
			Self::Array(array) => !matches!(array.len, ReadArrayLen::Const(_)),
			Self::Struct(structure) => structure.is_dst,
			_ => false,
		}
	}

	/// Fixed size and alignment for the selected PE pointer width.
	pub fn layout(&self, pointer_width: PointerWidth) -> Result<(u32, u32), &'static str> {
		match self {
			Self::U8 | Self::I8 => Ok((1, 1)),
			Self::U16 | Self::I16 => Ok((2, 2)),
			Self::U32 | Self::I32 | Self::F32 => Ok((4, 4)),
			Self::U64 | Self::I64 | Self::F64 => Ok((8, 8)),
			Self::Va | Self::Ptr(_) => Ok((pointer_width.bytes(), pointer_width.bytes())),
			Self::CStr => Err("cstr is unsized"),
			Self::Array(array) => match array.len {
				ReadArrayLen::Const(_) => Ok((array.size, array.align)),
				_ => Err("dynamic array is unsized"),
			},
			Self::Struct(structure) => if self.is_dst() { Err("struct with a dynamic field is unsized") } else { Ok((structure.size, structure.align)) },
		}
	}
}

fn align_up(offset: u32, align: u32) -> Result<u32, &'static str> {
	offset.checked_add(align - 1).map(|value| value & !(align - 1))
		.ok_or_else(|| "type layout overflow")
}

impl ReadType {
	/// Parse and lay out a type after the PE pointer width is known.
	pub fn parse(input: &str, pointer_width: PointerWidth) -> Result<ReadType, String> {
		parse(input, pointer_width)
	}
}

fn parse(input: &str, pointer_width: PointerWidth) -> Result<ReadType, String> {
	let mut parser = Parser { input, pos: 0, pointer_width };
	let ty = parser.ty(0, &[])?;
	parser.whitespace();
	if parser.pos != input.len() {
		return Err(parser.error("unexpected trailing input"));
	}
	Ok(ty)
}

struct Parser<'a> {
	input: &'a str,
	pos: usize,
	pointer_width: PointerWidth,
}

impl<'a> Parser<'a> {
	fn error(&self, message: impl fmt::Display) -> String {
		format!("{message} at byte {}", self.pos)
	}

	fn whitespace(&mut self) {
		while self.input.as_bytes().get(self.pos).is_some_and(u8::is_ascii_whitespace) {
			self.pos += 1;
		}
	}

	fn eat(&mut self, token: u8) -> bool {
		self.whitespace();
		if self.input.as_bytes().get(self.pos) != Some(&token) {
			return false;
		}
		self.pos += 1;
		true
	}

	fn expect(&mut self, token: u8) -> Result<(), String> {
		if !self.eat(token) {
			return Err(self.error(format_args!("expected '{}'", char::from(token))));
		}
		Ok(())
	}

	fn identifier(&mut self) -> Result<&'a str, String> {
		self.whitespace();
		let start = self.pos;
		if !self.input.as_bytes().get(self.pos).is_some_and(|byte| byte.is_ascii_alphabetic() || *byte == b'_') {
			return Err(self.error("expected an identifier"));
		}
		self.pos += 1;
		while self.input.as_bytes().get(self.pos).is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte == b'_') {
			self.pos += 1;
		}
		Ok(&self.input[start..self.pos])
	}

	fn array_len(&mut self) -> Result<u32, String> {
		self.whitespace();
		let start = self.pos;
		while self.input.as_bytes().get(self.pos).is_some_and(u8::is_ascii_digit) {
			self.pos += 1;
		}
		self.input[start..self.pos].parse::<u32>()
			.map_err(|_| self.error("expected a decimal array length"))
	}

	fn ty(&mut self, depth: usize, fields: &[ReadStructFieldType]) -> Result<ReadType, String> {
		if depth >= 64 {
			return Err(self.error("type nesting is too deep"));
		}

		if self.eat(b'*') {
			return Ok(ReadType::Ptr(Box::new(self.ty(depth + 1, fields)?)));
		}

		if self.eat(b'[') {
			let ty = self.ty(depth + 1, fields)?;
			self.expect(b';')?;
			self.whitespace();
			let len = if self.input.as_bytes().get(self.pos).is_some_and(u8::is_ascii_digit) {
				ReadArrayLen::Const(self.array_len()?)
			}
			else {
				let name = self.identifier()?;
				let field = fields.iter().find(|field| field.name == name)
					.ok_or_else(|| self.error(format_args!("array length field '{name}' must be an earlier field")))?;
				match field.ty {
					ReadType::U8 => ReadArrayLen::DynU8(field.offset),
					ReadType::U16 => ReadArrayLen::DynU16(field.offset),
					ReadType::U32 => ReadArrayLen::DynU32(field.offset),
					ReadType::U64 => ReadArrayLen::DynU64(field.offset),
					_ => return Err(self.error(format_args!("array length field '{name}' must be u8, u16, u32, or u64"))),
				}
			};
			self.expect(b']')?;
			let (element_size, align) = ty.layout(self.pointer_width).map_err(|message| self.error(message))?;
			let size = match len {
				ReadArrayLen::Const(len) => element_size.checked_mul(len).ok_or_else(|| self.error("array size overflow"))?,
				_ => 0,
			};
			return Ok(ReadType::Array(Box::new(ReadArrayType { ty, len, size, align })));
		}

		let name = self.identifier()?;
		match name {
			"u8" => Ok(ReadType::U8),
			"u16" => Ok(ReadType::U16),
			"u32" => Ok(ReadType::U32),
			"u64" => Ok(ReadType::U64),
			"i8" => Ok(ReadType::I8),
			"i16" => Ok(ReadType::I16),
			"i32" => Ok(ReadType::I32),
			"i64" => Ok(ReadType::I64),
			"f32" => Ok(ReadType::F32),
			"f64" => Ok(ReadType::F64),
			"ptr" => Ok(ReadType::Va),
			"cstr" => Ok(ReadType::CStr),
			"struct" => self.structure(depth + 1, false),
			"union" => self.structure(depth + 1, true),
			_ => Err(self.error(format_args!("unknown type '{name}'"))),
		}
	}

	fn structure(&mut self, depth: usize, is_union: bool) -> Result<ReadType, String> {
		// Optional struct name
		let name = if self.eat(b'{') { None }
		else {
			let name = self.identifier()?;
			self.expect(b'{')?;
			Some(name)
		};

		// Parse fields
		let mut fields: Vec<ReadStructFieldType> = Vec::new();
		let mut field_names = HashSet::new();
		let mut offset = 0;
		let mut size = 0;
		let mut align = 1;
		let mut is_dst = false;
		while !self.eat(b'}') {
			// Field name
			let name = self.identifier()?.to_owned();
			// Field type
			self.expect(b':')?;
			let ty = self.ty(depth, &fields)?;
			// Field properties
			let dynamic = ty.is_dst();
			is_dst |= dynamic;
			if dynamic && is_union {
				return Err(self.error("dynamic field is not allowed in a union"));
			}
			let (field_size, field_align) = if dynamic {
				(0, match &ty { ReadType::Array(array) => array.align, ReadType::CStr => 1, _ => unreachable!() })
			} else { ty.layout(self.pointer_width).map_err(|message| self.error(message))? };
			let field_offset = if is_union { 0 } else { align_up(offset, field_align).map_err(|message| self.error(message))? };
			let field_end = field_offset.checked_add(field_size).ok_or_else(|| self.error("struct size overflow"))?;
			// Field discard
			if name != "_" {
				// Enforce unique name
				if !field_names.insert(name.clone()) {
					return Err(self.error(format_args!("duplicate field '{name}'")));
				}
				fields.push(ReadStructFieldType { name, offset: field_offset, ty });
			}
			// Update struct properties
			if !is_union {
				offset = field_end;
			}
			size = size.max(field_end);
			align = align.max(field_align);
			if !self.eat(b',') {
				self.expect(b'}')?;
				break;
			}
			if dynamic {
				if !self.eat(b'}') {
					return Err(self.error("dynamic field must be the last struct field"));
				}
				break;
			}
		}

		let size = align_up(size, align).map_err(|message| self.error(message))?;
		let name = name.map(str::to_owned);
		Ok(ReadType::Struct(Box::new(ReadStructType { name, fields, size, align, is_dst })))
	}
}

#[cfg(test)]
mod tests;
