use pelite::pe64::{image, PeFile};
use super::*;

pub(super) fn parse(file: PeFile<'_>, rva: u32) -> pelite::Result<ThrowOutput> {
	let pe = Wrap::T64(file);
	data_address(pe, rva)?;
	let info = file.derva::<image::ThrowInfo>(rva)?;
	if info.attributes & !0x1f != 0 { return Err(pelite::Error::Invalid) }
	let destructor_rva = code_address(pe, info.unwind)?;
	let forward_compat_rva = code_address(pe, info.forward_compat)?;
	data_address(pe, info.catchable_type_array)?;
	let array = file.derva::<image::CatchableTypeArray>(info.catchable_type_array)?;
	if !(1..=4096).contains(&array.catchable_types) { return Err(pelite::Error::Invalid) }
	let entries = file.derva_slice::<u32>(info.catchable_type_array.checked_add(4).ok_or(pelite::Error::Overflow)?, array.catchable_types as usize)?;
	let mut catchable_types = Vec::new();
	for &entry in entries {
		data_address(pe, entry)?;
		let ty = file.derva::<image::CatchableType>(entry)?;
		if ty.size_or_offset <= 0 || ty.properties & !0x1f != 0 { return Err(pelite::Error::Invalid) }
		catchable_types.push(CatchableOutput {
			rva: entry,
			type_descriptor_rva: ty.type_descriptor,
			name: type_name(pe, ty.type_descriptor, 16)?,
			properties: ty.properties,
			simple_type: ty.properties & 1 != 0,
			by_reference_only: ty.properties & 2 != 0,
			has_virtual_bases: ty.properties & 4 != 0,
			displacement: Displacement { mdisp: ty.pmd.mdisp, pdisp: ty.pmd.pdisp, vdisp: ty.pmd.vdisp },
			size_or_offset: ty.size_or_offset,
			copy_function_rva: code_address(pe, ty.copy_function)?,
		});
	}
	Ok(ThrowOutput { rva, attributes: info.attributes, destructor_rva, forward_compat_rva,
		catchable_type_array_rva: info.catchable_type_array, references: Vec::new(), catchable_types })
}
