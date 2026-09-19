use super::*;

#[derive(Clone, Copy, Debug)]
pub struct RvaRange {
	pub start: u32,
	pub end: u32,
}

impl RvaRange {
	pub fn parse(value: &str) -> result::Result<RvaRange, String> {
		let Some((start, end)) = value.split_once("..")
		else {
			return Err("expected a range in the form START..END".to_owned());
		};
		if end.contains("..") {
			return Err("expected exactly one '..' range separator".to_owned());
		}
		let start = parse_rva(start).map_err(|error| format!("invalid start RVA: {error}"))?;
		let end = parse_rva(end).map_err(|error| format!("invalid end RVA: {error}"))?;
		if start >= end {
			return Err("range start must be less than range end".to_owned());
		}
		Ok(RvaRange { start, end })
	}
}

fn parse_rva(value: &str) -> result::Result<u32, String> {
	let value = value.trim();
	let digits = value.strip_prefix("0x").or_else(|| value.strip_prefix("0X")).unwrap_or(value);
	if digits.is_empty() {
		return Err("missing hexadecimal value".to_owned());
	}
	u32::from_str_radix(digits, 16).map_err(|_| format!("'{value}' is not a 32-bit hexadecimal value"))
}

#[test]
fn parses_hexadecimal_rva_ranges() {
	let bare = RvaRange::parse("1000..10ff").unwrap();
	assert_eq!((bare.start, bare.end), (0x1000, 0x10ff));
	let prefixed = RvaRange::parse("0x20..0X30").unwrap();
	assert_eq!((prefixed.start, prefixed.end), (0x20, 0x30));
}

#[test]
fn rejects_invalid_rva_ranges() {
	assert!(RvaRange::parse("1000").is_err());
	assert!(RvaRange::parse("1000..1000").is_err());
	assert!(RvaRange::parse("2000..1000").is_err());
	assert!(RvaRange::parse("xyz..1000").is_err());
}
