/*!
PE file.
*/

use super::image::*;
use super::pe::{Pe, validate_headers};
use ::{Error, Result};

/// View into an unmapped PE file.
#[derive(Copy, Clone)]
pub struct PeFile<'a> {
	image: &'a [u8],
}

impl<'a> PeFile<'a> {
	/// Try to read the given bytes as an unmapped PE file.
	pub fn from_bytes<T: AsRef<[u8]> + ?Sized>(image: &'a T) -> Result<PeFile<'a>> {
		let image = image.as_ref();
		let _ = validate_headers(image)?;
		Ok(PeFile { image })
	}
	fn find_section(&self, rva: Rva, min_size: usize) -> Result<&'a [u8]> {
		// Can't reuse `self.rva_to_file_offset` because it doesn't return the size of the section
		for it in self.section_headers() {
			if rva >= it.VirtualAddress && rva < (it.VirtualAddress + it.VirtualSize) {
				if rva < (it.VirtualAddress + it.SizeOfRawData) {
					let start = (rva - it.VirtualAddress + it.PointerToRawData) as FileOffset;
					let end = (it.PointerToRawData + it.SizeOfRawData) as FileOffset;
					return match self.image.get(start..end) {
						Some(bytes) if bytes.len() >= min_size => Ok(bytes),
						_ => if start + min_size > (it.VirtualAddress + it.VirtualSize) as usize { Err(Error::OOB) } else { Err(Error::ZeroFill) },
					};
				}
				return Err(Error::ZeroFill);
			}
		}
		Err(Error::OOB)
	}
}

impl<'a> Pe<'a> for PeFile<'a> {
	fn image(&self) -> &'a [u8] {
		self.image
	}
	fn slice(&self, rva: Rva, min_size: usize, align: usize) -> Result<&'a [u8]> {
		if rva == 0 {
			Err(Error::Null)
		}
		else if rva as FileOffset & (align - 1) != 0 {
			Err(Error::Misalign)
		}
		else {
			self.find_section(rva, min_size)
		}
	}
	fn read(&self, va: Va, min_size: usize, align: usize) -> Result<&'a [u8]> {
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
			let rva = (va - image_base) as Rva;
			if rva as FileOffset & (align - 1) != 0 {
				Err(Error::Misalign)
			}
			else {
				self.find_section(rva, min_size)
			}
		}
	}
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::PeFile;
	use ::Error;

	#[test]
	fn from_byte_slice() {
		assert!(match PeFile::from_bytes(&[][..]) { Err(Error::OOB) => true, _ => false });
	}
}
