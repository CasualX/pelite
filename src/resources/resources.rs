use core::prelude::v1::*;
use core::{char, fmt, iter, mem, slice};

use crate::image::*;
use crate::resources::Directory;
use crate::resources::types::RSRC_TYPES;
use crate::{Error, Pod, Result};

#[cfg(all(feature = "alloc", feature = "std"))]
use crate::resources::version_info;

/// Resources filesystem.
#[derive(Copy, Clone)]
pub struct Resources<'a> {
	pub(crate) section: &'a [u8],
	pub(crate) dir: &'a IMAGE_DATA_DIRECTORY,
}

impl<'a> Resources<'a> {
	
	/// Parse the bytes as PE resources.
	///
	/// No validation or integrity checking is done ahead of time.
	pub fn new(section: &'a [u8], dir: &'a IMAGE_DATA_DIRECTORY) -> Resources<'a> {
		// All offsets _except_ the data entry offsets are relative to the resource directory.
		// Data entry offsets are relative virtual addresses from the PE image.
		// Microsoft... Why would you do this?
		Resources { section, dir }
	}

	/// Gets the root directory.
	pub fn root(&self) -> Result<Directory<'a>> {
		Directory::try_from(*self, 0)
	}

	/// Filesystem consistency check.
	///
	/// Simply walks the filesystem checking all references are valid.
	pub fn fsck(&self) -> Result<()> {
		self.root()?.fsck()
	}

	#[inline]
	pub fn slice<T: Pod>(&self, offset: u32) -> Result<&'a T> {
		let start = offset as usize;
		let end = mem::size_of::<T>().wrapping_add(start);
		// Alignment checking
		if !cfg!(feature = "unsafe_alignment") && start & (mem::align_of::<T>() - 1) != 0 {
			return Err(Error::Misaligned);
		}
		// Range checking done by the indexing operator
		let bytes = self.section.get(start..end).ok_or(Error::Bounds)?;
		// Safe because size and alignment are checked and T is Pod
		Ok(unsafe { &*(bytes.as_ptr() as *const T) })
	}

	#[inline]
	#[allow(dead_code)] // unused for now...
	fn slice_len<T: Pod>(&self, offset: u32, len: usize) -> Result<&'a [T]> {
		let start = offset as usize;
		let size_of = mem::size_of::<T>().checked_mul(len).ok_or(Error::Overflow)?;
		let end = start.wrapping_add(size_of);
		// Alignment checking
		if !cfg!(feature = "unsafe_alignment") && start & (mem::align_of::<T>() - 1) != 0 {
			return Err(Error::Misaligned);
		}
		// Range checking done by the indexing operator
		let bytes = self.section.get(start..end).ok_or(Error::Bounds)?;
		Ok(unsafe { slice::from_raw_parts(bytes.as_ptr() as *const T, len) })
	}
	
	#[inline]
	pub fn slice_ws(&self, offset: u32) -> Result<&'a [u16]> {
		let offset = offset as usize;
		// Alignment checking
		if !cfg!(feature = "unsafe_alignment") && offset & 1 != 0 {
			return Err(Error::Misaligned);
		}
		// The name is prefixed by its length in words
		let len = self.section.get(offset..offset + 2).ok_or(Error::Bounds)?;
		let len = unsafe { *(len.as_ptr() as *const u16) } as usize;
		// Extract the name given its length
		let name = self.section.get(offset + 2..offset + 2 + len * 2).ok_or(Error::Bounds)?;
		let name = unsafe { slice::from_raw_parts(name.as_ptr() as *const u16, len) };
		Ok(name)
	}
}

impl<'a> fmt::Debug for Resources<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("Resources { .. }")
	}
}

//----------------------------------------------------------------
