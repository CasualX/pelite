/*!
*/

use std::{mem, ptr, slice};

use winapi::um::memoryapi::{VirtualAlloc, VirtualProtect};
use winapi::um::errhandlingapi::GetLastError;
use winapi::shared::minwindef::{LPVOID};
use winapi::um::winnt::{PAGE_READONLY, PAGE_READWRITE, MEM_COMMIT, MEM_RESERVE};

use super::{Align, Pe};

/// Allocates enough virtual memory to map this PE image.
pub unsafe fn mm_alloc<'a, P: Pe<'a> + Copy>(pe: P) -> Result<*mut [u8], u32> {
	let image_size = pe.optional_header().SizeOfImage as usize;
	let vbase = VirtualAlloc(ptr::null_mut(), image_size, MEM_COMMIT|MEM_RESERVE, PAGE_READWRITE);
	if vbase == ptr::null_mut() {
		Err(GetLastError())
	}
	else {
		let image = slice::from_raw_parts_mut(vbase as *mut u8, image_size);
		Ok(image)
	}
}

/// Copies the headers and raw section data from to the destination image.
pub unsafe fn mm_copy<'a, P: Pe<'a> + Copy>(pe: P, image: *mut [u8]) {
	let src = pe.image().as_ptr();
	let image_ptr = (*image).as_mut_ptr();

	// Write PE header
	let size_of_headers = pe.optional_header().SizeOfHeaders as usize;
	ptr::copy_nonoverlapping(src, image_ptr, size_of_headers);

	for section in pe.section_headers() {
		// Get the src pointer depending on src alignment
		let pointer = match pe.align() {
			Align::File => section.PointerToRawData as usize,
			Align::Section => section.VirtualAddress as usize,
		};
		// Write section data
		ptr::copy_nonoverlapping(
			src.offset(pointer as isize),
			image_ptr.offset(section.VirtualAddress as isize),
			section.SizeOfRawData as usize
		);
	}
}

/// Rebase the image.
pub unsafe fn mm_rebase<'a, P: Pe<'a> + Copy>(pe: P, image: *mut [u8]) {
	mm_rebase_ex(pe, image, (*image).as_mut_ptr() as usize)
}
/// Rebase the image to the given virtual base address.
pub unsafe fn mm_rebase_ex<'a, P: Pe<'a> + Copy>(pe: P, image: *mut [u8], vbase: usize) {
	let image = (*image).as_mut_ptr();

	// Offset all absolute pointers by this delta to correct them from the old ImageBase to the new vbase
	let delta = {
		let image_base = pe.optional_header().ImageBase as usize;
		vbase.wrapping_sub(image_base)
	};

	// If the module is loaded at its preferred base address then no relocation is necessary
	if delta != 0 {
		// Write the new vbase to the optional header's ImageBase
		// I don't know where this behavior is documented, but it's definitely a thing
		let offset_of_image_base = &pe.optional_header().ImageBase as *const _ as usize - pe.image().as_ptr() as usize;
		*(image.offset(offset_of_image_base as isize) as *mut usize) = vbase;

		// Correct all base relocations by this delta
		if let Ok(base_relocs) = pe.base_relocs() {
			for rva in base_relocs.into_iter().flat_map(|relocs| relocs) {
				let p = image.offset(rva as isize) as *mut usize;
				let fixed_addr = ptr::read_unaligned(p).wrapping_add(delta);
				ptr::write_unaligned(p, fixed_addr);
			}
		}
	}
}

