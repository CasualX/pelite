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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadArrayType {
	pub ty: ReadType,
	pub len: u32,
	/// Layout computed when the array is parsed for the PE pointer width.
	pub size: u32,
	pub align: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReadType {
	U8, U16, U32, U64,
	I8, I16, I32, I64,
	F32, F64,

	Va,
	CStr, // Unsized type

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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadStructFieldType {
	pub name: String,
	pub offset: u32,
	pub ty: ReadType,
}

impl ReadType {
	/// Fixed size and alignment for the selected PE pointer width.
	pub fn layout(&self, pointer_width: PointerWidth) -> Result<(u32, u32), &'static str> {
		match self {
			Self::U8 | Self::I8 => Ok((1, 1)),
			Self::U16 | Self::I16 => Ok((2, 2)),
			Self::U32 | Self::I32 | Self::F32 => Ok((4, 4)),
			Self::U64 | Self::I64 | Self::F64 => Ok((8, 8)),
			Self::Va | Self::Ptr(_) => Ok((pointer_width.bytes(), pointer_width.bytes())),
			Self::CStr => Err("cstr is unsized; use *cstr in arrays and struct fields"),
			Self::Array(array) => Ok((array.size, array.align)),
			Self::Struct(structure) => Ok((structure.size, structure.align)),
		}
	}
}

fn align_up(offset: u32, align: u32) -> Result<u32, &'static str> {
	offset.checked_add(align - 1).map(|value| value & !(align - 1))
		.ok_or_else(|| "type layout exceeds RVA range")
}

impl ReadType {
	/// Parse and lay out a type after the PE pointer width is known.
	pub fn parse(input: &str, pointer_width: PointerWidth) -> Result<ReadType, String> {
		parse(input, pointer_width)
	}
}

