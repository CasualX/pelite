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
}

impl<'a> Pe<'a> for PeFile<'a> {
	fn image(&self) -> &'a [u8] {
		self.image
	}
	fn slice_rva(&self, rva: Rva, size: usize, align: usize) -> Result<&'a [u8]> {
		if rva == 0 {
			Err(Error::Null)
		}
		else if rva as usize & (align - 1) != 0 {
			Err(Error::Misalign)
		}
		else {
			// Can't reuse `self.rva_to_file_offset` because it doesn't return the size of the section
			for it in self.section_headers() {
				if rva >= it.VirtualAddress && rva < (it.VirtualAddress + it.VirtualSize) {
					if rva < (it.VirtualAddress + it.SizeOfRawData) {
						let start = (rva - it.VirtualAddress + it.PointerToRawData) as FileOffset;
						let end = (it.PointerToRawData + it.SizeOfRawData) as FileOffset;
						return match self.image.get(start..end) {
							Some(bytes) if bytes.len() >= size => Ok(bytes),
							_ => if start + size > (it.VirtualAddress + it.VirtualSize) as usize { Err(Error::OOB) } else { Err(Error::ZeroFill) },
						};
					}
					return Err(Error::ZeroFill);
				}
			}
			Err(Error::OOB)
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
