use super::*;

#[test]
fn function_json_merges_within_and_across_sources() {
	let width = ty::PointerWidth::Bits64;
	let mut indexed = IndexedFacts::default();
	for source in [
		r#"#factmap
Fx10 {"bytes":244,"runtime_function":123312,"metadata":{"name":"old","keep":true},"values":[1,2],"nullable":1,"object_to_scalar":{"x":1},"scalar_to_object":1}
Fx10 {"metadata":{"name":"new"},"values":[3],"nullable":null}
"#,
		r#"#factmap
Fx10 {"bytes":300,"metadata":{"added":42},"object_to_scalar":false,"scalar_to_object":{"x":2}}
Fx20 {"other":true}
"#,
	] {
		indexed.extend(FactMap::parse(source, width).unwrap().facts, 0x180000000);
	}
	let fact = &indexed.functions[&0x180000010];
	assert_eq!(fact.rva, 0x10);
	let content: serde_json::Value = serde_json::from_str(&fact.content).unwrap();
	assert_eq!(content, serde_json::json!({
		"bytes":300, "runtime_function":123312,
		"metadata":{"name":"new","keep":true,"added":42},
		"values":[3], "nullable":null, "object_to_scalar":false, "scalar_to_object":{"x":2},
	}));
	assert_eq!(indexed.functions.len(), 2);
}

#[test]
fn later_function_facts_replace_content_within_and_across_sources() {
	let width = ty::PointerWidth::Bits64;
	let mut indexed = IndexedFacts::default();
	let first = FactMap::parse("#factmap\nFx10 first\nFx10 second\nFx20 {}\n", width).unwrap();
	indexed.extend(first.facts, 0x180000000);
	assert_eq!(indexed.functions[&0x180000010].content, "second");
	let second = FactMap::parse("#factmap\nFx10 arbitrary replacement data\n", width).unwrap();
	indexed.extend(second.facts, 0x180000000);
	assert_eq!(indexed.functions.len(), 2);
	assert_eq!(indexed.functions[&0x180000010].rva, 0x10);
	assert_eq!(indexed.functions[&0x180000010].content, "arbitrary replacement data");
	assert_eq!(indexed.functions[&0x180000020].content, "{}");
	assert!(indexed.symbols.is_empty());
	indexed.extend([Fact::Function(FunctionFact { rva: 1, content: "{}".into() })], u64::MAX);
	assert_eq!(indexed.functions.len(), 2);
}

#[test]
fn comment_facts_are_indexed_and_later_comments_override() {
	let width = ty::PointerWidth::Bits64;
	let mut indexed = IndexedFacts::default();
	let first = FactMap::parse("#factmap\nSx10 code fn\nCx10 \"generated\"\nCx20 \"other\"\n", width).unwrap();
	let second = FactMap::parse("#factmap\nCx10 \"user override\"\n", width).unwrap();
	indexed.extend(first.facts, 0x180000000);
	indexed.extend(second.facts, 0x180000000);
	assert_eq!(indexed.symbols.len(), 1);
	assert_eq!(indexed.comments.len(), 2);
	assert_eq!(indexed.comments[&0x180000010], "user override");
	assert_eq!(indexed.comments[&0x180000020], "other");
}

#[test]
fn indexing_ignores_non_symbol_facts() {
	let map = FactMap::parse("#factmap\nSx10 code fn\nCx10 \"comment\"\nRx10 0x20\nCx20 \"another comment\"\n", ty::PointerWidth::Bits64).unwrap();
	let mut symbols = HashMap::new();
	index(&mut symbols, map.facts, 0x180000000);
	assert_eq!(symbols.len(), 1);
	assert_eq!(symbols[&0x180000010].to_string(), "fn_10");
}

