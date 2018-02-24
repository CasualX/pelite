extern crate winapi;

use super::Protect;

use std::{io, mem, ptr, slice};
use std::ffi::OsStr;
use std::path::Path;
use std::ops::Range;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::io::{AsRawHandle, RawHandle};

use self::winapi::um::fileapi::{CreateFileW, GetFileSizeEx, OPEN_EXISTING};
use self::winapi::um::memoryapi::{
	CreateFileMappingW, MapViewOfFile, UnmapViewOfFile, VirtualProtect,
	FILE_MAP_READ, FILE_MAP_COPY,
};
use self::winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use self::winapi::shared::ntdef::{NULL, HANDLE};
use self::winapi::shared::minwindef::{FALSE, LPVOID};
use self::winapi::um::winnt::{
	PAGE_READONLY, PAGE_READWRITE, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE,
	SEC_IMAGE, GENERIC_READ, FILE_SHARE_READ, FILE_ATTRIBUTE_NORMAL,
};

impl Protect {
	fn to_os_protect(self) -> u32 {
		const VALUES: [u8; 4] = [
			PAGE_READONLY as u8,
			PAGE_READWRITE as u8,
			PAGE_EXECUTE_READ as u8,
			PAGE_EXECUTE_READWRITE as u8,
		];
		VALUES[self as u8 as usize] as u32
	}
}

//----------------------------------------------------------------

/// Memory-mapped image.
pub struct ImageMap {
	handle: HANDLE,
	bytes: *mut [u8],
}
impl ImageMap {
	/// Maps the executable image into memory with correctly aligned sections.
	pub fn open<P: AsRef<Path> + ?Sized>(path: &P) -> io::Result<ImageMap> {
		unsafe { Self::_open(path.as_ref()) }
	}
	unsafe fn _open(path: &Path) -> io::Result<ImageMap> {
		// Get its file handle
		let file = {
			// Get the path as a nul terminated wide string
			let path: &OsStr = path.as_ref();
			let mut wpath: Vec<u16> = path.encode_wide().collect();
			wpath.push(0);
			CreateFileW(wpath.as_ptr(), GENERIC_READ, FILE_SHARE_READ, ptr::null_mut(), OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL)
		};
		if file != INVALID_HANDLE_VALUE {
			// Create the image file mapping, `SEC_IMAGE` does its magic thing
			let map = CreateFileMappingW(file, ptr::null_mut(), PAGE_READONLY | SEC_IMAGE, 0, 0, ptr::null());
			CloseHandle(file);
			if map != NULL {
				// Map view of the file
				let view = MapViewOfFile(map, FILE_MAP_COPY, 0, 0, 0);
				if view != ptr::null_mut() {
					// Trust the OS with correctly mapping the image.
					// Trust me to have read and understood the documentation.
					// There is no validation and 64bit headers are used because the offsets are the same for PE32.
					use image::{IMAGE_DOS_HEADER, IMAGE_NT_HEADERS64};
					let dos_header = view as *const IMAGE_DOS_HEADER;
					let nt_header = (view as usize + (*dos_header).e_lfanew as usize) as *const IMAGE_NT_HEADERS64;
					let size_of = (*nt_header).OptionalHeader.SizeOfImage;
					let bytes = slice::from_raw_parts_mut(view as *mut u8, size_of as usize);
					return Ok(ImageMap { handle: map, bytes });
				}
				let err = io::Error::last_os_error();
				CloseHandle(map);
				return Err(err);
			}
		}
		Err(io::Error::last_os_error())
	}
	// Change the memory protection of a range of this image.
	pub fn protect(&self, address: Range<u32>, protect: Protect) -> bool {
		unsafe {
			let bytes = &(*self.bytes)[address.start as usize..address.end as usize];
			let mut old_protect = 0;
			VirtualProtect(bytes.as_ptr() as LPVOID, bytes.len(), protect.to_os_protect(), &mut old_protect) != 0
		}
	}
}
impl AsRawHandle for ImageMap {
	fn as_raw_handle(&self) -> RawHandle {
		self.handle as RawHandle
	}
}
impl AsRef<[u8]> for ImageMap {
	fn as_ref(&self) -> &[u8] {
		unsafe { &*self.bytes }
	}
}
impl AsMut<[u8]> for ImageMap {
	fn as_mut(&mut self) -> &mut [u8] {
		unsafe { &mut *self.bytes }
	}
}
impl Drop for ImageMap {
	fn drop(&mut self) {
		unsafe {
			UnmapViewOfFile((*self.bytes).as_ptr() as LPVOID);
			CloseHandle(self.handle);
		}
	}
}

//----------------------------------------------------------------

/// Memory-mapped file.
pub struct FileMap {
	handle: HANDLE,
	bytes: *mut [u8],
}
impl FileMap {
	/// Maps the whole file into memory.
	pub fn open<P: AsRef<Path> + ?Sized>(path: &P) -> io::Result<FileMap> {
		unsafe { Self::_open(path.as_ref()) }
	}
	unsafe fn _open(path: &Path) -> io::Result<FileMap> {
		// Get its file handle
		let file = {
			// Get the path as a nul terminated wide string
			let path: &OsStr = path.as_ref();
			let mut wpath: Vec<u16> = path.encode_wide().collect();
			wpath.push(0);
			CreateFileW(wpath.as_ptr(), GENERIC_READ, FILE_SHARE_READ, ptr::null_mut(), OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL)
		};
		if file == INVALID_HANDLE_VALUE {
			return Err(io::Error::last_os_error());
		}
		// Get the file size as we'll be mapping it wholesome
		let mut file_size = mem::uninitialized();
		let e = GetFileSizeEx(file, &mut file_size);
		let size = mem::transmute::<_, u64>(file_size) as usize;
		if e == FALSE {
			let err = io::Error::last_os_error();
			CloseHandle(file);
			return Err(err);
		}
		// Create the memory file mapping
		let map = CreateFileMappingW(file, ptr::null_mut(), PAGE_READONLY, 0, 0, ptr::null());
		CloseHandle(file);
		if map == NULL {
			return Err(io::Error::last_os_error());
		}
		// Map view of the file
		let view = MapViewOfFile(map, FILE_MAP_READ, 0, 0, 0);
		if view == ptr::null_mut() {
			let err = io::Error::last_os_error();
			CloseHandle(map);
			return Err(err);
		}
		let bytes = slice::from_raw_parts_mut(view as *mut u8, size);
		Ok(FileMap { handle: map, bytes })
	}
}
impl AsRawHandle for FileMap {
	fn as_raw_handle(&self) -> RawHandle {
		self.handle as RawHandle
	}
}
impl AsRef<[u8]> for FileMap {
	fn as_ref(&self) -> &[u8] {
		unsafe { &*self.bytes }
	}
}
impl AsMut<[u8]> for FileMap {
	fn as_mut(&mut self) -> &mut [u8] {
		unsafe { &mut *self.bytes }
	}
}
impl Drop for FileMap {
	fn drop(&mut self) {
		unsafe {
			UnmapViewOfFile((*self.bytes).as_ptr() as LPVOID);
			CloseHandle(self.handle);
		}
	}
}
