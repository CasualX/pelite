use super::*;
use super::disassembly::static_memory_address;
use iced_x86::{InstructionInfoFactory, InstructionInfoOptions, Mnemonic, OpAccess, OpKind, Register};

impl Analysis<'_> {
	/// Experimentally carry static global references forward to indirect register calls.
	/// This is a linear scan, without control-flow or calling-convention analysis.
	pub fn forward_feed(&mut self) {
		for section in self.pe.section_headers() {
			if section.Characteristics & image::IMAGE_SCN_MEM_EXECUTE == 0 {
				continue;
			}
			let bytes = match self.pe.get_section_bytes(section) {
				Ok(bytes) => bytes,
				Err(error) => {
					eprintln!("analysis: forward feeder at section RVA {:#x}: {error}", section.VirtualAddress);
					continue;
				},
			};
			let virtual_size = if section.VirtualSize == 0 { section.SizeOfRawData } else { section.VirtualSize };
			let len = bytes.len().min(virtual_size as usize).min(self.size.saturating_sub(section.VirtualAddress) as usize);
			self.feed_bytes(&bytes[..len], section.VirtualAddress);
		}
	}

	fn feed_bytes(&mut self, bytes: &[u8], rva: u32) {
		let Some(ip) = self.pe.image_base().checked_add(u64::from(rva)) else { return };
		let mut decoder = iced_x86::Decoder::with_ip(self.bitness, bytes, ip, iced_x86::DecoderOptions::NONE);
		let mut info = InstructionInfoFactory::new();
		let mut globals = [None; 16];
		while decoder.can_decode() {
			let instruction = decoder.decode();
			if instruction.is_invalid() {
				globals.fill(None);
				continue;
			}
			if instruction.mnemonic() == Mnemonic::Call && instruction.op0_kind() == OpKind::Register {
				if let Some(register) = register_index(instruction.op0_register()) {
					if let Some(global) = globals[register].and_then(|rva| self.symbols.get(&rva)) {
						let rva = (instruction.ip() - self.pe.image_base()) as u32;
						let comment = symbols::IndexedSymbol::from(global).to_string();
						self.comments.insert(rva, factmap::CommentFact { rva, comment });
					}
				}
			}
			// Invalidate before recording a new load, including implicit and partial writes.
			for used in info.info_options(&instruction, InstructionInfoOptions::NO_MEMORY_USAGE).used_registers() {
				if matches!(used.access(), OpAccess::Write | OpAccess::CondWrite | OpAccess::ReadWrite | OpAccess::ReadCondWrite) {
					if let Some(register) = register_index(used.register()) {
						globals[register] = None;
					}
				}
			}
			if instruction.op0_kind() != OpKind::Register || instruction.op1_kind() != OpKind::Memory {
				continue;
			}
			let destination = instruction.op0_register();
			if !(destination.is_gpr32() || destination.is_gpr64()) {
				continue;
			}
			if instruction.mnemonic() != Mnemonic::Lea
				&& !(instruction.mnemonic() == Mnemonic::Mov && matches!(instruction.memory_size().size(), 4 | 8)) {
				continue;
			}
			let Some(va) = static_memory_address(&instruction) else { continue };
			let Ok(global_rva) = self.pe.va_to_rva(va) else { continue };
			if self.symbols.contains_key(&global_rva) {
				globals[register_index(destination).unwrap()] = Some(global_rva);
			}
		}
	}
}

