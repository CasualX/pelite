use super::*;

#[test]
fn discovered_symbols_write_parseable_factmap() {
	let symbols = vec![
		factmap::SymbolFact::new(0x1000, ty::Type::Code, factmap::SymbolName::Code),
		factmap::SymbolFact::new(0x1100, ty::Type::Code, factmap::SymbolName::Thunk),
		factmap::SymbolFact::new(0x2000, ty::Type::parse("union{u32,u64}", ty::PointerWidth::Bits64).unwrap(), factmap::SymbolName::Data),
		factmap::SymbolFact::new(0x2100, ty::Type::Unknown, factmap::SymbolName::Data),
		factmap::SymbolFact::new(0x3000, ty::Type::Code, factmap::SymbolName::Named("imp_Sleep".into())),
		factmap::SymbolFact::new(0x3100, ty::Type::Code, factmap::SymbolName::Named("imp_Sleep".into())),
	];
	let mut output = Vec::new();
	factmap::FactMap { facts: symbols.into_iter().map(factmap::Fact::Symbol).collect() }.write(&mut output, "").unwrap();
	let text = std::str::from_utf8(&output).unwrap();
	let map = factmap::FactMap::parse(text, ty::PointerWidth::Bits64).unwrap();
	let entries = map.facts.iter().map(|fact| {
		let factmap::Fact::Symbol(symbol) = fact else { panic!("expected a symbol fact") };
		symbol
	}).collect::<Vec<_>>();
	assert_eq!(entries[0].ty, ty::Type::Code);
	assert_eq!(entries[0].name, factmap::SymbolName::Code);
	assert_eq!(entries[1].name, factmap::SymbolName::Thunk);
	assert_eq!(entries[2].ty.to_string(), "union{u32,u64}");
	assert_eq!(entries[2].name, factmap::SymbolName::Data);
	assert_eq!(entries[3].ty, ty::Type::Unknown);
	assert_eq!(entries[3].name, factmap::SymbolName::Data);
	assert_eq!(entries[4].name, entries[5].name);
	assert_eq!(entries[4].name, factmap::SymbolName::Named("imp_Sleep".into()));
}

#[test]
fn upgrade_type_hints() {
	for width in [ty::PointerWidth::Bits32, ty::PointerWidth::Bits64] {
		let mut symbol = factmap::SymbolFact::new(0x2000, ty::Type::Unknown, factmap::SymbolName::Data);
		for hint in [ty::Type::Unknown, ty::Type::U32, ty::Type::U32, ty::Type::Unknown] {
			upgrade_type(&mut symbol, hint).unwrap();
		}
		assert_eq!(symbol.ty, ty::Type::U32);
		for hint in [width.pointer(ty::Type::Unknown), ty::Type::U64, ty::Type::U32, ty::Type::U64] {
			upgrade_type(&mut symbol, hint).unwrap();
		}
		assert_eq!(symbol.ty, ty::Type::parse("union{u32,*unk,u64}", width).unwrap());
		let previous = symbol.clone();
		assert!(upgrade_type(&mut symbol, ty::Type::CStr).is_err());
		assert_eq!(symbol, previous);
		upgrade_type(&mut symbol, ty::Type::Code).unwrap();
		upgrade_type(&mut symbol, ty::Type::U8).unwrap();
		assert_eq!(symbol.ty, ty::Type::Code);
		upgrade_type(&mut symbol, ty::Type::Fn).unwrap();
		for hint in [ty::Type::Code, ty::Type::U32, ty::Type::Unknown] {
			upgrade_type(&mut symbol, hint).unwrap();
		}
		assert_eq!(symbol.ty, ty::Type::Fn);
		assert_eq!(symbol.rva, 0x2000);
		assert_eq!(symbol.name, factmap::SymbolName::Data);
	}
}

#[test]
fn function_hints_override_data_and_union_hints() {
	for previous in [
		ty::Type::Unknown,
		ty::Type::U64,
		ty::Type::parse("union{u32,u64}", ty::PointerWidth::Bits64).unwrap(),
	] {
		let mut symbol = factmap::SymbolFact::new(0x1000, previous, factmap::SymbolName::Data);
		upgrade_type(&mut symbol, ty::Type::Fn).unwrap();
		assert_eq!(symbol.ty, ty::Type::Fn);
	}
}

#[test]
fn upgrade_type_union_layout() {
	let width = ty::PointerWidth::Bits64;
	let mut symbol = factmap::SymbolFact::new(0, ty::Type::parse("[u32;3]", width).unwrap(), factmap::SymbolName::Data);
	upgrade_type(&mut symbol, ty::Type::U64).unwrap();
	assert_eq!(symbol.ty.layout(), Ok((16, 8)));
	assert_eq!(symbol.ty, ty::Type::parse("union{[u32;3],u64}", width).unwrap());
}

#[test]
fn exception_comments_preserve_existing_symbols() {
	let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../demo/Demo64.dll");
	let map = pelite::FileMap::open(&path).unwrap();
	let pe = PeFile::from_bytes(&map).unwrap();
	let Wrap::T64(file) = pe else { unreachable!() };
	let rva = file.exception_x64().unwrap().image()[0].BeginAddress;
	for name in [
		factmap::SymbolName::Code,
		factmap::SymbolName::Data,
		factmap::SymbolName::RData,
		factmap::SymbolName::Thunk,
		factmap::SymbolName::Named("exported_function".into()),
		factmap::SymbolName::Named("imp_Sleep".into()),
		factmap::SymbolName::Named("ret0".into()),
	] {
		let expected = name.clone();
		let input = AnalysisInput::new(pe).unwrap();
		let mut analysis = AnalysisOutput::default();
		analysis.symbols.insert(rva, factmap::SymbolFact::new(rva, ty::Type::Unknown, name));
		metadata::scan_exceptions(&input, &mut analysis);
		assert_eq!(analysis.symbols[&rva].name, expected);
		assert_eq!(analysis.symbols[&rva].ty, ty::Type::Unknown);
		assert!(analysis.comments.contains_key(&rva));
		assert!(analysis.functions.is_empty());
	}
}
