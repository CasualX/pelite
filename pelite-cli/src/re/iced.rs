use std::collections::HashMap;
use std::cell::Cell;
use std::io::{self, Write};
use std::rc::Rc;
use std::sync::Arc;
use clap::builder::TypedValueParser;

use super::*;
use symbols::IndexedSymbol;

const MNEMONIC_COLOR: &str = "\x1b[1;97m";
const ADDRESS_COLOR: &str = "\x1b[38;2;200;174;130m";
const REGISTER_COLOR: &str = "\x1b[38;2;134;186;184m";
const NUMBER_COLOR: &str = "\x1b[38;2;185;190;198m";

/// The same decoded and annotated instruction is used by both output formats.
#[derive(serde::Serialize)]
pub struct DecodedInstruction<'a> {
	pub ip: u64,
	pub prefix: &'a str,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bytes: Option<&'a [u8]>,
	pub instruction: String,
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub comments: Vec<String>,
	#[serde(skip)]
	pub colored_instruction: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AddressLayout {
	None,
	Indent,
	Rva,
	FileOffset,
	Va,
}

/// Address conversion and prefix selection are supplied by the caller.
pub enum TextLayout<'a> {
	None,
	Indent,
	Address { prefix: &'a str, offset: u64 },
}

pub fn layout_arg(raw: bool) -> clap::Arg {
	let layouts = if raw { vec!["none", "indent", "fo", "va"] } else { vec!["none", "indent", "rva", "fo", "va"] };
	clap::Arg::new("layout")
		.long("layout")
		.value_name("MODE")
		.value_parser(clap::builder::PossibleValuesParser::new(layouts).map(|value| match value.as_str() {
			"none" => AddressLayout::None,
			"indent" => AddressLayout::Indent,
			"rva" => AddressLayout::Rva,
			"fo" => AddressLayout::FileOffset,
			"va" => AddressLayout::Va,
			_ => unreachable!("validated by clap"),
		}))
		.default_value("indent")
		.help("Choose plain, indented, or address layout for text output")
}

pub struct DecodeOptions<'a> {
	pub prefix: &'a str,
	pub color: bool,
	pub facts: symbols::IndexedFacts,
	/// Decode-slice-relative fixup offsets; the width is shared by the entire image.
	pub relocations: BTreeSet<usize>,
	/// Zero means relocation metadata is unavailable (raw input).
	pub relocation_width: usize,
}

impl<'a> DecodeOptions<'a> {
	pub fn new(format: OutputFormat, prefix: &'a str, facts: symbols::IndexedFacts) -> Self {
		Self {
			prefix,
			color: format == OutputFormat::Text && io::stdout().is_terminal(),
			facts,
			relocations: BTreeSet::new(),
			relocation_width: 0,
		}
	}
}

