/*!
PE view.
*/

use std::slice;
use std::marker::PhantomData;

use error::{Error, Result};

use super::image::*;
use super::pe::{Align, Pe, PeMut, validate_headers};

fn slice_impl<'a, P: Pe<'a> + Copy>(pe: P, rva: Rva, min_size: usize, align: usize) -> Result<&'a [u8]> {
	debug_assert!(align != 0 && align & (align - 1) == 0);
	let start = rva as usize;
	if rva == BADRVA {
		Err(Error::Null)
	}
	else if start & (align - 1) != 0 {
		Err(Error::Misalign)
	}
	else {
		match pe.image().get(start..) {
			Some(bytes) if bytes.len() >= min_size => Ok(bytes),
			_ => Err(Error::OOB),
		}
	}
}
fn read_impl<'a, P: Pe<'a> + Copy>(pe: P, va: Va, min_size: usize, align: usize) -> Result<&'a [u8]> {
	debug_assert!(align != 0 && align & (align - 1) == 0);
	let (image_base, size_of_image) = {
		let optional_header = pe.optional_header();
		(optional_header.ImageBase, optional_header.SizeOfImage)
	};
	if va == BADVA {
		Err(Error::Null)
	}
	else if va < image_base || va - image_base > size_of_image as Va {
		Err(Error::OOB)
	}
	else {
		let start = (va - image_base) as usize;
		if start & (align - 1) != 0 {
			Err(Error::Misalign)
		}
		else {
			match pe.image().get(start..) {
				Some(bytes) if bytes.len() >= min_size => Ok(bytes),
				_ => Err(Error::OOB),
			}
		}
	}
}

//----------------------------------------------------------------

/// View into a mapped PE image.
#[derive(Copy, Clone)]
pub struct PeView<'a> {
	image: *const [u8],
	_phantom: PhantomData<&'a [u8]>,
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
		let info = validate_headers(image)?;
		// Sanity check, this values should match.
		// If they don't, that's not a problem per sé as it would be caught later.
		if info.size_of_image as usize != image.len() {
			return Err(Error::Insanity);
		}
		Ok(PeView { image, _phantom: PhantomData })
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
			_phantom: PhantomData,
		}
	}
}

unsafe impl<'a> Pe<'a> for PeView<'a> {
	fn image(&self) -> &'a [u8] {
		unsafe { &*self.image }
	}
	fn align(&self) -> Align {
		Align::Section
	}
	fn slice(&self, rva: Rva, min_size: usize, align: usize) -> Result<&'a [u8]> {
		slice_impl(*self, rva, min_size, align)
	}
	fn read(&self, va: Va, min_size: usize, align: usize) -> Result<&'a [u8]> {
		read_impl(*self, va, min_size, align)
	}
}

//----------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct PeViewMut<'a> {
	image: *mut [u8],
	_phantom: PhantomData<&'a mut [u8]>,
}
impl<'a> PeViewMut<'a> {
	/// Try to read the given bytes as a mapped PE image.
	///
	/// Acquire unique lock on the image bytes allowing safe mutation.
	pub fn from_bytes_mut<T: AsMut<[u8]> + ?Sized>(image: &'a mut T) -> Result<PeViewMut<'a>> {
		let image = image.as_mut();
		let info = validate_headers(image)?;
		// Sanity check, this values should match.
		// If they don't, that's not a problem per sé as it would be caught later.
		if info.size_of_image as usize != image.len() {
			return Err(Error::Insanity);
		}
		Ok(PeViewMut { image, _phantom: PhantomData })
	}
}
unsafe impl<'a> Pe<'a> for PeViewMut<'a> {
	fn image(&self) -> &'a [u8] {
		unsafe { &*self.image }
	}
	fn align(&self) -> Align {
		Align::Section
	}
	fn slice(&self, rva: Rva, min_size: usize, align: usize) -> Result<&'a [u8]> {
		slice_impl(*self, rva, min_size, align)
	}
	fn read(&self, va: Va, min_size: usize, align: usize) -> Result<&'a [u8]> {
		read_impl(*self, va, min_size, align)
	}
}
unsafe impl<'a> PeMut<'a> for PeViewMut<'a> {}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use error::Error;
	use super::PeView;

	#[test]
	fn from_byte_slice() {
		assert!(match PeView::from_bytes(&[][..]) { Err(Error::OOB) => true, _ => false });
	}
}
