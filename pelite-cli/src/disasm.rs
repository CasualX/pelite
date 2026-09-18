use pelite::{image, Import, PeFile, Wrap};

use super::*;

#[derive(serde::Serialize)]
struct DisassembledInstruction<'a> {
	address: u64,
	bytes: &'a [u8],
	instruction: String,
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
		.about("Disassemble an RVA range using iced-x86")
		.after_help("The range is half-open and its endpoints are hexadecimal RVAs (for example, 1000..1100 or 0x1000..0x1100).")
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("range")
			.value_name("START..END")
			.value_parser(rva_range::parse)
			.required(true))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let range = *matches.get_one::<rva_range::RvaRange>("range").expect("required by clap");
	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let bitness = match pe.file_header().Machine {
		image::IMAGE_FILE_MACHINE_I386 => 32,
		image::IMAGE_FILE_MACHINE_AMD64 => 64,
		machine => return Err(err(format!("unsupported machine type {machine:#06x}; expected i386 or AMD64"))),
	};
	let len = usize::try_from(range.end - range.start)?;
	let bytes = pe.slice(range.start, len, 1)?;
	let bytes = &bytes[..len];
	let image_base = image_base(pe);
	let start_ip = image_base.checked_add(u64::from(range.start)).ok_or_else(|| err("image base plus start RVA overflows"))?;

	let mut decoder = iced_x86::Decoder::with_ip(bitness, bytes, start_ip, iced_x86::DecoderOptions::NONE);
	let symbols = Arc::new(build_symbols(pe, bitness, image_base));
	let mut formatter = iced_x86::IntelFormatter::with_options(Some(Box::new(PeSymbolResolver { symbols: Arc::clone(&symbols) })), None);
	let mut instructions = Vec::new();
	while decoder.can_decode() {
		let instruction = decoder.decode();
		let address = instruction.ip();
		let offset = usize::try_from(address - start_ip)?;
		let instruction_bytes = &bytes[offset..offset + instruction.len()];
		let mut text = String::new();
		iced_x86::Formatter::format(&mut formatter, &instruction, &mut text);
		instructions.push(DisassembledInstruction { address, bytes: instruction_bytes, instruction: text });
	}

	match format {
		OutputFormat::Json => print_json(&instructions, false),
		OutputFormat::JsonPretty => print_json(&instructions, true),
		OutputFormat::Text => {
			for item in instructions {
				if let Some(symbol) = symbols.get(&item.address) {
					println!("\n{symbol}:");
				}
				let bytes = item.bytes.iter().map(|byte| format!("{byte:02x}")).collect::<Vec<_>>().join(" ");
				let address_width = bitness as usize / 4 + 2;
				let section_name = item.address.checked_sub(image_base)
					.and_then(|rva| u32::try_from(rva).ok())
					.and_then(|rva| pe.section_headers().by_rva(rva))
					.and_then(|section| section.name().ok())
					.unwrap_or("<no section>");
				println!("{section_name}:{:#0address_width$x}  {bytes:<44} {}", item.address, item.instruction);
			}
			Ok(())
		},
	}
}

fn image_base(pe: PeFile<'_>) -> u64 {
	match pe.optional_header() {
		Wrap::T32(header) => u64::from(header.ImageBase),
		Wrap::T64(header) => header.ImageBase.get(),
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
