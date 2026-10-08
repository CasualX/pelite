use super::*;

#[derive(Clone, Debug)]
pub enum IndexedSymbol {
	Named(String),
	Generated { name: &'static str, rva: u32 },
	Weak(u32),
}

impl From<factmap::SymbolFact> for IndexedSymbol {
	fn from(symbol: factmap::SymbolFact) -> IndexedSymbol {
		match symbol.name {
			factmap::SymbolName::Named(name) => IndexedSymbol::Named(name),
			_ => IndexedSymbol::from(&symbol),
		}
	}
}

impl From<&factmap::SymbolFact> for IndexedSymbol {
	fn from(symbol: &factmap::SymbolFact) -> IndexedSymbol {
		match &symbol.name {
			factmap::SymbolName::Data => IndexedSymbol::Generated { name: "data", rva: symbol.rva },
			factmap::SymbolName::RData => IndexedSymbol::Generated { name: "rdata", rva: symbol.rva },
			factmap::SymbolName::Code => IndexedSymbol::Generated { name: "code", rva: symbol.rva },
			factmap::SymbolName::Weak => IndexedSymbol::Weak(symbol.rva),
			factmap::SymbolName::Fn => IndexedSymbol::Generated { name: "fn", rva: symbol.rva },
			factmap::SymbolName::Thunk => IndexedSymbol::Generated { name: "thunk", rva: symbol.rva },
			factmap::SymbolName::Undef => IndexedSymbol::Generated { name: "undef", rva: symbol.rva },
			factmap::SymbolName::Named(name) => IndexedSymbol::Named(name.clone()),
		}
	}
}

impl fmt::Display for IndexedSymbol {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			IndexedSymbol::Named(name) => f.write_str(name),
			IndexedSymbol::Generated { name, rva } => write!(f, "{name}_{rva:x}"),
			IndexedSymbol::Weak(rva) => write!(f, ".{rva:x}"),
		}
	}
}

pub fn arg() -> clap::Arg {
	clap::Arg::new("facts")
		.long("facts")
		.alias("symbols")
		.value_name("FACTS.txt|auto")
		.value_parser(clap::value_parser!(PathBuf))
		.action(clap::ArgAction::Append)
		.help("Load a factmap file or analyze the PE with auto; repeat to override earlier facts")
}

#[derive(Default)]
pub struct IndexedFacts {
	pub symbols: HashMap<u64, IndexedSymbol>,
	pub types: HashMap<u64, ty::Type>,
	pub comments: HashMap<u64, String>,
}

pub fn load(matches: &clap::ArgMatches, pointer_width: ty::PointerWidth, base: u64, pe: Option<pelite::PeFile<'_>>) -> Result<IndexedFacts> {
	let mut indexed = IndexedFacts::default();
	for path in matches.get_many::<PathBuf>("facts").into_iter().flatten() {
		let map = load_map(path, pointer_width, pe)?;
		indexed.extend(map.facts, base);
	}
	Ok(indexed)
}

/// Resolve one fact source, reserving the exact name `auto` for PE analysis.
pub fn load_map(path: &Path, pointer_width: ty::PointerWidth, pe: Option<pelite::PeFile<'_>>) -> Result<factmap::FactMap> {
	if path == Path::new("auto") {
		let pe = pe.ok_or_else(|| err("--facts auto requires a PE image"))?;
		return analysis::analyze(pe, false);
	}
	load_file(path, pointer_width)
}

/// Read and parse a factmap file, including files named `auto`.
pub fn load_file(path: &Path, pointer_width: ty::PointerWidth) -> Result<factmap::FactMap> {
	let source = fs::read_to_string(path).map_err(|error| err(format!("{}: {error}", path.display())))?;
	factmap::FactMap::parse(&source, pointer_width)
		.map_err(|error| err(format!("{}: {error}", path.display())))
}

impl IndexedFacts {
	pub fn extend(&mut self, facts: impl IntoIterator<Item = factmap::Fact>, base: u64) {
		for fact in facts {
			match fact {
				factmap::Fact::Comment(comment) => {
					if let Some(address) = base.checked_add(u64::from(comment.rva)) {
						self.comments.insert(address, comment.comment);
					}
				},
				factmap::Fact::Symbol(symbol) => {
					if let Some(address) = base.checked_add(u64::from(symbol.rva)) {
						if matches!(symbol.name, factmap::SymbolName::Undef) {
							self.types.remove(&address);
						}
						else if !matches!(symbol.name, factmap::SymbolName::Weak) || !self.symbols.contains_key(&address) {
							self.types.insert(address, symbol.ty.clone());
						}
					}
					index(&mut self.symbols, std::iter::once(factmap::Fact::Symbol(symbol)), base);
				},
				_ => {},
			}
		}
	}
}