fn parse(input: &str, pointer_width: PointerWidth) -> Result<ReadType, String> {
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

	fn ty(&mut self, depth: usize) -> Result<ReadType, String> {
		if depth >= 64 {
			return Err(self.error("type nesting is too deep"));
		}

		if self.eat(b'*') {
			return Ok(ReadType::Ptr(Box::new(self.ty(depth + 1)?)));
		}

		if self.eat(b'[') {
			let ty = self.ty(depth + 1)?;
			self.expect(b';')?;
			let len = self.array_len()?;
			self.expect(b']')?;
			let (element_size, align) = ty.layout(self.pointer_width).map_err(|message| self.error(message))?;
			let size = element_size.checked_mul(len).ok_or_else(|| self.error("array size exceeds RVA range"))?;
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
		while !self.eat(b'}') {
			// Field name
			let name = self.identifier()?.to_owned();
			// Field type
			self.expect(b':')?;
			let ty = self.ty(depth)?;
			// Field properties
			let (field_size, field_align) = ty.layout(self.pointer_width).map_err(|message| self.error(message))?;
			let field_offset = if is_union { 0 } else { align_up(offset, field_align).map_err(|message| self.error(message))? };
			let field_end = field_offset.checked_add(field_size).ok_or_else(|| self.error("struct size exceeds RVA range"))?;
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
		}

		let size = align_up(size, align).map_err(|message| self.error(message))?;
		let name = name.map(str::to_owned);
		Ok(ReadType::Struct(Box::new(ReadStructType { name, fields, size, align })))
	}
}

#[test]
fn parses_composite_types_and_whitespace() {
	for pointer_width in [PointerWidth::Bits32, PointerWidth::Bits64] {
		let width = pointer_width.bytes();
		assert_eq!(parse(" cstr ", pointer_width).unwrap(), ReadType::CStr);
		assert_eq!(parse("ptr", pointer_width).unwrap(), ReadType::Va);
		assert_eq!(parse("* cstr", pointer_width).unwrap(), ReadType::Ptr(Box::new(ReadType::CStr)));
		assert_eq!(parse(" * [ f32 ; 3 ] ", pointer_width).unwrap(), ReadType::Ptr(Box::new(
			ReadType::Array(Box::new(ReadArrayType { ty: ReadType::F32, len: 3, size: 12, align: 4 }))
		)));
		let array = parse("[i32; 256]", pointer_width).unwrap();
		let ReadType::Array(values) = &array else { panic!("expected an array") };
		assert_eq!((values.size, values.align), (1024, 4));
		assert_eq!(array.layout(pointer_width).unwrap(), (1024, 4));
		assert_eq!(parse("[[u16; 3]; 2]", pointer_width).unwrap().layout(pointer_width).unwrap(), (12, 2));
		assert_eq!(parse("[*cstr; 3]", pointer_width).unwrap().layout(pointer_width).unwrap(), (width * 3, width));
		assert!(parse("**cstr", pointer_width).is_ok());
		assert!(parse("struct {}", pointer_width).is_ok());
		assert!(parse("[u32; 0]", pointer_width).is_ok());
	}
}

#[test]
fn lays_out_structs_with_target_alignment_and_trailing_padding() {
	for (pointer_width, offsets, size) in [(PointerWidth::Bits32, [0, 4, 8], 12), (PointerWidth::Bits64, [0, 8, 16], 24)] {
		let width = pointer_width.bytes();
		let ty = parse("struct Entry { tag: u8, value: *f64, tail: u8, }", pointer_width).unwrap();
		let ReadType::Struct(structure) = &ty else { panic!("expected a struct") };
		assert_eq!(structure.name.as_deref(), Some("Entry"));
		assert_eq!(structure.fields.iter().map(|field| field.offset).collect::<Vec<_>>(), offsets);
		assert_eq!(structure.fields[1].name, "value");
		assert_eq!((structure.size, structure.align), (size, width));
		assert_eq!(ty.layout(pointer_width).unwrap(), (size, width));
		let array = parse("[struct { tag: u8, value: *f64, tail: u8 }; 2]", pointer_width).unwrap();
		assert_eq!(array.layout(pointer_width).unwrap(), (size * 2, width));
		let nested = parse("struct { tag: u8, inner: struct { value: ptr, tail: u8 }, end: u8 }", pointer_width).unwrap();
		let ReadType::Struct(structure) = &nested else { panic!("expected a struct") };
		assert_eq!(structure.fields[1].offset, width);
		assert_eq!(structure.fields[2].offset, width * 3);
		assert_eq!(nested.layout(pointer_width).unwrap(), (width * 4, width));
	}
}

#[test]
fn unions_overlay_fields_and_use_largest_field_layout() {
	for pointer_width in [PointerWidth::Bits32, PointerWidth::Bits64] {
		let width = pointer_width.bytes();
		let ty = parse("union Value { bytes: [u8; 3], word: u16, pointer: ptr, }", pointer_width).unwrap();
		let ReadType::Struct(structure) = &ty else { panic!("union should read as a struct") };
		assert_eq!(structure.name.as_deref(), Some("Value"));
		assert_eq!(structure.fields.iter().map(|field| field.offset).collect::<Vec<_>>(), [0, 0, 0]);
		assert_eq!((structure.size, structure.align), (width, width));
		assert_eq!(ty.layout(pointer_width).unwrap(), (width, width));
		let array = parse("[union { byte: u8, pointer: ptr }; 2]", pointer_width).unwrap();
		assert_eq!(array.layout(pointer_width).unwrap(), (width * 2, width));
		let nested = parse("struct { tag: u8, value: union { byte: u8, pointer: ptr }, tail: u8 }", pointer_width).unwrap();
		let ReadType::Struct(structure) = &nested else { panic!("expected a struct") };
		assert_eq!(structure.fields.iter().map(|field| field.offset).collect::<Vec<_>>(), [0, width, width * 2]);
		assert_eq!(nested.layout(pointer_width).unwrap(), (width * 3, width));
	}
	let ty = parse("union { bytes: [u8; 3], word: u16 }", PointerWidth::Bits32).unwrap();
	let ReadType::Struct(structure) = &ty else { panic!("expected a union") };
	assert_eq!((structure.size, structure.align), (4, 2));
	assert_eq!(ty.layout(PointerWidth::Bits32).unwrap(), (4, 2));
	let ReadType::Struct(empty) = parse("union {}", PointerWidth::Bits32).unwrap() else { panic!("expected a union") };
	assert_eq!((empty.size, empty.align), (0, 1));
}

#[test]
fn discarded_fields_contribute_to_layout() {
	for pointer_width in [PointerWidth::Bits32, PointerWidth::Bits64] {
		let ty = parse("struct { _: u8, a: u16, _: [u8; 3], _: u8, b: u32 }", pointer_width).unwrap();
		let ReadType::Struct(structure) = ty else { panic!("expected a struct") };
		assert_eq!(structure.fields.iter().map(|field| (field.name.as_str(), field.offset)).collect::<Vec<_>>(), [("a", 2), ("b", 8)]);
		assert_eq!((structure.size, structure.align), (12, 4));

		let ty = parse("union { _: [u8; 8], value: u16, _: u32 }", pointer_width).unwrap();
		let ReadType::Struct(structure) = ty else { panic!("expected a union") };
		assert_eq!(structure.fields.iter().map(|field| (field.name.as_str(), field.offset)).collect::<Vec<_>>(), [("value", 0)]);
		assert_eq!((structure.size, structure.align), (8, 4));
	}
}

#[test]
fn rejects_duplicate_named_fields_in_each_structure() {
	for pointer_width in [PointerWidth::Bits32, PointerWidth::Bits64] {
		for source in ["struct { x: u8, _: u8, x: u16 }", "union { x: u8, x: u16 }"] {
			assert!(parse(source, pointer_width).unwrap_err().contains("duplicate field 'x'"));
		}
		assert!(parse("struct { x: u8, inner: struct { x: u16 } }", pointer_width).is_ok());
		assert!(parse("struct { _: cstr }", pointer_width).is_err());
	}
}

#[test]
fn rejects_invalid_unsized_and_overflowing_types() {
	for pointer_width in [PointerWidth::Bits32, PointerWidth::Bits64] {
		let width = pointer_width.bytes();
		for source in [
			"", "U32", "u33", "i32 garbage", "u32[]", "*", "[i32]", "[i32; -1]",
			"[i32; 0x10]", "[i32; 3", "[i32; 18446744073709551616]", "[u64; 536870912]",
			"[cstr; 3]", "*[cstr; 3]", "*[CStr; 3]", "[cstr; 0]",
			"struct { text: cstr }", "struct { x: u8, x: i32 }", "struct { x u8 }",
			"struct { x: i32 y: i32 }", "struct { x: i32", "struct { , }", "struct { 1x: u8 }",
			"struct { x: [u8; 4294967295], y: u8 }", "struct { x: u64, y: [u8; 4294967287] }",
			"union { text: cstr }", "union { x: u8, x: u16 }", "union { x: u8",
		] {
			assert!(parse(source, pointer_width).is_err(), "accepted {source:?} for width {width}");
		}
	}
	assert!(parse(&format!("{}u8", "*".repeat(100)), PointerWidth::Bits64).is_err());
}
