use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Fact {
	Comment(CommentFact),
	Function(FunctionFact),
	Ref(RefFact),
	Symbol(SymbolFact),
}

impl fmt::Display for Fact {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Fact::Comment(c) => c.fmt(f),
			Fact::Function(fun) => fun.fmt(f),
			Fact::Ref(r) => r.fmt(f),
			Fact::Symbol(s) => s.fmt(f),
		}
	}
}

impl Fact {
	/// Order by RVA, then symbols, comments, references, and functions.
	pub fn sort_key(&self) -> u64 {
		let (rva, rank) = match self {
			Fact::Symbol(symbol) => (symbol.rva, 0),
			Fact::Comment(comment) => (comment.rva, 1),
			Fact::Ref(reference) => (reference.rva, 2),
			Fact::Function(function) => (function.rva, 3),
		};
		(u64::from(rva) << 8) | rank
	}

	pub fn parse(line: &str, pointer_width: ty::PointerWidth) -> result::Result<Fact, ParseLineError> {
		match line.chars().next() {
			Some('0' | 'S') => SymbolFact::parse(line, pointer_width).map(Fact::Symbol),
			Some('C') => CommentFact::parse(line).map(Fact::Comment),
			Some('R') => RefFact::parse(line).map(Fact::Ref),
			Some('F') => FunctionFact::parse(line).map(Fact::Function),
			Some(letter) => Err(ParseLineError::UnknownFact(letter)),
			None => Err(ParseLineError::MissingField),
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
	assert!(matches!(Fact::parse("Fx10 {}", width).unwrap(), Fact::Function(_)));
	assert_eq!(Fact::parse("unknown", width).unwrap_err(), ParseLineError::UnknownFact('u'));
	assert_eq!(Fact::parse("Zx10", width).unwrap_err(), ParseLineError::UnknownFact('Z'));
	assert_eq!(Fact::parse("", width).unwrap_err(), ParseLineError::MissingField);
	assert_eq!(Fact::parse("Cx10", width).unwrap_err(), ParseLineError::MissingField);
}
