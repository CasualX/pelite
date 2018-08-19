/*!
PE view.
*/

use std::slice;

use error::{Error, Result};

use super::image::*;
use super::pe::validate_headers;
use super::{Align, Pe};

/// View into a mapped PE image.
#[derive(Copy, Clone)]
pub struct PeView<'a> {
	image: &'a [u8],
}

current_target! {
	impl PeView<'static> {
		/// Creates a new instance of PeView of the module this code is executing in.
		#[inline]
		pub unsafe fn new() -> PeView<'static> {
			Self::module(image_base() as *const _ as *const u8)
		}
	}
}
impl<'a> PeView<'a> {
	/// Try to read the given bytes as a mapped PE image.
	pub fn from_bytes<T: AsRef<[u8]> + ?Sized>(image: &'a T) -> Result<PeView<'a>> {
		let image = image.as_ref();
		let size_of_image = validate_headers(image)?;
		// Sanity check, this values should match.
		// If they don't, that's not a problem per sé as it would be caught later.
		if size_of_image as usize != image.len() {
			return Err(Error::Insanity);
		}
		Ok(PeView { image })
	}
	/// Creates a new instance of `PeView` of a mapped image.
	///
	/// # Safety
	///
	/// The underlying memory is borrowed and an unbounded lifetime is returned. Make sure it outlives this PeView instance!
	///
	/// No sanity or safety checks are done to make sure this is really PE32(+) image.
	/// When using this with a `HMODULE` from the system the caller must be sure this is a PE32(+) image.
	#[inline]
	pub unsafe fn module(base: *const u8) -> PeView<'a> {
		let dos = &*(base as *const IMAGE_DOS_HEADER);
		let nt = &*(base.offset(dos.e_lfanew as isize) as *const IMAGE_NT_HEADERS);
		PeView {
			image: slice::from_raw_parts(base, nt.OptionalHeader.SizeOfImage as usize),
		}
	}
	fn slice_impl(self, rva: Rva, min_size_of: usize, align_of: usize) -> Result<&'a [u8]> {
		debug_assert!(align_of != 0 && align_of & (align_of - 1) == 0);
		let start = rva as usize;
		if rva == 0 {
			Err(Error::Null)
		}
		else if usize::wrapping_add(self.image.as_ptr() as usize, start) & (align_of - 1) != 0 {
			Err(Error::Misalign)
		}
		else {
			match self.image.get(start..) {
				Some(bytes) if bytes.len() >= min_size_of => Ok(bytes),
				_ => Err(Error::OOB),
			}
		}
	}
	fn read_impl(self, va: Va, min_size_of: usize, align_of: usize) -> Result<&'a [u8]> {
		debug_assert!(align_of != 0 && align_of & (align_of - 1) == 0);
		let (image_base, size_of_image) = {
			let optional_header = self.optional_header();
			(optional_header.ImageBase, optional_header.SizeOfImage)
		};
		if va == 0 {
			Err(Error::Null)
		}
		else if va < image_base || va - image_base > size_of_image as Va {
			Err(Error::OOB)
		}
		else {
			let start = (va - image_base) as usize;
			if usize::wrapping_add(self.image.as_ptr() as usize, start) & (align_of - 1) != 0 {
				Err(Error::Misalign)
			}
			else {
				match self.image.get(start..) {
					Some(bytes) if bytes.len() >= min_size_of => Ok(bytes),
					_ => Err(Error::OOB),
				}
			}
		}
	}
}

unsafe impl<'a> Pe<'a> for PeView<'a> {
	fn image(&self) -> &'a [u8] {
		self.image
	}
	fn align(&self) -> Align {
		Align::Section
	}
	fn slice(&self, rva: Rva, min_size_of: usize, align_of: usize) -> Result<&'a [u8]> {
		self.slice_impl(rva, min_size_of, align_of)
	}
	fn read(&self, va: Va, min_size_of: usize, align_of: usize) -> Result<&'a [u8]> {
		self.read_impl(va, min_size_of, align_of)
	}
	#[cfg(feature = "serde")]
	const SERDE_NAME: &'static str = "PeView";
}

//----------------------------------------------------------------

#[cfg(feature = "serde")]
impl<'a> ::serde::Serialize for PeView<'a> {
	fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error> {
		super::pe::serialize_pe(self, serializer)
	}
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use error::Error;
	use super::PeView;

	#[test]
	fn from_byte_slice() {
		assert!(match PeView::from_bytes(&[]) { Err(Error::OOB) => true, _ => false });
	}
}
