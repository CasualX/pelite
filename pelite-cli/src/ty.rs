//! Type DSL for describing data and code in a PE image.

use super::*;

/// A type syntax or layout error at a byte offset in the input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseError {
	/// Zero-based byte offset where parsing stopped, possibly at the end of input.
	pub offset: usize,
	/// The reason parsing failed.
	pub kind: ParseErrorKind,
}

/// Reasons a type cannot be parsed or laid out.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseErrorKind {
	/// Text remains after a complete type.
	UnexpectedTrailingInput,
	/// A required punctuation character is missing.
	ExpectedToken(char),
	/// An identifier was expected at this position.
	ExpectedIdentifier,
	/// A decimal array length is missing or exceeds `u32`.
	InvalidArrayLength,
	/// Type nesting exceeds the parser's limit.
	NestingTooDeep,
	/// An identifier does not name a supported type.
	UnknownType(String),
	/// An unsized type was used where a fixed layout is required.
	UnsizedType(&'static str),
	/// Multiplying the element size by the array length exceeds `u32`.
	ArraySizeOverflow,
	/// A union field is unsized.
	DynamicUnionField,
	/// Rounding a size or offset up to its alignment exceeds `u32`.
	TypeLayoutOverflow,
	/// Adding a field's size to its offset exceeds `u32`.
	StructSizeOverflow,
	/// Two fields in the same struct or union have the same explicit name.
	DuplicateField(String),
	/// An unsized struct field is followed by another field.
	DynamicFieldNotLast,
}

impl fmt::Display for ParseErrorKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::UnexpectedTrailingInput => f.write_str("unexpected trailing input"),
			Self::ExpectedToken(token) => write!(f, "expected '{token}'"),
			Self::ExpectedIdentifier => f.write_str("expected an identifier"),
			Self::InvalidArrayLength => f.write_str("expected a decimal array length"),
			Self::NestingTooDeep => f.write_str("type nesting is too deep"),
			Self::UnknownType(name) => write!(f, "unknown type '{name}'"),
			Self::UnsizedType(message) => f.write_str(message),
			Self::ArraySizeOverflow => f.write_str("array size overflow"),
			Self::DynamicUnionField => f.write_str("dynamic field is not allowed in a union"),
			Self::TypeLayoutOverflow => f.write_str("type layout overflow"),
			Self::StructSizeOverflow => f.write_str("struct size overflow"),
			Self::DuplicateField(name) => write!(f, "duplicate field '{name}'"),
			Self::DynamicFieldNotLast => f.write_str("dynamic field must be the last struct field"),
		}
	}
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} at byte {}", self.kind, self.offset)
	}
}

impl std::error::Error for ParseError {}

/// Pointer width of the target PE image, independent of the host architecture.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PointerWidth {
	/// Four-byte pointers in PE32 images.
	Bits32,
	/// Eight-byte pointers in PE32+ images.
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
	/// An unsigned integer with the target pointer's size and alignment.
	pub const fn unsigned(self) -> Type {
		match self {
			Self::Bits32 => Type::U32,
			Self::Bits64 => Type::U64,
		}
	}

	/// Construct a pointer whose size and alignment use this width.
	pub fn pointer(self, pointee: Type) -> Type {
		match self {
			Self::Bits32 => Type::Ptr32(Box::new(pointee)),
			Self::Bits64 => Type::Ptr64(Box::new(pointee)),
		}
	}

	/// Size and natural alignment of a pointer, in bytes.
	#[inline]
	pub const fn bytes(self) -> u32 {
		match self {
			Self::Bits32 => 4,
			Self::Bits64 => 8,
		}
	}
}

/// Number of elements in an array, written after the semicolon in `[T; len]`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArrayLen {
	/// A fixed decimal element count.
	Fixed(u32),
	/// A containing struct field whose unsigned integer value supplies the count.
	/// The field is resolved at read time, rather than during parsing.
	Dyn(String),
}

/// An array of fixed-size elements with layout cached for the target PE.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArrayType {
	/// Element type; must have a fixed layout.
	pub ty: Type,
	/// Fixed element count or name of the field providing it.
	pub len: ArrayLen,
	/// Total size in bytes, computed when parsed; zero for a dynamic array.
	pub size: u32,
	/// Alignment in bytes, inherited from the element type.
	pub align: u32,
}

