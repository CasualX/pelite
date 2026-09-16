/*!
Group Icons and Cursors.

References:

* <http://msdn.microsoft.com/en-us/library/ms997538.aspx>
* <https://devblogs.microsoft.com/oldnewthing/20120720-00/?p=7083>
* <https://github.com/MathewSachin/NIco/wiki/Ico,-Cur-and-PE-Formats>

# Examples

The following example prints all group icon resource names which contain a PNG image.

```
// Aqcuire the resources of a Portable Executable file
let resources: pelite::resources::ResourceDirectory;

# fn example(resources: pelite::resources::ResourceDirectory<'_>) {
// Iterate over the group icons in the resources and throw away any invalid results
// If the resources contain no group icons the iterator is empty
for (name, group) in resources.icons().filter_map(Result::ok) {
	// Enumerate the entries in the group
	for entry in group.entries() {
		// Fetch the image data for this entry
		match group.image(entry.nId) {
			Ok(image) => {
				// Check if the image data starts with the PNG magic bytes
				if image.starts_with(b"\x89PNG") {
					println!("{}: contains PNG", name);
				}
			},
			Err(err) => {
				println!("{}: Error {}!", name, err)
			},
		}
	}
}
# }
```

 */

#[cfg(all(feature = "std", feature = "serde"))]
use alloc::vec::Vec;
use core::{fmt, mem, slice};

#[cfg(feature = "std")]
use std::io;

use crate::util::AlignTo;
use crate::Error;

use super::{ResourceDirectory, ResourceFindError};

use self::image::*;

//----------------------------------------------------------------

/// Icon or Cursor type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ResourceGroupType {
	Icon,
	Cursor,
}
impl ResourceGroupType {
	#[inline]
	pub fn id(self) -> u16 {
		match self {
			ResourceGroupType::Icon => crate::image::RT_ICON,
			ResourceGroupType::Cursor => crate::image::RT_CURSOR,
		}
	}
}
impl<'a> From<ResourceGroupType> for super::ResourceName<'a> {
	fn from(resource_type: ResourceGroupType) -> super::ResourceName<'a> {
		resource_type.id().into()
	}
}

