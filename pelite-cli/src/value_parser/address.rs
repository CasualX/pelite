use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Address {
	Rva(u32),
	Va(u64),
	Fo(usize),
}

impl Address {
	pub fn parse(value: &str) -> result::Result<Self, String> {
		let (kind, digits) = value.split_once(':').ok_or("expected rva:NUMBER, va:NUMBER, or fo:NUMBER")?;
		Self::parse_parts(kind, digits)
	}

	pub(super) fn parse_parts(kind: &str, digits: &str) -> result::Result<Self, String> {
		let (digits, radix) = match digits.strip_prefix("0x").or_else(|| digits.strip_prefix("0X")) {
			Some(hex) => (hex, 16),
			None => (digits, 10),
		};
		if digits.is_empty() {
			return Err("missing address value".to_owned());
		}
		match kind {
			"rva" => u32::from_str_radix(digits, radix).map(Self::Rva).map_err(|_| "RVA must be a 32-bit decimal or 0x-prefixed hexadecimal value".to_owned()),
			"va" => u64::from_str_radix(digits, radix).map(Self::Va).map_err(|_| "VA must be a 64-bit decimal or 0x-prefixed hexadecimal value".to_owned()),
			"fo" => usize::from_str_radix(digits, radix).map(Self::Fo).map_err(|_| "file offset must be a decimal or 0x-prefixed hexadecimal value".to_owned()),
			_ => Err("expected rva:NUMBER, va:NUMBER, or fo:NUMBER".to_owned()),
		}
	}
}

#[test]
fn parses_all_address_kinds() {
	assert_eq!(Address::parse("rva:1000"), Ok(Address::Rva(1000)));
	assert_eq!(Address::parse("rva:0x1000"), Ok(Address::Rva(4096)));
	assert_eq!(Address::parse("va:6442455040"), Ok(Address::Va(0x180001000)));
	assert_eq!(Address::parse("va:0X180001000"), Ok(Address::Va(0x180001000)));
	assert_eq!(Address::parse("fo:1024"), Ok(Address::Fo(1024)));
	assert_eq!(Address::parse("fo:0x400"), Ok(Address::Fo(0x400)));
}

#[test]
fn rejects_invalid_addresses() {
	assert!(Address::parse("1000").is_err());
	assert!(Address::parse("rva:").is_err());
	assert!(Address::parse("rva:0x").is_err());
	assert!(Address::parse("rva:1000h").is_err());
	assert!(Address::parse("rva:ff").is_err());
	assert!(Address::parse("rva:4294967296").is_err());
	assert!(Address::parse("rva:0x100000000").is_err());
	assert!(Address::parse("va:xyz").is_err());
	assert!(Address::parse("va:18446744073709551616").is_err());
	assert!(Address::parse("offset:400").is_err());
}