/// A parsed DSL type describing a value, pointee, or unsized region.
///
/// Integer and floating-point types describe little-endian values. Pointers
/// store their width explicitly, even when their pointee is unsized.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
	U8, U16, U32, U64,
	I8, I16, I32, I64,
	F32, F64,

	/// `cstr`: an unsized NUL-terminated byte string with byte alignment.
	CStr,
	/// `utf16lez`: an unsized NUL-terminated UTF-16LE string with two-byte alignment.
	Utf16LEZ,
	/// `code`: unsized code without a declared layout or confirmed function identity.
	Code,
	/// `fn`: an unsized confirmed function; `*fn` denotes a function pointer.
	Fn,
	/// `unk`: unsized data without a declared layout; `*unk` denotes an opaque pointer.
	Unknown,

	/// `[T; len]`: fixed-size elements with a fixed or field-supplied count.
	Array(Box<ArrayType>),
	/// `*T`: A four-byte virtual address pointing to a value or unsized region.
	Ptr32(Box<Type>),
	/// `*T`: An eight-byte virtual address pointing to a value or unsized region.
	Ptr64(Box<Type>),
	/// A `struct` or `union` with its fields and target layout.
	Struct(Box<StructType>),
}

/// A struct or union with field offsets and layout computed during parsing.
///
/// Struct fields follow natural C alignment; union fields all start at offset
/// zero. The total size is rounded up to the largest field alignment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructType {
	/// Optional descriptive name, with no effect on layout.
	pub name: Option<String>,
	/// Fields in declaration order, including discarded and unnamed fields.
	pub fields: Vec<Field>,
	/// Whether fields overlap as a union instead of following one another.
	pub is_union: bool,
	/// Size in bytes, including trailing padding; for a DST, only its fixed prefix.
	/// A DST's complete size is unavailable through `Type::layout`.
	pub size: u32,
	/// Largest field alignment, in bytes; one for an empty struct or union.
	pub align: u32,
	/// Whether the last struct field is unsized; always false for unions.
	pub is_dst: bool,
}

/// A field and its position within a struct or union.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Field {
	/// Name or output treatment of the field.
	pub name: FieldName,
	/// Byte offset from the start of the containing type; zero for union fields.
	pub offset: u32,
	/// Field type, which contributes to layout even when its value is discarded.
	pub ty: Type,
}

/// How a field is named and represented when reading a composite value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FieldName {
	/// An explicit name, unique within the containing struct or union.
	Named(String),
	/// `_`: contributes to layout but omits the value from read output.
	Discarded,
	/// A union field without a name, output under its zero-based field index.
	Unnamed,
}

impl Type {
	/// Whether this type has no readable contents: `code`, `fn`, or `unk`.
	/// Pointers to these types return the target RVA without dereferencing it.
	pub fn is_opaque(&self) -> bool {
		matches!(self, Self::Code | Self::Fn | Self::Unknown)
	}

	/// Alignment of the starting address, including dynamically sized types.
	pub fn alignment(&self) -> u32 {
		match self {
			Self::CStr | Self::Code | Self::Fn | Self::Unknown => 1,
			Self::Utf16LEZ => 2,
			Self::Array(array) => array.align,
			Self::Struct(structure) => structure.align,
			_ => self.layout().expect("fixed-size type").1,
		}
	}

	/// Whether this type has no fixed size. Pointers to unsized types are sized.
	pub fn is_dst(&self) -> bool {
		match self {
			Self::CStr | Self::Utf16LEZ | Self::Code | Self::Fn | Self::Unknown => true,
			Self::Array(array) => !matches!(&array.len, ArrayLen::Fixed(_)),
			Self::Struct(structure) => structure.is_dst,
			_ => false,
		}
	}

	/// Fixed size and alignment.
	///
	/// Returns `(size, alignment)` in bytes, or an error for an unsized type.
	pub fn layout(&self) -> Result<(u32, u32), &'static str> {
		match self {
			Self::U8 | Self::I8 => Ok((1, 1)),
			Self::U16 | Self::I16 => Ok((2, 2)),
			Self::U32 | Self::I32 | Self::F32 => Ok((4, 4)),
			Self::U64 | Self::I64 | Self::F64 => Ok((8, 8)),
			Self::Ptr32(_) => Ok((4, 4)),
			Self::Ptr64(_) => Ok((8, 8)),
			Self::CStr => Err("cstr is unsized"),
			Self::Utf16LEZ => Err("utf16lez is unsized"),
			Self::Code => Err("code is unsized"),
			Self::Fn => Err("fn is unsized"),
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
			Self::CStr => f.write_str("cstr"),
			Self::Utf16LEZ => f.write_str("utf16lez"),
			Self::Code => f.write_str("code"),
			Self::Fn => f.write_str("fn"),
			Self::Unknown => f.write_str("unk"),
			Self::Ptr32(ty) | Self::Ptr64(ty) => write!(f, "*{ty}"),
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
	///
	/// Accepts whitespace between tokens and requires the entire input to be a
	/// single type. Errors describe invalid syntax or layout and include a byte
	/// position. Dynamic array field names are resolved later, when reading data.
	pub fn parse(input: &str, pointer_width: PointerWidth) -> Result<Type, ParseError> {
		parse(input, pointer_width)
	}
}

fn parse(input: &str, pointer_width: PointerWidth) -> Result<Type, ParseError> {
	let mut parser = Parser { input, pos: 0, pointer_width };
	let ty = parser.ty(0)?;
	parser.whitespace();
	if parser.pos != input.len() {
		return Err(parser.error(ParseErrorKind::UnexpectedTrailingInput));
	}
	Ok(ty)
}

struct Parser<'a> {
	input: &'a str,
	pos: usize,
	pointer_width: PointerWidth,
}

