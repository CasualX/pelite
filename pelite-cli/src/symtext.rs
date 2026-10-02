//! Parser and writer for the line-oriented `#symtext` symbol format.

use super::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SymbolDatabase {
	pub entries: Vec<Symbol>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Symbol {
	pub rva: u32,
	pub ty: ty::Type,
	pub name: SymbolName,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SymbolName {
	/// Data.
	Data,
	/// Code.
	Code,
	/// Function.
	Fn,
	/// Function thunk.
	Thunk,
	/// Remove an earlier symbol at the same RVA.
	Undef,
	/// Named symbol.
	Named(String),
}

impl fmt::Display for SymbolName {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			SymbolName::Data => f.write_str("D"),
			SymbolName::Code => f.write_str("C"),
			SymbolName::Fn => f.write_str("fn"),
			SymbolName::Thunk => f.write_str("thunk"),
			SymbolName::Undef => f.write_str("undef"),
			SymbolName::Named(name) => f.write_str(&serde_json::to_string(name).map_err(|_| fmt::Error)?),
		}
	}
}

impl Symbol {
	pub fn new(rva: u32, ty: ty::Type, name: SymbolName) -> Self {
		Self { rva, ty, name }
	}
}

impl SymbolDatabase {
	pub fn parse(input: &str, pointer_width: ty::PointerWidth) -> result::Result<Self, String> {
		let mut entries = Vec::new();
		let mut lines = input.lines();
		if lines.next() != Some("#symtext") {
			return Err(format!("missing #symtext header"));
		}
		let mut index = 1;
		for line in lines {
			index += 1;
			let line = line.trim_ascii();
			if line.is_empty() || line.starts_with('#') {
				continue;
			}
			let symbol = parse_symbol(line, pointer_width).map_err(|error| format!("line {}: {error}", index))?;
			entries.push(symbol);
		}
		Ok(Self { entries })
	}

	pub fn write(&self, mut output: impl Write, comment: &str) -> io::Result<()> {
		writeln!(output, "#symtext")?;
		for line in comment.lines() {
			writeln!(output, "# {line}")?;
		}
		for symbol in &self.entries {
			write!(output, "0x{:x} ", symbol.rva)?;
			let type_text = symbol.ty.to_string();
			if type_text.chars().any(char::is_whitespace) {
				write!(output, "{}", serde_json::to_string(&type_text).expect("string serialization cannot fail"))?;
			}
			else {
				write!(output, "{type_text}")?;
			}
			writeln!(output, " {}", symbol.name)?;
		}
		Ok(())
	}
}

fn parse_symbol(line: &str, pointer_width: ty::PointerWidth) -> result::Result<Symbol, String> {
	let (rva, rest) = token(line)?;
	let hex = rva.strip_prefix("0x").filter(|digits| !digits.is_empty())
		.ok_or_else(|| "RVA must be 0x-prefixed hexadecimal".to_owned())?;
	let rva = u32::from_str_radix(hex, 16).map_err(|_| "RVA must fit in 32 bits".to_owned())?;
	let (type_token, rest) = token(rest)?;
	let type_text = if type_token.starts_with('"') {
		serde_json::from_str::<String>(type_token).map_err(|error| format!("invalid quoted type: {error}"))?
	}
	else {
		type_token.to_owned()
	};
	let (name_token, rest) = token(rest)?;
	if !rest.trim().is_empty() {
		return Err("unexpected text after symbol name".to_owned());
	}
	let name = match name_token {
		"C" => SymbolName::Code,
		"D" => SymbolName::Data,
		"fn" => SymbolName::Fn,
		"thunk" => SymbolName::Thunk,
		"undef" => SymbolName::Undef,
		name if name.starts_with('"') => SymbolName::Named(
			serde_json::from_str(name).map_err(|error| format!("invalid quoted name: {error}"))?
		),
		_ => return Err("name must be D, C, fn, thunk, undef, or a quoted string".to_owned()),
	};
	let ty = ty::Type::parse(&type_text, pointer_width).map_err(|error| format!("invalid type: {error}"))?;
	Ok(Symbol::new(rva, ty, name))
}

fn token(input: &str) -> result::Result<(&str, &str), String> {
	let input = input.trim_start();
	if input.is_empty() {
		return Err("expected another field".to_owned());
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
		return Err("unterminated quoted field".to_owned());
	}
	let end = input.find(char::is_whitespace).unwrap_or(input.len());
	Ok((&input[..end], &input[end..]))
}

#[cfg(test)]
mod tests;
