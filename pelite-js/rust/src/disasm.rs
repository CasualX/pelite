use super::*;

fn get_section_name_by_rva<'a>(pe: pelite::PeFile<'a>, rva: u32) -> &'a str {
	pe.section_headers().by_rva(rva)
		.and_then(|section| section.name().ok())
		.unwrap_or("<invalid>")
}

#[derive(serde::Serialize)]
struct DisassembledInstruction<'a> {
	address: String,
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

#[unsafe(export_name = "pefileDisasm")]
pub unsafe fn disasm(pefile: *mut PeFile, start: u32, end: u32) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	let bitness = match pefile.file_header().Machine {
		pelite::image::IMAGE_FILE_MACHINE_I386 => 32,
		pelite::image::IMAGE_FILE_MACHINE_AMD64 => 64,
		machine => return return_error(format!("unsupported machine type {machine:#06x}; expected i386 or AMD64")),
	};
	let len = match end.checked_sub(start).and_then(|len| usize::try_from(len).ok()) {
		Some(len) => len,
		None => return return_error("disassembly range end must not precede start"),
	};
	let bytes = match pefile.slice(start, len, 1) {
		Ok(bytes) => bytes,
		Err(err) => return return_error(err),
	};
	let image_base = pefile.image_base();
	let start_ip = image_base + start as u64;
	let end_ip = image_base + end as u64;

	let symbols = Arc::new(build_symbols(pefile, image_base));
	let mut decoder = iced_x86::Decoder::with_ip(bitness, bytes, start_ip, iced_x86::DecoderOptions::NONE);
	let mut formatter = iced_x86::IntelFormatter::with_options(Some(Box::new(PeSymbolResolver { symbols })), None);
	let mut instructions = Vec::new();
	while decoder.can_decode() && decoder.ip() < end_ip {
		let instruction = decoder.decode();
		let address = instruction.ip();
		let offset = (address - start_ip) as usize;
		let instruction_bytes = &bytes[offset..offset + instruction.len()];
		let section_name = get_section_name_by_rva(pefile, (address - image_base) as u32);
		let mut text = String::new();
		iced_x86::Formatter::format(&mut formatter, &instruction, &mut text);
		instructions.push(DisassembledInstruction {
			address: format!("{section_name}:{address:#x}"),
			bytes: instruction_bytes,
			instruction: text,
		});
	}

	return_json(instructions);
}

fn build_symbols(pefile: pelite::PeFile<'_>, image_base: u64) -> HashMap<u64, String> {
	let mut symbols = HashMap::new();

	if let Ok(by) = pefile.exports().and_then(|exports| exports.by()) {
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

	if let Ok(imports) = pefile.imports() {
		let pointer_size = pefile.bits().size();
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
					pelite::Import::ByName { name, .. } => {
						let Ok(name) = name.to_str() else {
							continue;
						};
						format!("{dll}!{name}")
					},
					pelite::Import::ByOrdinal { ord } => format!("{dll}!#{ord}"),
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
