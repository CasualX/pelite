use super::Wrap;
use crate::*;

/// Format agnostic PE view.
pub type PeView<'a> = Wrap<pe32::PeView<'a>, pe64::PeView<'a>>;

impl<'a> PeView<'a> {
    pub fn from_bytes<T: AsRef<[u8]> + ?Sized>(image: &'a T) -> Result<PeView<'a>> {
        match pe64::PeView::from_bytes(image) {
            Ok(file) => Ok(Wrap::T64(file)),
            Err(Error::PeMagic) => Ok(Wrap::T32(pe32::PeView::from_bytes(image)?)),
            Err(err) => Err(err),
        }
    }
}
