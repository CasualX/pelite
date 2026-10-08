use super::*;

/// Scan executable bytes linearly for candidate addresses and type hints.
pub fn scan_code(input: &AnalysisInput<'_>, output: &mut AnalysisOutput) {
	for section in input.pe.section_headers() {
		if section.Characteristics & image::IMAGE_SCN_MEM_EXECUTE == 0 {
			continue;
		}
		let bytes = match input.pe.get_section_bytes(section) {
			Ok(bytes) => bytes,
			Err(error) => {
				eprintln!("analysis: disassembly at section RVA {:#x}: {error}", section.VirtualAddress);
				continue;
			},
		};
		// Exclude file alignment padding and bytes beyond the mapped image.
		let virtual_size = if section.VirtualSize == 0 { section.SizeOfRawData } else { section.VirtualSize };
		let len = bytes.len().min(virtual_size as usize).min(input.size.saturating_sub(section.VirtualAddress) as usize);
		if len != 0 {
			disassemble_bytes(input, output, input.bitness, &bytes[..len], section.VirtualAddress);
		}
	}
}

fn disassemble_bytes(input: &AnalysisInput<'_>, output: &mut AnalysisOutput, bitness: u32, bytes: &[u8], rva: u32) {
	let Some(ip) = input.pe.image_base().checked_add(rva as u64) else { return };
	let mut decoder = iced_x86::Decoder::with_ip(bitness, bytes, ip, iced_x86::DecoderOptions::NONE);
	while decoder.can_decode() {
		let instruction = decoder.decode();
		if instruction.is_invalid() {
			continue;
		}
		for operand in instruction.op_kinds() {
			use iced_x86::{Mnemonic, OpKind};
			match operand {
				OpKind::NearBranch16 | OpKind::NearBranch32 | OpKind::NearBranch64 => {
					let target = instruction.near_branch_target();
					output.add_va(input, target, Some(ty::Type::Code));
					if instruction.mnemonic() == Mnemonic::Call {
						if let Some(rva) = target.checked_sub(input.pe.image_base()).and_then(|rva| u32::try_from(rva).ok()) {
							if input.mapped(rva) && input.executable(rva) {
								output.function_candidates.insert(rva);
							}
						}
					}
				},
				OpKind::Memory => {
					let Some(va) = static_memory_address(&instruction) else { continue };
					let ty = if instruction.mnemonic() == Mnemonic::Lea { None } else { interpretation(instruction.memory_size()) };
					output.add_va(input, va, ty);
				},
				OpKind::Immediate32 => output.add_va(input, instruction.immediate32() as u64, None),
				OpKind::Immediate64 => output.add_va(input, instruction.immediate64(), None),
				OpKind::Immediate32to64 => output.add_va(input, instruction.immediate32to64() as u64, None),
				_ => {},
			}
		}
	}
}

/// Resolve only memory operands whose address does not depend on runtime state.
pub(super) fn static_memory_address(instruction: &iced_x86::Instruction) -> Option<u64> {
	use iced_x86::Register;
	if matches!(instruction.segment_prefix(), Register::FS | Register::GS) {
		return None;
	}
	if instruction.is_ip_rel_memory_operand() {
		Some(instruction.ip_rel_memory_address())
	}
	else if instruction.memory_base() == Register::None && instruction.memory_index() == Register::None {
		Some(instruction.memory_displacement64())
	}
	else {
		None
	}
}

fn interpretation(size: iced_x86::MemorySize) -> Option<ty::Type> {
	use iced_x86::MemorySize::*;
	Some(match size {
		UInt8 => ty::Type::U8,
		UInt16 => ty::Type::U16,
		UInt32 => ty::Type::U32,
		UInt64 => ty::Type::U64,
		Int8 => ty::Type::I8,
		Int16 => ty::Type::I16,
		Int32 => ty::Type::I32,
		Int64 => ty::Type::I64,
		Float32 => ty::Type::F32,
		Float64 => ty::Type::F64,
		_ => return None,
	})
}
