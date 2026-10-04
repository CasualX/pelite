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
			symbol.upgrade_type(hint).unwrap();
		}
		assert_eq!(symbol.ty, ty::Type::U32);
		for hint in [width.pointer(ty::Type::Unknown), ty::Type::U64, ty::Type::U32, ty::Type::U64] {
			symbol.upgrade_type(hint).unwrap();
		}
		assert_eq!(symbol.ty, ty::Type::parse("union{u32,*unk,u64}", width).unwrap());
		let previous = symbol.clone();
		assert!(symbol.upgrade_type(ty::Type::CStr).is_err());
		assert_eq!(symbol, previous);
		symbol.upgrade_type(ty::Type::Code).unwrap();
		symbol.upgrade_type(ty::Type::U8).unwrap();
		assert_eq!(symbol.ty, ty::Type::Code);
		symbol.upgrade_type(ty::Type::Fn).unwrap();
		for hint in [ty::Type::Code, ty::Type::U32, ty::Type::Unknown] {
			symbol.upgrade_type(hint).unwrap();
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
		symbol.upgrade_type(ty::Type::Fn).unwrap();
		assert_eq!(symbol.ty, ty::Type::Fn);
	}
}

#[test]
fn metadata_functions_and_import_slot_integers() {
	for dll in ["Demo.dll", "Demo64.dll"] {
		let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../demo").join(dll);
		let map = pelite::FileMap::open(&path).unwrap();
		let pe = PeFile::from_bytes(&map).unwrap();
		let width = ty::PointerWidth::from(pe);
		let mut analysis = Analysis::new(pe).unwrap();
		let imports = imports::import_names(pe, analysis.bitness);
		assert!(!imports.is_empty());
		for &rva in imports.keys() {
			analysis.add(rva, Some(ty::Type::Code));
		}
		analysis.label_imports();
		for (rva, name) in imports {
			let symbol = &analysis.symbols[&rva];
			assert_eq!(symbol.ty, if width == ty::PointerWidth::Bits32 { ty::Type::U32 } else { ty::Type::U64 });
			assert_eq!(symbol.name, factmap::SymbolName::Named(format!("__imp_{name}")));
			assert_eq!(symbol.ty.layout(), Ok((width.bytes(), width.bytes())));
		}
		if let Wrap::T64(file) = pe {
			let exceptions = file.exception_x64().unwrap();
			assert!(!exceptions.image().is_empty());
			analysis.scan_exceptions();
			for function in exceptions.image() {
				let symbol = &analysis.symbols[&function.BeginAddress];
				assert_eq!(symbol.ty, ty::Type::Fn);
				assert_eq!(symbol.name, factmap::SymbolName::Fn);
			}
		}
		let facts = analysis.into_factmap();
		let mut output = Vec::new();
		facts.write(&mut output, "").unwrap();
		assert_eq!(factmap::FactMap::parse(std::str::from_utf8(&output).unwrap(), width).unwrap(), facts);
	}
}

#[test]
fn upgrade_type_union_layout() {
	let width = ty::PointerWidth::Bits64;
	let mut symbol = factmap::SymbolFact::new(0, ty::Type::parse("[u32;3]", width).unwrap(), factmap::SymbolName::Data);
	symbol.upgrade_type(ty::Type::U64).unwrap();
	assert_eq!(symbol.ty.layout(), Ok((16, 8)));
	assert_eq!(symbol.ty, ty::Type::parse("union{[u32;3],u64}", width).unwrap());
}
