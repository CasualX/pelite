use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RefFact {
	pub rva: u32,
	pub target_rva: u32,
}

impl RefFact {
	pub fn parse(line: &str) -> result::Result<RefFact, ParseLineError> {
		let (rva, rest) = token(line)?;
		let rva = rva.strip_prefix('R').ok_or(ParseLineError::InvalidPrefix("R"))?;
		let rva = parse_rva(rva)?;
		let (target_rva, rest) = token(rest)?;
		let target_rva = target_rva.strip_prefix('0').ok_or(ParseLineError::InvalidTargetRva)?;
		let target_rva = parse_rva(target_rva).map_err(|_| ParseLineError::InvalidTargetRva)?;
		if !rest.trim_ascii().is_empty() {
			return Err(ParseLineError::UnexpectedText("target RVA"));
		}
		Ok(RefFact { rva, target_rva })
	}
}

impl fmt::Display for RefFact {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Rx{:x} {:#x}", self.rva, self.target_rva)
	}
}

#[test]
fn parse_reference() {
	let fact = RefFact::parse(" Rx1234\t0xABCD ").unwrap();
	assert_eq!(fact, RefFact { rva: 0x1234, target_rva: 0xabcd });
	assert_eq!(fact.to_string(), "Rx1234 0xabcd");
	assert_eq!(RefFact::parse(&fact.to_string()).unwrap(), fact);
	assert_eq!(RefFact::parse("Rx0 0xffffffff").unwrap(), RefFact { rva: 0, target_rva: u32::MAX });
}

#[test]
fn reject_invalid_reference() {
	for (line, error) in [
		("", ParseLineError::MissingField),
		("x1 0x2", ParseLineError::InvalidPrefix("R")),
		("Cx1 0x2", ParseLineError::InvalidPrefix("R")),
		("Rx1", ParseLineError::MissingField),
		("R0x1 0x2", ParseLineError::InvalidRva),
		("Rx100000000 0x2", ParseLineError::InvalidRva),
		("Rx1 x2", ParseLineError::InvalidTargetRva),
		("Rx1 0x", ParseLineError::InvalidTargetRva),
		("Rx1 0xgg", ParseLineError::InvalidTargetRva),
		("Rx1 0x100000000", ParseLineError::InvalidTargetRva),
		("Rx1 0x2 extra", ParseLineError::UnexpectedText("target RVA")),
	] {
		assert_eq!(RefFact::parse(line).unwrap_err(), error, "{line}");
	}
}
