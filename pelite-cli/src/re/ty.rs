use std::collections::HashSet;
use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PointerWidth {
	Bits32,
	Bits64,
}

impl From<pelite::PeFile<'_>> for PointerWidth {
	fn from(pe: pelite::PeFile<'_>) -> Self {
		match pe {
			pelite::Wrap::T32(_) => Self::Bits32,
			pelite::Wrap::T64(_) => Self::Bits64,
		}
	}
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArrayLen {
	Fixed(u32),
	Dyn(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArrayType {
	pub ty: Type,
	pub len: ArrayLen,
	/// Fixed layout computed when parsed; size is zero for a dynamic array.
	pub size: u32,
	pub align: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
	U8, U16, U32, U64,
	I8, I16, I32, I64,
	F32, F64,

	Va,
	CStr,
	Utf16LEZ,
	Code,
	Unknown,

	Array(Box<ArrayType>),
	Ptr(Box<Type>),
	Struct(Box<StructType>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructType {
	pub name: Option<String>,
	pub fields: Vec<Field>,
	pub is_union: bool,
	/// Layout computed when the struct or union is parsed for the PE pointer width.
	pub size: u32,
	pub align: u32,
	/// True if the last field is dynamic.
	pub is_dst: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Field {
	pub name: FieldName,
	pub offset: u32,
	pub ty: Type,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FieldName {
	Named(String),
	Discarded,
	Unnamed,
}

impl Type {
	/// Alignment of the starting address, including dynamically sized types.
	pub fn alignment(&self, pointer_width: PointerWidth) -> u32 {
		match self {
			Self::CStr | Self::Code | Self::Unknown => 1,
			Self::Utf16LEZ => 2,
			Self::Array(array) => array.align,
			Self::Struct(structure) => structure.align,
			_ => self.layout(pointer_width).expect("fixed-size type").1,
		}
	}

	pub fn is_dst(&self) -> bool {
		match self {
			Self::CStr | Self::Utf16LEZ | Self::Code | Self::Unknown => true,
			Self::Array(array) => !matches!(&array.len, ArrayLen::Fixed(_)),
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
			Self::Utf16LEZ => Err("utf16lez is unsized"),
			Self::Code => Err("code is unsized"),
			Self::Unknown => Err("unk is unsized"),
			Self::Array(array) => match &array.len {
				ArrayLen::Fixed(_) => Ok((array.size, array.align)),
				_ => Err("dynamic array is unsized"),
			},
			Self::Struct(structure) => if self.is_dst() { Err("struct with a dynamic field is unsized") } else { Ok((structure.size, structure.align)) },
		}
	}
}

impl fmt::Display for Type {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::U8 => f.write_str("u8"),
			Self::U16 => f.write_str("u16"),
			Self::U32 => f.write_str("u32"),
			Self::U64 => f.write_str("u64"),
			Self::I8 => f.write_str("i8"),
			Self::I16 => f.write_str("i16"),
			Self::I32 => f.write_str("i32"),
			Self::I64 => f.write_str("i64"),
			Self::F32 => f.write_str("f32"),
			Self::F64 => f.write_str("f64"),
			Self::Va => f.write_str("ptr"),
			Self::CStr => f.write_str("cstr"),
			Self::Utf16LEZ => f.write_str("utf16lez"),
			Self::Code => f.write_str("code"),
			Self::Unknown => f.write_str("unk"),
			Self::Ptr(ty) => write!(f, "*{ty}"),
			Self::Array(array) => {
				write!(f, "[{};", array.ty)?;
				match &array.len {
					ArrayLen::Fixed(len) => write!(f, "{len}")?,
					ArrayLen::Dyn(name) => f.write_str(name)?,
				}
				f.write_str("]")
			},
			Self::Struct(structure) => {
				f.write_str(if structure.is_union { "union" } else { "struct" })?;
				if let Some(name) = &structure.name {
					write!(f, " {name}")?;
				}
				f.write_str("{")?;
				for (index, field) in structure.fields.iter().enumerate() {
					if index != 0 {
						f.write_str(",")?;
					}
					match &field.name {
						FieldName::Named(name) => write!(f, "{name}:")?,
						FieldName::Discarded => f.write_str("_:")?,
						FieldName::Unnamed => {},
					}
					write!(f, "{}", field.ty)?;
				}
				f.write_str("}")
			},
		}
	}
}

fn align_up(offset: u32, align: u32) -> Result<u32, &'static str> {
	offset.checked_add(align - 1).map(|value| value & !(align - 1))
		.ok_or_else(|| "type layout overflow")
}

impl Type {
	/// Parse and lay out a type after the PE pointer width is known.
	pub fn parse(input: &str, pointer_width: PointerWidth) -> Result<Type, String> {
		parse(input, pointer_width)
	}
}

fn parse(input: &str, pointer_width: PointerWidth) -> Result<Type, String> {
	let mut parser = Parser { input, pos: 0, pointer_width };
	let ty = parser.ty(0)?;
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

	fn field_name(&mut self, is_union: bool) -> Result<FieldName, String> {
		let start = self.pos;
		if is_union {
			if let Ok(name) = self.identifier() {
				let name = name.to_owned();
				if self.eat(b':') {
					return Ok(if name == "_" { FieldName::Discarded } else { FieldName::Named(name) });
				}
			}
			self.pos = start;
			return Ok(FieldName::Unnamed);
		}
		let name = self.identifier()?.to_owned();
		self.expect(b':')?;
		Ok(if name == "_" { FieldName::Discarded } else { FieldName::Named(name) })
	}

	fn ty(&mut self, depth: usize) -> Result<Type, String> {
		if depth >= 64 {
			return Err(self.error("type nesting is too deep"));
		}

		if self.eat(b'*') {
			return Ok(Type::Ptr(Box::new(self.ty(depth + 1)?)));
		}

		if self.eat(b'[') {
			let ty = self.ty(depth + 1)?;
			self.expect(b';')?;
			self.whitespace();
			let len = if self.input.as_bytes().get(self.pos).is_some_and(u8::is_ascii_digit) {
				ArrayLen::Fixed(self.array_len()?)
			}
			else {
				ArrayLen::Dyn(self.identifier()?.to_owned())
			};
			self.expect(b']')?;
			let (element_size, align) = ty.layout(self.pointer_width).map_err(|message| self.error(message))?;
			let size = match &len {
				ArrayLen::Fixed(len) => element_size.checked_mul(*len).ok_or_else(|| self.error("array size overflow"))?,
				ArrayLen::Dyn(_) => 0,
			};
			return Ok(Type::Array(Box::new(ArrayType { ty, len, size, align })));
		}

		let name = self.identifier()?;
		match name {
			"u8" => Ok(Type::U8),
			"u16" => Ok(Type::U16),
			"u32" => Ok(Type::U32),
			"u64" => Ok(Type::U64),
			"i8" => Ok(Type::I8),
			"i16" => Ok(Type::I16),
			"i32" => Ok(Type::I32),
			"i64" => Ok(Type::I64),
			"f32" => Ok(Type::F32),
			"f64" => Ok(Type::F64),
			"ptr" => Ok(Type::Va),
			"cstr" => Ok(Type::CStr),
			"utf16lez" => Ok(Type::Utf16LEZ),
			"code" => Ok(Type::Code),
			"unk" => Ok(Type::Unknown),
			"struct" => self.structure(depth + 1, false),
			"union" => self.structure(depth + 1, true),
			_ => Err(self.error(format_args!("unknown type '{name}'"))),
		}
	}

	fn structure(&mut self, depth: usize, is_union: bool) -> Result<Type, String> {
		// Optional struct name
		let name = if self.eat(b'{') { None }
		else {
			let name = self.identifier()?;
			self.expect(b'{')?;
			Some(name)
		};

		// Parse fields
		let mut fields: Vec<Field> = Vec::new();
		let mut field_names = HashSet::new();
		let mut offset = 0;
		let mut size = 0;
		let mut align = 1;
		let mut is_dst = false;
		while !self.eat(b'}') {
			let name = self.field_name(is_union)?;
			let ty = self.ty(depth)?;
			// Field properties
			let dynamic = ty.is_dst();
			is_dst |= dynamic;
			if dynamic && is_union {
				return Err(self.error("dynamic field is not allowed in a union"));
			}
			let (field_size, field_align) = if dynamic {
				(0, ty.alignment(self.pointer_width))
			} else { ty.layout(self.pointer_width).map_err(|message| self.error(message))? };
			let field_offset = if is_union { 0 } else { align_up(offset, field_align).map_err(|message| self.error(message))? };
			let field_end = field_offset.checked_add(field_size).ok_or_else(|| self.error("struct size overflow"))?;
			if let FieldName::Named(name) = &name {
				if !field_names.insert(name.clone()) {
					return Err(self.error(format_args!("duplicate field '{name}'")));
				}
			}
			fields.push(Field { name, offset: field_offset, ty });
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
		Ok(Type::Struct(Box::new(StructType { name, fields, is_union, size, align, is_dst })))
	}
}

#[cfg(test)]
mod tests;
