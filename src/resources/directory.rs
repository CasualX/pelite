use core::prelude::v1::*;
use core::{char, fmt, iter, mem, slice};

use std::path::Path;

use crate::image::*;
use crate::resources::{Entries, Entry, FindError, Name, Resources};
use crate::{Error};

/// Directory.
#[derive(Copy, Clone)]
pub struct Directory<'a> {
	resources: Resources<'a>,
	image: &'a IMAGE_RESOURCE_DIRECTORY,
}

impl<'a> Directory<'a> {

	pub fn try_from(resources: Resources<'a>, offset: u32) -> super::super::Result<Directory<'a>> {
		let image: &IMAGE_RESOURCE_DIRECTORY = resources.slice(offset)?;
		// Validate the number of directory entries
		// This code has been carefully written to avoid panicking on overflow
		// It also validates the unsafe blocks below cf. size and alignment
		let entries_size = (image.NumberOfNamedEntries as usize + image.NumberOfIdEntries as usize) * mem::size_of::<IMAGE_RESOURCE_DIRECTORY_ENTRY>();
		let entries_offset = offset as usize + mem::size_of::<IMAGE_RESOURCE_DIRECTORY>();
		if entries_size > resources.section.len() - entries_offset {
			return Err(Error::Bounds);
		}
		Ok(Directory { resources, image })
	}

	/// Gets the resources.
	pub fn resources(&self) -> Resources<'a> {
		self.resources
	}

	/// Gets the underlying resource directory image.
	pub fn image(&self) -> &'a IMAGE_RESOURCE_DIRECTORY {
		self.image
	}

	/// Gets the directory entries.
	pub fn entries(&self) -> Entries<'a, impl Clone + FnMut(&'a IMAGE_RESOURCE_DIRECTORY_ENTRY) -> DirectoryEntry<'a>> {
		// Validated by constructor
		let slice = unsafe {
			let p = (self.image as *const IMAGE_RESOURCE_DIRECTORY).offset(1) as *const IMAGE_RESOURCE_DIRECTORY_ENTRY;
			let len = self.image.NumberOfNamedEntries as usize + self.image.NumberOfIdEntries as usize;
			slice::from_raw_parts(p, len)
		};
		let resources = self.resources;
		slice.iter().map(move |image| DirectoryEntry { resources, image })
	}

	/// Gets the named entries in this directory.
	///
	/// Note that while it would be a violation of the format spec, there's no strict safety guarantee that these are only named entries.
	pub fn named_entries(&self) -> Entries<'a, impl Clone + FnMut(&'a IMAGE_RESOURCE_DIRECTORY_ENTRY) -> DirectoryEntry<'a>> {
		// Validated by constructor
		let slice = unsafe {
			// Named entries come first in the array (see chapter "PE File Resources" in "Peering Inside the PE: A Tour of the Win32 Portable Executable File Format")
			let p = (self.image as *const IMAGE_RESOURCE_DIRECTORY).offset(1) as *const IMAGE_RESOURCE_DIRECTORY_ENTRY;
			let len = self.image.NumberOfNamedEntries as usize;
			slice::from_raw_parts(p, len)
		};
		let resources = self.resources;
		slice.iter().map(move |image| DirectoryEntry { resources, image })
	}

	/// Gets the id entries in this directory.
	///
	/// Note that while it would be a violation of the format spec, there's no strict safety guarantee that these are only id entries.
	pub fn id_entries(&self) -> Entries<'a, impl Clone + FnMut(&'a IMAGE_RESOURCE_DIRECTORY_ENTRY) -> DirectoryEntry<'a>> {
		// Validated by the constructor
		let slice = unsafe {
			// Id entries come last in the array
			let p = ((self.image as *const IMAGE_RESOURCE_DIRECTORY).offset(1) as *const IMAGE_RESOURCE_DIRECTORY_ENTRY).offset(self.image.NumberOfNamedEntries as isize);
			let len = self.image.NumberOfIdEntries as usize;
			slice::from_raw_parts(p, len)
		};
		let resources = self.resources;
		slice.iter().map(move |image| DirectoryEntry { resources, image })
	}

	/// Filesystem consistency check.
	///
	/// Simply walks the filesystem checking all references are valid.
	pub fn fsck(&self) -> super::super::Result<()> {
		self.entries().try_for_each(|e| e.fsck())
	}

	/// Looks up the directory by name.
	pub fn get_dir(&self, name: Name<'_>) -> Result<Directory<'a>, FindError> {
		self.entries().find(|de| de.name() == Ok(name)).ok_or(FindError::NotFound)?.entry()?.dir().ok_or(FindError::UnDataEntry)
	}

	/// Gets the first entry.
	pub fn first(&self) -> Result<Entry<'a>, FindError> {
		self.entries().next().ok_or(FindError::NotFound)?.entry().map_err(FindError::Pe)
	}

	/// Gets the first data entry.
	pub fn first_data(&self) -> Result<DataEntry<'a>, FindError> {
		self.entries().next().ok_or(FindError::NotFound)?.entry()?.data().ok_or(FindError::UnDirectory)
	}

	/// Gets the first directory.
	pub fn first_dir(&self) -> Result<Directory<'a>, FindError> {
		self.entries().next().ok_or(FindError::NotFound)?.entry()?.dir().ok_or(FindError::UnDataEntry)
	}

		/// Looks up the entry by name.
	pub fn get(&self, name: Name<'_>) -> Result<Entry<'a>, FindError> {
		self.entries().find(|de| de.name() == Ok(name)).ok_or(FindError::NotFound)?.entry().map_err(FindError::Pe)
	}

	/// Looks up the data entry by name.
	pub fn get_data(&self, name: Name<'_>) -> Result<DataEntry<'a>, FindError> {
		self.entries()
			.find(|de| de.name() == Ok(name))
			.ok_or(FindError::NotFound)?
			.entry()?
			.data()
			.ok_or(FindError::UnDirectory)
	}

	/// Finds a file or directory by its path.
	pub fn find<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<Entry<'a>, FindError> {
		self.find_internal(path.as_ref())
	}

	/// Finds a file by its path.
	pub fn find_data<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<DataEntry<'a>, FindError> {
		self.find(path).and_then(|e| e.data().ok_or(FindError::UnDirectory))
	}

	/// Finds a directory by its path.
	pub fn find_dir<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<Directory<'a>, FindError> {
		self.find(path).and_then(|e| e.dir().ok_or(FindError::UnDataEntry))
	}

	pub fn find_internal(&self, path: &Path) -> Result<Entry<'a>, FindError> {
		let mut entry = Entry::Directory(*self);
		'parts: for part in path {
			// The names of resources are UTF16
			let name = Name::Str(part.to_str().ok_or(FindError::Bad8Path)?);
			match entry {
				Entry::Directory(dir) => {
					// Find a child with matching name for this part of the path
					for child in dir.entries() {
						if child.name() == Ok(name) {
							entry = child.entry()?;
							continue 'parts;
						}
					}
					return Err(FindError::NotFound);
				},
				Entry::DataEntry(_) => {
					return Err(FindError::UnDataEntry);
				},
			};
		}
		Ok(entry)
	}
}