#[test]
fn weak_entries_preserve_strong_symbols_and_can_be_replaced_or_removed() {
	let width = ty::PointerWidth::Bits64;
	let first = FactMap::parse("#factmap\nSx1000 code \"named\"\nSx1010 code fn\nSx1020 code _\nSx1030 code _\nSx1040 code _\nSx1050 code fn\n", width).unwrap();
	let second = FactMap::parse("#factmap\nSx1000 code _\nSx1010 code _\nSx1020 code \"replacement\"\nSx1030 code C\nSx1040 unk undef\nSx1050 unk undef\nSx1050 code _\nSx1060 code _\n", width).unwrap();
	let base = 0x180000000;
	let mut symbols = HashMap::new();
	index(&mut symbols, first.facts, base);
	index(&mut symbols, second.facts, base);
	assert_eq!(symbols.len(), 6);
	assert_eq!(symbols[&(base + 0x1000)].to_string(), "named");
	assert_eq!(symbols[&(base + 0x1010)].to_string(), "fn_1010");
	assert_eq!(symbols[&(base + 0x1020)].to_string(), "replacement");
	assert_eq!(symbols[&(base + 0x1030)].to_string(), "code_1030");
	assert!(!symbols.contains_key(&(base + 0x1040)));
	for rva in [0x1050, 0x1060] {
		assert!(matches!(symbols[&(base + u64::from(rva))], IndexedSymbol::Weak(value) if value == rva));
	}
}

#[test]
fn undef_removes_symbols_and_later_entries_can_reintroduce_them() {
	let width = ty::PointerWidth::Bits64;
	let first = FactMap::parse("#factmap\nSx1000 code fn\nSx1010 u32 D\n", width).unwrap();
	let second = FactMap::parse("#factmap\nSx1000 unk undef\nSx1010 unk undef\nSx1020 unk undef\nSx1030 code fn\nSx1030 unk undef\nSx1000 code \"new\"\n", width).unwrap();
	let mut symbols = HashMap::new();
	index(&mut symbols, first.facts, 0x180000000);
	index(&mut symbols, second.facts, 0x180000000);
	assert_eq!(symbols.len(), 1);
	assert_eq!(symbols[&0x180001000].to_string(), "new");
}

#[test]
fn later_symbol_entries_replace_earlier_names() {
	let width = ty::PointerWidth::Bits64;
	let first = FactMap::parse("#factmap\nSx1000 code C\nSx1010 code \"first\"\nSx1020 code fn\nSx1030 code thunk\nSx2000 u32 D\nSx2010 u32 R\n", width).unwrap();
	let second = FactMap::parse("#factmap\nSx1010 code \"last\"\n", width).unwrap();
	let mut symbols = HashMap::new();
	index(&mut symbols, first.facts, 0x180000000);
	index(&mut symbols, second.facts, 0x180000000);
	assert_eq!(symbols[&0x180001000].to_string(), "code_1000");
	assert_eq!(symbols[&0x180001010].to_string(), "last");
	assert_eq!(symbols[&0x180001020].to_string(), "fn_1020");
	assert_eq!(symbols[&0x180001030].to_string(), "thunk_1030");
	assert_eq!(symbols[&0x180002000].to_string(), "data_2000");
	assert_eq!(symbols[&0x180002010].to_string(), "rdata_2010");
	let decoded = iced::decode_bytes(&[0x8b, 0x05, 0xfa, 0x0f, 0, 0], 64, 0x180001000, 0x180001000, 0x180001006, false, Arc::new(symbols));
	assert!(decoded[0].instruction.contains("data_2000"), "{}", decoded[0].instruction);
}

#[test]
fn auto_facts_require_a_pe_image() {
	let error = load_map(Path::new("auto"), ty::PointerWidth::Bits64, None).unwrap_err();
	assert!(error.to_string().contains("--facts auto requires a PE image"));
	assert_ne!(Path::new("./auto"), Path::new("auto"));
}

#[test]
fn indexed_types_follow_symbol_overrides_weak_anchors_and_removals() {
	let mut indexed = IndexedFacts::default();
	let width = ty::PointerWidth::Bits64;
	for source in [
		"#factmap\nSx10 u32 \"first\"\nSx20 u32 D\nSx30 code _\n",
		"#factmap\nSx10 code _\nSx20 unk undef\nSx30 cstr \"last\"\n",
	] {
		indexed.extend(FactMap::parse(source, width).unwrap().facts, 0);
	}
	assert_eq!(indexed.types.len(), 2);
	assert_eq!(indexed.types[&0x10], ty::Type::U32);
	assert_eq!(indexed.types[&0x30], ty::Type::CStr);
	assert_eq!(indexed.symbols[&0x10].to_string(), "first");
	assert_eq!(indexed.symbols[&0x30].to_string(), "last");
	assert!(!indexed.symbols.contains_key(&0x20));
}
