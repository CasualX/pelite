use super::*;

/// Address-associated record in a fact map.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Fact {
	/// Textual annotation.
	///
	/// ```text
	/// Cx1000 "Comment"
	/// ```
	Comment(CommentFact),
	/// Decodable region with a specified arch.
	///
	/// ```text
	/// Dx1000 255 x86_64
	/// ```
	Decode(DecodeFact),
	/// Function metadata.
	///
	/// ```text
	/// Fx1000 {"bytes":199,...}
	/// ```
	Function(FunctionFact),
	/// References.
	///
	/// ```text
	/// Rx1000 0x2080
	/// ```
	Ref(RefFact),
	/// Typed symbol.
	///
	/// ```text
	/// Sx2000 u32 R
	/// ```
	Symbol(SymbolFact),
}

impl fmt::Display for Fact {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Fact::Comment(c) => c.fmt(f),
			Fact::Decode(d) => d.fmt(f),
			Fact::Function(fun) => fun.fmt(f),
			Fact::Ref(r) => r.fmt(f),
			Fact::Symbol(s) => s.fmt(f),
		}
	}
}

impl Fact {
	/// Order by RVA, then symbols, comments, references, functions, and decode regions.
	pub fn sort_key(&self) -> u64 {
		let (rva, rank) = match self {
			Fact::Symbol(symbol) => (symbol.rva, 0),
			Fact::Comment(comment) => (comment.rva, 1),
			Fact::Ref(reference) => (reference.rva, 2),
			Fact::Function(function) => (function.rva, 3),
			Fact::Decode(decode) => (decode.rva, 4),
		};
		(u64::from(rva) << 8) | rank
	}

	/// Parse a record by its prefix, accepting legacy `0xRVA` symbol records.
	///
	/// The prefix must start at the first character. `pointer_width` determines
	/// pointer layouts for symbol types; other fact kinds do not use it.
	pub fn parse(line: &str, pointer_width: ty::PointerWidth) -> Result<Fact, ParseLineError> {
		match line.chars().next() {
			Some('0' | 'S') => SymbolFact::parse(line, pointer_width).map(Fact::Symbol),
			Some('C') => line.parse().map(Fact::Comment),
			Some('D') => line.parse().map(Fact::Decode),
			Some('R') => line.parse().map(Fact::Ref),
			Some('F') => line.parse().map(Fact::Function),
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
	assert!(matches!(Fact::parse("Dx10 5 x86_64", width).unwrap(), Fact::Decode(_)));
	assert_eq!(Fact::parse("unknown", width).unwrap_err(), ParseLineError::UnknownFact('u'));
	assert_eq!(Fact::parse("Zx10", width).unwrap_err(), ParseLineError::UnknownFact('Z'));
	assert_eq!(Fact::parse("", width).unwrap_err(), ParseLineError::MissingField);
	assert_eq!(Fact::parse("Cx10", width).unwrap_err(), ParseLineError::MissingField);
}
