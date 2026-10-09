use super::*;

use super::disassembly::static_memory_address;
use super::metadata::import_names;

/// Inspect the first instruction at each discovered code address, without following
/// jumps or changing symbol identity.
pub fn refine_labels(input: &AnalysisInput<'_>, output: &mut AnalysisOutput) {
	let pe = input.pe;
	let bitness = input.bitness;
	let size = input.size;
	use iced_x86::{Mnemonic, OpKind};
	let imports = import_names(pe, bitness);
	for symbol in output.symbols.values_mut() {
		if symbol.name != factmap::SymbolName::Code {
			continue;
		}
		let rva = symbol.rva;
		let Some(ip) = pe.image_base().checked_add(rva as u64) else { continue };
		let Ok(bytes) = pe.slice(rva, 1, 1) else { continue };
		let mut len = bytes.len().min(size.saturating_sub(rva) as usize).min(15);
		if let Some(section) = pe.section_headers().iter().find(|section| {
			rva.checked_sub(section.VirtualAddress).is_some_and(|offset| offset < section.VirtualSize.max(section.SizeOfRawData))
		}) {
			let extent = if section.VirtualSize == 0 { section.SizeOfRawData } else { section.VirtualSize };
			len = len.min(extent.saturating_sub(rva - section.VirtualAddress) as usize);
		}
		let instruction = iced_x86::Decoder::with_ip(bitness, &bytes[..len], ip, iced_x86::DecoderOptions::NONE).decode();
		if instruction.is_invalid() {
			continue;
		}
		let label = match instruction.mnemonic() {
			Mnemonic::Ret => {
				let pop = if instruction.op_count() == 0 { 0 } else { instruction.immediate16() };
				format!("ret{pop}")
			},
			Mnemonic::Jmp => match instruction.op0_kind() {
				OpKind::NearBranch16 | OpKind::NearBranch32 | OpKind::NearBranch64 => "thunk".to_owned(),
				OpKind::Memory if matches!(instruction.memory_size(), iced_x86::MemorySize::DwordOffset | iced_x86::MemorySize::QwordOffset) => {
					let Some(va) = static_memory_address(&instruction) else { continue };
					let name = va.checked_sub(pe.image_base()).and_then(|rva| u32::try_from(rva).ok())
						.and_then(|rva| imports.get(&rva));
					match name {
						Some(name) => format!("imp_{name}"),
						None => "indirect".to_owned(),
					}
				},
				_ => continue,
			},
			_ => continue,
		};
		symbol.name = if label == "thunk" { factmap::SymbolName::Thunk }
			else { factmap::SymbolName::Named(label) };
		upgrade_type(symbol, ty::Type::Code)
			.expect("code hints always upgrade successfully");
	}
}
