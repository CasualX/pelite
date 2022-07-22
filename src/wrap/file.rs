use super::Wrap;
use crate::*;

/// Format agnostic PE file.
pub type PeFile<'a> = Wrap<pe32::PeFile<'a>, pe64::PeFile<'a>>;

impl<'a> PeFile<'a> {
    /// Constructs a PeFile from byte slice.
    pub fn from_bytes<T: AsRef<[u8]> + ?Sized>(image: &'a T) -> Result<PeFile<'a>> {
        match pe64::PeFile::from_bytes(image) {
            Ok(file) => Ok(Wrap::T64(file)),
            Err(Error::PeMagic) => Ok(Wrap::T32(pe32::PeFile::from_bytes(image)?)),
            Err(err) => Err(err),
        }
    }
}