pub fn index(symbols: &mut HashMap<u64, IndexedSymbol>, facts: impl IntoIterator<Item = factmap::Fact>, base: u64) {
	for fact in facts {
		let factmap::Fact::Symbol(symbol) = fact else { continue };
		if let Some(address) = base.checked_add(u64::from(symbol.rva)) {
			if matches!(symbol.name, factmap::SymbolName::Undef) {
				symbols.remove(&address);
			}
			else if matches!(symbol.name, factmap::SymbolName::Weak) {
				symbols.entry(address).or_insert_with(|| IndexedSymbol::from(symbol));
			}
			else {
				symbols.insert(address, IndexedSymbol::from(symbol));
			}
		}
	}
}

#[test]
fn comment_facts_are_indexed_and_later_comments_override() {
	let width = ty::PointerWidth::Bits64;
	let mut indexed = IndexedFacts::default();
	let first = factmap::FactMap::parse("#factmap\nSx10 code fn\nCx10 \"generated\"\nCx20 \"other\"\n", width).unwrap();
	let second = factmap::FactMap::parse("#factmap\nCx10 \"user override\"\n", width).unwrap();
	indexed.extend(first.facts, 0x180000000);
	indexed.extend(second.facts, 0x180000000);
	assert_eq!(indexed.symbols.len(), 1);
	assert_eq!(indexed.comments.len(), 2);
	assert_eq!(indexed.comments[&0x180000010], "user override");
	assert_eq!(indexed.comments[&0x180000020], "other");
}

#[test]
fn indexing_ignores_non_symbol_facts() {
	let map = factmap::FactMap::parse("#factmap\nSx10 code fn\nCx10 \"comment\"\nRx10 0x20\nCx20 \"another comment\"\n", ty::PointerWidth::Bits64).unwrap();
	let mut symbols = HashMap::new();
	index(&mut symbols, map.facts, 0x180000000);
	assert_eq!(symbols.len(), 1);
	assert_eq!(symbols[&0x180000010].to_string(), "fn_10");
}

#[test]
fn weak_entries_preserve_strong_symbols_and_can_be_replaced_or_removed() {
	let width = ty::PointerWidth::Bits64;
	let first = factmap::FactMap::parse("#factmap\nSx1000 code \"named\"\nSx1010 code fn\nSx1020 code _\nSx1030 code _\nSx1040 code _\nSx1050 code fn\n", width).unwrap();
	let second = factmap::FactMap::parse("#factmap\nSx1000 code _\nSx1010 code _\nSx1020 code \"replacement\"\nSx1030 code C\nSx1040 unk undef\nSx1050 unk undef\nSx1050 code _\nSx1060 code _\n", width).unwrap();
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
	let first = factmap::FactMap::parse("#factmap\nSx1000 code fn\nSx1010 u32 D\n", width).unwrap();
	let second = factmap::FactMap::parse("#factmap\nSx1000 unk undef\nSx1010 unk undef\nSx1020 unk undef\nSx1030 code fn\nSx1030 unk undef\nSx1000 code \"new\"\n", width).unwrap();
	let mut symbols = HashMap::new();
	index(&mut symbols, first.facts, 0x180000000);
	index(&mut symbols, second.facts, 0x180000000);
	assert_eq!(symbols.len(), 1);
	assert_eq!(symbols[&0x180001000].to_string(), "new");
}

#[test]
fn later_symbol_entries_replace_earlier_names() {
	let width = ty::PointerWidth::Bits64;
	let first = factmap::FactMap::parse("#factmap\nSx1000 code C\nSx1010 code \"first\"\nSx1020 code fn\nSx1030 code thunk\nSx2000 u32 D\nSx2010 u32 R\n", width).unwrap();
	let second = factmap::FactMap::parse("#factmap\nSx1010 code \"last\"\n", width).unwrap();
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
		indexed.extend(factmap::FactMap::parse(source, width).unwrap().facts, 0);
	}
	assert_eq!(indexed.types.len(), 2);
	assert_eq!(indexed.types[&0x10], ty::Type::U32);
	assert_eq!(indexed.types[&0x30], ty::Type::CStr);
	assert_eq!(indexed.symbols[&0x10].to_string(), "first");
	assert_eq!(indexed.symbols[&0x30].to_string(), "last");
	assert!(!indexed.symbols.contains_key(&0x20));
}
