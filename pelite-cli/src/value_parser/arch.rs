#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Arch {
	X86,
	X86_64,
}

impl Arch {
	pub fn parse(value: &str) -> Result<Self, String> {
		match value {
			"x86" => Ok(Self::X86),
			"x86_64" => Ok(Self::X86_64),
			_ => Err("expected x86 or x86_64".to_owned()),
		}
	}

	pub fn bitness(self) -> u32 {
		match self {
			Self::X86 => 32,
			Self::X86_64 => 64,
		}
	}
}
