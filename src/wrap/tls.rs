use super::*;

/// TLS Directory.
impl<'a, Pe32: Copy + pe32::Pe<'a>, Pe64: Copy + pe64::Pe<'a>> Wrap<pe32::TlsDirectory<'a, Pe32>, pe64::TlsDirectory<'a, Pe64>> {
	/// Returns the PE instance.
	#[inline]
	pub fn pe(&self) -> Wrap<Pe32, Pe64> {
		match self {
			Wrap::T32(tls) => Wrap::T32(tls.pe()),
			Wrap::T64(tls) => Wrap::T64(tls.pe()),
		}
	}
	/// Returns the underlying TLS directory image.
	#[inline]
	pub fn image(&self) -> Wrap<&'a image::IMAGE_TLS_DIRECTORY32, &'a image::IMAGE_TLS_DIRECTORY64> {
		match self {
			Wrap::T32(tls) => Wrap::T32(tls.image()),
			Wrap::T64(tls) => Wrap::T64(tls.image()),
		}
	}
	/// Returns the initialized TLS data.
	///
	/// # Errors
	///
	/// * [`Invalid`][crate::Error::Invalid]: The data's end address precedes its start address.
	/// * [`Overflow`][crate::Error::Overflow]: The declared data length cannot fit in `usize`.
	/// * [`Null`][crate::Error::Null], [`Bounds`][crate::Error::Bounds], [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The data cannot be read from the image.
	#[inline]
	pub fn raw_data(&self) -> Result<&'a [u8]> {
		match self {
			Wrap::T32(tls) => tls.raw_data(),
			Wrap::T64(tls) => tls.raw_data(),
		}
	}
	/// Returns the TLS slot location.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The slot address is zero.
	/// * [`Bounds`][crate::Error::Bounds], [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The slot cannot be read from the image.
	#[inline]
	pub fn slot(&self) -> Result<&'a u32> {
		match self {
			Wrap::T32(tls) => tls.slot(),
			Wrap::T64(tls) => tls.slot(),
		}
	}
	/// Returns the TLS callback addresses.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The callback pointer is null.
	/// * [`Bounds`][crate::Error::Bounds]: The callbacks have no terminating zero or lie outside the image.
	/// * [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The callback array cannot be read.
	#[inline]
	pub fn callbacks(&self) -> Result<Wrap<&'a [u32], &'a [u64]>> {
		match self {
			Wrap::T32(tls) => Wrap::T32(tls.callbacks()).transpose(),
			Wrap::T64(tls) => Wrap::T64(tls.callbacks()).transpose(),
		}
	}
}
