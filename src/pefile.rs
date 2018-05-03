use *;

pub enum PeFile<'a> {
	PeFile32(pe32::PeFile<'a>),
	PeFile64(pe64::PeFile<'a>),
}
impl<'a> PeFile<'a> {
	pub fn from_bytes<T: AsRef<[u8]> + ?Sized>(image: &'a T) -> Result<PeFile<'a>> {
		match pe32::PeFile::from_bytes(image) {
			Ok(file) => Ok(PeFile::PeFile32(file)),
			Err(err) if err == Error::BadMagic => {
				pe64::PeFile::from_bytes(image).map(PeFile::PeFile64)
			},
			Err(err) => Err(err),
		}
	}
	pub fn map<F, G, R>(self, mut f: F, mut g: G) -> R
		where F: FnMut(pe32::PeFile) -> R, G: FnMut(pe64::PeFile) -> R
	{
		match self {
			PeFile::PeFile32(file) => f(file),
			PeFile::PeFile64(file) => g(file),
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
				PeFile::PeFile32(pe) => pe.serialize(serializer),
				PeFile::PeFile64(pe) => pe.serialize(serializer),
			}
		}
	}
}
