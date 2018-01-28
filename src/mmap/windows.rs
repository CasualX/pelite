
use std::{io, mem, ptr, slice};
use std::ffi::OsStr;
use std::path::Path;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::io::RawHandle;
use std::os::raw::c_void;

const INVALID_HANDLE_VALUE: RawHandle = !0 as RawHandle;
const NULL: RawHandle = 0 as RawHandle;
const FALSE: i32 = 0;

extern "system" {
	fn CreateFileW(
		lpFileName: *const u16,
		dwDesiredAccess: u32,
		dwShareMode: u32,
		lpSecurityAttributes: *const c_void,
		dwCreationDisposition: u32,
		dwFlagsAndAttributes: u32,
		hTemplateFile: RawHandle,
	) -> RawHandle;
	fn CreateFileMappingW(
		hFile: RawHandle,
		lpAttributes: *const c_void,
		flProtect: u32,
		dwMaximumSizeHigh: u32,
		dwMaximumSizeLow: u32,
		lpName: *const u16,
	) -> RawHandle;
	fn MapViewOfFile(
		hFileMappingObject: RawHandle,
		dwDesiredAccess: u32,
		dwFileOffsetHigh: u32,
		dwFileOffsetLow: u32,
		dwNumberOfBytesToMap: usize,
	) -> *const c_void;
	fn UnmapViewOfFile(
		lpBaseAddress: *const c_void,
	) -> i32;
	fn GetFileSizeEx(
		hFile: RawHandle,
		lpFileSize: *mut u64,
	) -> i32;
	fn CloseHandle(
		hObject: RawHandle,
	) -> i32;
	fn VirtualAlloc(
		lpAddress: *const c_void,
		dwSize: usize,
		flAllocationType: u32,
		flProtect: u32,
	) -> *mut c_void;
	fn VirtualProtect(
		lpAddress: *const c_void,
		dwSize: usize,
		flNewProtect: u32,
		lpflOldProtect: *mut u32,
	) -> i32;
	fn LoadLibraryW(
		lpFileName: *const u16,
	) -> *const c_void;
	fn TlsAlloc() -> u32;
	// fn TlsSetValue(
	// 	dwTlsIndex: u32,
	// 	lpTlsValue: *const c_void,
	// ) -> i32;
	fn GetLastError() -> i32;
	fn GetModuleHandleW(
		lpModuleName: *const u16,
	) -> *const c_void;
	fn GetProcAddress(
		hModule: *const c_void,
		lpProcName: *const u8,
	) -> *const c_void;
}

macro_rules! close_handle {
	($e:expr) => {
		let e = CloseHandle($e);
		debug_assert!(e != FALSE, "CloseHandle failed with {:?}", io::Error::last_os_error());
	}
}

//----------------------------------------------------------------

/// Memory-mapped image.
pub struct ImageMap {
	map: RawHandle,
	view: *const c_void,
}
impl ImageMap {
	/// Maps the executable image into memory with correctly aligned sections.
	pub fn open<P: AsRef<Path> + ?Sized>(path: &P) -> io::Result<ImageMap> {
		Self::_open(path.as_ref())
	}
	fn _open(path: &Path) -> io::Result<ImageMap> { unsafe {
		// Get its file handle
		let file = {
			// Get the path as a nul terminated wide string
			let path: &OsStr = path.as_ref();
			let mut wpath: Vec<u16> = path.encode_wide().collect();
			wpath.push(0);
			CreateFileW(wpath.as_ptr(), /*GENERIC_READ*/0x80000000, /*FILE_SHARE_READ*/0x00000001, ptr::null(), /*OPEN_EXISTING*/3, /*FILE_ATTRIBUTE_NORMAL*/0x00000080, NULL)
		};
		if file != INVALID_HANDLE_VALUE {
			// Create the image file mapping, `SEC_IMAGE` does its magic thing
			let map = CreateFileMappingW(file, ptr::null(), /*PAGE_READONLY*/0x02 | /*SEC_IMAGE*/0x1000000, 0, 0, ptr::null());
			close_handle!(file);
			if map != NULL {
				// Map view of the file
				let view = MapViewOfFile(map, /*FILE_MAP_READ*/0x0004, 0, 0, 0);
				if view != ptr::null() {
					// All good! Trust the OS with correctly mapping the image...
					return Ok(ImageMap { map, view });
				}
				let err = io::Error::last_os_error();
				close_handle!(map);
				return Err(err);
			}
		}
		Err(io::Error::last_os_error())
	}}
}
impl AsRef<[u8]> for ImageMap {
	fn as_ref(&self) -> &[u8] {
		unimplemented!()
	}
}
impl Drop for ImageMap {
	fn drop(&mut self) { unsafe {
		UnmapViewOfFile(self.view);
		close_handle!(self.map);
	}}
}

