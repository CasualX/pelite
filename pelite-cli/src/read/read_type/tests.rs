use super::*;

#[test]
fn parses_composite_types_and_whitespace() {
	for pointer_width in [PointerWidth::Bits32, PointerWidth::Bits64] {
		let width = pointer_width.bytes();
		assert_eq!(parse(" cstr ", pointer_width).unwrap(), ReadType::CStr);
		assert!(ReadType::CStr.is_dst());
		assert_eq!(parse("ptr", pointer_width).unwrap(), ReadType::Va);
		assert_eq!(parse("* cstr", pointer_width).unwrap(), ReadType::Ptr(Box::new(ReadType::CStr)));
		assert!(!parse("*cstr", pointer_width).unwrap().is_dst());
		assert_eq!(parse(" * [ f32 ; 3 ] ", pointer_width).unwrap(), ReadType::Ptr(Box::new(
			ReadType::Array(Box::new(ReadArrayType { ty: ReadType::F32, len: ReadArrayLen::Const(3), size: 12, align: 4 }))
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
		assert!(parse("struct { _: cstr }", pointer_width).unwrap().is_dst());
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
			"struct { x: u8, x: i32 }", "struct { x u8 }",
			"struct { x: i32 y: i32 }", "struct { x: i32", "struct { , }", "struct { 1x: u8 }",
			"struct { x: [u8; 4294967295], y: u8 }", "struct { x: u64, y: [u8; 4294967287] }",
			"union { text: cstr }", "union { x: u8, x: u16 }", "union { x: u8",
		] {
			assert!(parse(source, pointer_width).is_err(), "accepted {source:?} for width {width}");
		}
	}
	assert!(parse(&format!("{}u8", "*".repeat(100)), PointerWidth::Bits64).is_err());
}

#[test]
fn parses_dynamic_array_lengths_from_earlier_unsigned_fields() {
	for pointer_width in [PointerWidth::Bits32, PointerWidth::Bits64] {
		for (integer, expected) in [
			("u8", ReadArrayLen::DynU8(1)),
			("u16", ReadArrayLen::DynU16(2)),
			("u32", ReadArrayLen::DynU32(4)),
			("u64", ReadArrayLen::DynU64(8)),
		] {
			let source = format!("struct {{ pad: u8, count: {integer}, values: [u16; count] }}");
			let ty = parse(&source, pointer_width).unwrap();
			let ReadType::Struct(structure) = &ty else { panic!("expected struct") };
			assert!(ty.is_dst());
			assert!(ty.layout(pointer_width).is_err());
			let ReadType::Array(array) = &structure.fields[2].ty else { panic!("expected array") };
			assert_eq!(array.len, expected);
			assert!(array.ty.layout(pointer_width).is_ok());
			assert!(structure.fields[2].offset > structure.fields[1].offset);
		}
	}
}

#[test]
fn rejects_invalid_dynamic_array_fields() {
	for source in [
		"[u8; count]",
		"struct { values: [u8; count], count: u8 }",
		"struct { count: i32, values: [u8; count] }",
		"struct { count: ptr, values: [u8; count] }",
		"struct { count: u8, values: [u8; count], tail: u8 }",
		"union { count: u8, values: [u8; count] }",
		"[struct { count: u8, values: [u8; count] }; 2]",
		"struct { count: u8, values: [[u8; count]; 2] }",
	] {
		assert!(parse(source, PointerWidth::Bits64).is_err(), "accepted {source}");
	}
}

#[test]
fn trailing_cstr_is_dynamic_but_pointers_have_fixed_layout() {
	for pointer_width in [PointerWidth::Bits32, PointerWidth::Bits64] {
		let ty = parse("struct { tag: u16, text: cstr }", pointer_width).unwrap();
		let ReadType::Struct(structure) = &ty else { panic!("expected struct") };
		assert_eq!(structure.fields[1].offset, 2);
		assert!(ty.is_dst());
		assert!(ty.layout(pointer_width).is_err());
		let pointer = parse("*struct { text: cstr }", pointer_width).unwrap();
		assert!(!pointer.is_dst());
		assert_eq!(pointer.layout(pointer_width).unwrap(), (pointer_width.bytes(), pointer_width.bytes()));
		assert!(parse("struct { count: u8, values: *[u8; count], tail: u8 }", pointer_width).is_ok());
		assert!(parse("struct { text: cstr, tail: u8 }", pointer_width).is_err());
		assert!(parse("union { text: cstr }", pointer_width).is_err());
		assert!(parse("[cstr; 1]", pointer_width).is_err());
	}
}
