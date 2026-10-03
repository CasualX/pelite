use super::*;

#[derive(serde::Serialize)]
struct ListedSymbol {
	rva: u32,
	name: String,
	ty: String,
}

#[derive(serde::Serialize)]
struct Lookup {
	start_rva: u32,
	end_rva: u32,
	symbols: Vec<ListedSymbol>,
}

pub fn command() -> clap::Command {
	clap::Command::new("symbol")
		.about("Look up symbols in a PE address range")
		.after_help(include_str!("../docs/re-symbol.md"))
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("range")
			.value_name("RANGE")
			.value_parser(AddressRange::parse)
			.required(true))
		.arg(symbols::arg())
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let range = *matches.get_one::<AddressRange>("range").expect("required by clap");
	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let range = range.to_rva(pe)?;
	let mut symbols = BTreeMap::new();
	for path in matches.get_many::<PathBuf>("facts").into_iter().flatten() {
		let map = symbols::load_map(path, ty::PointerWidth::from(pe), Some(pe))?;
		for fact in map.facts {
			let factmap::Fact::Symbol(symbol) = fact else { continue };
			if matches!(symbol.name, factmap::SymbolName::Undef) {
				symbols.remove(&symbol.rva);
			}
			else if matches!(symbol.name, factmap::SymbolName::Weak) {
				symbols.entry(symbol.rva).or_insert_with(|| symbol);
			}
			else {
				symbols.insert(symbol.rva, symbol);
			}
		}
	}
	let selected = symbols.range(range.start..range.end).map(|(_, symbol)| {
		Ok(ListedSymbol {
			rva: symbol.rva,
			name: symbols::IndexedSymbol::from(symbol).to_string(),
			ty: symbol.ty.to_string(),
		})
	}).collect::<Result<Vec<_>>>()?;
	print("Symbols", &Lookup { start_rva: range.start, end_rva: range.end, symbols: selected }, format)
}
