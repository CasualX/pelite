use pelite::pe64::{image, PeFile, Rva};

use super::*;

struct RawVTable<'a> {
	col: &'a image::RTTICompleteObjectLocator,
	methods: usize,
	rva: Rva,
}

struct RawType<'a> {
	name: &'a str,
	class: &'a image::RTTIClassHierarchyDescriptor,
	vtables: Vec<RawVTable<'a>>,
}

pub fn analyze(file: PeFile<'_>) -> Result<Vec<TypeOutput>> {
	let text = file.section_headers().by_name(".text").ok_or_else(|| err("no .text section found"))?;
	let rdata = file.section_headers().by_name(".rdata").ok_or_else(|| err("no .rdata section found"))?;
	let relocs = file.base_relocs().map_err(|_| err("no base relocations found"))?;

	let mut method_refs = Vec::new();
	let mut locator_refs = Vec::new();
	relocs.for_each(|rva, _| {
		if !in_section(rdata, rva) {
			return;
		}
		let Ok(target_va) = file.derva_copy::<u64>(rva, false) else {
			return;
		};
		let Ok(target_rva) = file.va_to_rva(target_va) else {
			return;
		};
		if in_section(text, target_rva) {
			method_refs.push(rva);
		}
		else if in_section(rdata, target_rva) {
			locator_refs.push((rva, target_rva));
		}
	});
	method_refs.sort_unstable();
	method_refs.dedup();
	locator_refs.sort_unstable();
	locator_refs.dedup();

	let mut types = Vec::new();
	let mut type_indices = HashMap::new();
	for (locator_ref_rva, locator_rva) in locator_refs {
		let Some(vtable_rva) = locator_ref_rva.checked_add(8) else {
			continue;
		};
		let Ok(method_index) = method_refs.binary_search(&vtable_rva) else {
			continue;
		};
		let Ok(col) = file.derva::<image::RTTICompleteObjectLocator>(locator_rva) else {
			continue;
		};
		// The self RVA distinguishes a version 1 locator from other data that
		// happens to be referenced immediately before a code-pointer array.
		if col.signature != 1 || col.self_rva != locator_rva {
			continue;
		}
		let key = (col.type_descriptor, col.class_descriptor);
		let index = if let Some(&index) = type_indices.get(&key) {
			index
		}
		else {
			let Ok(name) = type_name(file, col.type_descriptor) else {
				continue;
			};
			if !name.starts_with(".?A") {
				continue;
			}
			let Ok(class) = file.derva::<image::RTTIClassHierarchyDescriptor>(col.class_descriptor) else {
				continue;
			};
			if class.signature != 0 || class.num_base_classes == 0 {
				continue;
			}
			let index = types.len();
			types.push(RawType { name, class, vtables: Vec::new() });
			type_indices.insert(key, index);
			index
		};
		let mut methods = 1;
		while let Some(next_rva) = vtable_rva.checked_add((methods as u32).saturating_mul(8)) {
			if method_refs.get(method_index + methods) != Some(&next_rva) {
				break;
			}
			methods += 1;
		}
		types[index].vtables.push(RawVTable { col, methods, rva: vtable_rva });
	}

	let mut output = Vec::new();
	for mut item in types {
		item.vtables.sort_by_key(|vtable| vtable.col.offset);
		if let Ok(item) = render_type(file, item) {
			output.push(item);
		}
	}
	output.sort_by(|left, right| left.name.cmp(&right.name));
	Ok(output)
}

fn in_section(section: &image::IMAGE_SECTION_HEADER, rva: Rva) -> bool {
	rva >= section.VirtualAddress && rva < section.VirtualAddress.saturating_add(section.VirtualSize)
}

fn type_name(file: PeFile<'_>, descriptor_rva: Rva) -> pelite::Result<&str> {
	let name_rva = descriptor_rva.checked_add(16).ok_or(pelite::Error::Overflow)?;
	Ok(file.derva_c_str(name_rva)?.to_str()?)
}

fn render_type(file: PeFile<'_>, item: RawType<'_>) -> pelite::Result<TypeOutput> {
	let base_rvas = file.derva_slice::<Rva>(item.class.base_class_array, item.class.num_base_classes as usize)?;
	let mut bases = Vec::with_capacity(base_rvas.len());
	for &base_rva in base_rvas {
		bases.push(file.derva::<image::RTTIBaseClassDescriptor>(base_rva)?);
	}
	let inheritance = inheritance_kind(item.class.attributes, bases.iter().any(|base| base.pmd.pdisp != -1));

	let mut vtables = Vec::new();
	for vtable in item.vtables {
		let mut for_type = None;
		for base in &bases {
			if base.pmd.mdisp == vtable.col.offset as i32 {
				for_type = Some(type_name(file, base.type_descriptor)?.to_owned());
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
	for base in bases {
		let depth = stack.len() - 1;
		let virtual_base = base.pmd.pdisp != -1;
		hierarchy.push(BaseClassOutput {
			depth,
			offset: if virtual_base { None } else { Some(base.pmd.mdisp) },
			virtual_base,
			name: type_name(file, base.type_descriptor)?.to_owned(),
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
	Ok(TypeOutput {
		name: item.name.to_owned(),
		inheritance,
		vtables,
		hierarchy,
	})
}
