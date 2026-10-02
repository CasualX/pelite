use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::Arc;

use super::*;
use symbols::IndexedSymbol;

const MNEMONIC_COLOR: &str = "\x1b[1;97m";
const ADDRESS_COLOR: &str = "\x1b[38;2;200;174;130m";
const REGISTER_COLOR: &str = "\x1b[38;2;134;186;184m";
const NUMBER_COLOR: &str = "\x1b[38;2;185;190;198m";

pub struct DecodedInstruction<'a> {
	pub ip: u64,
	pub bytes: &'a [u8],
	pub instruction: String,
	pub colored_instruction: Option<String>,
}

pub enum TextPrefix<'a> {
	None,
	Indent,
	Address { label: &'a str, value: u64 },
}

pub fn print_text<'a, W: Write>(
	output: &mut W, instructions: &[DecodedInstruction<'_>], symbols: &HashMap<u64, IndexedSymbol>,
	color: bool, show_hex: bool, mut prefix: impl FnMut(u64) -> TextPrefix<'a>,
) -> io::Result<()> {
	let longest_instruction_bytes = instructions.iter().map(|item| item.bytes.len()).max().unwrap_or(0);
	let hex = hex::HexPrinter::new(false);
	for item in instructions {
		if let Some(symbol) = symbols.get(&item.ip) {
			if color {
				writeln!(output, "\n\x1b[1m{ADDRESS_COLOR}{symbol}:\x1b[0m")?;
			}
			else {
				writeln!(output, "\n{symbol}:")?;
			}
		}
		match prefix(item.ip) {
			TextPrefix::None => {},
			TextPrefix::Indent => write!(output, "    ")?,
			TextPrefix::Address { label, value } if color => {
				write!(output, "\x1b[90m{label}\x1b[0m:{ADDRESS_COLOR}{value:#x}\x1b[0m  ")?;
			},
			TextPrefix::Address { label, value } => write!(output, "{label}:{value:#x}  ")?,
		}
		if show_hex {
			if color {
				write!(output, "\x1b[90m")?;
			}
			hex.write_bytes(output, item.bytes, b"")?;
			if color {
				write!(output, "\x1b[0m")?;
			}
			write!(output, "{:width$} ", "", width = (longest_instruction_bytes - item.bytes.len()) * 2)?;
		}
		write!(output, "{}", item.colored_instruction.as_deref().unwrap_or(&item.instruction))?;
		let mut first_symbol = true;
		for offset in 1..item.bytes.len() {
			let Some(address) = item.ip.checked_add(offset as u64) else { break };
			let Some(symbol) = symbols.get(&address) else { continue };
			if first_symbol {
				write!(output, " ; +{offset:#x}: {symbol}")?;
				first_symbol = false;
			}
			else {
				write!(output, ", +{offset:#x}: {symbol}")?;
			}
		}
		writeln!(output)?;
	}
	Ok(())
}

struct InstructionText {
	plain: String,
	colored: Option<String>,
}

impl iced_x86::FormatterOutput for InstructionText {
	fn write(&mut self, text: &str, kind: iced_x86::FormatterTextKind) {
		use iced_x86::FormatterTextKind as Kind;

		self.plain.push_str(text);
		let Some(colored) = &mut self.colored else {
			return;
		};
		let color = match kind {
			Kind::Mnemonic | Kind::Directive | Kind::Prefix => MNEMONIC_COLOR,
			Kind::Register => REGISTER_COLOR,
			Kind::Number | Kind::SelectorValue => NUMBER_COLOR,
			Kind::Data | Kind::Label | Kind::Function | Kind::LabelAddress | Kind::FunctionAddress => ADDRESS_COLOR,
			_ => "",
		};
		if !color.is_empty() {
			colored.push_str(color);
		}
		colored.push_str(text);
		if !color.is_empty() {
			colored.push_str("\x1b[0m");
		}
	}

