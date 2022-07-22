use std::ffi::OsStr;
use std::os::raw::c_void;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::io::{AsRawHandle, RawHandle};
use std::path::Path;
use std::{io, mem, ptr};

use windows_sys::Win32::Foundation::HANDLE;
use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
use windows_sys::Win32::Storage::FileSystem::{CreateFileW, OPEN_EXISTING};
use windows_sys::Win32::Storage::FileSystem::{
    FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_READ, FILE_SHARE_READ,
};
use windows_sys::Win32::System::Memory::{
    CreateFileMappingW, MapViewOfFile, UnmapViewOfFile, VirtualQuery, FILE_MAP_COPY, FILE_MAP_READ,
    PAGE_READONLY, SEC_IMAGE,
};

const NULL: isize = 0;
//----------------------------------------------------------------

/// Memory mapped image.
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
            CreateFileW(
                wpath.as_ptr(),
                FILE_GENERIC_READ,
                FILE_SHARE_READ,
                ptr::null_mut(),
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                NULL,
            )
        };
        if file != INVALID_HANDLE_VALUE {
            // Create the image file mapping, `SEC_IMAGE` does its magic thing
            let map = CreateFileMappingW(
                file,
                ptr::null_mut(),
                PAGE_READONLY | SEC_IMAGE,
                0,
                0,
                ptr::null(),
            );
            CloseHandle(file);
            if map != NULL {
                // Map view of the file
                let view = MapViewOfFile(map, FILE_MAP_COPY, 0, 0, 0);
                if view != ptr::null_mut() {
                    // Trust the OS with correctly mapping the image.
                    // Trust me to have read and understood the documentation.
                    // There is no validation and 64bit headers are used because the offsets are the same for PE32.
                    use crate::image::{IMAGE_DOS_HEADER, IMAGE_NT_HEADERS64};
                    let dos_header = view as *const IMAGE_DOS_HEADER;
                    let nt_header = (view as usize + (*dos_header).e_lfanew as usize)
                        as *const IMAGE_NT_HEADERS64;
                    let size_of = (*nt_header).OptionalHeader.SizeOfImage;
                    let bytes = ptr::slice_from_raw_parts_mut(view as *mut u8, size_of as usize);
                    return Ok(ImageMap { handle: map, bytes });
                }
                let err = io::Error::last_os_error();
                CloseHandle(map);
                return Err(err);
            }
        }
        Err(io::Error::last_os_error())
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
impl Drop for ImageMap {
    fn drop(&mut self) {
        unsafe {
            UnmapViewOfFile((*self.bytes).as_ptr() as *const c_void);
            CloseHandle(self.handle);
        }
    }
}

//----------------------------------------------------------------

/// Memory mapped file.
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
            CreateFileW(
                wpath.as_ptr(),
                FILE_GENERIC_READ,
                FILE_SHARE_READ,
                ptr::null_mut(),
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                NULL,
            )
        };
        if file == INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error());
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
        // Get the size of the file mapping, should never fail...
        let mut mem_basic_info = mem::zeroed();
        let vq_result = VirtualQuery(view, &mut mem_basic_info, mem::size_of_val(&mem_basic_info));
        debug_assert_eq!(vq_result, mem::size_of_val(&mem_basic_info));
        // Now have enough information to construct the FileMap
        let bytes =
            ptr::slice_from_raw_parts_mut(view as *mut u8, mem_basic_info.RegionSize as usize);
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
impl Drop for FileMap {
    fn drop(&mut self) {
        unsafe {
            UnmapViewOfFile((*self.bytes).as_ptr() as *const c_void);
            CloseHandle(self.handle);
        }
    }
}
