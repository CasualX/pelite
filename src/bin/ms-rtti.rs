extern crate pelite;

use ::std::env;
use ::std::process::exit;

use pelite::FileMap;
use pelite::pe32::{Pe, PeFile};
use pelite::pe32::image::{Rva, Va};
use pelite::pe32::msvc::*;
use pelite::util::strn;

//----------------------------------------------------------------

struct VTable<'a> {
	col: &'a RTTICompleteObjectLocator,
	vtable: &'a [Va],
	rva: Rva,
}
struct Type<'a> {
	type_rva: Rva,
	class_rva: Rva,
	ty_name: &'a str,
	class_desc: &'a RTTIClassHierarchyDescriptor,
	vtables: Vec<VTable<'a>>,
}

fn main() {
	let mut args = env::args_os();
	if args.len() != 2 {
		println!("Dumps MSVC RTTI from windows PE binaries.");
		exit(0);
	}
	let path = args.nth(1).unwrap();

	let file_map = FileMap::open(&path).expect("ms-rtti: failed to open the given file");
	let file = PeFile::from_bytes(&file_map).expect("ms-rtti: the file isn't a PE32 binary");

	// Find .text and .rdata sections
	let text = file.section_headers().iter().find(|sec| strn(&sec.Name) == b".text").expect("ms-rtti: no `.text` section found");
	let rdata = file.section_headers().iter().find(|sec| strn(&sec.Name) == b".rdata").expect("ms-rtti: no `.rdata` section found");

	// Abuse relocations for xrefs
	let base_relocs = file.base_relocs().expect("ms-rtti: no base relocations found");

	// Vtables are arrays of function pointers stored in rdata
	// This means we're looking for pointers from rdata to text
	// Pointers can be found with base relocations

	// I'm sure this can all be done with more heuristics without requiring all the above informatoin

	// Collect all xrefs from rdata to text
	let mut vrefs: Vec<Rva> = base_relocs.into_iter().flat_map(|relocs| relocs).filter_map(|rva| {
		// Look for xrefs from rdata (the virtual function pointers)
		if rva < rdata.VirtualAddress || rva >= (rdata.VirtualAddress + rdata.VirtualSize) {
			return None;
		}
		// Read the pointer being relocated
		let &target_va = file.derva::<Va>(rva).expect(&format!("ms-rtti: corrupt reloc at {:08X}", rva));
		let target_rva = file.va_to_rva(target_va).expect(&format!("ms-rtti: corrupt xref at {:08X}", rva));
		// Look for xrefs to text (the virtual functions themselves)
		if target_rva < text.VirtualAddress || target_rva >= (text.VirtualAddress + text.VirtualSize) {
			return None;
		}
		Some(rva)
	}).collect();

	// The vtable length will be detected by a 'run' of relocated pointers so ensure they are sorted
	vrefs.sort();

	// The vtables themselves will be referenced from other places (such as the RTTI data, constructors and dynamic_casts)
	// By collecting all xrefs to the previously collected pointers, we can find the start of the vtable
	let mut xrefs: Vec<usize> = base_relocs.into_iter().flat_map(|relocs| relocs).filter_map(|rva| {
		// Read the pointer being relocated
		let &target_va = file.derva::<Va>(rva).expect(&format!("ms-rtti: corrupt reloc at {:08X}", rva));
		let target_rva = file.va_to_rva(target_va).expect(&format!("ms-rtti: corrupt xref at {:08X}", rva));
		// Find the target_rva in the coderefs
		vrefs.binary_search(&target_rva).ok()
	}).collect();

	// There may be many references to the same vtable so clean up the data
	xrefs.sort();
	xrefs.dedup();

	// Collect all found types and their vtables
	let mut types = Vec::new();

	// Now `xrefs` contains indices in `vrefs` which are potential vtables
	// Go through them and collect vtables and their types
	for &xref in &xrefs {
		// Try to interpret as a vtable and ignore anything which fails
		let _ = vtable(file, &mut types, xref, &vrefs);
	}

	// The order in which these types was found is not significant
	// To present a better diff between versions, sort the types by their name
	types.sort_by_key(|ty| ty.ty_name);
	for ty in &mut types {
		ty.vtables.sort_by_key(|vtable| vtable.col.offset);
	}

	for ty in &types {
		// Print the class name and attributes
		let _ = print(file, ty);
		// Print its vtables
		for vtable in &ty.vtables {
			let _ = print_vtable(file, ty, vtable);
		}
		// Print the class hierarchy
		let _ = print_class(file, ty);
		// Newline for better clarity
		println!();
	}
}

