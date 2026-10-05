use super::*;

mod comment;
mod fact;
mod map;
mod reference;
mod sym;

pub use self::comment::*;
pub use self::fact::*;
pub use self::map::*;
pub use self::reference::*;
pub use self::sym::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseError {
	pub line: usize,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseLineError {
	MissingHeader,
	UnknownFact,
	InvalidPrefix(&'static str),
	MissingField,
	UnterminatedQuotedField,
	InvalidRva,
	InvalidTargetRva,
	InvalidComment(String),
	InvalidQuotedType(String),
	InvalidQuotedName(String),
	InvalidType(ty::ParseError),
	InvalidSymbolName,
	UnexpectedText(&'static str),
}

impl fmt::Display for ParseLineError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ParseLineError::MissingHeader => f.write_str("missing #factmap header"),
			ParseLineError::UnknownFact => f.write_str("expected a symbol, comment, or reference fact"),
			ParseLineError::InvalidPrefix(prefix) => write!(f, "expected {prefix} fact prefix"),
			ParseLineError::MissingField => f.write_str("expected another field"),
			ParseLineError::UnterminatedQuotedField => f.write_str("unterminated quoted field"),
			ParseLineError::InvalidRva => f.write_str("RVA must be x-prefixed hex fitting in 32 bits"),
			ParseLineError::InvalidTargetRva => f.write_str("target RVA must be 0xhex fitting in 32 bits"),
			ParseLineError::InvalidComment(error) => write!(f, "invalid quoted comment: {error}"),
			ParseLineError::InvalidQuotedType(error) => write!(f, "invalid quoted type: {error}"),
			ParseLineError::InvalidQuotedName(error) => write!(f, "invalid quoted name: {error}"),
			ParseLineError::InvalidType(error) => write!(f, "invalid type: {error}"),
			ParseLineError::InvalidSymbolName => f.write_str("name must be D, C, _, fn, thunk, undef, or a quoted string"),
			ParseLineError::UnexpectedText(field) => write!(f, "unexpected text after {field}"),
		}
	}
}

impl error::Error for ParseLineError {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		match self {
			Self::InvalidType(error) => Some(error),
			_ => None,
		}
	}
}

fn parse_rva(input: &str) -> result::Result<u32, ParseLineError> {
	let hex = input.strip_prefix('x').ok_or(ParseLineError::InvalidRva)?;
	if hex.is_empty() || !hex.bytes().all(|ch| ch.is_ascii_hexdigit()) {
		return Err(ParseLineError::InvalidRva);
	}
	u32::from_str_radix(hex, 16).map_err(|_| ParseLineError::InvalidRva)
}

fn token(input: &str) -> result::Result<(&str, &str), ParseLineError> {
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