	fn write_number(
		&mut self, instruction: &iced_x86::Instruction, _operand: u32, instruction_operand: Option<u32>, text: &str, _value: u64,
		_number_kind: iced_x86::NumberKind, kind: iced_x86::FormatterTextKind,
	) {
		let absolute_memory = instruction_operand.is_some_and(|operand| instruction.op_kind(operand) == iced_x86::OpKind::Memory)
			&& (instruction.is_ip_rel_memory_operand()
				|| (instruction.memory_base() == iced_x86::Register::None && instruction.memory_index() == iced_x86::Register::None));
		let kind = if kind == iced_x86::FormatterTextKind::Number && absolute_memory { iced_x86::FormatterTextKind::LabelAddress } else { kind };
		self.write(text, kind);
	}
}

struct SymbolResolver {
	symbols: Arc<HashMap<u64, IndexedSymbol>>,
}

impl iced_x86::SymbolResolver for SymbolResolver {
	fn symbol(&mut self, _instruction: &iced_x86::Instruction, _operand: u32, _instruction_operand: Option<u32>, address: u64, _address_size: u32) -> Option<iced_x86::SymbolResult<'_>> {
		self.symbols.get(&address).map(|name| match name {
			IndexedSymbol::Named(name) => iced_x86::SymbolResult::with_str(address, name),
			IndexedSymbol::Generated { .. } => iced_x86::SymbolResult::with_string(address, name.to_string()),
		})
	}
}

pub fn decode_bytes<'a>(bytes: &'a [u8], bitness: u32, decode_ip: u64, start_ip: u64, end_ip: u64, color: bool, symbols: Arc<HashMap<u64, IndexedSymbol>>) -> Vec<DecodedInstruction<'a>> {
	let mut decoder = iced_x86::Decoder::with_ip(bitness, bytes, decode_ip, iced_x86::DecoderOptions::NONE);
	let mut formatter = iced_x86::IntelFormatter::with_options(Some(Box::new(SymbolResolver { symbols })), None);
	let options = iced_x86::Formatter::options_mut(&mut formatter);
	options.set_hex_prefix("0x");
	options.set_hex_suffix("");
	options.set_uppercase_hex(false);
	let mut instructions = Vec::new();
	while decoder.can_decode() && decoder.ip() < end_ip {
		let instruction = decoder.decode();
		if instruction.next_ip() <= start_ip {
			continue;
		}
		let offset = (instruction.ip() - decode_ip) as usize;
		let instruction_bytes = &bytes[offset..offset + instruction.len()];
		let mut text = InstructionText { plain: String::new(), colored: color.then(String::new) };
		iced_x86::Formatter::format(&mut formatter, &instruction, &mut text);
		instructions.push(DecodedInstruction { ip: instruction.ip(), bytes: instruction_bytes, instruction: text.plain, colored_instruction: text.colored });
	}
	instructions
}

#[test]
fn symbols_inside_instructions_are_comments_in_address_order() {
	let bytes = [0xb8, 1, 0, 0, 0, 0xc3];
	let instructions = [
		DecodedInstruction { ip: 0x1000, bytes: &bytes[..5], instruction: "mov eax,1".into(), colored_instruction: None },
		DecodedInstruction { ip: 0x1005, bytes: &bytes[5..], instruction: "ret".into(), colored_instruction: None },
	];
	let symbols = HashMap::from([
		(0x1004, IndexedSymbol::Generated { name: "data", rva: 0x1004 }),
		(0x1005, IndexedSymbol::Named("next".into())),
		(0x1001, IndexedSymbol::Named("inside".into())),
		(0x1000, IndexedSymbol::Named("start".into())),
	]);
	let mut output = Vec::new();
	print_text(&mut output, &instructions, &symbols, false, false, |_| TextPrefix::None).unwrap();
	assert_eq!(String::from_utf8(output).unwrap(), "\nstart:\nmov eax,1 ; +0x1: inside, +0x4: data_1004\n\nnext:\nret\n");
}
