//! Instruction architectures and their textual names.

use super::*;

/// An unrecognized instruction architecture name.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArchParseError;

impl fmt::Display for ArchParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("expected x86_16, x86_32 (alias x86), or x86_64")
	}
}

impl error::Error for ArchParseError {}

/// Instruction architecture and decoding mode, independent of the host architecture.
///
/// Display uses `x86_16`, `x86_32`, or `x86_64`. Parsing accepts these names
/// and the alias `x86` for `x86_32`, with exact casing and no surrounding whitespace.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Arch {
	/// x86 instructions decoded in 16-bit mode.
	X86_16,
	/// x86 instructions decoded in 32-bit mode.
	X86_32,
	/// x86 instructions decoded in 64-bit mode.
	X86_64,
}

impl Arch {
	/// Bitness of the instruction decoding mode: 16, 32, or 64.
	pub fn bitness(self) -> u32 {
		match self {
			Self::X86_16 => 16,
			Self::X86_32 => 32,
			Self::X86_64 => 64,
		}
	}
}

impl str::FromStr for Arch {
	type Err = ArchParseError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		match value {
			"x86_16" => Ok(Self::X86_16),
			"x86_32" | "x86" => Ok(Self::X86_32),
			"x86_64" => Ok(Self::X86_64),
			_ => Err(ArchParseError),
		}
	}
}

impl fmt::Display for Arch {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(match self {
			Self::X86_16 => "x86_16",
			Self::X86_32 => "x86_32",
			Self::X86_64 => "x86_64",
		})
	}
}

#[test]
fn architecture_round_trip() {
	for arch in [Arch::X86_16, Arch::X86_32, Arch::X86_64] {
		assert_eq!(arch.to_string().parse::<Arch>().unwrap(), arch);
	}
	assert_eq!("x86".parse::<Arch>().unwrap(), Arch::X86_32);
	for input in ["", "arm64", "X86", " x86", "x86 "] {
		assert_eq!(input.parse::<Arch>(), Err(ArchParseError));
	}
	assert_eq!(ArchParseError.to_string(), "expected x86_16, x86_32 (alias x86), or x86_64");
}