impl<'a> Parser<'a> {
	fn error(&self, kind: ParseErrorKind) -> ParseError {
		ParseError { offset: self.pos, kind }
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

	fn expect(&mut self, token: u8) -> Result<(), ParseError> {
		if !self.eat(token) {
			return Err(self.error(ParseErrorKind::ExpectedToken(char::from(token))));
		}
		Ok(())
	}

	fn identifier(&mut self) -> Result<&'a str, ParseError> {
		self.whitespace();
		let start = self.pos;
		if !self.input.as_bytes().get(self.pos).is_some_and(|byte| byte.is_ascii_alphabetic() || *byte == b'_') {
			return Err(self.error(ParseErrorKind::ExpectedIdentifier));
		}
		self.pos += 1;
		while self.input.as_bytes().get(self.pos).is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte == b'_') {
			self.pos += 1;
		}
		Ok(&self.input[start..self.pos])
	}

	fn array_len(&mut self) -> Result<u32, ParseError> {
		self.whitespace();
		let start = self.pos;
		while self.input.as_bytes().get(self.pos).is_some_and(u8::is_ascii_digit) {
			self.pos += 1;
		}
		self.input[start..self.pos].parse::<u32>()
			.map_err(|_| self.error(ParseErrorKind::InvalidArrayLength))
	}

	fn field_name(&mut self, is_union: bool) -> Result<FieldName, ParseError> {
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

	fn ty(&mut self, depth: usize) -> Result<Type, ParseError> {
		if depth >= 64 {
			return Err(self.error(ParseErrorKind::NestingTooDeep));
		}

		if self.eat(b'*') {
			let pointee = self.ty(depth + 1)?;
			return Ok(self.pointer_width.pointer(pointee));
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
			let (element_size, align) = ty.layout().map_err(|message| self.error(ParseErrorKind::UnsizedType(message)))?;
			let size = match &len {
				ArrayLen::Fixed(len) => element_size.checked_mul(*len).ok_or_else(|| self.error(ParseErrorKind::ArraySizeOverflow))?,
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
			"cstr" => Ok(Type::CStr),
			"utf16lez" => Ok(Type::Utf16LEZ),
			"code" => Ok(Type::Code),
			"fn" => Ok(Type::Fn),
			"unk" => Ok(Type::Unknown),
			"struct" => self.structure(depth + 1, false),
			"union" => self.structure(depth + 1, true),
			_ => Err(self.error(ParseErrorKind::UnknownType(name.to_owned()))),
		}
	}

	fn structure(&mut self, depth: usize, is_union: bool) -> Result<Type, ParseError> {
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
				return Err(self.error(ParseErrorKind::DynamicUnionField));
			}

			// Compute offset, size and alignment
			let (field_size, field_align) = if dynamic {
				(0, ty.alignment())
			}
			else {
				ty.layout().map_err(|message| self.error(ParseErrorKind::UnsizedType(message)))?
			};
			let field_offset = if is_union {
				0
			}
			else {
				align_up(offset, field_align).map_err(|_| self.error(ParseErrorKind::TypeLayoutOverflow))?
			};
			let field_end = field_offset.checked_add(field_size).ok_or_else(|| self.error(ParseErrorKind::StructSizeOverflow))?;

			// Append the field
			if let FieldName::Named(name) = &name {
				if !field_names.insert(name.clone()) {
					return Err(self.error(ParseErrorKind::DuplicateField(name.clone())));
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
					return Err(self.error(ParseErrorKind::DynamicFieldNotLast));
				}
				break;
			}
		}

		let size = align_up(size, align).map_err(|_| self.error(ParseErrorKind::TypeLayoutOverflow))?;
		let name = name.map(str::to_owned);
		Ok(Type::Struct(Box::new(StructType { name, fields, is_union, size, align, is_dst })))
	}
}

#[cfg(test)]
mod tests;
