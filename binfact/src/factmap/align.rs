use super::*;

/// Padding from an RVA up to the next `1 << align` byte boundary.
///
/// ```text
/// Ax100e 4
/// ```
///
/// This example marks two padding bytes before the 16-byte boundary at `0x1010`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlignFact {
	/// Start of the padding.
	pub rva: u32,
	/// Base-two exponent of the alignment, rather than a byte count.
	pub align: u8,
}

impl str::FromStr for AlignFact {
	type Err = ParseLineError;

	/// Parse `AxRVA align`, with a hexadecimal RVA and decimal alignment exponent.
	fn from_str(line: &str) -> Result<Self, Self::Err> {
		let (rva, rest) = token(line)?;
		let rva = rva.strip_prefix('A').ok_or(ParseLineError::InvalidPrefix("A"))?;
		let rva = parse_rva(rva)?;
		let (align, rest) = token(rest)?;
		let align = align.parse().map_err(|_| ParseLineError::InvalidAlign)?;
		if !rest.trim_ascii().is_empty() {
			return Err(ParseLineError::UnexpectedText("align"));
		}
		Ok(AlignFact { rva, align })
	}
}

impl fmt::Display for AlignFact {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Ax{:x} {}", self.rva, self.align)
	}
}

#[test]
fn parse_align() {
	for (line, fact, expected) in [
		(" Ax100E\t4 ", AlignFact { rva: 0x100e, align: 4 }, "Ax100e 4"),
		("Ax0 0", AlignFact { rva: 0, align: 0 }, "Ax0 0"),
		("Axffffffff 255", AlignFact { rva: u32::MAX, align: u8::MAX }, "Axffffffff 255"),
	] {
		assert_eq!(AlignFact::from_str(line).unwrap(), fact);
		assert_eq!(fact.to_string(), expected);
		assert_eq!(AlignFact::from_str(&fact.to_string()).unwrap(), fact);
	}
}

#[test]
fn reject_invalid_align() {
	for (line, error) in [
		("", ParseLineError::MissingField),
		("x1 4", ParseLineError::InvalidPrefix("A")),
		("A0x1 4", ParseLineError::InvalidRva),
		("Ax 4", ParseLineError::InvalidRva),
		("Ax100000000 4", ParseLineError::InvalidRva),
		("Ax1", ParseLineError::MissingField),
		("Ax1 -1", ParseLineError::InvalidAlign),
		("Ax1 1.5", ParseLineError::InvalidAlign),
		("Ax1 256", ParseLineError::InvalidAlign),
		("Ax1 4 extra", ParseLineError::UnexpectedText("align")),
	] {
		assert_eq!(AlignFact::from_str(line).unwrap_err(), error, "{line}");
	}
}
