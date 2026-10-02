#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Address {
	Rva(u32),
	Va(u64),
	Fo(usize),
}

impl Address {
	pub fn parse(value: &str) -> Result<Self, String> {
		let (kind, expression) = value.split_once(':').ok_or("expected rva:EXPR, va:EXPR, or fo:EXPR")?;
		Self::parse_parts(kind, expression)
	}

	fn parse_parts(kind: &str, expression: &str) -> Result<Self, String> {
		let value = Expr::eval(expression)?;
		match kind {
			"rva" => u32::try_from(value).map(Self::Rva).map_err(|_| "RVA must fit in 32 bits".to_owned()),
			"va" => Ok(Self::Va(value)),
			"fo" => usize::try_from(value).map(Self::Fo).map_err(|_| "file offset must fit in usize".to_owned()),
			_ => Err("expected rva:EXPR, va:EXPR, or fo:EXPR".to_owned()),
		}
	}
}

#[test]
fn parses_all_address_kinds() {
	assert_eq!(Address::parse("rva:1000"), Ok(Address::Rva(1000)));
	assert_eq!(Address::parse("rva:0x1000"), Ok(Address::Rva(4096)));
	assert_eq!(Address::parse("va:6442455040"), Ok(Address::Va(0x180001000)));
	assert_eq!(Address::parse("va:0x180001000"), Ok(Address::Va(0x180001000)));
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
	assert!(Address::parse("rva:0xffffffff+1").is_err());
	assert!(Address::parse("va:xyz").is_err());
	assert!(Address::parse("va:0X180001000").is_err());
	assert!(Address::parse("va:18446744073709551616").is_err());
	assert!(Address::parse("offset:400").is_err());
}