#[rustfmt::skip]
impl<'a> fmt::Debug for Directory<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Directory")
			.field("entries", &self.entries())
			.finish()
	}
}



/// Directory child entry.
///
/// Contains a name and a reference to the associated data or directory entry.
#[derive(Copy, Clone)]
pub struct DirectoryEntry<'a> {
	resources: Resources<'a>,
	image: &'a IMAGE_RESOURCE_DIRECTORY_ENTRY,
}

impl<'a> DirectoryEntry<'a> {

	/// Gets the resources.
	pub fn resources(&self) -> Resources<'a> {
		self.resources
	}

	/// Gets the underlying resource directory entry image.
	pub fn image(&self) -> &'a IMAGE_RESOURCE_DIRECTORY_ENTRY {
		self.image
	}

	/// Gets the name for this entry.
	pub fn name(&self) -> super::super::Result<Name<'a>> {
		if self.image.Name & 0x80000000 != 0 {
			let offset = self.image.Name & !0x80000000;
			let words = self.resources.slice_ws(offset)?;
			Ok(Name::Wide(words))
		}
		else {
			// TODO: What if this doesn't fit in u16...
			Ok(Name::Id(self.image.Name))
		}
	}

	/// Returns if this entry is a directory.
	pub fn is_dir(&self) -> bool {
		self.image.Offset & 0x80000000 != 0
	}

	/// Returns the directory or data entry for this entry.
	pub fn entry(&self) -> super::super::Result<Entry<'a>> {
		if self.is_dir() {
			let offset = self.image.Offset & !0x80000000;
			Directory::try_from(self.resources, offset).map(Entry::Directory)
		}
		else {
			let offset = self.image.Offset;
			DataEntry::try_from(self.resources, offset).map(Entry::DataEntry)
		}
	}

	/// Filesystem consistency check.
	///
	/// Simply walks the filesystem checking all references are valid.
	pub fn fsck(&self) -> super::super::Result<()> {
		self.name()?;
		match self.entry()? {
			Entry::Directory(dir) => dir.fsck(),
			Entry::DataEntry(data) => data.fsck(),
		}
	}
}

impl<'a> fmt::Debug for DirectoryEntry<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("DirectoryEntry")
			.field("name", &format_args!("{:?}", self.name()))
			.field("entry", &if self.is_dir() { "Directory(..)" } else { "DataEntry(..)" })
			.finish()
	}
}

//----------------------------------------------------------------

/// Data entry.
#[derive(Copy, Clone)]
pub struct DataEntry<'a> {
	pub(crate) resources: Resources<'a>,
	image: &'a IMAGE_RESOURCE_DATA_ENTRY,
}

impl<'a> DataEntry<'a> {

	fn try_from(resources: Resources<'a>, offset: u32) -> super::super::Result<DataEntry<'a>> {
		let image = resources.slice(offset)?;
		Ok(DataEntry { resources, image })
	}

	/// Gets the resources.
	pub fn resources(&self) -> Resources<'a> {
		self.resources
	}

	/// Gets the underlying resource data entry image.
	pub fn image(&self) -> &'a IMAGE_RESOURCE_DATA_ENTRY {
		self.image
	}

	/// Gets the actual data.
	pub fn bytes(&self) -> super::super::Result<&'a [u8]> {
		let start = u32::checked_sub(self.image.OffsetToData, self.resources.dir.VirtualAddress).ok_or(Error::Overflow)?;
		let end = u32::checked_add(start, self.image.Size).ok_or(Error::Overflow)?;
		self.resources.section.get(start as usize..end as usize).ok_or(Error::Bounds)
	}

	/// Gets the data size.
	pub fn size(&self) -> usize {
		self.image.Size as usize
	}

	/// Gets the code page.
	pub fn code_page(&self) -> u32 {
		self.image.CodePage
	}
	
	/// Filesystem consistency check.
	///
	/// Simply walks the filesystem checking all references are valid.
	pub fn fsck(&self) -> super::super::Result<()> {
		self.bytes()?;
		Ok(())
	}

}

#[rustfmt::skip]
impl<'a> fmt::Debug for DataEntry<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("DataEntry")
			.field("data.len", &self.image.Size)
			.finish()
	}
}