//----------------------------------------------------------------

/// Memory-mapped file.
pub struct FileMap {
	map: RawHandle,
	view: *const c_void,
	size: usize,
}
impl FileMap {
	/// Maps the whole file into memory.
	pub fn open<P: AsRef<Path> + ?Sized>(path: &P) -> io::Result<FileMap> {
		Self::_open(path.as_ref())
	}
	fn _open(path: &Path) -> io::Result<FileMap> { unsafe {
		// Get its file handle
		let file = {
			// Get the path as a nul terminated wide string
			let path: &OsStr = path.as_ref();
			let mut wpath: Vec<u16> = path.encode_wide().collect();
			wpath.push(0);
			CreateFileW(wpath.as_ptr(), /*GENERIC_READ*/0x80000000, /*FILE_SHARE_READ*/0x00000001, ptr::null(), /*OPEN_EXISTING*/3, /*FILE_ATTRIBUTE_NORMAL*/0x00000080, NULL)
		};
		if file == INVALID_HANDLE_VALUE {
			return Err(io::Error::last_os_error());
		}
		// Get the file size as we'll be mapping it wholesome
		let mut file_size = 0u64;
		let e = GetFileSizeEx(file, &mut file_size);
		let size = file_size as usize;
		if e == FALSE {
			let err = io::Error::last_os_error();
			close_handle!(file);
			return Err(err);
		}
		// Create the memory file mapping
		let map = CreateFileMappingW(file, ptr::null(), /*PAGE_READONLY*/0x02, 0, 0, ptr::null());
		close_handle!(file);
		if map == NULL {
			return Err(io::Error::last_os_error());
		}
		// Map view of the file
		let view = MapViewOfFile(map, /*FILE_MAP_READ*/0x0004, 0, 0, 0);
		if view == ptr::null() {
			let err = io::Error::last_os_error();
			close_handle!(map);
			return Err(err);
		}
		Ok(FileMap { map, view, size })
	}}
}
impl AsRef<[u8]> for FileMap {
	fn as_ref(&self) -> &[u8] { unsafe {
		slice::from_raw_parts(self.view as *const u8, self.size)
	}}
}
impl Drop for FileMap {
	fn drop(&mut self) { unsafe {
		UnmapViewOfFile(self.view);
		close_handle!(self.map);
	}}
}

//----------------------------------------------------------------

use pe::{Align, Pe, PeView, Va};
use pe::imports::Desc;
use pe::exports::Export;
use util::CStr;
use Error as PeError;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MMError {
	Alloc(i32),
	Rebase(PeError),
	DllName(PeError),
	LoadLib(i32),
	Imports(PeError),
	Exports(PeError),
}

#[derive(Copy, Clone)]
pub struct ManualMap<P> {
	pe: P,
}
impl<'a, P: Pe<'a> + Copy> ManualMap<P> {
	pub(crate) fn new(pe: P) -> ManualMap<P> {
		ManualMap { pe }
	}
	/// Manually maps the given PE image without going through the system's LoadLibrary.
	///
	/// Dependencies are loaded through LoadLibrary. When loading fails the dependencies are not cleaned up.
	///
	/// # Safety
	///
	/// The assumption is that if this function is called, the input PE image is trusted as it's intended to executed code from.
	/// Therefore the bare minimum is validated and the rest is assumed to be correct. If this is not the case, here be dragons!
	///
	/// Errors from eg. system calls are still checked.
	pub unsafe fn load(self) -> Result<*mut u8, MMError> {
		let v = mm_alloc(self.pe)?;
		mm_copy(self.pe, v);
		mm_rebase(self.pe, v)?;
		mm_deps(self.pe, v)?;
		mm_safeseh(self.pe, v);
		mm_tls(self.pe, v);
		mm_protect(self.pe, v);
		Ok(v)
	}
}


