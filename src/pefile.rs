use crate::*;

/// View into an unmapped PE32 or PE32+ file.
#[derive(Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PeFile<'a> {
	Pe32(pe32::PeFile<'a>),
	Pe64(pe64::PeFile<'a>),
}
impl<'a> PeFile<'a> {
	/// Constructs a PeFile from byte slice.
	pub fn from_bytes<T: AsRef<[u8]> + ?Sized>(image: &'a T) -> Result<PeFile<'a>> {
		match pe64::PeFile::from_bytes(image) {
			Ok(file) => Ok(PeFile::Pe64(file)),
			Err(Error::PeMagic) => Ok(PeFile::Pe32(pe32::PeFile::from_bytes(image)?)),
			Err(err) => Err(err),
		}
	}
}
