use super::*;

/// Decodable region with a specified arch.
///
/// ```text
/// Dx1000 x86_64 255
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

	/// Parse `DxRVA arch bytes`, with a hexadecimal RVA and decimal byte count.
	fn from_str(line: &str) -> Result<Self, Self::Err> {
		let (rva, rest) = token(line)?;
		let rva = rva.strip_prefix('D').ok_or(ParseLineError::InvalidPrefix("D"))?;
		let rva = parse_rva(rva)?;
		let (first, rest) = token(rest)?;
		let (second, rest) = token(rest)?;
		let (arch, bytes, last_field) = match first.parse::<Arch>() {
			Ok(arch) => (arch, second, "bytes"),
			Err(_) => (second.parse().map_err(ParseLineError::InvalidArch)?, first, "arch"),
		};
		let bytes = parse_u32(bytes).map_err(|_| ParseLineError::InvalidByteCount)?;
		if !rest.trim_ascii_end().is_empty() {
			return Err(ParseLineError::UnexpectedText(last_field));
		}
		Ok(DecodeFact { rva, bytes, arch })
	}
}

impl fmt::Display for DecodeFact {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Dx{:x} {} {}", self.rva, self.arch, self.bytes)
	}
}

#[test]
fn parse_decode() {
	for arch in [Arch::X86_16, Arch::X86_32, Arch::X86_64] {
		let fact = DecodeFact::from_str(&format!(" DxABCD\t{arch} 15 ")).unwrap();
		assert_eq!(fact, DecodeFact { rva: 0xabcd, bytes: 15, arch });
		assert_eq!(fact.to_string(), format!("Dxabcd {arch} 15"));
		assert_eq!(DecodeFact::from_str(&fact.to_string()).unwrap(), fact);
		assert_eq!(DecodeFact::from_str(&format!(" DxABCD\t15 {arch} ")).unwrap(), fact);
		assert_eq!(DecodeFact::from_str(&format!("DxABCD {arch} 0xf")).unwrap(), fact);
		assert_eq!(DecodeFact::from_str(&format!("DxABCD 0xf {arch}")).unwrap(), fact);
	}
	for line in ["Dx0 x86 0", "Dx0 0 x86"] {
		assert_eq!(DecodeFact::from_str(line).unwrap(), DecodeFact { rva: 0, bytes: 0, arch: Arch::X86_32 });
	}
	for line in ["Dxffffffff x86_64 4294967295", "Dxffffffff 4294967295 x86_64"] {
		assert_eq!(DecodeFact::from_str(line).unwrap(), DecodeFact {
			rva: u32::MAX, bytes: u32::MAX, arch: Arch::X86_64,
		});
	}
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
		("Dx1 x86", ParseLineError::MissingField),
		("Dx1 5", ParseLineError::MissingField),
		("Dx1 x86 -1", ParseLineError::InvalidByteCount),
		("Dx1 x86 1.5", ParseLineError::InvalidByteCount),
		("Dx1 x86 4294967296", ParseLineError::InvalidByteCount),
		("Dx1 -1 x86", ParseLineError::InvalidByteCount),
		("Dx1 1.5 x86", ParseLineError::InvalidByteCount),
		("Dx1 4294967296 x86", ParseLineError::InvalidByteCount),
		("Dx1 x86 5 extra", ParseLineError::UnexpectedText("bytes")),
		("Dx1 5 x86 extra", ParseLineError::UnexpectedText("arch")),
	] {
		assert_eq!(DecodeFact::from_str(line).unwrap_err(), error, "{line}");
	}
	for line in ["Dx1 arm64 5", "Dx1 5 arm64"] {
		let error = DecodeFact::from_str(line).unwrap_err();
		assert_eq!(error, ParseLineError::InvalidArch(ArchParseError));
		let source = std::error::Error::source(&error).unwrap();
		assert!(source.is::<ArchParseError>());
	}
}
