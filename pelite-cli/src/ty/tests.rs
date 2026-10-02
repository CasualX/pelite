use super::*;

#[test]
fn parses_composite_types_and_whitespace() {
	for pointer_width in [PointerWidth::Bits32, PointerWidth::Bits64] {
		let width = pointer_width.bytes();
		assert_eq!(parse(" cstr ", pointer_width).unwrap(), Type::CStr);
		assert!(Type::CStr.is_dst());
		assert_eq!(parse("ptr", pointer_width).unwrap(), Type::Va);
		for (source, pointee) in [("*code", Type::Code), ("*unk", Type::Unknown)] {
			assert_eq!(parse(source, pointer_width).unwrap(), Type::Ptr(Box::new(pointee)));
			assert_eq!(parse(source, pointer_width).unwrap().layout(pointer_width).unwrap(), (width, width));
		}
		assert_eq!(parse("* cstr", pointer_width).unwrap(), Type::Ptr(Box::new(Type::CStr)));
		assert!(!parse("*cstr", pointer_width).unwrap().is_dst());
		assert_eq!(parse(" * [ f32 ; 3 ] ", pointer_width).unwrap(), Type::Ptr(Box::new(
			Type::Array(Box::new(ArrayType { ty: Type::F32, len: ArrayLen::Fixed(3), size: 12, align: 4 }))
		)));
		let array = parse("[i32; 256]", pointer_width).unwrap();
		let Type::Array(values) = &array else { panic!("expected an array") };
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
fn formatted_types_parse_back_to_the_same_type() {
	for pointer_width in [PointerWidth::Bits32, PointerWidth::Bits64] {
		for source in [
			"u64", "ptr", "code", "unk", "**cstr", "utf16lez", "*utf16lez", "[struct { a: u8, b: *f64 }; 3]",
			"union Value { _: [u8; 8], first: u32, second: u64 }",
			"struct { _: u8, count: u16, values: [u32; count] }",
			"struct Outer { count: u8, values: *[u8; count], nested: union { x: u16, y: i16 } }",
			"struct { values: *[u8; count], count: u8 }",
			"union{u32,u64}",
		] {
			let ty = parse(source, pointer_width).unwrap();
			let formatted = ty.to_string();
			assert_eq!(parse(&formatted, pointer_width).unwrap(), ty, "{source} formatted as {formatted}");
		}
	}
}

#[test]
fn lays_out_structs_with_target_alignment_and_trailing_padding() {
	for (pointer_width, offsets, size) in [(PointerWidth::Bits32, [0, 4, 8], 12), (PointerWidth::Bits64, [0, 8, 16], 24)] {
		let width = pointer_width.bytes();
		let ty = parse("struct Entry { tag: u8, value: *f64, tail: u8, }", pointer_width).unwrap();
		let Type::Struct(structure) = &ty else { panic!("expected a struct") };
		assert_eq!(structure.name.as_deref(), Some("Entry"));
		assert_eq!(structure.fields.iter().map(|field| field.offset).collect::<Vec<_>>(), offsets);
		assert_eq!(structure.fields[1].name, FieldName::Named("value".to_owned()));
		assert_eq!((structure.size, structure.align), (size, width));
		assert_eq!(ty.layout(pointer_width).unwrap(), (size, width));
		let array = parse("[struct { tag: u8, value: *f64, tail: u8 }; 2]", pointer_width).unwrap();
		assert_eq!(array.layout(pointer_width).unwrap(), (size * 2, width));
		let nested = parse("struct { tag: u8, inner: struct { value: ptr, tail: u8 }, end: u8 }", pointer_width).unwrap();
		let Type::Struct(structure) = &nested else { panic!("expected a struct") };
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
		let Type::Struct(structure) = &ty else { panic!("union should read as a struct") };
		assert_eq!(structure.name.as_deref(), Some("Value"));
		assert_eq!(structure.fields.iter().map(|field| field.offset).collect::<Vec<_>>(), [0, 0, 0]);
		assert_eq!((structure.size, structure.align), (width, width));
		assert_eq!(ty.layout(pointer_width).unwrap(), (width, width));
		let array = parse("[union { byte: u8, pointer: ptr }; 2]", pointer_width).unwrap();
		assert_eq!(array.layout(pointer_width).unwrap(), (width * 2, width));
		let nested = parse("struct { tag: u8, value: union { byte: u8, pointer: ptr }, tail: u8 }", pointer_width).unwrap();
		let Type::Struct(structure) = &nested else { panic!("expected a struct") };
		assert_eq!(structure.fields.iter().map(|field| field.offset).collect::<Vec<_>>(), [0, width, width * 2]);
		assert_eq!(nested.layout(pointer_width).unwrap(), (width * 3, width));
	}
	let ty = parse("union { bytes: [u8; 3], word: u16 }", PointerWidth::Bits32).unwrap();
	let Type::Struct(structure) = &ty else { panic!("expected a union") };
	assert_eq!((structure.size, structure.align), (4, 2));
	assert_eq!(ty.layout(PointerWidth::Bits32).unwrap(), (4, 2));
	let Type::Struct(empty) = parse("union {}", PointerWidth::Bits32).unwrap() else { panic!("expected a union") };
	assert_eq!((empty.size, empty.align), (0, 1));
}

#[test]
fn unnamed_union_fields_are_distinct_from_discarded_fields() {
	for pointer_width in [PointerWidth::Bits32, PointerWidth::Bits64] {
		let ty = parse("union{u32,u64}", pointer_width).unwrap();
		assert_eq!(ty.to_string(), "union{u32,u64}");
		assert_eq!(ty.layout(pointer_width), Ok((8, 8)));
		let Type::Struct(union) = ty else { panic!("expected union") };
		assert_eq!(union.fields.iter().map(|field| field.name.clone()).collect::<Vec<_>>(), [FieldName::Unnamed, FieldName::Unnamed]);
		let mixed = parse("union{named:u8,_:u16,u32}", pointer_width).unwrap();
		assert_eq!(mixed.to_string(), "union{named:u8,_:u16,u32}");
		let compound = parse("union{[u8;3],*u32}", pointer_width).unwrap();
		assert_eq!(compound.to_string(), "union{[u8;3],*u32}");
		assert!(parse("struct{u32}", pointer_width).is_err());
		assert!(parse("union{code}", pointer_width).is_err());
	}
}

#[test]
fn discarded_fields_contribute_to_layout() {
	for pointer_width in [PointerWidth::Bits32, PointerWidth::Bits64] {
		let ty = parse("struct { _: u8, a: u16, _: [u8; 3], _: u8, b: u32 }", pointer_width).unwrap();
		let Type::Struct(structure) = ty else { panic!("expected a struct") };
		assert_eq!(structure.fields.iter().map(|field| (field.name.clone(), field.offset)).collect::<Vec<_>>(),
			[(FieldName::Discarded, 0), (FieldName::Named("a".to_owned()), 2), (FieldName::Discarded, 4),
				(FieldName::Discarded, 7), (FieldName::Named("b".to_owned()), 8)]);
		assert_eq!((structure.size, structure.align), (12, 4));

		let ty = parse("union { _: [u8; 8], value: u16, _: u32 }", pointer_width).unwrap();
		let Type::Struct(structure) = ty else { panic!("expected a union") };
		assert_eq!(structure.fields.iter().map(|field| (field.name.clone(), field.offset)).collect::<Vec<_>>(),
			[(FieldName::Discarded, 0), (FieldName::Named("value".to_owned()), 0), (FieldName::Discarded, 0)]);
		assert_eq!((structure.size, structure.align), (8, 4));
	}
}

#[test]
fn rejects_duplicate_named_fields() {
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
fn parses_dynamic_array_length_names() {
	for pointer_width in [PointerWidth::Bits32, PointerWidth::Bits64] {
		for integer in ["u8", "u16", "u32", "u64"] {
			let source = format!("struct {{ pad: u8, count: {integer}, values: [u16; count] }}");
			let ty = parse(&source, pointer_width).unwrap();
			let Type::Struct(structure) = &ty else { panic!("expected struct") };
			assert!(ty.is_dst());
			assert!(ty.layout(pointer_width).is_err());
			let Type::Array(array) = &structure.fields[2].ty else { panic!("expected array") };
			assert_eq!(array.len, ArrayLen::Dyn("count".to_owned()));
			assert!(array.ty.layout(pointer_width).is_ok());
			assert!(structure.fields[2].offset > structure.fields[1].offset);
		}
		for source in ["[u8; count]", "struct { values: *[u8; count], count: u8 }", "struct { count: i32, values: [u8; count] }"] {
			assert!(parse(source, pointer_width).is_ok(), "rejected {source}");
		}
	}
}

#[test]
fn rejects_invalid_dynamic_array_layouts() {
	for source in [
		"struct { values: [u8; count], count: u8 }",
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
		let Type::Struct(structure) = &ty else { panic!("expected struct") };
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

#[test]
fn unsized_types() {
	for pointer_width in [PointerWidth::Bits32, PointerWidth::Bits64] {
		for (source, expected) in [("code", Type::Code), ("unk", Type::Unknown), ("cstr", Type::CStr)] {
			let ty = parse(source, pointer_width).unwrap();
			assert_eq!(ty, expected);
			assert!(ty.is_dst());
			assert!(ty.layout(pointer_width).is_err());
			assert_eq!(ty.alignment(pointer_width), 1);
			assert!(parse(&format!("[{source}; 2]"), pointer_width).is_err());
			assert!(parse(&format!("union {{ data: {source} }}"), pointer_width).is_err());
			let trailing = parse(&format!("struct {{ tag: u16, data: {source} }}"), pointer_width).unwrap();
			assert!(trailing.is_dst());
			assert!(parse(&format!("struct {{ data: {source}, tail: u8 }}"), pointer_width).is_err());
		}
	}
}

#[test]
fn wide_strings_have_two_byte_alignment_and_no_fixed_size() {
	for pointer_width in [PointerWidth::Bits32, PointerWidth::Bits64] {
		let ty = parse("utf16lez", pointer_width).unwrap();
		assert_eq!(ty, Type::Utf16LEZ);
		assert!(ty.is_dst());
		assert_eq!(ty.alignment(pointer_width), 2);
		assert!(ty.layout(pointer_width).is_err());
		let Type::Struct(structure) = parse("struct { tag: u8, text: utf16lez }", pointer_width).unwrap() else { panic!("expected struct") };
		assert_eq!(structure.fields[1].offset, 2);
		assert!(structure.is_dst);
		let width = pointer_width.bytes();
		assert_eq!(parse("[*utf16lez; 3]", pointer_width).unwrap().layout(pointer_width), Ok((width * 3, width)));
		for source in ["[utf16lez; 1]", "union { text: utf16lez }", "struct { text: utf16lez, tail: u8 }"] {
			assert!(parse(source, pointer_width).is_err(), "accepted {source}");
		}
	}
}
