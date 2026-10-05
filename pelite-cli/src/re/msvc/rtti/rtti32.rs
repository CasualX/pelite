use pelite::pe32::{image, PeFile, Ptr, Rva, Va};

use super::*;

struct RawVTable<'a> {
	col: &'a image::RTTICompleteObjectLocator,
	methods: usize,
	rva: Rva,
}

struct RawType<'a> {
	type_ptr: Ptr<image::TypeDescriptor>,
	class_ptr: Ptr<image::RTTIClassHierarchyDescriptor>,
	name: &'a str,
	class: &'a image::RTTIClassHierarchyDescriptor,
	vtables: Vec<RawVTable<'a>>,
}

pub fn analyze(file: PeFile<'_>) -> Result<Vec<TypeOutput>> {
	let text = file.section_headers().by_name(".text").ok_or_else(|| err("no .text section found"))?;
	let rdata = file.section_headers().by_name(".rdata").ok_or_else(|| err("no .rdata section found"))?;
	let relocs = file.base_relocs().map_err(|_| err("no base relocations found"))?;

	let mut vrefs = Vec::new();
	relocs.for_each(|rva, _| {
		if rva < rdata.VirtualAddress || rva >= rdata.VirtualAddress.saturating_add(rdata.VirtualSize) {
			return;
		}
		let Ok(target_va) = file.derva_copy(rva, false) else {
			return;
		};
		let Ok(target_rva) = file.va_to_rva(target_va) else {
			return;
		};
		if target_rva >= text.VirtualAddress && target_rva < text.VirtualAddress.saturating_add(text.VirtualSize) {
			vrefs.push(rva);
		}
	});
	vrefs.sort_unstable();
	vrefs.dedup();

	let mut xrefs = Vec::new();
	relocs.for_each(|rva, _| {
		let Ok(target_va) = file.derva_copy(rva, false) else {
			return;
		};
		let Ok(target_rva) = file.va_to_rva(target_va) else {
			return;
		};
		if let Ok(index) = vrefs.binary_search(&target_rva) {
			xrefs.push(index);
		}
	});
	xrefs.sort_unstable();
	xrefs.dedup();

	let mut types = Vec::new();
	for xref in xrefs {
		let _ = add_vtable(file, &mut types, xref, &vrefs);
	}
	types.sort_by_key(|item| item.name);
	for item in &mut types {
		item.vtables.sort_by_key(|vtable| vtable.col.offset);
	}
	types.into_iter().map(|item| render_type(file, item)).collect()
}

fn add_vtable<'a>(file: PeFile<'a>, types: &mut Vec<RawType<'a>>, xref: usize, vrefs: &[Rva]) -> pelite::Result<()> {
	let Some(&vtable_rva) = vrefs.get(xref) else {
		return Err(pelite::Error::Bounds);
	};
	let Some(locator_rva) = vtable_rva.checked_sub(4) else {
		return Err(pelite::Error::Bounds);
	};
	let col_ptr: Ptr<image::RTTICompleteObjectLocator> = Ptr::from(*file.derva::<Va>(locator_rva)?);
	let col = file.deref(col_ptr)?;
	let index = if let Some(index) = types.iter().position(|item| item.type_ptr == col.type_descriptor && item.class_ptr == col.class_descriptor) {
		index
	}
	else {
		let name = file.deref_c_str(col.type_descriptor.offset(8))?.to_str()?;
		let class = file.deref(col.class_descriptor)?;
		types.push(RawType {
			type_ptr: col.type_descriptor,
			class_ptr: col.class_descriptor,
			name,
			class,
			vtables: Vec::new(),
		});
		types.len() - 1
	};
	let mut methods = 1;
	while xref + methods < vrefs.len() && vrefs[xref + methods - 1].checked_add(4) == Some(vrefs[xref + methods]) {
		methods += 1;
	}
	types[index].vtables.push(RawVTable { col, methods, rva: vtable_rva });
	Ok(())
}

fn render_type(file: PeFile<'_>, item: RawType<'_>) -> Result<TypeOutput> {
	let base_classes = file.deref_slice(item.class.base_class_array, item.class.num_base_classes as usize)?;
	let mut vtables = Vec::new();
	for vtable in item.vtables {
		let mut for_type = None;
		for &base_ptr in base_classes {
			let base = file.deref(base_ptr)?;
			if base.pmd.mdisp == vtable.col.offset as i32 {
				for_type = Some(file.deref_c_str(base.type_descriptor.offset(8))?.to_str()?.to_owned());
				break;
			}
		}
		vtables.push(VTableOutput {
			rva: vtable.rva,
			for_type,
			methods: vtable.methods,
		});
	}

	let mut hierarchy = Vec::new();
	let mut stack = vec![item.class.num_base_classes];
	for &base_ptr in base_classes {
		let base = file.deref(base_ptr)?;
		let depth = stack.len() - 1;
		let virtual_base = base.pmd.pdisp != -1;
		hierarchy.push(BaseClassOutput {
			depth,
			offset: if virtual_base { None } else { Some(base.pmd.mdisp) },
			virtual_base,
			name: file.deref_c_str(base.type_descriptor.offset(8))?.to_str()?.to_owned(),
		});
		let consumed = base.num_contained_bases.saturating_add(1);
		if let Some(remaining) = stack.last_mut() {
			*remaining = remaining.saturating_sub(consumed);
		}
		if base.num_contained_bases != 0 {
			stack.push(base.num_contained_bases);
		}
		else {
			while stack.len() > 1 && stack.last() == Some(&0) {
				stack.pop();
			}
		}
	}
	let inheritance = inheritance_kind(item.class.attributes, hierarchy.iter().any(|base| base.virtual_base));
	Ok(TypeOutput {
		name: item.name.to_owned(),
		inheritance,
		vtables,
		hierarchy,
	})
}
