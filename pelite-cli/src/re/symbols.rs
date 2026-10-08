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
	pub functions: HashMap<u64, factmap::FunctionFact>,
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
				factmap::Fact::Function(function) => {
					if let Some(address) = base.checked_add(u64::from(function.rva)) {
						match self.functions.entry(address) {
							std::collections::hash_map::Entry::Occupied(mut entry) => {
								entry.get_mut().merge_content(function.content);
							},
							std::collections::hash_map::Entry::Vacant(entry) => {
								entry.insert(function);
							},
						}
					}
				},
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

#[cfg(test)]
mod tests;