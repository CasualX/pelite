use crate::*;

use super::Wrap;

/// Load Config Directory.
impl<'a, Pe32: Copy + pe32::Pe<'a>, Pe64: Copy + pe64::Pe<'a>> Wrap<pe32::LoadConfigDirectory<'a, Pe32>, pe64::LoadConfigDirectory<'a, Pe64>> {
	/// Returns the PE instance.
	#[inline]
	pub fn pe(&self) -> Wrap<Pe32, Pe64> {
		match self {
			Wrap::T32(load_config) => Wrap::T32(load_config.pe()),
			Wrap::T64(load_config) => Wrap::T64(load_config.pe()),
		}
	}
	/// Returns the size declared by the load config directory.
	#[inline]
	pub fn size(&self) -> u32 {
		match self {
			Wrap::T32(load_config) => load_config.size(),
			Wrap::T64(load_config) => load_config.size(),
		}
	}
	/// Returns the time and date stamp, or zero if the field is absent.
	#[inline]
	pub fn time_date_stamp(&self) -> u32 {
		match self {
			Wrap::T32(load_config) => load_config.time_date_stamp(),
			Wrap::T64(load_config) => load_config.time_date_stamp(),
		}
	}
	/// Returns the load config version, or version `0.0` if the field is absent.
	#[inline]
	pub fn version(&self) -> image::IMAGE_VERSION<u16> {
		match self {
			Wrap::T32(load_config) => load_config.version(),
			Wrap::T64(load_config) => load_config.version(),
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
	/// Returns the default security cookie.
	///
	/// # Errors
	///
	/// * [`Bounds`][crate::Error::Bounds]: This directory revision does not contain the cookie field, or the cookie lies outside the image.
	/// * [`Null`][crate::Error::Null]: The cookie address is zero.
	/// * [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The cookie cannot be read.
	#[inline]
	pub fn security_cookie(&self) -> Result<&'a u32> {
		match self {
			Wrap::T32(load_config) => load_config.security_cookie(),
			Wrap::T64(load_config) => load_config.security_cookie(),
		}
	}
	/// Returns the structured exception handler table.
	///
	/// # Errors
	///
	/// * [`Bounds`][crate::Error::Bounds]: This directory revision lacks the table fields, or the table lies outside the image.
	/// * [`Null`][crate::Error::Null]: The table address is zero.
	/// * [`Overflow`][crate::Error::Overflow]: The declared table length overflows.
	/// * [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The table cannot be read.
	#[inline]
	pub fn se_handler_table(&self) -> Result<Wrap<&'a [u32], &'a [u64]>> {
		match self {
			Wrap::T32(load_config) => Wrap::T32(load_config.se_handler_table()).transpose(),
			Wrap::T64(load_config) => Wrap::T64(load_config.se_handler_table()).transpose(),
		}
	}
}
