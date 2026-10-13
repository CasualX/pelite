//! JSON disassembly, maintained separately from pelite-cli's implementation.

use std::{cell::Cell, collections::BTreeSet, rc::Rc};

use super::*;

fn get_section_name_by_rva<'a>(pe: pelite::PeFile<'a>, rva: u32) -> &'a str {
	pe.section_headers().by_rva(rva)
		.and_then(|section| section.name().ok())
		.unwrap_or("<invalid>")
}

#[derive(serde::Serialize)]
struct DecodedInstruction<'a> {
	rva: u32,
	prefix: &'a str,
	label: Option<String>,
	bytes: &'a [u8],
	instruction: String,
	comments: Vec<String>,
}

struct DecodeOptions<'a> {
	decode_rva: u32,
	prefix: &'a str,
	symbols: HashMap<u64, String>,
	relocations: BTreeSet<usize>,
	relocation_width: usize,
}

struct Formatter {
	inner: iced_x86::IntelFormatter,
	symbol_operands: Rc<Cell<u32>>,
}

impl Formatter {
	fn new(symbols: Arc<HashMap<u64, String>>) -> Self {
		let symbol_operands = Rc::new(Cell::new(0));
		let resolver = Box::new(PeSymbolResolver { symbols, operands: Rc::clone(&symbol_operands) });
		let mut inner = iced_x86::IntelFormatter::with_options(Some(resolver), None);
		let options = iced_x86::Formatter::options_mut(&mut inner);
		options.set_hex_prefix("0x");
		options.set_hex_suffix("");
		options.set_uppercase_hex(false);
		Self { inner, symbol_operands }
	}

	fn format(&mut self, instruction: &iced_x86::Instruction, constants: iced_x86::ConstantOffsets, relocations: &[usize], relocation_width: usize) -> String {
		self.symbol_operands.set(symbol_operands(instruction, constants, relocations, relocation_width));
		let mut text = String::new();
		iced_x86::Formatter::format(&mut self.inner, instruction, &mut text);
		text
	}
}

struct PeSymbolResolver {
	symbols: Arc<HashMap<u64, String>>,
	operands: Rc<Cell<u32>>,
}

impl iced_x86::SymbolResolver for PeSymbolResolver {
	fn symbol(&mut self, instruction: &iced_x86::Instruction, _operand: u32, instruction_operand: Option<u32>, address: u64, address_size: u32) -> Option<iced_x86::SymbolResult<'_>> {
		let operand = instruction_operand?;
		if self.operands.get() & (1 << operand) == 0 {
			return None;
		}
		// A far branch's selector shares the instruction operand with its relocated offset.
		if instruction.op_kind(operand) == iced_x86::OpKind::FarBranch32 && address_size != 4 {
			return None;
		}
		self.symbols.get(&address).map(|name| iced_x86::SymbolResult::with_str(address, name))
	}
}

#[unsafe(export_name = "pefileDisasm")]
pub unsafe fn disasm(pefile: *mut PeFile, start: u32, end: u32, lookback: u32) {
	let pefile = match pelite::PeFile::from_bytes(unsafe { &*pefile }.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};
	match disassemble(pefile, start, end, lookback) {
		Ok(instructions) => return_json(instructions),
		Err(err) => return_error(err),
	}
}