pub fn omit_bytes(instructions: &mut [DecodedInstruction<'_>]) {
	for item in instructions {
		item.bytes = None;
	}
}

pub fn print(instructions: &[DecodedInstruction<'_>], format: OutputFormat, layout: TextLayout<'_>) -> Result {
	match format {
		OutputFormat::Nul => Ok(()),
		OutputFormat::Json | OutputFormat::JsonPretty => print_json(instructions, format == OutputFormat::JsonPretty),
		OutputFormat::Text => {
			print_text(&mut io::stdout().lock(), instructions, layout)?;
			Ok(())
		},
	}
}

pub fn print_json(instructions: &[DecodedInstruction<'_>], pretty: bool) -> Result {
	crate::printer::print_json(&instructions, pretty)
}

pub fn print_text<W: Write>(output: &mut W, instructions: &[DecodedInstruction<'_>], layout: TextLayout<'_>) -> io::Result<()> {
	let longest_instruction_bytes = instructions.iter().filter_map(|item| item.bytes.map(<[u8]>::len)).max().unwrap_or(0);
	let hex = hex::HexPrinter::new(false);
	for item in instructions {
		let color = item.colored_instruction.is_some();
		if let Some(symbol) = &item.label {
			if color {
				writeln!(output, "\n\x1b[1m{ADDRESS_COLOR}{symbol}:\x1b[0m")?;
			}
			else {
				writeln!(output, "\n{symbol}:")?;
			}
		}
		match layout {
			TextLayout::None => {},
			TextLayout::Indent => write!(output, "    ")?,
			TextLayout::Address { prefix, offset } => {
				let value = item.ip.wrapping_sub(offset);
				if color {
					write!(output, "\x1b[90m{prefix}\x1b[0m:{ADDRESS_COLOR}{value:#x}\x1b[0m  ")?;
				}
				else {
					write!(output, "{prefix}:{value:#x}  ")?;
				}
			},
		}
		if let Some(bytes) = item.bytes {
			if color {
				write!(output, "\x1b[90m")?;
			}
			hex.write_bytes(output, bytes, b"")?;
			if color {
				write!(output, "\x1b[0m")?;
			}
			write!(output, "{:width$} ", "", width = (longest_instruction_bytes - bytes.len()) * 2)?;
		}
		write!(output, "{}", item.colored_instruction.as_deref().unwrap_or(&item.instruction))?;
		if !item.comments.is_empty() {
			if color { write!(output, "\x1b[90m")?; }
			for comment in &item.comments {
				// Preserve multiline metadata in JSON, keep text on one instruction line.
				write!(output, " ; {}", comment.lines().collect::<Vec<_>>().join(" ; "))?;
			}
			if color { write!(output, "\x1b[0m")?; }
		}
		writeln!(output)?;
	}
	Ok(())
}

/// Intel syntax with shared number formatting, optional symbols, and color.
pub struct Formatter {
	inner: iced_x86::IntelFormatter,
	color: bool,
	symbol_operands: Rc<Cell<u32>>,
}

impl Formatter {
	pub fn new(color: bool, symbols: Option<Arc<HashMap<u64, IndexedSymbol>>>) -> Formatter {
		let symbol_operands = Rc::new(Cell::new(0));
		let resolver = symbols.map(|symbols| Box::new(SymbolResolver { symbols, operands: Rc::clone(&symbol_operands) }) as Box<dyn iced_x86::SymbolResolver>);
		let mut inner = iced_x86::IntelFormatter::with_options(resolver, None);
		let options = iced_x86::Formatter::options_mut(&mut inner);
		options.set_hex_prefix("0x");
		options.set_hex_suffix("");
		options.set_uppercase_hex(false);
		Formatter { inner, color, symbol_operands }
	}

	pub fn format(&mut self, instruction: &iced_x86::Instruction, constants: iced_x86::ConstantOffsets, relocations: &[usize], relocation_width: usize) -> InstructionText {
		self.symbol_operands.set(symbol_operands(instruction, constants, relocations, relocation_width));
		let mut text = InstructionText { plain: String::new(), colored: self.color.then(String::new) };
		iced_x86::Formatter::format(&mut self.inner, instruction, &mut text);
		text
	}
}

fn symbol_operands(instruction: &iced_x86::Instruction, constants: iced_x86::ConstantOffsets, relocations: &[usize], relocation_width: usize) -> u32 {
	// Raw bytes have no relocation metadata, so retain their address-based lookup.
	if relocation_width == 0 {
		return u32::MAX;
	}
	let relocated = |offset, width| width == relocation_width && relocations.contains(&offset);
	let immediate = relocated(constants.immediate_offset(), constants.immediate_size());
	let immediate2 = relocated(constants.immediate_offset2(), constants.immediate_size2());
	let displacement = relocated(constants.displacement_offset(), constants.displacement_size());
	let mut operands = 0;
	for operand in 0..instruction.op_count() {
		use iced_x86::OpKind;
		let resolve = match instruction.op_kind(operand) {
			OpKind::NearBranch16 | OpKind::NearBranch32 | OpKind::NearBranch64 => true,
			OpKind::Memory => instruction.is_ip_rel_memory_operand() || displacement,
			OpKind::Immediate8_2nd => immediate2,
			OpKind::Immediate8 | OpKind::Immediate16 | OpKind::Immediate32 | OpKind::Immediate64
				| OpKind::Immediate8to16 | OpKind::Immediate8to32 | OpKind::Immediate8to64
				| OpKind::Immediate32to64 | OpKind::FarBranch16 | OpKind::FarBranch32 => immediate,
			_ => false,
		};
		if resolve {
			operands |= 1 << operand;
		}
	}
	operands
}

#[derive(Clone, Debug)]
pub struct InstructionText {
	pub plain: String,
	pub colored: Option<String>,
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
	operands: Rc<Cell<u32>>,
}

impl iced_x86::SymbolResolver for SymbolResolver {
	fn symbol(&mut self, instruction: &iced_x86::Instruction, _operand: u32, instruction_operand: Option<u32>, address: u64, address_size: u32) -> Option<iced_x86::SymbolResult<'_>> {
		let operands = self.operands.get();
		if operands != u32::MAX {
			let operand = instruction_operand?;
			if operands & (1 << operand) == 0 {
				return None;
			}
			// A far branch's selector shares the instruction operand with its relocated offset.
			if instruction.op_kind(operand) == iced_x86::OpKind::FarBranch32 && address_size != 4 {
				return None;
			}
		}
		self.symbols.get(&address).and_then(|name| match name {
			IndexedSymbol::Named(name) => Some(iced_x86::SymbolResult::with_str(address, name)),
			IndexedSymbol::Generated { .. } => Some(iced_x86::SymbolResult::with_string(address, name.to_string())),
			IndexedSymbol::Weak(_) => None,
		})
	}
}

pub fn decode_bytes<'a>(bytes: &'a [u8], bitness: u32, decode_ip: u64, start_ip: u64, end_ip: u64, options: DecodeOptions<'a>) -> Vec<DecodedInstruction<'a>> {
	let symbols = Arc::new(options.facts.symbols);
	let mut decoder = iced_x86::Decoder::with_ip(bitness, bytes, decode_ip, iced_x86::DecoderOptions::NONE);
	let mut formatter = Formatter::new(options.color, Some(Arc::clone(&symbols)));

	let mut instructions = Vec::new();
	while decoder.can_decode() && decoder.ip() < end_ip {
		// Decode an instruction
		let offset = decoder.position();
		let instruction = decoder.decode();

		// Filter lookback instructions
		if instruction.next_ip() <= start_ip {
			continue;
		}

		// Collect relocations pointed inside the instruction
		let instruction_bytes = &bytes[offset..decoder.position()];
		let relocations = options.relocations.range(offset..decoder.position())
			.map(|&field| field - offset)
			.filter(|&field| field + options.relocation_width <= instruction_bytes.len())
			.collect::<Vec<_>>();

		// Collect comments for this IP
		let ip = instruction.ip();
		let mut comments = Vec::new();
		if let Some(comment) = options.facts.comments.get(&ip).filter(|comment| !comment.is_empty()) {
			comments.push(comment.to_string());
		}

		// Collect any mid instruction symbols and add as comments
		for offset in 1..instruction_bytes.len() {
			let address = ip + offset as u64;
			if let Some(symbol) = symbols.get(&address).and_then(symbol_name) {
				comments.push(format!("+{offset:#x}: {symbol}"));
			}
		}

		// Render the symbol name
		let label = symbols.get(&ip).and_then(symbol_name);
		// Format instruction
		let constants = decoder.get_constant_offsets(&instruction);
		let text = formatter.format(&instruction, constants, &relocations, options.relocation_width);

		instructions.push(DecodedInstruction {
			ip, prefix: options.prefix, label, bytes: Some(instruction_bytes),
			instruction: text.plain, comments, colored_instruction: text.colored,
		});
	}
	instructions
}

fn symbol_name(symbol: &IndexedSymbol) -> Option<String> {
	(!matches!(symbol, IndexedSymbol::Weak(_))).then(|| symbol.to_string())
}

/// Trace one straight-line block, including its terminating control transfer.
pub fn trace_bytes(bytes: &[u8], bitness: u32, ip: u64, trace_limit: usize) -> Result<usize> {
	let mut decoder = iced_x86::Decoder::with_ip(bitness, bytes, ip, iced_x86::DecoderOptions::NONE);
	let mut count = 0;
	while decoder.can_decode() {
		let position = decoder.position();
		let instr = decoder.decode();
		if instr.is_invalid() {
			return Err(err(format!("(bad): unable to decode instruction at {:#x} after {count} instructions, {position} bytes", instr.ip())));
		}
		count += 1;
		let is_control_flow = instr.flow_control() != iced_x86::FlowControl::Next || instr.mnemonic() == iced_x86::Mnemonic::Hlt;
		if is_control_flow || (trace_limit != 0 && count >= trace_limit) {
			return Ok(decoder.position());
		}
	}
	Err(err(format!("no control-transfer instruction before end of available data at {:#x} after {count} instructions, {} bytes", decoder.ip(), decoder.position())))
}
