use iced_x86::{Decoder, DecoderOptions, Mnemonic, OpKind};
use pelite::pe32::{image, PeFile, Ptr};
use super::*;

pub fn analyze(file: PeFile<'_>) -> Result<Vec<ThrowOutput>> {
	if file.file_header().Machine != image::IMAGE_FILE_MACHINE_I386 {
		return Err(err("MSVC throw analysis supports x86 and x64 only"));
	}
	let mut candidates = std::collections::BTreeMap::<u32, Vec<u32>>::new();
	for section in file.section_headers() {
		if !section.is_code() {
			continue;
		}
		let Ok(bytes) = file.get_section_bytes(section) else { continue };
		let base = file.rva_to_va(section.VirtualAddress)?;
		let mut decoder = Decoder::with_ip(32, bytes, base as u64, DecoderOptions::NONE);
		while decoder.can_decode() {
			let instruction = decoder.decode();
			if let Some(target) = instruction_target(instruction) {
				if let (Ok(rva), Ok(reference)) = (file.va_to_rva(target), file.va_to_rva(instruction.ip() as u32)) {
					if rva != 0 && rva % 4 == 0 {
						candidates.entry(rva).or_default().push(reference);
					}
				}
			}
		}
	}
	let mut output = Vec::new();
	for (rva, references) in candidates {
		if let Ok(mut item) = parse(file, rva) {
			item.references = references;
			output.push(item);
		}
	}
	Ok(output)
}

fn instruction_target(instruction: iced_x86::Instruction) -> Option<u32> {
	if instruction.mnemonic() == Mnemonic::Push && instruction.op0_kind() == OpKind::Immediate32 {
		Some(instruction.immediate32())
	}
	else if instruction.mnemonic() == Mnemonic::Mov && instruction.op1_kind() == OpKind::Immediate32 {
		Some(instruction.immediate32())
	}
	else {
		None
	}
}

fn pointer_rva<T: ?Sized>(file: PeFile<'_>, pointer: Ptr<T>) -> pelite::Result<u32> {
	if pointer.is_null() {
		return Err(pelite::Error::Invalid);
	}
	file.va_to_rva(pointer.into())
}

fn optional_pointer(file: PeFile<'_>, pointer: Ptr) -> pelite::Result<Option<u32>> {
	let va: u32 = pointer.into();
	if va == 0 {
		return Ok(None);
	}
	let rva = file.va_to_rva(va)?;
	if file.section_headers().by_rva(rva).is_some_and(|s| s.is_code()) {
		file.derva::<u8>(rva)?;
		Ok(Some(rva))
	}
	else {
		Err(pelite::Error::Invalid)
	}
}

fn data_address(file: PeFile<'_>, rva: u32) -> pelite::Result<()> {
	// Type descriptors may be in writable data; only require readable, non-code data.
	if rva != 0 && file.section_headers().by_rva(rva).is_some_and(|s|
		s.Characteristics & (image::IMAGE_SCN_MEM_EXECUTE | image::IMAGE_SCN_MEM_READ) == image::IMAGE_SCN_MEM_READ) {
		Ok(())
	}
	else {
		Err(pelite::Error::Invalid)
	}
}

fn type_name(file: PeFile<'_>, rva: u32) -> pelite::Result<String> {
	data_address(file, rva)?;
	let name = file.derva_c_str(rva.checked_add(8).ok_or(pelite::Error::Overflow)?)?.to_str()?;
	// Exception descriptors include primitive and pointer types, not just .?A classes.
	if !name.starts_with('.') || name.len() < 2 || name.len() > 4096 || !name.is_ascii() {
		return Err(pelite::Error::Invalid);
	}
	Ok(name.to_owned())
}

pub fn parse(file: PeFile<'_>, rva: u32) -> pelite::Result<ThrowOutput> {
	data_address(file, rva)?;
	let info = file.derva::<image::ThrowInfo>(rva)?;
	if info.attributes & !0x1f != 0 { return Err(pelite::Error::Invalid) }
	let destructor_rva = optional_pointer(file, info.unwind)?;
	let forward_compat_rva = optional_pointer(file, info.forward_compat)?;
	let array_rva = pointer_rva(file, info.catchable_type_array)?;
	data_address(file, array_rva)?;
	let array = file.deref(info.catchable_type_array)?;
	if !(1..=4096).contains(&array.catchable_types) { return Err(pelite::Error::Invalid) }
	let entries = file.derva_slice::<Ptr<image::CatchableType>>(array_rva.checked_add(4).ok_or(pelite::Error::Overflow)?, array.catchable_types as usize)?;
	let mut catchable_types = Vec::new();
	for &entry in entries {
		let entry_rva = pointer_rva(file, entry)?;
		data_address(file, entry_rva)?;
		let ty = file.deref(entry)?;
		if ty.size_or_offset <= 0 || ty.properties & !0x1f != 0 { return Err(pelite::Error::Invalid) }
		let descriptor = pointer_rva(file, ty.type_descriptor)?;
		catchable_types.push(CatchableOutput {
			rva: entry_rva,
			type_descriptor_rva: descriptor,
			name: type_name(file, descriptor)?,
			properties: ty.properties,
			simple_type: ty.properties & 1 != 0,
			by_reference_only: ty.properties & 2 != 0,
			has_virtual_bases: ty.properties & 4 != 0,
			displacement: Displacement { mdisp: ty.pmd.mdisp, pdisp: ty.pmd.pdisp, vdisp: ty.pmd.vdisp },
			size_or_offset: ty.size_or_offset,
			copy_function_rva: optional_pointer(file, ty.copy_function)?,
		});
	}
	Ok(ThrowOutput { rva, attributes: info.attributes, destructor_rva, forward_compat_rva,
		catchable_type_array_rva: array_rva, references: Vec::new(), catchable_types })
}
