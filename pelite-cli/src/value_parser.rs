pub fn parse_u16(value: &str) -> Result<u16, String> {
	let (value, base) = match value.strip_prefix("0x") {
		Some(value) => (value, 16),
		None => (value, 10),
	};
	u16::from_str_radix(value, base).map_err(|error| error.to_string())
}
pub fn parse_u32(value: &str) -> Result<u32, String> {
	let (value, base) = match value.strip_prefix("0x") {
		Some(value) => (value, 16),
		None => (value, 10),
	};
	u32::from_str_radix(value, base).map_err(|error| error.to_string())
}
pub fn parse_u64(value: &str) -> Result<u64, String> {
	let (value, base) = match value.strip_prefix("0x") {
		Some(value) => (value, 16),
		None => (value, 10),
	};
	u64::from_str_radix(value, base).map_err(|error| error.to_string())
}
pub fn parse_usize(value: &str) -> Result<usize, String> {
	let (value, base) = match value.strip_prefix("0x") {
		Some(value) => (value, 16),
		None => (value, 10),
	};
	usize::from_str_radix(value, base).map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn numeric_parsers_accept_decimal_and_lowercase_hex_prefix_with_typed_bounds() {
		macro_rules! check {
			($parse:ident, $ty:ty) => {{
				assert_eq!($parse("42"), Ok(42));
				assert_eq!($parse("0x2A"), Ok(42));
				assert_eq!($parse("0"), Ok(0));
				assert_eq!($parse(&<$ty>::MAX.to_string()), Ok(<$ty>::MAX));
				assert_eq!($parse(&format!("{:#x}", <$ty>::MAX)), Ok(<$ty>::MAX));
				assert!($parse(&format!("{}", u128::from(<$ty>::MAX as u64) + 1)).is_err());
				assert!($parse(&format!("0x{:x}", u128::from(<$ty>::MAX as u64) + 1)).is_err());
				for value in ["", "0x", "0X2A", "0b101010", "2A", "-1", " 42", "42 ", "1+2"] {
					assert!($parse(value).is_err(), "accepted {value:?}");
				}
			}};
		}
		check!(parse_u16, u16);
		check!(parse_u32, u32);
		check!(parse_u64, u64);
		check!(parse_usize, usize);
	}
}