/*
/// Resolve dependencies by loading them with LoadLibrary.
unsafe fn mm_deps<'a, P: Pe<'a> + Copy>(pe: P, image: *mut u8) -> Result<(), MMError> {
	if let Ok(imports) = pe.imports() {
		// Resolve all dependent modules
		for desc in imports {
			// Load dependencies through bog-standard LoadLibrary
			let dll_name = desc.dll_name().map_err(|err| MMError::DllName(err))?.to_os_str();
			let hmod = {
				let wide_name: Vec<u16> = dll_name.encode_wide().collect();
				LoadLibraryW(wide_name.as_ptr()) as *const u8
			};
			if hmod.is_null() {
				return Err(MMError::LoadLib(GetLastError()));
			}
			// Fill in the imports from this loaded module
			let view = PeView::module(hmod);
			mm_deps_import(&desc, image, view)?;
		}
	}
	Ok(())
}

/// Resolve the imports for a specific dependency.
unsafe fn mm_deps_import<'a, 'b, P, Q>(desc: &Desc<'a, P>, image: *mut u8, dep: Q) -> Result<(), MMError>
	where P: Pe<'a> + Copy, Q: Pe<'b> + Copy
{
	// Grab the import name table for the desired imports and the export table from the dependency
	let int = desc.int().map_err(|err| MMError::Imports(err))?;
	let exp_by = dep.exports().and_then(|exp| exp.by()).map_err(|err| MMError::Exports(err))?;

	// Grab the IAT to write the pointers to
	let iat_ptr = image.offset(desc.image().FirstThunk as isize) as *mut Va;
	let iat_len = int.as_slice().len();
	let iat = slice::from_raw_parts_mut(iat_ptr, iat_len);

	// Resolve the imported functions one by one
	for (imp_result, dest) in int.zip(iat) {
		// Read the imported function description
		// This shouldn't ever fail really, unless your PE is really corrupt...
		let imp = match imp_result {
			Ok(imp) => imp,
			Err(_) => continue,
		};

		// Lookup the exported function for the import
		let func = match exp_by.import(imp) {
			Ok(Export::Symbol(&rva)) => dep.rva_to_va(rva).unwrap(),
			Ok(Export::Forward(name)) => mm_deps_import_fwd(name),
			Ok(Export::None) => 0,
			// The export directory is corrupt or something...
			Err(_) => continue,
		};
		// And write the exported VA to the IAT
		*dest = func;
	}
	Ok(())
}
unsafe fn mm_deps_import_fwd(name: &CStr) -> Va {
	// Split the name in the module name (before the first `.`) and the import name (after the first `.`)
	// Just abort and return zero if no `.` was found...
	let index = match name.as_ref().iter().enumerate().find(|&(_i, &byte)| byte == b'.') {
		Some((i, _)) => i + 1,
		None => return 0,
	};
	// Get or load the forwarded module
	let hmod = {
		// Extract the module name as an `OsStr` including the `.` separator
		let module_name_u8: &OsStr = mem::transmute(&name.as_ref()[..index]);
		// Encode it in a wide string and overwrite the `.` separator to a null terminator
		let mut module_name: Vec<u16> = module_name_u8.encode_wide().collect();
		module_name[index - 1] = 0;
		// First attempt to just get a handle to the module
		let mut module_handle = GetModuleHandleW(module_name.as_ptr());
		// If that fails, try to load it instead
		if module_handle.is_null() {
			module_handle = LoadLibraryW(module_name.as_ptr())
		}
		module_handle
	};
	// Give up if still not found
	if hmod.is_null() {
		return 0;
	}
	// Just dispatch to the OS to query the forwarded export
	GetProcAddress(hmod, name[index..].as_ptr()) as Va
}

/// Registers the image for exception handling.
unsafe fn mm_safeseh<'a, P: Pe<'a> + Copy>(pe: P, image: *mut u8) {
	// For x64 use RtlAddFunctionTable and RtlDeleteFunctionTable
	// For x86 use the non exported RtlInsertInvertedFunctionTable and RtlRemoveInvertedFunctionTable
}

/// Initialize TLS support if needed.
unsafe fn mm_tls<'a, P: Pe<'a> + Copy>(pe: P, image: *mut u8) {
	// If this library requires TLS support
	if let Ok(tls) = pe.tls() {
		// Start by allocating a TLS slot
		let rva = pe.va_to_rva(tls.image().AddressOfIndex).unwrap();
		let pindex = image.offset(rva as isize) as *mut u32;
		*pindex = TlsAlloc();
		// Initialize the thread locals for this thread
		// FIXME! Is this correct?
		//        MSDN says you cannot use implicit __threadlocals in a DLL loaded with LoadLibrary...
		let raw_data = tls.raw_data().unwrap();
		let tls_data = VirtualAlloc(ptr::null_mut(), raw_data.len(), /*MEM_COMMIT|MEM_RESERVE*/0x00003000, /*PAGE_READWRITE*/0x04);
		ptr::copy_nonoverlapping(
			raw_data.as_ptr(),
			tls_data as *mut u8,
			raw_data.len(),
		);
		mm_set_tls(*pindex, tls_data);
	}
}
#[cfg(target_arch = "x86_64")]
pub unsafe fn mm_set_tls(index: u32, data: *const c_void) {
	let index = index as u64;
	asm!("
		mov rax, gs:58h
		mov qword ptr [rax + $0*8], $1"
		:
		: "r"(index), "r"(data)
		: "rax"
		: "intel");
}
#[cfg(target_arch = "x86")]
pub unsafe fn mm_set_tls(index: u32, data: *const c_void) {
	asm!("
		mov eax, fs:2Ch
		mov dword ptr [eax + $0*4], $1"
		:
		: "r"(index), "r"(data)
		: "eax"
		: "intel");
}
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub unsafe fn mm_set_tls(index: u32, data: *const c_void) {
	unimplemented!()
}
*/

/// Apply page protections to the image sections.
pub unsafe fn mm_protect<'a, P: Pe<'a> + Copy>(pe: P, image: *mut [u8]) -> bool {
	let image = (*image).as_mut_ptr();
	let mut old_protect = mem::uninitialized();

	// Mark the headers read-only
	let size_of_headers = pe.optional_header().SizeOfHeaders as usize;
	let mut success = VirtualProtect(image as LPVOID, size_of_headers, PAGE_READONLY, &mut old_protect) != 0;

	// Eight entries, bit 0: eXecute, bit 1: Read, bit 2: Write
	let table = [
		/*PAGE_NOACCESS*/0x01_u8,
		/*PAGE_EXECUTE*/0x10_u8,
		/*PAGE_READONLY*/0x02_u8,
		/*PAGE_EXECUTE_READ*/0x20_u8,
		/*PAGE_READWRITE*/0x04_u8,
		/*PAGE_EXECUTE_READWRITE*/0x40_u8,
		/*PAGE_READWRITE*/0x04_u8,
		/*PAGE_EXECUTE_READWRITE*/0x40_u8,
	];

	// Protect the sections
	for section in pe.section_headers() {
		let index = ((section.Characteristics >> 29) & 0b111) as usize;
		let protect = table[index] as u32;
		let address = image.offset(section.VirtualAddress as isize) as LPVOID;
		let size = section.VirtualSize as usize;
		success &= VirtualProtect(address, size, protect, &mut old_protect) != 0;
	}
	return success;
}
