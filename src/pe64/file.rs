/*!
PE file.
*/

use std::cmp;
use std::marker::PhantomData;

use error::{Error, Result};

use super::image::*;
use super::pe::{Align, Pe, validate_headers};

/// View into an unmapped PE file.
#[derive(Copy, Clone)]
pub struct PeFile<'a> {
	image: *const [u8],
	_phantom: PhantomData<&'a [u8]>,
}

impl<'a> PeFile<'a> {
	/// Try to read the given bytes as an unmapped PE file.
	pub fn from_bytes<T: AsRef<[u8]> + ?Sized>(image: &'a T) -> Result<PeFile<'a>> {
		let image = image.as_ref();
		let _ = validate_headers(image)?;
		Ok(PeFile { image, _phantom: PhantomData })
	}
	/// Try to read the given bytes as an unmapped PE file.
	///
	/// Acquire unique lock on the image bytes allowing safe mutation.
	pub fn from_bytes_mut<T: AsMut<[u8]> + ?Sized>(image: &'a mut T) -> Result<PeFile<'a>> {
		Self::from_bytes(image.as_mut())
	}
	fn range_to_slice(&self, rva: Rva, min_size: usize) -> Result<&'a [u8]> {
		// Cannot reuse `self.rva_to_file_offset` because it doesn't return the size of the section
		// FIXME! What to do about all the potential overflows?
		for it in self.section_headers() {
			#[allow(non_snake_case)]
			let VirtualEnd = it.VirtualAddress + cmp::max(it.VirtualSize, it.SizeOfRawData);
			// Rva is contained within the virtual space of a section
			if rva >= it.VirtualAddress && rva <= VirtualEnd {
				let start = (rva - it.VirtualAddress + it.PointerToRawData) as usize;
				let end = (it.PointerToRawData + it.SizeOfRawData) as usize;
				return match self.image().get(start..end) {
					Some(bytes) if bytes.len() >= min_size => Ok(bytes),
					// Identify the reason the slice fails
					_ => Err(if rva + min_size as Rva > VirtualEnd { Error::OOB } else { Error::ZeroFill }),
				};
			}
		}
		Err(Error::OOB)
	}
	#[inline(never)]
	fn slice_impl(self, rva: Rva, min_size: usize, align: usize) -> Result<&'a [u8]> {
		debug_assert!(align != 0 && align & (align - 1) == 0);
		if rva == BADRVA {
			Err(Error::Null)
		}
		else if rva as usize & (align - 1) != 0 {
			Err(Error::Misalign)
		}
		else {
			self.range_to_slice(rva, min_size)
		}
	}
	#[inline(never)]
	fn read_impl(self, va: Va, min_size: usize, align: usize) -> Result<&'a [u8]> {
		debug_assert!(align != 0 && align & (align - 1) == 0);
		let (image_base, size_of_image) = {
			let optional_header = self.optional_header();
			(optional_header.ImageBase, optional_header.SizeOfImage)
		};
		if va == BADVA {
			Err(Error::Null)
		}
		else if va < image_base || va - image_base > size_of_image as Va {
			Err(Error::OOB)
		}
		else {
			let rva = (va - image_base) as Rva;
			if rva as usize & (align - 1) != 0 {
				Err(Error::Misalign)
			}
			else {
				self.range_to_slice(rva, min_size)
			}
		}
	}
}

unsafe impl<'a> Pe<'a> for PeFile<'a> {
	fn image(&self) -> &'a [u8] {
		unsafe { &*self.image }
	}
	fn align(&self) -> Align {
		Align::File
	}
	fn slice(&self, rva: Rva, min_size: usize, align: usize) -> Result<&'a [u8]> {
		self.slice_impl(rva, min_size, align)
	}
	fn read(&self, va: Va, min_size: usize, align: usize) -> Result<&'a [u8]> {
		self.read_impl(va, min_size, align)
	}
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use error::Error;
	use super::PeFile;

	#[test]
	fn from_byte_slice() {
		assert!(match PeFile::from_bytes(&[][..]) { Err(Error::OOB) => true, _ => false });
	}
}
