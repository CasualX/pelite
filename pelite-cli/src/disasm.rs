use pelite::{image, Import, PeFile};

use super::*;

const MNEMONIC_COLOR: &str = "\x1b[1;97m";
const ADDRESS_COLOR: &str = "\x1b[38;2;200;174;130m";
const REGISTER_COLOR: &str = "\x1b[38;2;134;186;184m";
const NUMBER_COLOR: &str = "\x1b[38;2;185;190;198m";

#[derive(serde::Serialize)]
struct DisassembledInstruction<'a> {
	address: u64,
	bytes: &'a [u8],
	instruction: String,
	#[serde(skip)]
	colored_instruction: Option<String>,
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
		.after_help("Ranges are half-open and hexadecimal, with one rva:, va:, or fo: prefix applying to both endpoints (for example, rva:0x1000..0x1100).")
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("range")
			.value_name("KIND:START..END")
			.value_parser(AddressRange::parse)
			.required(true))
		.arg(clap::Arg::new("hex")
			.long("hex")
			.action(clap::ArgAction::SetTrue)
			.help("Show instruction opcode bytes in text output"))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let range = *matches.get_one::<AddressRange>("range").expect("required by clap");
	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let range = range.to_rva(pe)?;
	let bitness = match pe.file_header().Machine {
		image::IMAGE_FILE_MACHINE_I386 => 32,
		image::IMAGE_FILE_MACHINE_AMD64 => 64,
		machine => return Err(err(format!("unsupported machine type {machine:#06x}; expected i386 or AMD64"))),
	};
	let len = usize::try_from(range.end - range.start)?;
	let bytes = pe.slice(range.start, len, 1)?;
	let image_base = pe.image_base();
	let start_ip = image_base + range.start as u64;
	let end_ip = image_base + range.end as u64;

	let mut decoder = iced_x86::Decoder::with_ip(bitness, bytes, start_ip, iced_x86::DecoderOptions::NONE);
	let symbols = Arc::new(build_symbols(pe, bitness, image_base));
	let mut formatter = iced_x86::IntelFormatter::with_options(Some(Box::new(PeSymbolResolver { symbols: Arc::clone(&symbols) })), None);
	let color = format == OutputFormat::Text && io::stdout().is_terminal();
	let mut instructions = Vec::new();
	while decoder.can_decode() && decoder.ip() < end_ip {
		let instruction = decoder.decode();
		let address = instruction.ip();
		let offset = usize::try_from(address - start_ip)?;
		let instruction_bytes = &bytes[offset..offset + instruction.len()];
		let mut text = InstructionText { plain: String::new(), colored: color.then(String::new) };
		iced_x86::Formatter::format(&mut formatter, &instruction, &mut text);
		instructions.push(DisassembledInstruction { address, bytes: instruction_bytes, instruction: text.plain, colored_instruction: text.colored });
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
				if let Some(symbol) = symbols.get(&item.address) {
					if color {
						writeln!(output, "\n\x1b[1m{ADDRESS_COLOR}{symbol}:\x1b[0m")?;
					}
					else {
						writeln!(output, "\n{symbol}:")?;
					}
				}
				let address_width = bitness as usize / 4 + 2;
				let section_name = item.address.checked_sub(image_base)
					.and_then(|rva| u32::try_from(rva).ok())
					.and_then(|rva| pe.section_headers().by_rva(rva))
					.and_then(|section| section.name().ok())
					.unwrap_or("<no section>");
				if color {
					write!(output, "\x1b[90m{section_name}\x1b[0m:{ADDRESS_COLOR}{:#0address_width$x}\x1b[0m  ", item.address)?;
				}
				else {
					write!(output, "{section_name}:{:#0address_width$x}  ", item.address)?;
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

fn build_symbols(pe: PeFile<'_>, bitness: u32, image_base: u64) -> HashMap<u64, String> {
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
		let pointer_size = bitness / 8;
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
