//! Shared fact map and type definitions for binary analysis tools.

use std::collections::HashSet;
use std::io::{self, Write};
use std::{error, fmt, num, str};

#[cfg(test)]
use std::str::FromStr;

mod arch;
pub mod factmap;
pub mod ty;

pub use self::arch::*;

fn parse_u32(s: &str) -> Result<u32, num::ParseIntError> {
	let (s, base) = match s.strip_prefix("0x") {
		Some(s) => (s, 16),
		None => (s, 10),
	};
	u32::from_str_radix(s, base)
}
