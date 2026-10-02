#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Arch {
	X86_16,
	X86_32,
	X86_64,
}

impl Arch {
	pub fn parse(value: &str) -> Result<Self, String> {
		match value {
			"x86_16" => Ok(Self::X86_16),
			"x86_32" | "x86" => Ok(Self::X86_32),
			"x86_64" => Ok(Self::X86_64),
			_ => Err("expected x86_16, x86_32 (alias x86), or x86_64".to_owned()),
		}
	}

	pub fn bitness(self) -> u32 {
		match self {
			Self::X86_16 => 16,
			Self::X86_32 => 32,
			Self::X86_64 => 64,
		}
	}
}
