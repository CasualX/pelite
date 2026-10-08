use super::*;

#[test]
fn section_classification_includes_alignment_padding() {
	let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../demo/Demo64.dll");
	let map = pelite::FileMap::open(&path).unwrap();
	let pe = PeFile::from_bytes(&map).unwrap();
	let sections = pe.section_headers().iter().map(|section| (pe.offset_of(section), *section)).collect::<Vec<_>>();
	let mut section = sections[0].1;
	section.VirtualAddress = 0x1000;
	for (alignment, virtual_size, raw_size, rounded_size) in [
		(0x1000, 0x801, 0x200, 0x1000),
		(0x1000, 0x200, 0x1001, 0x1000),
		(0x1000, 0x600, 0x5000, 0x1000),
		(0x1000, 0, 0x1001, 0x2000),
		(0x200, 0x201, 0x100, 0x400),
		(0x1000, 0x1000, 0x1000, 0x1000),
		(0x1000, 0, 0, 0),
	] {
		section.VirtualSize = virtual_size;
		section.SizeOfRawData = raw_size;
		for (flags, executable, read_only) in [
			(image::IMAGE_SCN_MEM_READ | image::IMAGE_SCN_MEM_EXECUTE | image::IMAGE_SCN_CNT_CODE, true, false),
			(image::IMAGE_SCN_MEM_READ | image::IMAGE_SCN_CNT_INITIALIZED_DATA, false, true),
			(image::IMAGE_SCN_MEM_READ | image::IMAGE_SCN_MEM_WRITE, false, false),
		] {
			section.Characteristics = flags;
			let mut bytes = map.as_ref().to_vec();
			let view = dataview::DataView::from_mut(bytes.as_mut_slice());
			for &(offset, mut header) in &sections {
				header.Characteristics = 0;
				view.write(offset, &header);
			}
			view.write(sections[0].0, &section);
			let mut input = AnalysisInput::new(PeFile::from_bytes(&bytes).unwrap()).unwrap();
			input.alignment = alignment;
			assert!(!input.executable(0xfff));
			assert!(!input.read_only_data(0xfff));
			if rounded_size != 0 {
				let last = 0x1000 + rounded_size - 1;
				for rva in [0x1000, last] {
					assert_eq!(input.executable(rva), executable);
					assert_eq!(input.read_only_data(rva), read_only);
				}
			}
			let end = 0x1000 + rounded_size;
			assert!(!input.executable(end));
			assert!(!input.read_only_data(end));
		}
	}
}

#[test]
fn symbol_targets_accept_image_gaps_and_reject_out_of_image_addresses() {
	for filename in ["Demo.dll", "Demo64.dll"] {
		let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../demo").join(filename);
		let map = pelite::FileMap::open(&path).unwrap();
		let input = AnalysisInput::new(PeFile::from_bytes(&map).unwrap()).unwrap();
		let gap = match input.pe.optional_header() {
			Wrap::T32(h) => h.SizeOfHeaders,
			Wrap::T64(h) => h.SizeOfHeaders,
		};
		assert!(gap < input.size);
		assert!(!input.pe.section_headers().iter().any(|section| {
			gap.checked_sub(section.VirtualAddress)
				.is_some_and(|offset| offset < section.VirtualSize.max(section.SizeOfRawData))
		}));
		let mut output = AnalysisOutput::default();
		for rva in [0, gap, input.size - 1] {
			assert!(output.add_symbol(&input, rva, None));
			assert!(output.symbols.contains_key(&rva));
		}
		assert_eq!(output.symbols[&gap].name, factmap::SymbolName::Data);
		for rva in [input.size, input.size + 1, u32::MAX] {
			assert!(!output.add_symbol(&input, rva, None));
			assert!(!output.symbols.contains_key(&rva));
		}
		let mut output = AnalysisOutput::default();
		let base = input.pe.image_base();
		output.add_symbol_va(&input, base + u64::from(gap), None);
		output.add_symbol_va(&input, base - 1, None);
		output.add_symbol_va(&input, base + u64::from(input.size), None);
		output.add_symbol_va(&input, base + u64::from(input.size) + 1, None);
		assert_eq!(output.symbols.len(), 1);
		assert!(output.symbols.contains_key(&gap));
	}
}

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
fn upgrade_type_union_layout() {
	let width = ty::PointerWidth::Bits64;
	let mut symbol = factmap::SymbolFact::new(0, ty::Type::parse("[u32;3]", width).unwrap(), factmap::SymbolName::Data);
	symbol.upgrade_type(ty::Type::U64).unwrap();
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
