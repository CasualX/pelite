use super::*;

#[derive(Clone, Debug)]
pub enum IndexedSymbol {
	Named(String),
	Generated { name: &'static str, rva: u32 },
}

impl From<symtext::Symbol> for IndexedSymbol {
	fn from(symbol: symtext::Symbol) -> Self {
		match symbol.name {
			symtext::SymbolName::Named(name) => Self::Named(name),
			_ => Self::from(&symbol),
		}
	}
}

impl From<&symtext::Symbol> for IndexedSymbol {
	fn from(symbol: &symtext::Symbol) -> Self {
		match &symbol.name {
			symtext::SymbolName::Named(name) => Self::Named(name.clone()),
			symtext::SymbolName::Code => Self::Generated { name: "code", rva: symbol.rva },
			symtext::SymbolName::D => Self::Generated { name: "data", rva: symbol.rva },
			symtext::SymbolName::Fn => Self::Generated { name: "fn", rva: symbol.rva },
			symtext::SymbolName::Thunk => Self::Generated { name: "thunk", rva: symbol.rva },
		}
	}
}

impl fmt::Display for IndexedSymbol {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Named(name) => f.write_str(name),
			Self::Generated { name, rva } => write!(f, "{name}_{rva:x}"),
		}
	}
}

pub fn arg() -> clap::Arg {
	clap::Arg::new("symbols")
		.long("symbols")
		.value_name("SYMBOLS.txt")
		.value_parser(clap::value_parser!(PathBuf))
		.action(clap::ArgAction::Append)
		.help("Load a symtext file; repeat to override earlier symbols at the same location")
}

pub fn load(matches: &clap::ArgMatches, pointer_width: ty::PointerWidth, base: u64) -> Result<HashMap<u64, IndexedSymbol>> {
	let mut symbols = HashMap::new();
	for path in matches.get_many::<PathBuf>("symbols").into_iter().flatten() {
		let source = fs::read_to_string(path).map_err(|error| err(format!("{}: {error}", path.display())))?;
		let database = symtext::SymbolDatabase::parse(&source, pointer_width)
			.map_err(|error| err(format!("{}: {error}", path.display())))?;
		index(&mut symbols, database, base);
	}
	Ok(symbols)
}

pub fn index(symbols: &mut HashMap<u64, IndexedSymbol>, database: symtext::SymbolDatabase, base: u64) {
	for symbol in database.entries {
		if let Some(address) = base.checked_add(u64::from(symbol.rva)) {
			symbols.insert(address, IndexedSymbol::from(symbol));
		}
	}
}

#[test]
fn later_symbol_entries_replace_earlier_names() {
	let width = ty::PointerWidth::Bits64;
	let first = symtext::SymbolDatabase::parse("#symtext\n0x1000 code C\n0x1010 code \"first\"\n0x1020 code Fn\n0x1030 code Thunk\n0x2000 u32 D\n", width).unwrap();
	let second = symtext::SymbolDatabase::parse("#symtext\n0x1010 code \"last\"\n", width).unwrap();
	let mut symbols = HashMap::new();
	index(&mut symbols, first, 0x180000000);
	index(&mut symbols, second, 0x180000000);
	assert_eq!(symbols[&0x180001000].to_string(), "code_1000");
	assert_eq!(symbols[&0x180001010].to_string(), "last");
	assert_eq!(symbols[&0x180001020].to_string(), "fn_1020");
	assert_eq!(symbols[&0x180001030].to_string(), "thunk_1030");
	assert_eq!(symbols[&0x180002000].to_string(), "data_2000");
	let decoded = iced::decode_bytes(&[0x8b, 0x05, 0xfa, 0x0f, 0, 0], 64, 0x180001000, 0x180001000, 0x180001006, false, Arc::new(symbols));
	assert!(decoded[0].instruction.contains("data_2000"), "{}", decoded[0].instruction);
}
