
use std::{io, ptr, slice};
use std::ffi::OsStr;
use std::path::Path;
use std::ops::Range;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::io::{AsRawHandle, RawHandle};
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
	fn VirtualProtect(
		lpAddress: *const c_void,
		dwSize: usize,
		flNewProtect: u32,
		lpflOldProtect: *mut u32,
	) -> i32;
}

macro_rules! close_handle {
	($e:expr) => {
		let e = CloseHandle($e);
		debug_assert!(e != FALSE, "CloseHandle failed with {:?}", io::Error::last_os_error());
	}
}

/// Memory protection values.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Protect {
	ReadOnly,
	ReadWrite,
	ExecuteRead,
	ExecuteReadWrite,
}
impl Protect {
	fn to_os_protect(self) -> u32 {
		const VALUES: [u8; 4] = [
			/*PAGE_READONLY*/0x02,
			/*PAGE_READWRITE*/0x04,
			/*PAGE_EXECUTE_READ*/0x20,
			/*PAGE_EXECUTE_READWRITE*/0x40,
		];
		VALUES[self as u8 as usize] as u32
	}
}

//----------------------------------------------------------------

/// Memory-mapped image.
pub struct ImageMap {
	handle: RawHandle,
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
			CreateFileW(wpath.as_ptr(), /*GENERIC_READ*/0x80000000, /*FILE_SHARE_READ*/0x00000001, ptr::null(), /*OPEN_EXISTING*/3, /*FILE_ATTRIBUTE_NORMAL*/0x00000080, NULL)
		};
		if file != INVALID_HANDLE_VALUE {
			// Create the image file mapping, `SEC_IMAGE` does its magic thing
			let map = CreateFileMappingW(file, ptr::null(), /*PAGE_READONLY*/0x02 | /*SEC_IMAGE*/0x1000000, 0, 0, ptr::null());
			close_handle!(file);
			if map != NULL {
				// Map view of the file
				let view = MapViewOfFile(map, /*FILE_MAP_COPY*/0x0001, 0, 0, 0);
				if view != ptr::null() {
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
				close_handle!(map);
				return Err(err);
			}
		}
		Err(io::Error::last_os_error())
	}
	// Change the memory protection of a range of this image.
	pub fn protect<U: Into<usize>>(&self, address: Range<U>, protect: Protect) -> bool {
		unsafe {
			let bytes = &(*self.bytes)[address.start.into()..address.end.into()];
			let mut old_protect = 0;
			VirtualProtect(bytes.as_ptr() as *const c_void, bytes.len(), protect.to_os_protect(), &mut old_protect) != 0
		}
	}
}
impl AsRawHandle for ImageMap {
	fn as_raw_handle(&self) -> RawHandle {
		self.handle
	}
}
impl AsRef<[u8]> for ImageMap {
	fn as_ref(&self) -> &[u8] {
		unsafe { &*self.bytes }
	}
}
impl Drop for ImageMap {
	fn drop(&mut self) {
		unsafe {
			UnmapViewOfFile((*self.bytes).as_ptr() as *const c_void);
			close_handle!(self.handle);
		}
	}
}

//----------------------------------------------------------------

/// Memory-mapped file.
pub struct FileMap {
	handle: RawHandle,
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
		let bytes = slice::from_raw_parts_mut(view as *mut u8, size);
		Ok(FileMap { handle: map, bytes })
	}
}
impl AsRawHandle for FileMap {
	fn as_raw_handle(&self) -> RawHandle {
		self.handle
	}
}
impl AsRef<[u8]> for FileMap {
	fn as_ref(&self) -> &[u8] {
		unsafe { &*self.bytes }
	}
}
impl Drop for FileMap {
	fn drop(&mut self) {
		unsafe {
			UnmapViewOfFile((*self.bytes).as_ptr() as *const c_void);
			close_handle!(self.handle);
		}
	}
}
