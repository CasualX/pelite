use pelite::{image, Import, PeFile};

use super::*;

const MNEMONIC_COLOR: &str = "\x1b[1;97m";
const ADDRESS_COLOR: &str = "\x1b[38;2;200;174;130m";
const REGISTER_COLOR: &str = "\x1b[38;2;134;186;184m";
const NUMBER_COLOR: &str = "\x1b[38;2;185;190;198m";

#[derive(serde::Serialize)]
struct DisassembledInstruction<'a> {
	address: u32,
	bytes: &'a [u8],
	instruction: String,
	#[serde(skip)]
	colored_instruction: Option<String>,
}

pub(super) struct DecodedInstruction<'a> {
	pub ip: u64,
	pub bytes: &'a [u8],
	pub instruction: String,
	pub colored_instruction: Option<String>,
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

struct PeSymbolResolver {
	symbols: Arc<HashMap<u64, String>>,
}

impl iced_x86::SymbolResolver for PeSymbolResolver {
	fn symbol(&mut self, _instruction: &iced_x86::Instruction, _operand: u32, _instruction_operand: Option<u32>, address: u64, _address_size: u32) -> Option<iced_x86::SymbolResult<'_>> {
		self.symbols.get(&address).map(|name| iced_x86::SymbolResult::with_str(address, name))
	}
}

pub fn command() -> clap::Command {
	clap::Command::new("disasm")
		.about("Disassemble an address range using iced-x86")
		.after_help(include_str!("../docs/disasm.md"))
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("range")
			.value_name("RANGE")
			.value_parser(AddressRange::parse)
			.required(true))
		.arg(clap::Arg::new("arch")
			.long("arch")
			.value_name("ARCH")
			.value_parser(Arch::parse)
			.help("Override the PE machine header (x86 or x86_64)"))
		.arg(clap::Arg::new("hex")
			.long("hex")
			.action(clap::ArgAction::SetTrue)
			.help("Show instruction opcode bytes in text output"))
		.arg(clap::Arg::new("lookback")
			.long("lookback")
			.value_name("BYTES")
			.value_parser(clap::value_parser!(u32))
			.default_value("0")
			.help("Decode up to BYTES earlier (decimal); include any instruction overlapping the requested start. Correct alignment requires starting at an instruction boundary"))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let range = *matches.get_one::<AddressRange>("range").expect("required by clap");
	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let range = range.to_rva(pe)?;
	let bitness = if let Some(arch) = matches.get_one::<Arch>("arch") {
		arch.bitness()
	}
	else {
		match pe.file_header().Machine {
			image::IMAGE_FILE_MACHINE_I386 => 32,
			image::IMAGE_FILE_MACHINE_AMD64 => 64,
			machine => return Err(err(format!("unsupported machine type {machine:#06x}; expected i386 or AMD64"))),
		}
	};
	let len = usize::try_from(range.end - range.start)?;
	let bytes = pe.slice(range.start, len, 1)?;
	let lookback = *matches.get_one::<u32>("lookback").expect("defaulted by clap");
	let (decode_start, bytes) = pe.section_headers().by_rva(range.start)
		.filter(|_| lookback != 0)
		.and_then(|section| {
			let start = range.start.saturating_sub(lookback).max(section.VirtualAddress);
			let len = usize::try_from(range.end - start).ok()?;
			pe.slice(start, len, 1).ok().map(|bytes| (start, bytes))
		})
		.unwrap_or((range.start, bytes));
	let image_base = pe.image_base();
	let start_ip = image_base + range.start as u64;
	let decode_ip = image_base + decode_start as u64;
	let end_ip = image_base + range.end as u64;

	let color = format == OutputFormat::Text && io::stdout().is_terminal();
	let symbols = Arc::new(build_symbols(pe, image_base));
	let decoded = decode_bytes(bytes, bitness, decode_ip, start_ip, end_ip, color, Arc::clone(&symbols));
	let mut instructions = Vec::with_capacity(decoded.len());
	for item in decoded {
		instructions.push(DisassembledInstruction {
			address: u32::try_from(item.ip - image_base)?,
			bytes: item.bytes,
			instruction: item.instruction,
			colored_instruction: item.colored_instruction,
		});
	}

	match format {
		OutputFormat::Json => print_json(&instructions, false),
		OutputFormat::JsonPretty => print_json(&instructions, true),
		OutputFormat::Text => {
			let show_hex = matches.get_flag("hex");
			let longest_instruction_bytes = instructions.iter().map(|item| item.bytes.len()).max().unwrap_or(0);
			let stdout = io::stdout();
			let hex = HexPrinter::new(false);
			let mut output = stdout.lock();
			for item in instructions {
				let address = image_base + u64::from(item.address);
				if let Some(symbol) = symbols.get(&address) {
					if color {
						writeln!(output, "\n\x1b[1m{ADDRESS_COLOR}{symbol}:\x1b[0m")?;
					}
					else {
						writeln!(output, "\n{symbol}:")?;
					}
				}
				let address_width = bitness as usize / 4 + 2;
				let section_name = get_section_name_by_rva(pe, item.address);
				if color {
					write!(output, "\x1b[90m{section_name}\x1b[0m:{ADDRESS_COLOR}{address:#0address_width$x}\x1b[0m  ")?;
				}
				else {
					write!(output, "{section_name}:{address:#0address_width$x}  ")?;
				}
				if show_hex {
					if color {
						write!(output, "\x1b[90m")?;
					}
					hex.write_bytes(&mut output, item.bytes, b"")?;
					if color {
						write!(output, "\x1b[0m")?;
					}
					write!(output, "{:width$} ", "", width = (longest_instruction_bytes - item.bytes.len()) * 2)?;
				}
				writeln!(output, "{}", item.colored_instruction.as_deref().unwrap_or(&item.instruction))?;
			}
			Ok(())
		},
	}
}

