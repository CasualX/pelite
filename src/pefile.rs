use *;

/// View into an unmapped PE32 or PE32+ file.
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

#[cfg(feature = "serde")]
mod serde {
	use util::serde_helper::*;
	use super::{PeFile};

	impl<'a> Serialize for PeFile<'a> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			match self {
				PeFile::Pe32(file) => file.serialize(serializer),
				PeFile::Pe64(file) => file.serialize(serializer),
			}
		}
	}
}
