use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Fact {
	Comment(CommentFact),
	Ref(RefFact),
	Symbol(SymbolFact),
}

impl fmt::Display for Fact {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Fact::Comment(c) => c.fmt(f),
			Fact::Ref(r) => r.fmt(f),
			Fact::Symbol(s) => s.fmt(f),
		}
	}
}

impl Fact {
	pub fn parse(line: &str, pointer_width: ty::PointerWidth) -> result::Result<Fact, ParseLineError> {
		if line.starts_with("0") || line.starts_with("S") {
			SymbolFact::parse(line, pointer_width).map(Fact::Symbol)
		}
		else if line.starts_with("C") {
			CommentFact::parse(line).map(Fact::Comment)
		}
		else if line.starts_with("R") {
			RefFact::parse(line).map(Fact::Ref)
		}
		else {
			Err(ParseLineError::UnknownFact)
		}
	}
}

#[test]
fn dispatch_fact() {
	let width = ty::PointerWidth::Bits64;
	assert!(matches!(Fact::parse("0x10 code fn", width).unwrap(), Fact::Symbol(_)));
	assert!(matches!(Fact::parse("Sx10 code fn", width).unwrap(), Fact::Symbol(_)));
	assert!(matches!(Fact::parse("Cx10 \"comment\"", width).unwrap(), Fact::Comment(_)));
	assert!(matches!(Fact::parse("Rx10 0x20", width).unwrap(), Fact::Ref(_)));
	assert_eq!(Fact::parse("unknown", width).unwrap_err(), ParseLineError::UnknownFact);
	assert_eq!(Fact::parse("Cx10", width).unwrap_err(), ParseLineError::MissingField);
}
