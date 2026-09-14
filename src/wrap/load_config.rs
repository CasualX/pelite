use crate::*;

use super::Wrap;

/// Load Config Directory.
impl<'a, Pe32: pe32::Pe<'a>, Pe64: pe64::Pe<'a>> Wrap<pe32::load_config::LoadConfig<'a, Pe32>, pe64::load_config::LoadConfig<'a, Pe64>> {
	/// Gets the PE instance.
	#[inline]
	pub fn pe(&self) -> Wrap<Pe32, Pe64> {
		match self {
			Wrap::T32(load_config) => Wrap::T32(load_config.pe()),
			Wrap::T64(load_config) => Wrap::T64(load_config.pe()),
		}
	}
	/// Copies the load config directory into the latest known image structure.
	///
	/// Fields which are not present in this revision of the directory are zero.
	#[inline]
	pub fn image_copy(&self) -> Wrap<image::IMAGE_LOAD_CONFIG_DIRECTORY32, image::IMAGE_LOAD_CONFIG_DIRECTORY64> {
		match self {
			Wrap::T32(load_config) => Wrap::T32(load_config.image_copy()),
			Wrap::T64(load_config) => Wrap::T64(load_config.image_copy()),
		}
	}
	/// Gets the default security cookie for the image.
	#[inline]
	pub fn security_cookie(&self) -> Result<&'a u32> {
		match self {
			Wrap::T32(load_config) => load_config.security_cookie(),
			Wrap::T64(load_config) => load_config.security_cookie(),
		}
	}
	/// Gets the structured exception handler table.
	#[inline]
	pub fn se_handler_table(&self) -> Result<Wrap<&'a [u32], &'a [u64]>> {
		match self {
			Wrap::T32(load_config) => Wrap::T32(load_config.se_handler_table()).transpose(),
			Wrap::T64(load_config) => Wrap::T64(load_config.se_handler_table()).transpose(),
		}
	}
}
