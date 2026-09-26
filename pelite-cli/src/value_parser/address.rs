use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Address {
	Rva(u32),
	Va(u64),
	Fo(usize),
}

impl Address {
	pub fn parse(value: &str) -> result::Result<Self, String> {
		let (kind, digits) = value.split_once(':').ok_or("expected rva:HEX, va:HEX, or fo:HEX")?;
		Self::parse_parts(kind, digits)
	}

	pub(super) fn parse_parts(kind: &str, digits: &str) -> result::Result<Self, String> {
		let digits = digits.strip_prefix("0x").or_else(|| digits.strip_prefix("0X")).unwrap_or(digits);
		let digits = digits.strip_suffix("h").unwrap_or(digits);
		if digits.is_empty() {
			return Err("missing hexadecimal address".to_owned());
		}
		match kind {
			"rva" => u32::from_str_radix(digits, 16).map(Self::Rva).map_err(|_| "RVA must be a 32-bit hexadecimal value".to_owned()),
			"va" => u64::from_str_radix(digits, 16).map(Self::Va).map_err(|_| "VA must be a 64-bit hexadecimal value".to_owned()),
			"fo" => usize::from_str_radix(digits, 16).map(Self::Fo).map_err(|_| "file offset must be a hexadecimal value".to_owned()),
			_ => Err("expected rva:HEX, va:HEX, or fo:HEX".to_owned()),
		}
	}
}

#[test]
fn parses_all_address_kinds() {
	assert_eq!(Address::parse("rva:1000"), Ok(Address::Rva(0x1000)));
	assert_eq!(Address::parse("va:0X180001000"), Ok(Address::Va(0x180001000)));
	assert_eq!(Address::parse("fo:0x400"), Ok(Address::Fo(0x400)));
}

#[test]
fn rejects_invalid_addresses() {
	assert!(Address::parse("1000").is_err());
	assert!(Address::parse("rva:").is_err());
	assert!(Address::parse("rva:100000000").is_err());
	assert!(Address::parse("va:xyz").is_err());
	assert!(Address::parse("offset:400").is_err());
}
