use super::*;

/// Describes the PE headers.
impl<'a, Pe32: Copy + pe32::Pe<'a>, Pe64: Copy + pe64::Pe<'a>> Wrap<pe32::PeHeaders<Pe32>, pe64::PeHeaders<Pe64>> {
	/// Returns the PE instance.
	#[inline]
	pub fn pe(&self) -> Wrap<Pe32, Pe64> {
		match self {
			Wrap::T32(headers) => Wrap::T32(headers.pe()),
			Wrap::T64(headers) => Wrap::T64(headers.pe()),
		}
	}
	/// Returns the PE headers as a byte slice.
	#[inline]
	pub fn image(&self) -> &'a [u8] {
		match self {
			Wrap::T32(headers) => headers.image(),
			Wrap::T64(headers) => headers.image(),
		}
	}
	/// Calculates the optional header's CheckSum.
	#[inline]
	pub fn check_sum(&self) -> u32 {
		match self {
			Wrap::T32(headers) => headers.check_sum(),
			Wrap::T64(headers) => headers.check_sum(),
		}
	}
}
