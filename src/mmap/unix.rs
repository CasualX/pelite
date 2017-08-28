extern crate libc;

use std::{io, mem, ptr, slice};
use std::fs::File;
use std::path::Path;
use std::os::unix::io::AsRawFd;

/// Memory-mapped file.
pub struct FileMap {
	ptr: *mut libc::c_void,
	size: usize,
}
impl FileMap {
	/// Maps the whole file into memory.
	pub fn open<P: AsRef<Path> + ?Sized>(path: &P) -> io::Result<FileMap> {
		Self::_open(path.as_ref())
	}
	fn _open(path: &Path) -> io::Result<FileMap> {
		// Open the file and get its fd
		let file = File::open(path)?;
		let fd = file.as_raw_fd();

		// Find its file size
		let size = unsafe {
			let mut stat = mem::uninitialized();
			if libc::fstat(fd, &mut stat) < 0 {
				return Err(io::Error::last_os_error());
			}
			stat.st_size as usize
		};

		// Mmap the file
		unsafe {
			let ptr = libc::mmap(
				ptr::null_mut(),
				size as libc::size_t,
				libc::PROT_READ,
				libc::MAP_SHARED,
				fd,
				0
			);
			if ptr == libc::MAP_FAILED {
				Err(io::Error::last_os_error())
			}
			else {
				Ok(FileMap { ptr, size })
			}
		}
	}
}
impl Drop for FileMap {
	fn drop(&mut self) {
		unsafe {
			let _result = libc::munmap(self.ptr, self.size as libc::size_t);
			debug_assert_eq!(_result, 0, "unable to munmap: {}", io::Error::last_os_error());
		}
	}
}
impl AsRef<[u8]> for FileMap {
	fn as_ref(&self) -> &[u8] {
		unsafe {
			slice::from_raw_parts(self.ptr as *const _, self.size)
		}
	}
}
