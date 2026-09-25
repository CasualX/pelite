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
	#[doc = include_str!("../docs/rva_to_file_offset.md")]
	#[inline]
	pub fn rva_to_file_offset(&self, rva: u32) -> Result<usize> {
		match self {
			Wrap::T32(headers) => headers.rva_to_file_offset(rva),
			Wrap::T64(headers) => headers.rva_to_file_offset(rva),
		}
	}
	#[doc = include_str!("../docs/file_offset_to_rva.md")]
	#[inline]
	pub fn file_offset_to_rva(&self, file_offset: usize) -> Result<u32> {
		match self {
			Wrap::T32(headers) => headers.file_offset_to_rva(file_offset),
			Wrap::T64(headers) => headers.file_offset_to_rva(file_offset),
		}
	}
}
