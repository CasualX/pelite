use super::*;

/// Format agnostic PE file.
pub type PeFile<'a> = Wrap<pe32::PeFile<'a>, pe64::PeFile<'a>>;

impl<'a> PeFile<'a> {
	/// Constructs a PE file from a byte slice.
	///
	/// Detects whether the image is PE32 or PE32+.
	///
	/// # Errors
	///
	/// * [`Bounds`][crate::Error::Bounds]: The image is too small for its declared headers or directories.
	/// * [`Misaligned`][crate::Error::Misaligned]: The image or its headers are misaligned.
	/// * [`BadMagic`][crate::Error::BadMagic]: The DOS or PE signature is invalid.
	/// * [`Insanity`][crate::Error::Insanity]: Reasonable limits on header fields are exceeded.
	/// * [`Overflow`][crate::Error::Overflow]: A header offset or directory size overflows.
	pub fn from_bytes<T: AsRef<[u8]> + ?Sized>(image: &'a T) -> Result<PeFile<'a>> {
		match pe64::PeFile::from_bytes(image) {
			Ok(file) => Ok(Wrap::T64(file)),
			Err(Error::PeMagic) => Ok(Wrap::T32(pe32::PeFile::from_bytes(image)?)),
			Err(err) => Err(err),
		}
	}

	#[doc = include_str!("../docs/image_base.md")]
	#[inline]
	pub fn image_base(self) -> u64 {
		match self {
			Wrap::T32(pe) => pe.image_base() as u64,
			Wrap::T64(pe) => pe.image_base(),
		}
	}
}