fn disassemble<'a>(pefile: pelite::PeFile<'a>, start: u32, end: u32, lookback: u32) -> Result<Vec<DecodedInstruction<'a>>, String> {
	let bitness = match pefile.file_header().Machine {
		pelite::image::IMAGE_FILE_MACHINE_I386 => 32,
		pelite::image::IMAGE_FILE_MACHINE_AMD64 => 64,
		machine => return Err(format!("unsupported machine type {machine:#06x}; expected i386 or AMD64").into()),
	};
	let len = end.checked_sub(start).ok_or("disassembly range end must not precede start")? as usize;
	let bytes = pefile.slice(start, len, 1).map_err(|err| err.to_string())?;
	let (decode_start, bytes) = pefile.section_headers().by_rva(start)
		.filter(|_| lookback != 0 && len != 0)
		.and_then(|section| {
			let decode_start = start.saturating_sub(lookback).max(section.VirtualAddress);
			let len = (end - decode_start) as usize;
			pefile.slice(decode_start, len, 1).ok().map(|bytes| (decode_start, bytes))
		})
		.unwrap_or((start, bytes));
	// Include enough trailing bytes to complete an instruction starting before end.
	let decode_len = if len == 0 { 0 } else { ((end - decode_start) as usize).saturating_add(14) };
	let bytes = &bytes[..bytes.len().min(decode_len)];
	let image_base = pefile.image_base();
	let decode_ip = image_base + u64::from(decode_start);
	let start_ip = image_base + u64::from(start);
	let end_ip = image_base + u64::from(end);
	let (relocations, relocation_width) = match pefile {
		pelite::Wrap::T32(pe) => (load_relocs32(pe, decode_start, bytes.len()), 4),
		pelite::Wrap::T64(pe) => (load_relocs64(pe, decode_start, bytes.len()), 8),
	};
	let options = DecodeOptions {
		decode_rva: decode_start,
		prefix: get_section_name_by_rva(pefile, start),
		symbols: build_symbols(pefile, image_base),
		relocations,
		relocation_width,
	};
	Ok(decode_bytes(bytes, bitness, decode_ip, start_ip, end_ip, options))
}

fn decode_bytes<'a>(bytes: &'a [u8], bitness: u32, decode_ip: u64, start_ip: u64, end_ip: u64, options: DecodeOptions<'a>) -> Vec<DecodedInstruction<'a>> {
	let symbols = Arc::new(options.symbols);
	let mut decoder = iced_x86::Decoder::with_ip(bitness, bytes, decode_ip, iced_x86::DecoderOptions::NONE);
	let mut formatter = Formatter::new(Arc::clone(&symbols));
	let mut instructions = Vec::new();
	while decoder.can_decode() && decoder.ip() < end_ip {
		// Decode an instruction.
		let offset = decoder.position();
		let instruction = decoder.decode();
		if instruction.next_ip() <= start_ip {
			continue;
		}

		// Match fixups to the encoded operand fields.
		let instruction_bytes = &bytes[offset..decoder.position()];
		let relocations = options.relocations.range(offset..decoder.position())
			.map(|&field| field - offset)
			.filter(|&field| field + options.relocation_width <= instruction_bytes.len())
			.collect::<Vec<_>>();
		let ip = instruction.ip();
		let mut comments = Vec::new();
		for offset in 1..instruction_bytes.len() {
			if let Some(symbol) = symbols.get(&(ip + offset as u64)) {
				comments.push(format!("+{offset:#x}: {symbol}"));
			}
		}
		let label = symbols.get(&ip).cloned();
		let constants = decoder.get_constant_offsets(&instruction);
		let text = formatter.format(&instruction, constants, &relocations, options.relocation_width);
		instructions.push(DecodedInstruction {
			rva: options.decode_rva + offset as u32,
			prefix: options.prefix, label, bytes: instruction_bytes, instruction: text, comments,
		});
	}
	instructions
}

fn symbol_operands(instruction: &iced_x86::Instruction, constants: iced_x86::ConstantOffsets, relocations: &[usize], relocation_width: usize) -> u32 {
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

/// Collect offsets of complete HIGHLOW fields within the decode slice.
fn load_relocs32(pe: pelite::pe32::PeFile<'_>, start: u32, len: usize) -> BTreeSet<usize> {
	let mut result = BTreeSet::new();
	let Ok(relocs) = pe.base_relocs() else { return result };
	relocs.for_each(|rva, ty| {
		if ty != pelite::image::IMAGE_REL_BASED_HIGHLOW {
			return;
		}
		let Some(offset) = rva.checked_sub(start) else { return };
		let offset = offset as usize;
		if offset.checked_add(4).is_some_and(|end| end <= len) {
			result.insert(offset);
		}
	});
	result
}

/// Collect offsets of complete DIR64 fields within the decode slice.
fn load_relocs64(pe: pelite::pe64::PeFile<'_>, start: u32, len: usize) -> BTreeSet<usize> {
	let mut result = BTreeSet::new();
	let Ok(relocs) = pe.base_relocs() else { return result };
	relocs.for_each(|rva, ty| {
		if ty != pelite::image::IMAGE_REL_BASED_DIR64 {
			return;
		}
		let Some(offset) = rva.checked_sub(start) else { return };
		let offset = offset as usize;
		if offset.checked_add(8).is_some_and(|end| end <= len) {
			result.insert(offset);
		}
	});
	result
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