#[test]
fn parses_address_arithmetic() {
	for (value, expected) in [
		("rva:0x1000+0x100+32*8", Address::Rva(0x1200)),
		("va:0x1000+0x20*8-4*4", Address::Va(0x10f0)),
		("fo:2*3*4+10-2-3", Address::Fo(29)),
		("rva:4294967296-1", Address::Rva(u32::MAX)),
		("rva:0xffffffffffffffff-0xffffffffffffffff", Address::Rva(0)),
		("va:18446744073709551615", Address::Va(u64::MAX)),
		("va:0*18446744073709551615", Address::Va(0)),
	] {
		assert_eq!(Address::parse(value), Ok(expected), "{value}");
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AddressRange {
	pub start: Address,
	pub end: Address,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RvaRange {
	pub start: u32,
	pub end: u32,
}

impl AddressRange {
	pub fn parse(value: &str) -> Result<Self, String> {
		let (kind, range) = value.split_once(':').ok_or("expected rva:START..END, va:START..END, or fo:START..END")?;
		let (start, end) = range.split_once("..").ok_or("expected a range in the form KIND:START..END")?;
		if end.contains("..") {
			return Err("expected exactly one '..' range separator".to_owned());
		}
		let start = Address::parse_parts(kind, start.trim()).map_err(|error| format!("invalid range start: {error}"))?;
		let end = end.trim();
		let end = if let Some(offset) = end.strip_prefix('+') {
			let offset = offset.trim();
			if offset.starts_with('+') {
				return Err("invalid range end: expected a single '+' prefix".to_owned());
			}
			let offset = Address::parse_parts(kind, offset).map_err(|error| format!("invalid range end: {error}"))?;
			match (start, offset) {
				(Address::Rva(start), Address::Rva(offset)) => start.checked_add(offset).map(Address::Rva),
				(Address::Va(start), Address::Va(offset)) => start.checked_add(offset).map(Address::Va),
				(Address::Fo(start), Address::Fo(offset)) => start.checked_add(offset).map(Address::Fo),
				_ => unreachable!("range endpoints have the same address kind"),
			}.ok_or_else(|| "range end overflows".to_owned())?
		}
		else {
			Address::parse_parts(kind, end).map_err(|error| format!("invalid range end: {error}"))?
		};
		let ordered = match (start, end) {
			(Address::Rva(start), Address::Rva(end)) => start <= end,
			(Address::Va(start), Address::Va(end)) => start <= end,
			(Address::Fo(start), Address::Fo(end)) => start <= end,
			_ => false,
		};
		if !ordered {
			return Err("range start must be less than or equal to range end".to_owned());
		}
		Ok(Self { start, end })
	}
}

#[test]
fn parses_address_ranges() {
	for (value, start, end) in [
		("rva:1000..1100", Address::Rva(1000), Address::Rva(1100)),
		("rva:0x1000..0x1100", Address::Rva(0x1000), Address::Rva(0x1100)),
		("rva:4096..0x1100", Address::Rva(4096), Address::Rva(0x1100)),
		("rva:0x1000..+10", Address::Rva(0x1000), Address::Rva(0x100a)),
		("rva:0x1000..+0x10", Address::Rva(0x1000), Address::Rva(0x1010)),
		("va:6442455040..6442455296", Address::Va(0x180001000), Address::Va(0x180001100)),
		("va:0x180001000..+0x20", Address::Va(0x180001000), Address::Va(0x180001020)),
		("fo: 0x400 .. 0x500 ", Address::Fo(0x400), Address::Fo(0x500)),
		("fo: 0x400 .. + 10 ", Address::Fo(0x400), Address::Fo(0x40a)),
		("rva:0x1000+0x100+32*8..0x1000+0x400-4*4", Address::Rva(0x1200), Address::Rva(0x13f0)),
		("va:0x1000+0x20*8-4*4..+2*8+16-4", Address::Va(0x10f0), Address::Va(0x110c)),
		("fo:2*3..+4*5", Address::Fo(6), Address::Fo(26)),
		("rva:4294967296-1..+0", Address::Rva(u32::MAX), Address::Rva(u32::MAX)),
		("rva:1000..1000", Address::Rva(1000), Address::Rva(1000)),
		("va:0xffffffffffffffff..+0", Address::Va(u64::MAX), Address::Va(u64::MAX)),
		("fo:0..0", Address::Fo(0), Address::Fo(0)),
		("fo:400..400", Address::Fo(400), Address::Fo(400)),
		("rva:0x1000..+0", Address::Rva(0x1000), Address::Rva(0x1000)),
	] {
		assert_eq!(AddressRange::parse(value), Ok(AddressRange { start, end }));
	}
}

#[test]
fn rejects_invalid_address_ranges() {
	for value in [
		"1000..1100", "rva:1000", "va:2000..1000",
		"rva:xyz..1000", "rva:..1000", "rva:1000..", "rva:1000..1100..1200",
		"rva:1000..rva:1100", "rva:1000..va:1100", "offset:400..500",
		"rva:1000h..1100", "rva:1000..1100h", "rva:ff..1100",
		"rva:0..4294967296", "rva:0..0x100000000", "va:0..18446744073709551616",
		"rva:0x1000..+", "rva:0x1000..++10", "rva:0xfffffff0..+0x20",
		"va:0xffffffffffffffff..+1", "fo:0xffffffffffffffff..+1",
		"rva:+1..2", "rva:1+..2", "rva:1..2*", "rva:1..+-2", "rva:1..+1+-2",
		"rva:1+2..1*2", "rva:0..+1-2", "va:0..+18446744073709551616-1",
		"va:0..+18446744073709551615+1", "va:1..+18446744073709551615",
		"rva:0X1000..0x1100", "rva:0x1000..0X1100", "rva:0x1000..+0X10",
		"rva:0xffffffff+1..+0", "rva:0xfffffffe..+1*2",
	] {
		assert!(AddressRange::parse(value).is_err(), "accepted {value}");
	}
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Operator {
	Add,
	Sub,
}

/// Constant-space evaluator: acc holds completed terms, cur the current product.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(super) struct Expr {
	acc: u64,
	cur: u64,
	op: Operator,
}

impl Expr {
	/// Evaluates left to right, with multiplication taking precedence.
	pub(super) fn eval(mut s: &str) -> Result<u64, String> {
		let mut expr = Self { acc: 0, cur: literal(&mut s)?, op: Operator::Add };
		while let Some(operator) = s.as_bytes().first().copied() {
			s = &s[1..];
			match operator {
				b'*' => expr.cur = expr.cur.checked_mul(literal(&mut s)?).ok_or("expression multiplication overflows")?,
				b'+' | b'-' => {
					expr.accumulate()?;
					expr.op = if operator == b'+' { Operator::Add } else { Operator::Sub };
					expr.cur = literal(&mut s)?;
				},
				_ => unreachable!("literal stops only at an operator"),
			}
		}
		expr.accumulate()?;
		Ok(expr.acc)
	}

	fn accumulate(&mut self) -> Result<(), String> {
		self.acc = match self.op {
			Operator::Add => self.acc.checked_add(self.cur),
			Operator::Sub => self.acc.checked_sub(self.cur),
		}.ok_or("expression addition or subtraction overflows")?;
		Ok(())
	}
}

fn literal(s: &mut &str) -> Result<u64, String> {
	let end = s.find(['+', '-', '*']).unwrap_or(s.len());
	let (number, rest) = s.split_at(end);
	*s = rest;
	let (digits, radix) = match number.strip_prefix("0x") {
		Some(hex) => (hex, 16),
		None => (number, 10),
	};
	if digits.is_empty() || !digits.bytes().all(|byte| if radix == 16 { byte.is_ascii_hexdigit() } else { byte.is_ascii_digit() }) {
		return Err("expected a decimal or 0x-prefixed hexadecimal u64 literal".to_owned());
	}
	u64::from_str_radix(digits, radix).map_err(|_| "literal exceeds u64 range".to_owned())
}

#[test]
fn evaluates_expressions() {
	for (expression, expected) in [
		("0", 0),
		("0xff", 255),
		("0x1000+0x100+32*8", 0x1200),
		("0x1000+0x20*8-4*4", 0x10f0),
		("2*3*4+10-2-3", 29),
		("10-2*3+4", 8),
		("18446744073709551615-1+1", u64::MAX),
		("0*18446744073709551615*2", 0),
	] {
		assert_eq!(Expr::eval(expression), Ok(expected), "{expression}");
	}
}

#[test]
fn rejects_invalid_expressions() {
	for expression in [
		"", "0Xff", "1+0Xff", "0Xff*2",
		"+1", "-1", "*1", "1+", "1-", "1*", "1++2", "1--2", "1+-2", "1-+2", "1**2", "1*+2", "1*-2",
		"(1+2)*3", "1/2", "1%2", "1<<2", "1.0", "1_000", "1 2", "1 +2", "0x+1", "0x-1", "0xg", "１２",
		"18446744073709551616-1", "0x10000000000000000*0",
		"18446744073709551615+1", "18446744073709551615+1-1",
		"0-1", "1-2+3", "18446744073709551615*2", "2*18446744073709551615*0",
	] {
		assert!(Expr::eval(expression).is_err(), "accepted {expression}");
	}
}