fn vtable<'a>(file: PeFile<'a>, types: &mut Vec<Type<'a>>, xref: usize, vrefs: &[Rva]) -> pelite::Result<()> {
	// Grab the complete object locator
	let col = file.derva::<RTTICompleteObjectLocator>(file.va_to_rva(*file.derva(vrefs[xref] - 4)?)?)?;
	let type_rva = file.va_to_rva(col.type_descriptor)?;
	let class_rva = file.va_to_rva(col.class_descriptor)?;

	// Look for an existing type or create a new one
	let index = if let Some(index) = types.iter_mut().position(|ty| ty.type_rva == type_rva && ty.class_rva == class_rva) { index }
	else {
		// First time seeing this type, get its descriptors
		let ty_name = file.derva_c_str(type_rva + 8)?.to_str().or(Err(pelite::Error::Insanity))?;
		let class_desc = file.derva(class_rva)?;
		// And add it to the list of types
		types.push(Type { type_rva, class_rva, ty_name, class_desc, vtables: Vec::new() });
		types.len() - 1
	};
	// Unfortunate due to lexical lifetimes...
	let ty = &mut types[index];

	// Find the size of the vtable by counting the consecutive pointers in vrefs
	// This is why the vrefs vector was sorted and deduplicated
	let mut size = 1;
	while xref + size < vrefs.len() && vrefs[xref + size - 1] + 4 == vrefs[xref + size] {
		size += 1;
	}

	// Slice the vtable and add it
	let vtable = file.derva_slice(vrefs[xref], size)?;
	ty.vtables.push(VTable { col, vtable, rva: vrefs[xref] });

	Ok(())
}

fn print<'a>(_file: PeFile<'a>, ty: &Type<'a>) -> pelite::Result<()> {
	let kind = match ty.class_desc.attributes & 3 { 0 => " (SI)", 1 => " (MI)", 2 => " (VI)", 3 => " (MI VI)", _ => unreachable!() };
	println!("class {}{}", ty.ty_name, kind);
	Ok(())
}

fn print_vtable<'a>(file: PeFile<'a>, ty: &Type<'a>, vtable: &VTable<'a>) -> pelite::Result<()> {
	// Lookup the vtable type in the class descriptor
	let base_classes = file.derva_slice::<Va>(file.va_to_rva(ty.class_desc.base_class_array)?, ty.class_desc.num_base_classes as usize)?;
	// There is no direct link between the COL and the name of this particular vtable
	// So try to match its offset in the class itself with the pointer-to-member info in the base class descriptor
	for &base_class_va in base_classes {
		let base_class = file.derva::<RTTIBaseClassDescriptor>(file.va_to_rva(base_class_va)?)?;
		if base_class.pmd.mdisp == vtable.col.offset as i32 {
			let base_ty_name = file.derva_c_str(file.va_to_rva(base_class.type_descriptor)? + 8)?.to_str().or(Err(pelite::Error::Insanity))?;
			println!("{:#010X}: ??_7{}6B@ {{for `{}'}} ({} methods)", vtable.rva, &ty.ty_name[4..], base_ty_name, vtable.vtable.len());
			return Ok(())
		}
	}
	// Offset was not found... Probably VI, What do now?
	println!("{:#010X}: ??_7{}6B@ {{for `?'}} ({} methods)", vtable.rva, &ty.ty_name[4..], vtable.vtable.len());
	Ok(())
}

fn print_class<'a>(file: PeFile<'a>, ty: &Type<'a>) -> pelite::Result<()> {
	// Get the base class array
	let base_classes = file.derva_slice::<Va>(file.va_to_rva(ty.class_desc.base_class_array)?, ty.class_desc.num_base_classes as usize)?;
	// Dump the hierarchy
	let mut margin = [false; 32];
	let mut stack = [0u32; 32];
	stack[0] = ty.class_desc.num_base_classes;
	let mut depth = 0;
	let mut bc_iter = base_classes.iter();
	let mut s = String::new();
	while let Some(&base_class_va) = bc_iter.next() {
		let base_class = file.derva::<RTTIBaseClassDescriptor>(file.va_to_rva(base_class_va)?)?;
		// Mark virtual inheritance as special...
		// I'm not sure how to best display this information
		if base_class.pmd.pdisp != -1 { s += "****: "; }
		else { s += &format!("{:04X}: ", base_class.pmd.mdisp); }
		let is_last = base_class.num_contained_bases + 1 == stack[depth];
		if depth > 0 {
			// Print the margin
			for &is_last in &margin[1..depth] {
				s += if is_last { "    " } else { "|   " };
			}
			// Write the prefix
			let prefix = if is_last { "`-- " } else { "+-- " };
			s += prefix;
		}
		// Get its type name
		let base_ty_rva = file.va_to_rva(base_class.type_descriptor)?;
		// let base_ty = file.derva::<TypeDescriptor>(base_ty_rva)?;
		let base_ty_name = file.derva_c_str(base_ty_rva + 8)?.to_str().or(Err(pelite::Error::Insanity))?;
		s += base_ty_name; s += "\n";
		// Manage the inheritance stack...
		stack[depth] -= base_class.num_contained_bases + 1;
		if base_class.num_contained_bases != 0 {
			margin[depth] = is_last;
			depth += 1;
			stack[depth] = base_class.num_contained_bases;
		}
		else {
			while depth > 0 && stack[depth] == 0 {
				depth -= 1;
			}
		}
	}
	print!("{}", s);
	Ok(())
}
