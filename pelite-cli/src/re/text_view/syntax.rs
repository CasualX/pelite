//! Semantic tokens captured while emitting row text; no reparsing of display strings.
use std::io::{self, Write};

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Class {
	Keyword,
	Type,
	Symbol,
	Field,
	Mnemonic,
	Register,
	Number,
	Address,
	String,
	Comment,
	Directive,
	Punctuation,
	Unknown,
	Error,
}

/// Half-open UTF-8 byte range into a row's text, excluding its listing prefix.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct Range {
	pub start: usize,
	pub end: usize,
	pub class: Class,
}

#[derive(Clone, Debug, Default)]
pub struct Text {
	pub text: String,
	pub syntax: Vec<Range>,
}

impl Text {
	pub fn token(text: impl AsRef<str>, class: Class) -> Self {
		Self::default().then(text, class)
	}

	pub fn then(mut self, text: impl AsRef<str>, class: Class) -> Self {
		self.push(text.as_ref(), Some(class));
		self
	}

	pub fn plain(mut self, text: &str) -> Self {
		self.push(text, None);
		self
	}

	fn push(&mut self, text: &str, class: Option<Class>) {
		let start = self.text.len();
		self.text.push_str(text);
		if let Some(class) = class.filter(|_| !text.is_empty()) {
			if let Some(previous) = self.syntax.last_mut() && previous.end == start && previous.class == class {
				previous.end = self.text.len();
			}
			else {
				self.syntax.push(Range { start, end: self.text.len(), class });
			}
		}
	}

	pub fn declaration(path: &str, ty: &str, value: &str, class: Class) -> Self {
		let mut text = Self::token(path, if path.starts_with(['.', '[']) { Class::Field } else { Class::Symbol })
			.then(":", Class::Punctuation).plain(" ").then(ty, Class::Type)
			.plain(" ").then("=", Class::Punctuation).plain(" ");
		if let Some(address) = value.strip_prefix('&') {
			text = text.then("&", Class::Punctuation).then(address, Class::Address);
		}
		else {
			text = text.then(value, class);
		}
		text
	}

	pub fn directive(name: &str, count: u32) -> Self {
		Self::token(name, Class::Directive).plain(" ").then(count.to_string(), Class::Number)
	}
}

impl iced_x86::FormatterOutput for Text {
	fn write(&mut self, text: &str, kind: iced_x86::FormatterTextKind) {
		use iced_x86::FormatterTextKind as Kind;
		let class = match kind {
			Kind::Mnemonic => Some(Class::Mnemonic),
			Kind::Directive => Some(Class::Directive),
			Kind::Prefix | Kind::Keyword => Some(Class::Keyword),
			Kind::Register => Some(Class::Register),
			Kind::Number | Kind::SelectorValue => Some(Class::Number),
			Kind::LabelAddress | Kind::FunctionAddress => Some(Class::Address),
			Kind::Data | Kind::Label | Kind::Function => Some(Class::Symbol),
			Kind::Punctuation | Kind::Operator => Some(Class::Punctuation),
			_ => None,
		};
		self.push(text, class);
	}

	fn write_number(
		&mut self, instruction: &iced_x86::Instruction, _operand: u32, instruction_operand: Option<u32>, text: &str,
		_value: u64, _number_kind: iced_x86::NumberKind, kind: iced_x86::FormatterTextKind,
	) {
		let absolute_memory = instruction_operand.is_some_and(|operand| instruction.op_kind(operand) == iced_x86::OpKind::Memory)
			&& (instruction.is_ip_rel_memory_operand()
				|| (instruction.memory_base() == iced_x86::Register::None && instruction.memory_index() == iced_x86::Register::None));
		let kind = if kind == iced_x86::FormatterTextKind::Number && absolute_memory { iced_x86::FormatterTextKind::LabelAddress } else { kind };
		self.write(text, kind);
	}
}

impl Class {
	fn ansi(self) -> &'static str {
		match self {
			Self::Keyword | Self::Mnemonic | Self::Directive => "\x1b[1;97m",
			Self::Type => "\x1b[38;2;78;201;176m",
			Self::Register | Self::Field => "\x1b[38;2;134;186;184m",
			Self::Symbol | Self::Address => "\x1b[38;2;200;174;130m",
			Self::Number => "\x1b[38;2;185;190;198m",
			Self::String => "\x1b[38;2;206;145;120m",
			Self::Comment | Self::Unknown => "\x1b[90m",
			Self::Error => "\x1b[31m",
			Self::Punctuation => "",
		}
	}
}

/// Apply a palette without inserting escapes into the row or shifting its ranges.
pub fn write(output: &mut impl Write, text: &str, ranges: &[Range], color: bool) -> io::Result<()> {
	if !color { return output.write_all(text.as_bytes()); }
	let mut position = 0;
	for range in ranges {
		output.write_all(text[position..range.start].as_bytes())?;
		let ansi = range.class.ansi();
		output.write_all(ansi.as_bytes())?;
		output.write_all(text[range.start..range.end].as_bytes())?;
		if !ansi.is_empty() { output.write_all(b"\x1b[0m")?; }
		position = range.end;
	}
	output.write_all(text[position..].as_bytes())
}
