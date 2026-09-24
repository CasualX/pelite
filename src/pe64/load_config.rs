use super::*;

impl<'a> PeFile<'a> {
	#[doc = include_str!("../docs/load_config.md")]
	#[inline]
	pub fn load_config(self) -> Result<LoadConfigDirectory<'a, Self>> {
		LoadConfigDirectory::try_from(self)
	}
}

impl<'a> PeView<'a> {
	#[doc = include_str!("../docs/load_config.md")]
	#[inline]
	pub fn load_config(self) -> Result<LoadConfigDirectory<'a, Self>> {
		LoadConfigDirectory::try_from(self)
	}
}

/// Load Config Directory.
///
/// # Examples
///
/// ```
/// # #![allow(unused_variables)]
/// use pelite::pe64::{image, Pe, PeFile};
///
/// # #[allow(dead_code)]
/// fn example(file: PeFile<'_>) -> pelite::Result<()> {
/// 	// Access the load config directory and its basic metadata
/// 	let load_config = file.load_config()?;
/// 	let size = load_config.size();
/// 	let time_date_stamp = load_config.time_date_stamp();
/// 	let version = load_config.version();
///
/// 	// Read a field which is only present in newer directory revisions
/// 	let guard_flags = load_config.get(dataview::Field!(image::IMAGE_LOAD_CONFIG_DIRECTORY.GuardFlags));
///
/// 	// Access security-related fields when present
/// 	let security_cookie = load_config.security_cookie()?;
/// 	let se_handler_table = load_config.se_handler_table()?;
///
/// 	Ok(())
/// }
/// ```
#[derive(Copy, Clone)]
pub struct LoadConfigDirectory<'a, P> {
	pe: P,
	image: &'a [u8],
}
impl<'a, P: Copy + Pe<'a>> LoadConfigDirectory<'a, P> {
	pub(crate) fn try_from(pe: P) -> Result<LoadConfigDirectory<'a, P>> {
		let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG).ok_or(Error::Bounds)?;
		if datadir.VirtualAddress == 0 {
			return Err(Error::Null);
		}
		let directory_size = datadir.Size as usize;
		if directory_size < mem::size_of::<u32>() {
			return Err(Error::Invalid);
		}

		// The load config grows by appending fields. Bound the image by both
		// sizes so an older directory cannot expose adjacent section data as
		// fields from a newer revision.
		let header = pe.slice(datadir.VirtualAddress, mem::size_of::<u32>(), mem::align_of::<u32>())?;
		let image_size = u32::from_le_bytes([header[0], header[1], header[2], header[3]]) as usize;
		if image_size < mem::size_of::<u32>() {
			return Err(Error::Invalid);
		}
		let image_size = cmp::min(directory_size, image_size);
		let image = &pe.slice(datadir.VirtualAddress, image_size, mem::align_of::<u32>())?[..image_size];
		Ok(LoadConfigDirectory { pe, image })
	}
	/// Returns the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Returns the size declared by the load config directory.
	pub fn size(&self) -> u32 {
		// `try_from` validates that the complete size field is present.
		unsafe { ptr::read_unaligned(self.image.as_ptr().cast()) }
	}
	/// Returns the time and date stamp, or zero if the field is absent.
	pub fn time_date_stamp(&self) -> u32 {
		self.get(dataview::Field!(IMAGE_LOAD_CONFIG_DIRECTORY.TimeDateStamp)).unwrap_or_default()
	}
	/// Returns the load config version, or version `0.0` if the field is absent.
	pub fn version(&self) -> IMAGE_VERSION<u16> {
		self.get(dataview::Field!(IMAGE_LOAD_CONFIG_DIRECTORY.Version)).unwrap_or(IMAGE_VERSION { Major: 0, Minor: 0 })
	}
	/// Copies the load config directory into the latest known image structure.
	///
	/// Fields which are not present in this revision of the directory are zero.
	pub fn image_copy(&self) -> IMAGE_LOAD_CONFIG_DIRECTORY {
		// The source was bounded to the logical directory in `try_from`, and a
		// future directory is truncated to the known structure.
		let mut image: IMAGE_LOAD_CONFIG_DIRECTORY = dataview::zeroed();
		let image_bytes = dataview::bytes_mut(&mut image);
		let copy_len = cmp::min(self.image.len(), image_bytes.len());
		image_bytes[..copy_len].copy_from_slice(&self.image[..copy_len]);
		image
	}
	/// Copies a field from the load config directory if it is present in this revision of the structure.
	#[inline]
	pub fn get<T: Pod>(&self, field: dataview::Field<IMAGE_LOAD_CONFIG_DIRECTORY, T>) -> Option<T> {
		let bytes = self.image.get(field.span())?;
		// Safe because the bounds were checked above and `Pod` permits reading any initialized byte pattern as `T`.
		// Copying also avoids imposing the field's natural alignment on the image.
		Some(unsafe { ptr::read_unaligned(bytes.as_ptr().cast()) })
	}
	/// Returns the default security cookie.
	///
	/// # Errors
	///
	/// * [`Bounds`][crate::Error::Bounds]: This directory revision does not contain the cookie field, or the cookie lies outside the image.
	/// * [`Null`][crate::Error::Null]: The cookie address is zero.
	/// * [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The cookie cannot be read.
	pub fn security_cookie(&self) -> Result<&'a u32> {
		let security_cookie = self.get(dataview::Field!(IMAGE_LOAD_CONFIG_DIRECTORY.SecurityCookie)).ok_or(Error::Bounds)?;
		self.pe.deref(Va::from(security_cookie).into())
	}
	/// Returns the structured exception handler table.
	///
	/// # Errors
	///
	/// * [`Bounds`][crate::Error::Bounds]: This directory revision lacks the table fields, or the table lies outside the image.
	/// * [`Null`][crate::Error::Null]: The table address is zero.
	/// * [`Overflow`][crate::Error::Overflow]: The declared table length overflows.
	/// * [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The table cannot be read.
	pub fn se_handler_table(&self) -> Result<&'a [Va]> {
		let table = self.get(dataview::Field!(IMAGE_LOAD_CONFIG_DIRECTORY.SEHandlerTable)).ok_or(Error::Bounds)?;
		let count = self.get(dataview::Field!(IMAGE_LOAD_CONFIG_DIRECTORY.SEHandlerCount)).ok_or(Error::Bounds)?;
		self.pe.deref_slice(Va::from(table).into(), Va::from(count) as usize)
	}
}
impl<'a, P: Copy + Pe<'a>> fmt::Debug for LoadConfigDirectory<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("LoadConfigDirectory")
			.field("size", &self.size())
			.field("time_date_stamp", &self.time_date_stamp())
			.field("version", &self.version())
			.field("guard_flags", &format_args!("{:x?}", self.get(dataview::Field!(IMAGE_LOAD_CONFIG_DIRECTORY.GuardFlags))))
			.field("security_cookie", &format_args!("{:x?}", self.security_cookie()))
			.field("se_handler_table.len", &format_args!("{:?}", self.se_handler_table().map(|seh| seh.len())))
			.finish()
	}
}

serde_impl! {
	impl<'a, P: Copy + Pe<'a>> Serialize for LoadConfigDirectory<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("LoadConfigDirectory", 4)?;
			state.serialize_field("image", &self.image_copy())?;
			state.serialize_field("size", &self.size())?;
			state.serialize_field("security_cookie", &self.security_cookie().ok())?;
			state.serialize_field("se_handler_table", &self.se_handler_table().ok())?;
			state.end()
		}
	}
}

#[cfg(test)]
pub(crate) fn test_load_config<'a, P: Copy + Pe<'a>>(pe: P) -> Result<()> {
	let load_config = pe.load_config()?;
	let _ = format!("{:?}", load_config);
	let _image = load_config.image_copy();
	let _size = load_config.size();
	let _time_date_stamp = load_config.time_date_stamp();
	let _version = load_config.version();
	let _guard_flags = load_config.get(dataview::Field!(IMAGE_LOAD_CONFIG_DIRECTORY.GuardFlags));
	let _security_cookie = load_config.security_cookie();
	let _se_handler_table = load_config.se_handler_table();
	Ok(())
}