/// Allocates enough virtual memory to map this PE image.
pub unsafe fn mm_alloc<'a, P: Pe<'a> + Copy>(pe: P) -> Result<*mut u8, MMError> {
	let image_size = pe.optional_header().SizeOfImage as usize;
	let vbase = VirtualAlloc(ptr::null_mut(), image_size, /*MEM_COMMIT|MEM_RESERVE*/0x00003000, /*PAGE_READWRITE*/0x04);
	if vbase == NULL {
		Err(MMError::Alloc(GetLastError()))
	}
	else {
		Ok(vbase as *mut u8)
	}
}

/// Copies the headers and raw section data from to the destination image.
unsafe fn mm_copy<'a, P: Pe<'a> + Copy>(pe: P, image: *mut u8) {
	let src = pe.image().as_ptr();

	// Write PE header
	let size_of_headers = pe.optional_header().SizeOfHeaders as usize;
	ptr::copy_nonoverlapping(src, image, size_of_headers);

	for section in pe.section_headers() {
		// Get the src pointer depending on src alignment
		let pointer = match pe.align() {
			Align::File => section.PointerToRawData as usize,
			Align::Section => section.VirtualAddress as usize,
		};
		// Write section data
		ptr::copy_nonoverlapping(
			src.offset(pointer as isize),
			image.offset(section.VirtualAddress as isize),
			section.SizeOfRawData as usize
		);
	}
}

/// Rebase the image.
unsafe fn mm_rebase<'a, P: Pe<'a> + Copy>(pe: P, image: *mut u8) -> Result<(), MMError> {
	mm_rebase_ex(pe, image, image as usize)
}
/// Rebase the image to the given virtual base address.
unsafe fn mm_rebase_ex<'a, P: Pe<'a> + Copy>(pe: P, image: *mut u8, vbase: usize) -> Result<(), MMError> {
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
		let base_relocs = pe.base_relocs().map_err(|err| MMError::Rebase(err))?;
		for rva in base_relocs.into_iter().flat_map(|relocs| relocs) {
			let p = image.offset(rva as isize) as *mut usize;
			let fixed_addr = ptr::read_unaligned(p).wrapping_add(delta);
			ptr::write_unaligned(p, fixed_addr);
		}
	}
	Ok(())
}

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

#[cfg(all(nightly, target_arch = "x86_64"))]
unsafe fn mm_set_tls(index: u32, data: *const c_void) {
	let index = index as u64;
	asm!("
		mov rax, gs:58h
		mov qword ptr [rax + $0*8], $1"
		:
		: "r"(index), "r"(data)
		: "rax"
		: "intel");
}
#[cfg(all(nightly, target_arch = "x86"))]
unsafe fn mm_set_tls(index: u32, data: *const c_void) {
	asm!("
		mov eax, gs:2Ch
		mov dword ptr [eax + $0*4], $1"
		:
		: "r"(index), "r"(data)
		: "eax"
		: "intel");
}
#[cfg(not(all(nightly, any(target_arch = "x86", target_arch = "x86_64")))]
unsafe fn mm_set_tls(index: u32, data: *const c_void) {
	unimplemented!()
}

/// Apply page protections to the image sections.
unsafe fn mm_protect<'a, P: Pe<'a> + Copy>(pe: P, image: *mut u8) -> bool {
	let mut old_protect = mem::uninitialized();

	// Mark the headers read-only
	let size_of_headers = pe.optional_header().SizeOfHeaders as usize;
	let mut success = VirtualProtect(image as *const c_void, size_of_headers, /*PAGE_READONLY*/0x02, &mut old_protect) != 0;

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
		let address = image.offset(section.VirtualAddress as isize) as *const c_void;
		let size = section.VirtualSize as usize;
		success &= VirtualProtect(address, size, protect, &mut old_protect) != 0;
	}
	return success;
}