fn register_index(register: Register) -> Option<usize> {
	let full = register.full_register();
	full.is_gpr64().then(|| full.number())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[repr(align(8))]
	struct Aligned<const N: usize>([u8; N]);
	static IMAGE64: Aligned<{ include_bytes!("../../../../demo/Demo64.dll").len() }> = Aligned(*include_bytes!("../../../../demo/Demo64.dll"));
	static IMAGE32: Aligned<{ include_bytes!("../../../../demo/Demo.dll").len() }> = Aligned(*include_bytes!("../../../../demo/Demo.dll"));

	fn global_load(opcode: u8, rva: u32, global: u32) -> Vec<u8> {
		let mut bytes = vec![0x48, opcode, 0x05]; // mov/lea rax,[rip+disp32]
		bytes.extend_from_slice(&(global as i32 - (rva + 7) as i32).to_le_bytes());
		bytes
	}

	fn analyze_bytes(bytes: &[u8]) -> Analysis<'static> {
		let pe = PeFile::from_bytes(&IMAGE64.0).unwrap();
		let mut analysis = Analysis::new(pe).unwrap();
		for (rva, name) in [(0x3000, "__imp_Function"), (0x3010, "other_global")] {
			analysis.symbols.insert(rva, factmap::SymbolFact::new(rva, ty::Type::Va, factmap::SymbolName::Named(name.into())));
		}
		analysis.feed_bytes(bytes, 0x1000);
		analysis
	}

	#[test]
	fn annotates_loads_and_address_references() {
		for opcode in [0x8b, 0x8d] {
			let mut bytes = global_load(opcode, 0x1000, 0x3000);
			bytes.extend_from_slice(&[0xb1, 1, 0xff, 0xd0]); // mov cl,1; call rax
			let analysis = analyze_bytes(&bytes);
			assert_eq!(analysis.comments.len(), 1);
			assert_eq!(analysis.comments[&0x1009].comment, "__imp_Function");
			let map = analysis.into_factmap();
			let mut output = Vec::new();
			map.write(&mut output, "").unwrap();
			assert_eq!(factmap::FactMap::parse(std::str::from_utf8(&output).unwrap(), ty::PointerWidth::Bits64).unwrap(), map);
		}
	}

	#[test]
	fn all_alias_and_implicit_writes_invalidate() {
		for overwrite in [
			vec![0xb0, 1], // al
			vec![0xb4, 1], // ah
			vec![0x66, 0xb8, 1, 0], // ax
			vec![0xb8, 1, 0, 0, 0], // eax
			vec![0x48, 0x83, 0xc0, 1], // add rax,1
			vec![0x48, 0xf7, 0xe1], // mul rcx, implicitly writes rax and rdx
			vec![0x48, 0x0f, 0x44, 0xc1], // cmove rax,rcx, conditional write
			vec![0x48, 0x87, 0xc1], // xchg rcx,rax, writes both operands
		] {
			let mut bytes = global_load(0x8b, 0x1000, 0x3000);
			bytes.extend_from_slice(&overwrite);
			bytes.extend_from_slice(&[0xff, 0xd0]);
			assert!(analyze_bytes(&bytes).comments.is_empty(), "{overwrite:x?}");
		}
	}

	#[test]
	fn replacement_load_updates_origin_and_reads_preserve_it() {
		let mut bytes = global_load(0x8b, 0x1000, 0x3000);
		bytes.extend_from_slice(&global_load(0x8b, 0x1007, 0x3010));
		bytes.extend_from_slice(&[0x48, 0x85, 0xc0, 0xff, 0xd0]); // test rax,rax; call rax
		assert_eq!(analyze_bytes(&bytes).comments[&0x1011].comment, "other_global");
	}

	#[test]
	fn narrow_dynamic_and_segment_relative_loads_are_not_tracked() {
		for bytes in [
			vec![0x8a, 0x05, 0xfa, 0x1f, 0, 0, 0xff, 0xd0], // mov al,[rip+disp32]
			vec![0x66, 0x8b, 0x05, 0xf9, 0x1f, 0, 0, 0xff, 0xd0], // mov ax,[rip+disp32]
			vec![0x48, 0x8b, 0x01, 0xff, 0xd0], // mov rax,[rcx]
			vec![0x64, 0x48, 0x8b, 0x05, 0xf8, 0x1f, 0, 0, 0xff, 0xd0], // FS-relative
			global_load(0x8b, 0x1000, 0x4000).into_iter().chain([0xff, 0xd0]).collect(), // no known symbol
		] {
			assert!(analyze_bytes(&bytes).comments.is_empty());
		}
	}

	#[test]
	fn x86_dword_global_load() {
		let pe = PeFile::from_bytes(&IMAGE32.0).unwrap();
		let mut analysis = Analysis::new(pe).unwrap();
		analysis.symbols.insert(0x3000, factmap::SymbolFact::new(0x3000, ty::Type::Va, factmap::SymbolName::Data));
		let mut bytes = vec![0xa1]; // mov eax,[absolute address]
		bytes.extend_from_slice(&((pe.image_base() + 0x3000) as u32).to_le_bytes());
		bytes.extend_from_slice(&[0xff, 0xd0]); // call eax
		analysis.feed_bytes(&bytes, 0x1000);
		assert_eq!(analysis.comments[&0x1005].comment, "data_3000");
	}
}
