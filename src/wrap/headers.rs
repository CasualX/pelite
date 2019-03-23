use crate::*;
use super::Wrap;

/// Describes the PE headers.
impl<'a, Pe32: pe32::Pe<'a>, Pe64: pe64::Pe<'a>> Wrap<pe32::headers::Headers<Pe32>, pe64::headers::Headers<Pe64>> {
	/// Gets the PE instance.
	#[inline]
	pub fn pe(&self) -> Wrap<Pe32, Pe64> {
		match self {
			Wrap::T32(headers) => Wrap::T32(headers.pe()),
			Wrap::T64(headers) => Wrap::T64(headers.pe()),
		}
	}
	/// Gets the PE headers as a byte slice.
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
