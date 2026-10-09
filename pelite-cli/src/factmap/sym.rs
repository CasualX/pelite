use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SymbolName {
	/// Data.
	Data,
	/// Read-only data.
	RData,
	/// Code.
	Code,
	/// Weak
	///
	/// Weak anchors mark an instruction start, but do not contribute labels or symbols.
	/// They do not override a previously defined symbol.
	Weak,
	/// Function.
	Fn,
	/// Function thunk.
	Thunk,
	/// Remove an earlier symbol at the same RVA.
	Undef,
	/// Named symbol.
	Named(String),
}

impl fmt::Display for SymbolName {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			SymbolName::Data => f.write_str("D"),
			SymbolName::RData => f.write_str("R"),
			SymbolName::Code => f.write_str("C"),
			SymbolName::Weak => f.write_str("_"),
			SymbolName::Fn => f.write_str("fn"),
			SymbolName::Thunk => f.write_str("thunk"),
			SymbolName::Undef => f.write_str("undef"),
			SymbolName::Named(name) => f.write_str(&serde_json::to_string(name).map_err(|_| fmt::Error)?),
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SymbolFact {
	pub rva: u32,
	pub ty: ty::Type,
	pub name: SymbolName,
}

impl SymbolFact {
	pub fn new(rva: u32, ty: ty::Type, name: SymbolName) -> SymbolFact {
		SymbolFact { rva, ty, name }
	}
}

impl SymbolFact {
	pub fn parse(line: &str, pointer_width: ty::PointerWidth) -> result::Result<SymbolFact, ParseLineError> {
		let (rva, rest) = token(line)?;
		let rva = rva.strip_prefix('0').or_else(|| rva.strip_prefix('S'))
			.ok_or(ParseLineError::InvalidPrefix("0 or S"))?;
		let rva = parse_rva(rva)?;
		let (type_token, rest) = token(rest)?;
		let type_text = if type_token.starts_with('"') {
			serde_json::from_str::<String>(type_token).map_err(|error| ParseLineError::InvalidQuotedType(error.to_string()))?
		}
		else {
			type_token.to_owned()
		};
		let (name_token, rest) = token(rest)?;
		if !rest.trim_ascii().is_empty() {
			return Err(ParseLineError::UnexpectedText("symbol name"));
		}
		let name = match name_token {
			"D" => SymbolName::Data,
			"R" => SymbolName::RData,
			"C" => SymbolName::Code,
			"_" => SymbolName::Weak,
			"fn" => SymbolName::Fn,
			"thunk" => SymbolName::Thunk,
			"undef" => SymbolName::Undef,
			name if name.starts_with('"') => SymbolName::Named(
				serde_json::from_str(name).map_err(|error| ParseLineError::InvalidQuotedName(error.to_string()))?
			),
			_ => return Err(ParseLineError::InvalidSymbolName),
		};
		let ty = ty::Type::parse(&type_text, pointer_width).map_err(ParseLineError::InvalidType)?;
		Ok(SymbolFact { rva, ty, name })
	}
}

impl fmt::Display for SymbolFact {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Sx{:x} ", self.rva)?;
		let type_text = self.ty.to_string();
		if type_text.chars().any(char::is_whitespace) {
			write!(f, "{}", serde_json::to_string(&type_text).expect("string serialization cannot fail"))?;
		}
		else {
			write!(f, "{type_text}")?;
		}
		write!(f, " {}", self.name)
	}
}

#[test]
fn parse_symbol_names() {
	for (name, expected) in [
		("D", SymbolName::Data),
		("R", SymbolName::RData),
		("C", SymbolName::Code),
		("_", SymbolName::Weak),
		("fn", SymbolName::Fn),
		("thunk", SymbolName::Thunk),
		("undef", SymbolName::Undef),
		("\"fn\"", SymbolName::Named("fn".to_owned())),
		("\"R\"", SymbolName::Named("R".to_owned())),
	] {
		let fact = SymbolFact::parse(&format!("Sx1234 code {name}"), ty::PointerWidth::Bits64).unwrap();
		assert_eq!(fact, SymbolFact::new(0x1234, ty::Type::Code, expected));
		assert_eq!(SymbolFact::parse(&format!("0x1234 code {name}"), ty::PointerWidth::Bits64).unwrap(), fact);
		assert_eq!(fact.to_string(), format!("Sx1234 code {name}"));
		assert_eq!(SymbolFact::parse(&fact.to_string(), ty::PointerWidth::Bits64).unwrap(), fact);
	}
}

#[test]
fn parse_quoted_symbol() {
	for width in [ty::PointerWidth::Bits32, ty::PointerWidth::Bits64] {
		let fact = SymbolFact::parse(r#"Sx20 "union { u32, u64 }" "a \"quoted\" name""#, width).unwrap();
		assert_eq!(fact.rva, 0x20);
		assert_eq!(fact.ty.to_string(), "union{u32,u64}");
		assert_eq!(fact.name, SymbolName::Named("a \"quoted\" name".to_owned()));
		assert_eq!(SymbolFact::parse(&fact.to_string(), width).unwrap(), fact);
	}
}

#[test]
fn reject_invalid_symbol() {
	let width = ty::PointerWidth::Bits64;
	for (line, error) in [
		("", ParseLineError::MissingField),
		("x1 code fn", ParseLineError::InvalidPrefix("0 or S")),
		("Rx1 code fn", ParseLineError::InvalidPrefix("0 or S")),
		("Sx1 code", ParseLineError::MissingField),
		("S0x1 code fn", ParseLineError::InvalidRva),
		("Sx100000000 code fn", ParseLineError::InvalidRva),
		("Sx1 code nope", ParseLineError::InvalidSymbolName),
		("Sx1 code \"unfinished", ParseLineError::UnterminatedQuotedField),
		("Sx1 code fn extra", ParseLineError::UnexpectedText("symbol name")),
	] {
		assert_eq!(SymbolFact::parse(line, width).unwrap_err(), error, "{line}");
	}
	let error = SymbolFact::parse("Sx1 bad_type fn", width).unwrap_err();
	assert_eq!(error, ParseLineError::InvalidType(ty::ParseError {
		offset: 8,
		kind: ty::ParseErrorKind::UnknownType("bad_type".to_owned()),
	}));
	assert_eq!(error::Error::source(&error).unwrap().to_string(), "unknown type 'bad_type' at byte 8");
	assert!(matches!(SymbolFact::parse(r#"Sx1 "bad\q" fn"#, width), Err(ParseLineError::InvalidQuotedType(_))));
	assert!(matches!(SymbolFact::parse(r#"Sx1 code "bad\q""#, width), Err(ParseLineError::InvalidQuotedName(_))));
}
