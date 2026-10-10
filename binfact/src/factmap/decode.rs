use super::*;

/// Decodable region with a specified arch.
///
/// ```text
/// Dx1000 255 x86_64
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecodeFact {
	/// Start of the region.
	pub rva: u32,
	/// Number of bytes in the region.
	pub bytes: u32,
	/// Instruction arch used to decode the region.
	pub arch: Arch,
}

impl str::FromStr for DecodeFact {
	type Err = ParseLineError;

	/// Parse `DxRVA bytes arch`, with a hexadecimal RVA and decimal byte count.
	fn from_str(line: &str) -> Result<Self, Self::Err> {
		let (rva, rest) = token(line)?;
		let rva = rva.strip_prefix('D').ok_or(ParseLineError::InvalidPrefix("D"))?;
		let rva = parse_rva(rva)?;
		let (bytes, rest) = token(rest)?;
		let bytes = parse_u32(bytes).map_err(|_| ParseLineError::InvalidByteCount)?;
		let (arch, rest) = token(rest)?;
		let arch = arch.parse().map_err(ParseLineError::InvalidArch)?;
		if !rest.trim_ascii_end().is_empty() {
			return Err(ParseLineError::UnexpectedText("arch"));
		}
		Ok(DecodeFact { rva, bytes, arch })
	}
}

impl fmt::Display for DecodeFact {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Dx{:x} {} {}", self.rva, self.bytes, self.arch)
	}
}

#[test]
fn parse_decode() {
	for arch in [Arch::X86_16, Arch::X86_32, Arch::X86_64] {
		let fact = DecodeFact::from_str(&format!(" DxABCD\t15 {arch} ")).unwrap();
		assert_eq!(fact, DecodeFact { rva: 0xabcd, bytes: 15, arch });
		assert_eq!(fact.to_string(), format!("Dxabcd 15 {arch}"));
		assert_eq!(DecodeFact::from_str(&fact.to_string()).unwrap(), fact);
	}
	assert_eq!(DecodeFact::from_str("Dx0 0 x86").unwrap(), DecodeFact { rva: 0, bytes: 0, arch: Arch::X86_32 });
	assert_eq!(DecodeFact::from_str("Dxffffffff 4294967295 x86_64").unwrap(), DecodeFact {
		rva: u32::MAX, bytes: u32::MAX, arch: Arch::X86_64,
	});
}

#[test]
fn reject_invalid_decode() {
	for (line, error) in [
		("", ParseLineError::MissingField),
		("x1 5 x86", ParseLineError::InvalidPrefix("D")),
		("D0x1 5 x86", ParseLineError::InvalidRva),
		("Dx 5 x86", ParseLineError::InvalidRva),
		("Dx100000000 5 x86", ParseLineError::InvalidRva),
		("Dx1", ParseLineError::MissingField),
		("Dx1 5", ParseLineError::MissingField),
		("Dx1 -1 x86", ParseLineError::InvalidByteCount),
		("Dx1 1.5 x86", ParseLineError::InvalidByteCount),
		("Dx1 4294967296 x86", ParseLineError::InvalidByteCount),
		("Dx1 5 x86 extra", ParseLineError::UnexpectedText("arch")),
	] {
		assert_eq!(DecodeFact::from_str(line).unwrap_err(), error, "{line}");
	}
	let error = DecodeFact::from_str("Dx1 5 arm64").unwrap_err();
	assert_eq!(error, ParseLineError::InvalidArch(ArchParseError));
	let source = std::error::Error::source(&error).unwrap();
	assert!(source.is::<ArchParseError>());
}