pub(super) fn decode_bytes<'a>(bytes: &'a [u8], bitness: u32, decode_ip: u64, start_ip: u64, end_ip: u64, color: bool, symbols: Arc<HashMap<u64, String>>) -> Vec<DecodedInstruction<'a>> {
	let mut decoder = iced_x86::Decoder::with_ip(bitness, bytes, decode_ip, iced_x86::DecoderOptions::NONE);
	let mut formatter = iced_x86::IntelFormatter::with_options(Some(Box::new(PeSymbolResolver { symbols })), None);
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

fn build_symbols(pe: PeFile<'_>, image_base: u64) -> HashMap<u64, String> {
	let mut symbols = HashMap::new();

	if let Ok(by) = pe.exports().and_then(|exports| exports.by()) {
		let dll = by.dll_name().ok().and_then(|name| name.to_str().ok()).unwrap_or("<exports>");
		let mut names = HashMap::new();
		for (name, index) in by.iter_name_indices() {
			if let Ok(name) = name.and_then(|name| name.to_str().map_err(|_| pelite::Error::Encoding)) {
				names.insert(index, name.to_owned());
			}
		}
		for (index, export) in by.iter().enumerate() {
			let Some(rva) = export.ok().and_then(|export| export.symbol()) else {
				continue;
			};
			let name = names.remove(&index).unwrap_or_else(|| {
				let ordinal = u32::from(by.ordinal_base()).saturating_add(index as u32);
				format!("{dll}!#{ordinal}")
			});
			insert_symbol(&mut symbols, image_base, rva, name);
		}
	}

	if let Ok(imports) = pe.imports() {
		let pointer_size = pe.bits().size();
		for descriptor in imports {
			let Some(dll) = descriptor.dll_name().ok().and_then(|name| name.to_str().ok()) else {
				continue;
			};
			let first_thunk = descriptor.image().FirstThunk.get();
			let Ok(imports) = descriptor.int() else {
				continue;
			};
			for (index, import) in imports.enumerate() {
				let Ok(import) = import else {
					continue;
				};
				let Some(offset) = u32::try_from(index).ok().and_then(|index| index.checked_mul(pointer_size)) else {
					continue;
				};
				let Some(rva) = first_thunk.checked_add(offset) else {
					continue;
				};
				let name = match import {
					Import::ByName { name, .. } => {
						let Ok(name) = name.to_str() else {
							continue;
						};
						format!("{dll}!{name}")
					},
					Import::ByOrdinal { ord } => format!("{dll}!#{ord}"),
				};
				insert_symbol(&mut symbols, image_base, rva, name);
			}
		}
	}

	symbols
}

fn insert_symbol(symbols: &mut HashMap<u64, String>, image_base: u64, rva: u32, name: String) {
	if let Some(va) = image_base.checked_add(u64::from(rva)) {
		symbols.entry(va).or_insert(name);
	}
}