/// Group resources, Icons and Cursors.
#[derive(Copy, Clone)]
pub struct ResourceGroup<'a> {
	resources: ResourceDirectory<'a>,
	image: &'a GRPICONDIR,
}
impl<'a> ResourceGroup<'a> {
	/// Parses the resource group from the byte slice.
	///
	/// The pixel data of the group resource is stored in separate data entries, requiring the resources to access.
	pub fn new(resources: ResourceDirectory<'a>, bytes: &'a [u8]) -> Result<ResourceGroup<'a>, Error> {
		if !bytes.as_ptr().aligned_to(2) {
			return Err(Error::Misaligned);
		}
		if bytes.len() < mem::size_of::<GRPICONDIR>() {
			return Err(Error::Bounds);
		}
		let image: &'a GRPICONDIR = unsafe { &*(bytes.as_ptr() as *const GRPICONDIR) };
		if image.idReserved != 0 || !(image.idType == 1 || image.idType == 2) {
			return Err(Error::BadMagic);
		}
		let total_size = mem::size_of::<GRPICONDIR>() + image.idCount as usize * mem::size_of::<GRPICONDIRENTRY>();
		if bytes.len() != total_size {
			return Err(Error::Bounds);
		}
		Ok(ResourceGroup { resources, image })
	}
	/// Gets the Group header.
	pub fn header(&self) -> &'a GRPICONDIR {
		self.image
	}
	/// Gets the Group entries.
	pub fn entries(&self) -> &'a [GRPICONDIRENTRY] {
		let len = self.image.idCount as usize;
		// Checked by try_from constructor
		unsafe {
			let ptr = (self.image as *const GRPICONDIR).offset(1) as *const GRPICONDIRENTRY;
			slice::from_raw_parts(ptr, len)
		}
	}
	/// Gets the Group resource type.
	pub fn ty(&self) -> ResourceGroupType {
		match self.image.idType {
			1 => ResourceGroupType::Icon,
			2 => ResourceGroupType::Cursor,
			_ => unreachable!(), // Checked by constructor
		}
	}
	/// Gets the image data for the given icon id.
	pub fn image(&self, id: u16) -> Result<&'a [u8], ResourceFindError> {
		self.resources.root()?.get_dir(self.ty().into())?.get_dir(id.into())?.first_data()?.bytes().map_err(ResourceFindError::Pe)
	}
	/// Reassemble the file.
	#[cfg(feature = "std")]
	pub fn write(&self, dest: &mut dyn io::Write) -> io::Result<()> {
		// Start by appending the header
		dest.write(dataview::bytes(self.image))?;
		// Write all the icon entries
		let entries = self.entries();
		let mut image_offset = (6 + entries.len() * 16) as u32;
		for entry in entries {
			// Fixup the dwImageOffset field of the icon entry
			// NOTE! It is expected that the actual icon data size matches dwBytesInRes information!
			let mut icon_entry = [0u32; 4];
			dataview::bytes_mut(&mut icon_entry)[..14].copy_from_slice(dataview::bytes(entry));
			icon_entry[3] = image_offset;
			image_offset += entry.bytes_in_resource();
			dest.write(dataview::bytes(&icon_entry))?;
		}
		// Append the bytes for every entry
		for entry in entries {
			// Find the Icon data and append it
			// FIXME! What do if dwBytesInRes does not match the icon data size?
			// Ignoring this check may lead to corrupt icon files
			if let Ok(bytes) = self.image(entry.nId) {
				// assert_eq!(entry.bytes_in_resource() as usize, bytes.len());
				dest.write(bytes)?;
			}
		}
		Ok(())
	}
}

#[rustfmt::skip]
impl fmt::Debug for ResourceGroup<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("ResourceGroup")
			.field("type", &self.ty())
			.field("entries.len", &self.entries().len())
			.finish()
	}
}

#[cfg(all(feature = "std", feature = "serde"))]
impl serde::Serialize for ResourceGroup<'_> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		let mut bytes = Vec::new();
		mem::forget(self.write(&mut bytes));
		#[cfg(feature = "basenc")]
		if serializer.is_human_readable() {
			return serializer.serialize_str(&basenc::Base64Std.encode(&bytes));
		}
		serializer.serialize_bytes(&bytes)
	}
}

/// Group Icon.
pub type GroupIcon<'a> = ResourceGroup<'a>;
/// Group Cursor.
pub type GroupCursor<'a> = ResourceGroup<'a>;

//----------------------------------------------------------------

#[allow(non_snake_case)]
pub mod image {
	use crate::Pod;
	#[derive(Copy, Clone, Debug)]
	#[repr(C)]
	pub struct GRPICONDIR {
		pub idReserved: u16,
		pub idType: u16,
		pub idCount: u16,
		pub idEntries: [GRPICONDIRENTRY; 0],
	}
	#[derive(Copy, Clone, Debug)]
	#[repr(C)]
	pub struct GRPICONDIRENTRY {
		pub bWidth: u8,
		pub bHeight: u8,
		pub bColorCount: u8,
		pub bReserved: u8,
		pub wPlanes: u16,
		pub wBitCount: u16,
		pub dwBytesInResLo: u16,
		pub dwBytesInResHi: u16,
		pub nId: u16,
	}
	impl GRPICONDIRENTRY {
		pub fn bytes_in_resource(&self) -> u32 {
			self.dwBytesInResHi as u32 * 0x10000 + self.dwBytesInResLo as u32
		}
	}
	unsafe impl Pod for GRPICONDIR {}
	unsafe impl Pod for GRPICONDIRENTRY {}
}
