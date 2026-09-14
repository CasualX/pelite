/*!
Load Config Directory.

# Examples

```
# #![allow(unused_variables)]
use pelite::pe64::{image, Pe, PeFile};

# #[allow(dead_code)]
fn example(file: PeFile<'_>) -> pelite::Result<()> {
	// Access the load config directory
	let load_config = file.load_config()?;
	let size = load_config.get(image::IMAGE_LOAD_CONFIG_DIRECTORY::SIZE);

	// The only bits of interest here
	let security_cookie = load_config.security_cookie()?;
	let se_handler_table = load_config.se_handler_table()?;

	Ok(())
}
```
*/

use core::{cmp, fmt, mem, ptr};

use crate::{Error, Pod, Result};

use super::image::*;
use super::Pe;

/// Load Config Directory.
///
/// For more information see the [module-level documentation](index.html).
#[derive(Copy, Clone)]
pub struct LoadConfig<'a, P> {
	pe: P,
	image: &'a [u8],
}
impl<'a, P: Pe<'a>> LoadConfig<'a, P> {
	pub(crate) fn try_from(pe: P) -> Result<LoadConfig<'a, P>> {
		let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG).ok_or(Error::Bounds)?;
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
		Ok(LoadConfig { pe, image })
	}
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
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
	pub fn get<T: Pod>(&self, field: Field<IMAGE_LOAD_CONFIG_DIRECTORY, T>) -> Option<T> {
		let offset = field.offset();
		let end = offset.checked_add(mem::size_of::<T>())?;
		let bytes = self.image.get(offset..end)?;
		// Safe because the bounds were checked above and `Pod` permits reading any initialized byte pattern as `T`.
		// Copying also avoids imposing the field's natural alignment on the image.
		Some(unsafe { ptr::read_unaligned(bytes.as_ptr().cast()) })
	}
	/// Gets the default security cookie for the image.
	pub fn security_cookie(&self) -> Result<&'a u32> {
		let security_cookie = self.get(IMAGE_LOAD_CONFIG_DIRECTORY::SECURITY_COOKIE).ok_or(Error::Bounds)?;
		self.pe.deref(Va::from(security_cookie).into())
	}
	/// Gets the structured exception handler table.
	pub fn se_handler_table(&self) -> Result<&'a [Va]> {
		let table = self.get(IMAGE_LOAD_CONFIG_DIRECTORY::SE_HANDLER_TABLE).ok_or(Error::Bounds)?;
		let count = self.get(IMAGE_LOAD_CONFIG_DIRECTORY::SE_HANDLER_COUNT).ok_or(Error::Bounds)?;
		self.pe.deref_slice(Va::from(table).into(), Va::from(count) as usize)
	}
}
impl<'a, P: Pe<'a>> fmt::Debug for LoadConfig<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("LoadConfig")
			.field("security_cookie", &format_args!("{:x?}", self.security_cookie()))
			.field("se_handler_table.len", &format_args!("{:?}", self.se_handler_table().map(|seh| seh.len())))
			.finish()
	}
}

#[cfg(feature = "serde")]
mod serde {
	use crate::util::serde_helper::*;

	use super::{LoadConfig, Pe};

	impl<'a, P: Pe<'a>> Serialize for LoadConfig<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("LoadConfig", 2)?;
			state.serialize_field("security_cookie", &self.security_cookie().ok())?;
			state.serialize_field("se_handler_table", &self.se_handler_table().ok())?;
			state.end()
		}
	}
}

#[cfg(test)]
pub(crate) fn test<'a, P: Pe<'a>>(pe: P) -> Result<()> {
	let load_config = pe.load_config()?;
	let _ = format!("{:?}", load_config);
	let _image = load_config.image_copy();
	let _size = load_config.get(IMAGE_LOAD_CONFIG_DIRECTORY::SIZE);
	let _security_cookie = load_config.security_cookie();
	let _se_handler_table = load_config.se_handler_table();
	Ok(())
}
