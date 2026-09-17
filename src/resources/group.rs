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

use alloc::vec::Vec;
use core::{fmt, mem, slice};

#[cfg(feature = "std")]
use std::io;

use crate::Error;
use crate::util::AlignTo;

use super::{ResourceDirectory, ResourceFindError};

use self::image::*;

//----------------------------------------------------------------

const FILE_DIRECTORY_HEADER_SIZE: usize = 6;
const FILE_DIRECTORY_ENTRY_SIZE: usize = 16;

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
	/// Reassembles the group as an icon (`.ico`) or cursor (`.cur`) file.
	///
	/// Cursor image resources store their hotspot in the first four bytes of the
	/// image data. Those bytes are moved into the cursor directory entry in the
	/// returned file.
	pub fn to_vec(&self) -> Result<Vec<u8>, ResourceFindError> {
		let entries = self.entries();
		let directory_size = entries
			.len()
			.checked_mul(FILE_DIRECTORY_ENTRY_SIZE)
			.and_then(|size| size.checked_add(FILE_DIRECTORY_HEADER_SIZE))
			.ok_or(Error::Overflow)?;
		let mut image_offset = u32::try_from(directory_size).map_err(|_| Error::Overflow)?;
		let mut images = Vec::with_capacity(entries.len());

		for entry in entries {
			let resource = self.image(entry.nId)?;
			let image = match self.ty() {
				ResourceGroupType::Icon => resource,
				ResourceGroupType::Cursor => resource.get(4..).ok_or(Error::Bounds)?,
			};
			image_offset = image_offset.checked_add(u32::try_from(image.len()).map_err(|_| Error::Overflow)?).ok_or(Error::Overflow)?;
			images.push((resource, image));
		}

		let mut bytes = Vec::with_capacity(usize::try_from(image_offset).map_err(|_| Error::Overflow)?);
		bytes.extend_from_slice(dataview::bytes(self.image).get(..FILE_DIRECTORY_HEADER_SIZE).ok_or(Error::Bounds)?);
		let mut image_offset = directory_size as u32;
		for (entry, &(resource, image)) in entries.iter().zip(&images) {
			let entry_bytes = dataview::bytes(entry);
			match self.ty() {
				ResourceGroupType::Icon => bytes.extend_from_slice(&entry_bytes[..8]),
				ResourceGroupType::Cursor => {
					let width = u16::from_le_bytes([entry_bytes[0], entry_bytes[1]]);
					let height = u16::from_le_bytes([entry_bytes[2], entry_bytes[3]]);
					bytes.extend_from_slice(&[width as u8, height as u8, 0, 0]);
					bytes.extend_from_slice(&resource[..4]);
				},
			}
			let image_size = u32::try_from(image.len()).map_err(|_| Error::Overflow)?;
			bytes.extend_from_slice(&image_size.to_le_bytes());
			bytes.extend_from_slice(&image_offset.to_le_bytes());
			image_offset += image_size;
		}
		for (_, image) in images {
			bytes.extend_from_slice(image);
		}
		Ok(bytes)
	}
	/// Reassemble the file.
	#[cfg(feature = "std")]
	pub fn write(&self, dest: &mut dyn io::Write) -> io::Result<()> {
		let bytes = self.to_vec().map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
		dest.write_all(&bytes)
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

#[cfg(test)]
mod tests {
	use super::{FILE_DIRECTORY_ENTRY_SIZE, FILE_DIRECTORY_HEADER_SIZE};
	use crate::pe32::{Pe, PeFile};

	#[test]
	fn find_and_serialize_icon_by_name() {
		#[repr(align(8))]
		struct Aligned<T>(T);
		let image = Aligned(*include_bytes!("../../demo/Demo.dll"));
		let file = PeFile::from_bytes(&image.0).unwrap();
		let resources = file.resources().unwrap();
		let (name, group) = resources.icons().next().unwrap().unwrap();
		let icon = resources.find_icon(name).unwrap().to_vec().unwrap();

		assert_eq!(icon, group.to_vec().unwrap());
		assert_eq!(&icon[..4], &[0, 0, 1, 0]);
		assert_eq!(u16::from_le_bytes([icon[4], icon[5]]) as usize, group.entries().len());
		assert_eq!(
			u32::from_le_bytes(icon[18..22].try_into().unwrap()) as usize,
			FILE_DIRECTORY_HEADER_SIZE + group.entries().len() * FILE_DIRECTORY_ENTRY_SIZE,
		);
	}
}

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
