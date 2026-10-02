use pelite::pe32::{image, PeFile, Ptr};
use super::*;

fn pointer_rva<T: ?Sized>(file: PeFile<'_>, pointer: Ptr<T>) -> pelite::Result<u32> {
	file.va_to_rva(pointer.into())
}

fn optional_pointer(file: PeFile<'_>, pointer: Ptr) -> pelite::Result<Option<u32>> {
	let va: u32 = pointer.into();
	if va == 0 { Ok(None) } else { code_address(Wrap::T32(file), file.va_to_rva(va)?) }
}

pub(super) fn parse(file: PeFile<'_>, rva: u32) -> pelite::Result<ThrowOutput> {
	let pe = Wrap::T32(file);
	data_address(pe, rva)?;
	let info = file.derva::<image::ThrowInfo>(rva)?;
	if info.attributes & !0x1f != 0 { return Err(pelite::Error::Invalid) }
	let destructor_rva = optional_pointer(file, info.unwind)?;
	let forward_compat_rva = optional_pointer(file, info.forward_compat)?;
	let array_rva = pointer_rva(file, info.catchable_type_array)?;
	data_address(pe, array_rva)?;
	let array = file.deref(info.catchable_type_array)?;
	if !(1..=4096).contains(&array.catchable_types) { return Err(pelite::Error::Invalid) }
	let entries = file.derva_slice::<Ptr<image::CatchableType>>(array_rva.checked_add(4).ok_or(pelite::Error::Overflow)?, array.catchable_types as usize)?;
	let mut catchable_types = Vec::new();
	for &entry in entries {
		let entry_rva = pointer_rva(file, entry)?;
		data_address(pe, entry_rva)?;
		let ty = file.deref(entry)?;
		if ty.size_or_offset <= 0 || ty.properties & !0x1f != 0 { return Err(pelite::Error::Invalid) }
		let descriptor = pointer_rva(file, ty.type_descriptor)?;
		catchable_types.push(CatchableOutput {
			rva: entry_rva,
			type_descriptor_rva: descriptor,
			name: type_name(pe, descriptor, 8)?,
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
