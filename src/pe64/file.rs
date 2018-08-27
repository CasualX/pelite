/*!
PE file.
*/

use std::cmp;

use error::{Error, Result};

use super::image::*;
use super::pe::validate_headers;
use super::{Align, Pe};

/// View into an unmapped PE file.
#[derive(Copy, Clone)]
pub struct PeFile<'a> {
	image: &'a [u8],
}

impl<'a> PeFile<'a> {
	/// Constructs a file view from a byte slice.
	///
	/// # Errors
	///
	/// * [`Bounds`](../enum.Error.html#variant.Bounds):
	///   The byte slice is too small to fit the PE headers.
	///
	/// * [`Misaligned`](../enum.Error.html#variant.Misaligned):
	///   The minimum alignment of 4 is not satisfied.
	///
	/// * [`BadMagic`](../enum.Error.html#variant.BadMagic):
	///   This is not a PE file.
	///
	/// * [`PeMagic`](../enum.Error.html#variant.PeMagic):
	///   Trying to parse a PE32 file with the PE32+ parser and vice versa.
	///
	/// * [`Insanity`](../enum.Error.html#variant.Insanity):
	///   Reasonable limits on `e_lfanew`, `SizeOfHeaders` or `NumberOfSections` are exceeded.
	pub fn from_bytes<T: AsRef<[u8]> + ?Sized>(image: &'a T) -> Result<PeFile<'a>> {
		let image = image.as_ref();
		let _ = validate_headers(image)?;
		Ok(PeFile { image })
	}
	fn range_to_slice(&self, rva: Rva, min_size_of: usize) -> Result<&'a [u8]> {
		// Cannot reuse `self.rva_to_file_offset` because it doesn't return the size of the section
		// This code has been carefully designed to avoid panicking on overflow
		for it in self.section_headers() {
			// Compare if rva is contained within the virtual address space of a section
			// If the calculating the section end address overflows the corrupt section will be skipped
			#[allow(non_snake_case)]
			let VirtualEnd = it.VirtualAddress.wrapping_add(cmp::max(it.VirtualSize, it.SizeOfRawData));
			if it.VirtualAddress <= rva && rva < VirtualEnd { // $1
				// Isolate and range check the pointer and size of raw data
				// If this fails immediately abort and return an error
				let section_range = it.PointerToRawData as usize..it.PointerToRawData.wrapping_add(it.SizeOfRawData) as usize;
				let section_bytes = self.image.get(section_range).ok_or(Error::Invalid)?;
				// Calculate the offset in the section requested. cannot underflow, see $1
				let section_offset = (rva - it.VirtualAddress) as usize;
				return match section_bytes.get(section_offset..) {
					Some(bytes) if bytes.len() >= min_size_of => Ok(bytes),
					// Identify the reason the slice fails. cannot underflow, see $1
					_ => Err(if min_size_of > (VirtualEnd - rva) as usize { Error::Bounds } else { Error::ZeroFill }),
				};
			}
		}
		Err(Error::Bounds)
	}
	#[inline(never)]
	fn slice_impl(self, rva: Rva, min_size_of: usize, align_of: usize) -> Result<&'a [u8]> {
		debug_assert!(align_of != 0 && align_of & (align_of - 1) == 0);
		if rva == 0 {
			Err(Error::Null)
		}
		else if usize::wrapping_add(self.image.as_ptr() as usize, rva as usize) & (align_of - 1) != 0 {
			Err(Error::Misaligned)
		}
		else {
			self.range_to_slice(rva, min_size_of)
		}
	}
	#[inline(never)]
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
			Err(Error::Bounds)
		}
		else {
			let rva = (va - image_base) as Rva;
			if usize::wrapping_add(self.image.as_ptr() as usize, rva as usize) & (align_of - 1) != 0 {
				Err(Error::Misaligned)
			}
			else {
				self.range_to_slice(rva, min_size_of)
			}
		}
	}
}

unsafe impl<'a> Pe<'a> for PeFile<'a> {
	fn image(&self) -> &'a [u8] {
		self.image
	}
	fn align(&self) -> Align {
		Align::File
	}
	fn slice(&self, rva: Rva, min_size_of: usize, align_of: usize) -> Result<&'a [u8]> {
		self.slice_impl(rva, min_size_of, align_of)
	}
	fn read(&self, va: Va, min_size_of: usize, align_of: usize) -> Result<&'a [u8]> {
		self.read_impl(va, min_size_of, align_of)
	}
	#[cfg(feature = "serde")]
	const SERDE_NAME: &'static str = "PeFile";
}

//----------------------------------------------------------------

#[cfg(feature = "serde")]
impl<'a> ::serde::Serialize for PeFile<'a> {
	fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error> {
		super::pe::serialize_pe(self, serializer)
	}
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use error::Error;
	use super::PeFile;

	#[test]
	fn from_byte_slice() {
		assert!(match PeFile::from_bytes(&[]) { Err(Error::Bounds) => true, _ => false });
	}
}
