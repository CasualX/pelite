use super::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FactMap {
	pub facts: Vec<Fact>,
}

impl FactMap {
	pub fn parse(input: &str, pointer_width: ty::PointerWidth) -> result::Result<FactMap, ParseError> {
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
fn mixed_map_round_trip() {
	let width = ty::PointerWidth::Bits64;
	let input = "#factmap\n\n  # ignored\n 0x10 code fn \nCx10 \"comment\"\n\t\nRx10 0x20\nSx10 code undef\n";
	let map = FactMap::parse(input, width).unwrap();
	let mut output = Vec::new();
	map.write(&mut output, "first line\nsecond line").unwrap();
	let output = String::from_utf8(output).unwrap();
	assert_eq!(output, "#factmap\n# first line\n# second line\nSx10 code fn\nCx10 \"comment\"\nRx10 0x20\nSx10 code undef\n");
	assert_eq!(FactMap::parse(&output, width).unwrap(), map);
}

#[test]
fn map_error_line_numbers() {
	let width = ty::PointerWidth::Bits64;
	assert_eq!(FactMap::parse("", width).unwrap_err(), ParseError {
		line: 1,
		error: ParseLineError::MissingHeader,
	});
	let error = FactMap::parse("#factmap\n\n # ignored\nSx10 code fn\nunknown\n", width).unwrap_err();
	assert_eq!(error, ParseError { line: 5, error: ParseLineError::UnknownFact });
	assert_eq!(error.to_string(), "line 5: expected a symbol, comment, or reference fact");
}
