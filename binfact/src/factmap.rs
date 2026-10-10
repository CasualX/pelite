//! Facts about image addresses and their line-oriented `#factmap` text format.
//!
//! RVAs are byte offsets relative to the image base. Fact records preserve their
//! order; consumers decide how repeated facts at the same address are combined.

use super::*;

mod comment;
mod decode;
mod fact;
mod function;
mod map;
mod reference;
mod sym;

pub use self::comment::*;
pub use self::decode::*;
pub use self::fact::*;
pub use self::function::*;
pub use self::map::*;
pub use self::reference::*;
pub use self::sym::*;

/// A fact map parse failure with its source line number.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseError {
	/// One-based line number, including the header, blank lines, and comments.
	pub line: usize,
	/// The reason this line could not be parsed.
	pub error: ParseLineError,
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "line {}: {}", self.line, self.error)
	}
}

impl error::Error for ParseError {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		Some(&self.error)
	}
}

/// Reasons a fact record cannot be parsed.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ParseLineError {
	/// The first line is neither `#factmap` nor the legacy `#symtext` header.
	MissingHeader,
	/// The record starts with an unsupported fact prefix.
	UnknownFact(char),
	/// The record lacks the expected prefix given in the payload.
	InvalidPrefix(&'static str),
	/// A required field is absent.
	MissingField,
	/// A quoted field has no closing quotation mark.
	UnterminatedQuotedField,
	/// An RVA is not `x` followed by hexadecimal digits fitting in `u32`.
	InvalidRva,
	/// A reference target is not `0x` followed by hexadecimal digits fitting in `u32`.
	InvalidTargetRva,
	/// A decode byte count is not a decimal integer fitting in `u32`.
	InvalidByteCount,
	/// A decode architecture name is unsupported.
	InvalidArch(ArchParseError),
	/// A comment field is not a valid JSON string.
	InvalidComment(String),
	/// A quoted type field is not a valid JSON string.
	InvalidQuotedType(String),
	/// A quoted symbol name is not a valid JSON string.
	InvalidQuotedName(String),
	/// A symbol's type has invalid syntax or layout.
	InvalidType(ty::ParseError),
	/// A symbol name is neither a supported marker nor a quoted string.
	InvalidSymbolName,
	/// Unexpected text follows the field named in the payload.
	UnexpectedText(&'static str),
}

impl fmt::Display for ParseLineError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ParseLineError::MissingHeader => f.write_str("missing #factmap header"),
			ParseLineError::UnknownFact(letter) => write!(f, "unknown fact {letter:?}"),
			ParseLineError::InvalidPrefix(prefix) => write!(f, "expected {prefix} fact prefix"),
			ParseLineError::MissingField => f.write_str("expected another field"),
			ParseLineError::UnterminatedQuotedField => f.write_str("unterminated quoted field"),
			ParseLineError::InvalidRva => f.write_str("RVA must be x-prefixed hex fitting in 32 bits"),
			ParseLineError::InvalidTargetRva => f.write_str("target RVA must be 0xhex fitting in 32 bits"),
			ParseLineError::InvalidByteCount => f.write_str("byte count must be a decimal integer fitting in 32 bits"),
			ParseLineError::InvalidArch(error) => write!(f, "invalid architecture: {error}"),
			ParseLineError::InvalidComment(error) => write!(f, "invalid quoted comment: {error}"),
			ParseLineError::InvalidQuotedType(error) => write!(f, "invalid quoted type: {error}"),
			ParseLineError::InvalidQuotedName(error) => write!(f, "invalid quoted name: {error}"),
			ParseLineError::InvalidType(error) => write!(f, "invalid type: {error}"),
			ParseLineError::InvalidSymbolName => f.write_str("name must be D, R, C, _, fn, thunk, undef, or a quoted string"),
			ParseLineError::UnexpectedText(field) => write!(f, "unexpected text after {field}"),
		}
	}
}

impl error::Error for ParseLineError {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		match self {
			Self::InvalidType(error) => Some(error),
			Self::InvalidArch(error) => Some(error),
			_ => None,
		}
	}
}

fn parse_rva(input: &str) -> Result<u32, ParseLineError> {
	let hex = input.strip_prefix('x').ok_or(ParseLineError::InvalidRva)?;
	if hex.is_empty() || !hex.bytes().all(|ch| ch.is_ascii_hexdigit()) {
		return Err(ParseLineError::InvalidRva);
	}
	u32::from_str_radix(hex, 16).map_err(|_| ParseLineError::InvalidRva)
}

fn token(input: &str) -> Result<(&str, &str), ParseLineError> {
	let input = input.trim_ascii_start();
	if input.is_empty() {
		return Err(ParseLineError::MissingField);
	}
	if input.starts_with('"') {
		let mut escaped = false;
		for (index, ch) in input.char_indices().skip(1) {
			if escaped {
				escaped = false;
			}
			else if ch == '\\' {
				escaped = true;
			}
			else if ch == '"' {
				let end = index + ch.len_utf8();
				return Ok((&input[..end], &input[end..]));
			}
		}
		return Err(ParseLineError::UnterminatedQuotedField);
	}
	let end = input.find(char::is_whitespace).unwrap_or(input.len());
	Ok((&input[..end], &input[end..]))
}
