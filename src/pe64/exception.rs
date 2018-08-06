/*!
Exception Directory.
*/

use std::{fmt, iter, mem, slice};
use std::cmp::Ordering;

use error::{Error, Result};

use super::image::*;
use super::Pe;

//----------------------------------------------------------------

/// Exception Directory.
///
/// For more information see the [module-level documentation](index.html).
#[derive(Copy, Clone)]
pub struct Exception<'a, P> {
	pe: P,
	image: &'a [RUNTIME_FUNCTION],
}
impl<'a, P: Pe<'a> + Copy> Exception<'a, P> {
	pub(crate) fn new(pe: P) -> Result<Exception<'a, P>> {
		let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_EXCEPTION).ok_or(Error::OOB)?;
		let (len, rem) = (
			datadir.Size as usize / mem::size_of::<RUNTIME_FUNCTION>(),
			datadir.Size as usize % mem::size_of::<RUNTIME_FUNCTION>(),
		);
		if rem != 0 {
			return Err(Error::Corrupt);
		}
		let image = pe.derva_slice(datadir.VirtualAddress, len)?;
		Ok(Exception { pe, image })
	}
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Returns the functions slice.
	pub fn image(&self) -> &'a [RUNTIME_FUNCTION] {
		self.image
	}
	/// Gets an iterator over the function records.
	pub fn functions(&self)
		-> iter::Map<slice::Iter<'a, RUNTIME_FUNCTION>, impl Clone + FnMut(&'a RUNTIME_FUNCTION) -> Function<'a, P>>
	{
		let pe = self.pe;
		self.image.iter()
			.map(move |image| Function { pe, image })
	}
	/// Finds the index of the function for the given program counter.
	pub fn index_of(&self, pc: Rva) -> ::std::result::Result<usize, usize> {
		self.image.binary_search_by(|rf| {
			if rf.BeginAddress < pc {
				Ordering::Less
			}
			else if rf.EndAddress > pc {
				Ordering::Greater
			}
			else {
				Ordering::Equal
			}
		})
	}
	/// Finds the function for the given 'program counter' address.
	///
	/// The function records are sorted by their address allowing binary search for the record.
	pub fn lookup_function_entry(&self, pc: Rva) -> Option<Function<'a, P>> {
		self.index_of(pc).map(|index| Function {
			pe: self.pe,
			image: &self.image[index]
		}).ok()
	}
}
impl<'a, P: Pe<'a> + Copy> fmt::Debug for Exception<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Exception")
			.field("functions.len", &self.image.len())
			.finish()
	}
}

//----------------------------------------------------------------

/// Runtime function.
#[derive(Copy, Clone)]
pub struct Function<'a, P> {
	pe: P,
	image: &'a RUNTIME_FUNCTION,
}
impl<'a, P: Pe<'a> + Copy> Function<'a, P> {
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Returns the underlying runtime function image.
	pub fn image(&self) -> &'a RUNTIME_FUNCTION {
		self.image
	}
	/// Gets the function bytes.
	pub fn bytes(&self) -> Result<&'a [u8]> {
		let len = if self.image.BeginAddress > self.image.EndAddress { return Err(Error::Overflow); }
		else { (self.image.EndAddress - self.image.BeginAddress) as usize };
		self.pe.derva_slice(self.image.BeginAddress, len)
	}
	/// Gets the unwind info.
	pub fn unwind_info(&self) -> Result<UnwindInfo<'a, P>> {
		// FIXME! Check if CountOfCodes is valid...
		self.pe.derva(self.image.UnwindData).map(|image| UnwindInfo { pe: self.pe, image })
	}
}
impl<'a, P: Pe<'a> + Copy> fmt::Debug for Function<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Function")
			.field("bytes.len", &self.bytes().map(<[_]>::len))
			.finish()
	}
}

//----------------------------------------------------------------

/// Unwind info.
#[derive(Copy, Clone)]
pub struct UnwindInfo<'a, P> {
	pe: P,
	image: &'a UNWIND_INFO,
}
impl<'a, P: Pe<'a> + Copy> UnwindInfo<'a, P> {
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Returns the underlying unwind info image.
	pub fn image(&self) -> &'a UNWIND_INFO {
		self.image
	}
	pub fn version(&self) -> u8 {
		self.image.VersionFlags & 0b00000111
	}
	pub fn flags(&self) -> u8 {
		self.image.VersionFlags >> 3
	}
	pub fn size_of_prolog(&self) -> usize {
		self.image.SizeOfProlog as usize
	}
	pub fn frame_register(&self) -> u8 {
		self.image.FrameRegisterOffset & 0b00001111
	}
	pub fn frame_offset(&self) -> u8 {
		self.image.FrameRegisterOffset >> 4
	}
	pub fn unwind_codes(&self) -> &'a [UNWIND_CODE] {
		let len = self.image.CountOfCodes as usize;
		unsafe {
			slice::from_raw_parts(self.image.UnwindCode.as_ptr(), len)
		}
	}
}
impl<'a, P: Pe<'a> + Copy> fmt::Debug for UnwindInfo<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("UnwindInfo")
			.field("version", &self.version())
			.field("flags", &self.flags())
			.field("size_of_prolog", &self.size_of_prolog())
			.field("frame_register", &self.frame_register())
			.field("frame_offset", &self.frame_offset())
			.field("unwind_codes.len", &self.unwind_codes().len())
			.finish()
	}
}
