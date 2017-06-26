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
	pub fn from_bytes<T: AsRef<[u8]>>(image: &'a T) -> Result<PeFile<'a>> {
		let image = image.as_ref();
		let _ = validate_headers(image)?;
		Ok(PeFile { image })
	}
}

impl<'a> Pe<'a> for PeFile<'a> {
	fn image(&self) -> &'a [u8] {
		self.image
	}
	fn slice_rva(&self, rva: Rva, size: usize, _align: usize) -> Result<&'a [u8]> {
		if rva == 0 {
			Err(Error::Null)
		}
		// NOTE!
		// Sure to compare against the RVA?
		// Why not against the actual file offset?
		// Does it even make a difference?
		// else if rva as usize & (align - 1) != 0 {
		// 	Err(Error::Misalign)
		// }
		else {
			// FIXME! Reject slices over multiple sections!
			let start = self.rva_to_file_offset(rva)?;
			match self.image.get(start..) {
				Some(bytes) if bytes.len() >= size => Ok(bytes),
				_ => Err(Error::OOB),
			}
		}
	}
}
