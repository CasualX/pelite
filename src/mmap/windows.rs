
use ::std::path::Path;
use ::std::{io, ptr, slice};

use ::std::os::windows::io::RawHandle;
use ::std::os::windows::ffi::OsStrExt;
use ::std::os::raw::c_void;
use ::std::ffi::OsStr;

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
		// Get the path as a nul terminated wide string
		let path: &OsStr = path.as_ref();
		let mut wpath: Vec<u16> = path.encode_wide().collect();
		wpath.push(0);
		// Get its file handle
		let file = CreateFileW(wpath.as_ptr(), /*GENERIC_READ*/0x80000000, /*FILE_SHARE_READ*/0x00000001, ptr::null(), /*OPEN_EXISTING*/3, /*FILE_ATTRIBUTE_NORMAL*/0x00000080, NULL);
		drop(wpath);
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
		// Get the path as a nul terminated wide string
		let path: &OsStr = path.as_ref();
		let mut wpath: Vec<u16> = path.encode_wide().collect();
		wpath.push(0);
		// Get its file handle
		let file = CreateFileW(wpath.as_ptr(), /*GENERIC_READ*/0x80000000, /*FILE_SHARE_READ*/0x00000001, ptr::null(), /*OPEN_EXISTING*/3, /*FILE_ATTRIBUTE_NORMAL*/0x00000080, NULL);
		drop(wpath);
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
