use super::*;

#[test]
fn weak_symbols_round_trip_and_quoted_underscore_is_named() {
	let source = "#symtext\n0x100 code _\n0x110 code \"_\"\n";
	for width in [ty::PointerWidth::Bits32, ty::PointerWidth::Bits64] {
		let database = SymbolDatabase::parse(source, width).unwrap();
		assert_eq!(database.entries[0], Symbol::new(0x100, ty::Type::Code, SymbolName::Weak));
		assert_eq!(database.entries[1].name, SymbolName::Named("_".into()));
		let mut output = Vec::new();
		database.write(&mut output, "").unwrap();
		assert_eq!(std::str::from_utf8(&output).unwrap(), source);
		assert_eq!(SymbolDatabase::parse(std::str::from_utf8(&output).unwrap(), width).unwrap(), database);
	}
}

#[test]
fn undef_entries_round_trip_in_order() {
	let source = r#"#symtext
0x100 code fn
0x100 unk undef
0x100 code "undef"
"#;
	let database = SymbolDatabase::parse(source, ty::PointerWidth::Bits64).unwrap();
	assert_eq!(database.entries[1].name, SymbolName::Undef);
	assert_eq!(database.entries[2].name, SymbolName::Named("undef".to_owned()));
	let mut output = Vec::new();
	database.write(&mut output, "").unwrap();
	assert_eq!(std::str::from_utf8(&output).unwrap(), source);
}

#[test]
fn parses_and_writes_symbols() {
	let source = r#"#symtext
0x001536 code fn
0x001172 code thunk
0x001540 code "imp_MSVCR120.dll!?what@exception@std@@UEBAPEBDXZ"
0x0041a0 unk D
0x005160 u64 D
0x0056d8 "union { u32, u64 }" "a \"quoted\" name"
0x005700 code "fn"
0x005710 code C
"#;
	for width in [ty::PointerWidth::Bits32, ty::PointerWidth::Bits64] {
		let database = SymbolDatabase::parse(source, width).unwrap();
		assert_eq!(database.entries.len(), 8);
		assert_eq!(database.entries[0].ty, ty::Type::Code);
		assert_eq!(database.entries[2].name, SymbolName::Named("imp_MSVCR120.dll!?what@exception@std@@UEBAPEBDXZ".to_owned()));
		assert_eq!(database.entries[5].ty.to_string(), "union{u32,u64}");
		assert_eq!(database.entries[6].name, SymbolName::Named("fn".to_owned()));
		assert_eq!(database.entries[7].name, SymbolName::Code);
		let mut output = Vec::new();
		database.write(&mut output, "").unwrap();
		assert!(std::str::from_utf8(&output).unwrap().contains("0x5710 code C\n"));
		assert_eq!(SymbolDatabase::parse(std::str::from_utf8(&output).unwrap(), width).unwrap(), database);
	}
}

#[test]
fn named_symbol_uses_json_escapes() {
	let name = "quoted \" slash \\ newline \n nul \0 tab \t";
	let database = SymbolDatabase { entries: vec![Symbol::new(0x100, ty::Type::Code, SymbolName::Named(name.to_owned()))] };
	let mut output = Vec::new();
	database.write(&mut output, "").unwrap();
	let text = std::str::from_utf8(&output).unwrap();
	assert_eq!(text, format!("#symtext\n0x100 code {}\n", serde_json::to_string(name).unwrap()));
	assert_eq!(SymbolDatabase::parse(text, ty::PointerWidth::Bits64).unwrap(), database);
	assert_eq!(SymbolDatabase::parse("#symtext\n0x100 code C\n", ty::PointerWidth::Bits64).unwrap().entries[0].name, SymbolName::Code);
}

#[test]
fn skips_comments_and_empty_lines() {
	let source = "#symtext\n\n# first symbol\n0x10 code fn\n  # between symbols\n\t\n0x20 unk D\n# end\n";
	let database = SymbolDatabase::parse(source, ty::PointerWidth::Bits64).unwrap();
	assert_eq!(database.entries.iter().map(|symbol| symbol.rva).collect::<Vec<_>>(), [0x10, 0x20]);
	assert_eq!(SymbolDatabase::parse("#symtext\n\n # comment\n0x100 code", ty::PointerWidth::Bits64).unwrap_err(), "line 4: expected another field");
}

#[test]
fn rejects_invalid_lines_with_line_numbers() {
	for (source, expected) in [
		("#symtext\n0x100 code", "line 2"),
		("#symtext\n100 code Fn", "line 2"),
		("#symtext\n0x100 code Nope", "line 2"),
		("#symtext\n0x100 code Fn extra", "line 2"),
		("#symtext\n0x100000000 code Fn", "line 2"),
		("#symtext\n0x100 arbitrary_text D", "line 2: invalid type"),
		("#symtext\n0x100 code \"bad\\q\"", "line 2"),
		("#symtext\n0x100 \"union{u32,bad}\" D", "line 2: invalid type"),
	] {
		assert!(SymbolDatabase::parse(source, ty::PointerWidth::Bits64).unwrap_err().contains(expected), "{source:?}");
	}
}
