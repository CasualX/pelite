use super::*;

/// An ordered collection of facts about an image.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FactMap {
	/// Records in source order, including repeated addresses and fact kinds.
	pub facts: Vec<Fact>,
}

impl FactMap {
	/// Find the first symbol with an exact explicit name by linearly scanning the facts.
	///
	/// Markers and other fact kinds are ignored. Facts are searched in stored order;
	/// overrides and removals are not applied by this lookup.
	pub fn slow_lookup_symbol_by_name(&self, name: &str) -> Option<&SymbolFact> {
		self.facts.iter().find_map(|fact| match fact {
			Fact::Symbol(symbol) if matches!(&symbol.name, SymbolName::Named(value) if value.as_ref() == name) => Some(symbol),
			_ => None,
		})
	}

	/// Parse a complete `#factmap` file, also accepting the legacy `#symtext` header.
	///
	/// Blank lines and lines beginning with `#` after trimming are ignored.
	/// `pointer_width` determines pointer layouts in symbol types. Parsing preserves
	/// record order and reports the first invalid line.
	pub fn parse(input: &str, pointer_width: ty::PointerWidth) -> Result<FactMap, ParseError> {
		let mut facts = Vec::new();
		let mut lines = input.lines();
		if !matches!(lines.next(), Some("#factmap" | "#symtext")) {
			return Err(ParseError { line: 1, error: ParseLineError::MissingHeader });
		}
		let mut index = 1;
		for line in lines {
			index += 1;
			let line = line.trim_ascii();
			if line.is_empty() || line.starts_with('#') {
				continue;
			}
			let fact = Fact::parse(line, pointer_width).map_err(|error| ParseError { line: index, error })?;
			facts.push(fact);
		}
		Ok(FactMap { facts })
	}

	/// Write the `#factmap` header, optional file comment, and facts in stored order.
	///
	/// Each line of `comment` is prefixed with `# `. Facts use their canonical
	/// text representation; they are neither sorted nor merged.
	pub fn write(&self, mut output: impl Write, comment: &str) -> io::Result<()> {
		writeln!(output, "#factmap")?;
		for line in comment.lines() {
			writeln!(output, "# {line}")?;
		}
		for fact in &self.facts {
			writeln!(output, "{}", fact)?;
		}
		Ok(())
	}
}

#[test]
fn lookup_symbol_by_name() {
	let symbols = [
		SymbolFact::new(0x10, ty::Type::Code, SymbolName::Fn),
		SymbolFact::new(0x20, ty::Type::U32, SymbolName::Named("fn".into())),
		SymbolFact::new(0x30, ty::Type::Code, SymbolName::Named("fn".into())),
		SymbolFact::new(0x40, ty::Type::Code, SymbolName::Named("a::name+* with spaces".into())),
	];
	let mut map = FactMap::parse("#factmap\nCx0 \"fn\"\n", ty::PointerWidth::X64).unwrap();
	map.facts.extend(symbols.iter().cloned().map(Fact::Symbol));
	assert_eq!(map.slow_lookup_symbol_by_name("fn"), Some(&symbols[1]));
	assert_eq!(map.slow_lookup_symbol_by_name("a::name+* with spaces"), Some(&symbols[3]));
	assert_eq!(map.slow_lookup_symbol_by_name("Fn"), None);
	assert_eq!(map.slow_lookup_symbol_by_name("missing"), None);
	assert_eq!(FactMap::default().slow_lookup_symbol_by_name("fn"), None);
}

#[test]
fn mixed_map_round_trip() {
	let width = ty::PointerWidth::X64;
	let input = "#factmap\n\n  # ignored\n 0x10 code fn \nCx10 \"comment\"\n\t\nRx10 0x20\nFx10 { \"size\": 42 }\nSx10 code undef\n";
	let map = FactMap::parse(input, width).unwrap();
	let mut output = Vec::new();
	map.write(&mut output, "first line\nsecond line").unwrap();
	let output = String::from_utf8(output).unwrap();
	assert_eq!(output, "#factmap\n# first line\n# second line\nSx10 code fn\nCx10 \"comment\"\nRx10 0x20\nFx10 { \"size\": 42 }\nSx10 code undef\n");
	assert_eq!(FactMap::parse(&output, width).unwrap(), map);
}

#[test]
fn decode_map_round_trip() {
	let width = ty::PointerWidth::X64;
	let map = FactMap::parse("#factmap\nDx1000 x86 5\nCx1000 \"decode region\"\nDx2000 16 x86_64\n", width).unwrap();
	let mut output = Vec::new();
	map.write(&mut output, "").unwrap();
	let output = String::from_utf8(output).unwrap();
	assert_eq!(output, "#factmap\nDx1000 x86_32 5\nCx1000 \"decode region\"\nDx2000 x86_64 16\n");
	assert_eq!(FactMap::parse(&output, width).unwrap(), map);
	assert!(map.facts[1].sort_key() < map.facts[0].sort_key());
	assert!(map.facts[0].sort_key() < map.facts[2].sort_key());
	let error = FactMap::parse("#factmap\nDx1000 x86 bad\n", width).unwrap_err();
	assert_eq!(error, ParseError { line: 2, error: ParseLineError::InvalidByteCount });
}

#[test]
fn align_map_round_trip() {
	let width = ty::PointerWidth::X64;
	let map = FactMap::parse("#factmap\nAx100E 4\nDx100e x86_64 2\nAx1010 0\n", width).unwrap();
	let mut output = Vec::new();
	map.write(&mut output, "").unwrap();
	let output = String::from_utf8(output).unwrap();
	assert_eq!(output, "#factmap\nAx100e 4\nDx100e x86_64 2\nAx1010 0\n");
	assert_eq!(FactMap::parse(&output, width).unwrap(), map);
	assert!(map.facts[1].sort_key() < map.facts[0].sort_key());
	assert!(map.facts[0].sort_key() < map.facts[2].sort_key());
	assert_eq!(FactMap::parse("#factmap\nAx100e 256\n", width).unwrap_err(), ParseError {
		line: 2, error: ParseLineError::InvalidAlign,
	});
}

#[test]
fn map_error_line_numbers() {
	let width = ty::PointerWidth::X64;
	assert_eq!(FactMap::parse("", width).unwrap_err(), ParseError {
		line: 1,
		error: ParseLineError::MissingHeader,
	});
	let error = FactMap::parse("#factmap\n\n # ignored\nSx10 code fn\nunknown\n", width).unwrap_err();
	assert_eq!(error, ParseError { line: 5, error: ParseLineError::UnknownFact('u') });
	assert_eq!(error.to_string(), "line 5: unknown fact 'u'");
}
