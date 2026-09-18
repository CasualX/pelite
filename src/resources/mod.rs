/*!
Resources.
*/

use core::{char, fmt, iter, mem, slice};

use crate::image::*;
use crate::util::AlignTo;
use crate::{Error, Pod, Result};

//----------------------------------------------------------------

mod find;
pub use self::find::ResourceFindError;

mod art;

pub mod group;
pub mod string_table;
pub mod version_info;

//----------------------------------------------------------------

/// Resources directory.
#[derive(Copy, Clone)]
pub struct ResourceDirectory<'a> {
	section: &'a [u8],
	dir: &'a IMAGE_DATA_DIRECTORY,
}
impl<'a> ResourceDirectory<'a> {
	/// Parse the bytes as PE resources.
	///
	/// No validation or integrity checking is done ahead of time.
	pub fn new(section: &'a [u8], dir: &'a IMAGE_DATA_DIRECTORY) -> ResourceDirectory<'a> {
		// All offsets _except_ the data entry offsets are relative to the resource directory.
		// Data entry offsets are relative virtual addresses from the PE image.
		// Microsoft... Why would you do this?
		ResourceDirectory { section, dir }
	}
	/// Gets the root directory.
	pub fn root(&self) -> Result<ResourceDirectoryTable<'a>> {
		ResourceDirectoryTable::try_from(*self, 0)
	}
	/// Filesystem consistency check.
	///
	/// Simply walks the filesystem checking all references are valid.
	pub fn fsck(&self) -> Result<()> {
		self.root()?.fsck()
	}

	#[inline]
	fn slice<T: Pod>(&self, offset: u32) -> Result<&'a T> {
		let start = offset as usize;
		let end = mem::size_of::<T>().checked_add(start).ok_or(Error::Overflow)?;
		// Alignment checking
		let align = mem::align_of::<T>();
		if start & (align - 1) != 0 || !self.section.as_ptr().wrapping_add(start).aligned_to(align) {
			return Err(Error::Misaligned);
		}
		// Range checking done by the indexing operator
		let bytes = self.section.get(start..end).ok_or(Error::Bounds)?;
		// Safe because size and alignment are checked and T is Pod
		Ok(unsafe { &*(bytes.as_ptr() as *const T) })
	}
	#[inline]
	#[allow(dead_code)] // unused for now...
	fn slice_len<T: Pod>(&self, offset: u32, len: usize) -> Result<&'a [T]> {
		let start = offset as usize;
		let size_of = mem::size_of::<T>().checked_mul(len).ok_or(Error::Overflow)?;
		let end = start.checked_add(size_of).ok_or(Error::Overflow)?;
		// Alignment checking
		let align = mem::align_of::<T>();
		if start & (align - 1) != 0 || !self.section.as_ptr().wrapping_add(start).aligned_to(align) {
			return Err(Error::Misaligned);
		}
		// Range checking done by the indexing operator
		let bytes = self.section.get(start..end).ok_or(Error::Bounds)?;
		Ok(unsafe { slice::from_raw_parts(bytes.as_ptr() as *const T, len) })
	}
	#[inline]
	fn slice_ws(&self, offset: u32) -> Result<&'a [u16]> {
		// The name is prefixed by its length in words
		let len = *self.slice::<u16>(offset)? as usize;
		// Extract the name given its length
		let offset = offset.checked_add(mem::size_of::<u16>() as u32).ok_or(Error::Overflow)?;
		self.slice_len(offset, len)
	}
}

impl<'a> fmt::Debug for ResourceDirectory<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("ResourceDirectory { .. }")
	}
}

//----------------------------------------------------------------

/// A table in the resource directory tree.
#[derive(Copy, Clone)]
pub struct ResourceDirectoryTable<'a> {
	resources: ResourceDirectory<'a>,
	image: &'a IMAGE_RESOURCE_DIRECTORY,
	offset: u32,
}
impl<'a> ResourceDirectoryTable<'a> {
	fn try_from(resources: ResourceDirectory<'a>, offset: u32) -> Result<ResourceDirectoryTable<'a>> {
		let image: &IMAGE_RESOURCE_DIRECTORY = resources.slice(offset)?;
		// Validate the number of directory entries
		// This code has been carefully written to avoid panicking on overflow
		// It also validates the unsafe blocks below cf. size and alignment
		let entries_size = (image.NumberOfNamedEntries as usize + image.NumberOfIdEntries as usize) * mem::size_of::<IMAGE_RESOURCE_DIRECTORY_ENTRY>();
		let entries_offset = offset as usize + mem::size_of::<IMAGE_RESOURCE_DIRECTORY>();
		if entries_size > resources.section.len() - entries_offset {
			return Err(Error::Bounds);
		}
		Ok(ResourceDirectoryTable { resources, image, offset })
	}
	/// Gets the resources.
	pub fn resources(&self) -> ResourceDirectory<'a> {
		self.resources
	}
	/// Gets the underlying resource directory image.
	pub fn image(&self) -> &'a IMAGE_RESOURCE_DIRECTORY {
		self.image
	}
	fn entry_images(&self) -> &'a [IMAGE_RESOURCE_DIRECTORY_ENTRY] {
		let offset = self.offset + mem::size_of::<IMAGE_RESOURCE_DIRECTORY>() as u32;
		let len = self.image.NumberOfNamedEntries as usize + self.image.NumberOfIdEntries as usize;
		// `try_from` validated this exact range and its alignment.
		self.resources.slice_len(offset, len).expect("validated resource directory entries")
	}
	/// Gets the directory entries.
	pub fn entries(&self) -> ResourceDirectoryEntryIter<'a, impl Clone + FnMut(&'a IMAGE_RESOURCE_DIRECTORY_ENTRY) -> ResourceDirectoryEntry<'a> + use<'a>> {
		let resources = self.resources;
		self.entry_images().iter().map(move |image| ResourceDirectoryEntry { resources, image })
	}
	/// Gets the named entries in this directory.
	///
	/// Note that while it would be a violation of the format spec, there's no strict safety guarantee that these are only named entries.
	pub fn named_entries(&self) -> ResourceDirectoryEntryIter<'a, impl Clone + FnMut(&'a IMAGE_RESOURCE_DIRECTORY_ENTRY) -> ResourceDirectoryEntry<'a> + use<'a>> {
		// Named entries come first in the array (see chapter "PE File Resources" in "Peering Inside the PE: A Tour of the Win32 Portable Executable File Format")
		let images = &self.entry_images()[..self.image.NumberOfNamedEntries as usize];
		let resources = self.resources;
		images.iter().map(move |image| ResourceDirectoryEntry { resources, image })
	}
	/// Gets the id entries in this directory.
	///
	/// Note that while it would be a violation of the format spec, there's no strict safety guarantee that these are only id entries.
	pub fn id_entries(&self) -> ResourceDirectoryEntryIter<'a, impl Clone + FnMut(&'a IMAGE_RESOURCE_DIRECTORY_ENTRY) -> ResourceDirectoryEntry<'a> + use<'a>> {
		// Id entries come last in the array.
		let images = &self.entry_images()[self.image.NumberOfNamedEntries as usize..];
		let resources = self.resources;
		images.iter().map(move |image| ResourceDirectoryEntry { resources, image })
	}
	/// Filesystem consistency check.
	///
	/// Simply walks the filesystem checking all references are valid.
	pub fn fsck(&self) -> Result<()> {
		self.entries().try_for_each(|e| e.fsck())
	}
}
#[rustfmt::skip]
impl<'a> fmt::Debug for ResourceDirectoryTable<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("ResourceDirectoryTable")
			.field("entries", &self.entries())
			.finish()
	}
}

//----------------------------------------------------------------

/// Iterator over entries in a directory.
pub type ResourceDirectoryEntryIter<'a, F> = iter::Map<slice::Iter<'a, IMAGE_RESOURCE_DIRECTORY_ENTRY>, F>;

//----------------------------------------------------------------

/// Represents a resource name.
#[derive(Copy, Clone, Debug, Eq)]
pub enum ResourceName<'a> {
	/// Resource ID.
	///
	/// Technically allows `u32` ids, but some Windows APIs will be unable to use resources with an id which isn't `u16`.
	Id(u32),
	/// UTF-16 named resource.
	Wide(&'a [u16]),
	/// UTF-8 named resource.
	///
	/// This variant is used when accepting user input and will be interpreted liberally when compared against other names:
	/// When prefixed with '#' the string is parsed as a u32 and compared to resource ids.
	/// Otherwise compares against wide strings by doing an unicode aware case sensitive comparison.
	Str(&'a str),
}
/// Predefined resource name constants.
impl<'a> ResourceName<'a> {
	pub const MANIFEST: ResourceName<'a> = ResourceName::Id(crate::image::RT_MANIFEST as u32);
	pub const VERSION: ResourceName<'a> = ResourceName::Id(crate::image::RT_VERSION as u32);
	pub const STRING: ResourceName<'a> = ResourceName::Id(crate::image::RT_STRING as u32);
	pub const GROUP_ICON: ResourceName<'a> = ResourceName::Id(crate::image::RT_GROUP_ICON as u32);
	pub const GROUP_CURSOR: ResourceName<'a> = ResourceName::Id(crate::image::RT_GROUP_CURSOR as u32);
}
impl<'a> ResourceName<'a> {
	#[inline(never)]
	fn eq_string(&self, string: &str) -> bool {
		match self {
			&ResourceName::Id(id) => {
				// Resource id strings must start with #
				if !(string.len() >= 2 && string.as_bytes()[0] == b'#') {
					false
				}
				// Followed by an integer resource id
				else if string.as_bytes()[1] > b'0' && string.as_bytes()[1] <= b'9' {
					match string[1..].parse::<u32>() {
						Ok(string_id) if id == string_id => true,
						_ => false,
					}
				}
				// Followed by a predefined resource type name
				else {
					match RSRC_TYPES.get(id as usize) {
						Some(&Some(name)) if string == name => true,
						_ => false,
					}
				}
			},
			&ResourceName::Wide(words) => char::decode_utf16(words.iter().cloned()).eq(string.chars().map(Ok)),
			&ResourceName::Str(name) => string == name,
		}
	}
	fn rename_id(self, names: &[Option<&'a str>]) -> ResourceName<'a> {
		if let ResourceName::Id(id) = self {
			if let Some(&Some(name)) = names.get(id as usize) {
				return ResourceName::Str(name);
			}
		}
		self
	}
}
impl<'a> From<u16> for ResourceName<'a> {
	fn from(id: u16) -> ResourceName<'a> {
		ResourceName::Id(id as u32)
	}
}
impl<'a> From<&'a [u16]> for ResourceName<'a> {
	fn from(words: &'a [u16]) -> ResourceName<'a> {
		ResourceName::Wide(words)
	}
}
impl<'a> From<&'a str> for ResourceName<'a> {
	fn from(name: &'a str) -> ResourceName<'a> {
		ResourceName::Str(name)
	}
}
impl PartialEq for ResourceName<'_> {
	#[inline(never)]
	fn eq(&self, rhs: &ResourceName<'_>) -> bool {
		match (*self, *rhs) {
			// Strict checking between ids and wide strings
			(ResourceName::Id(lhs), ResourceName::Id(rhs)) => lhs == rhs,
			(ResourceName::Id(_), ResourceName::Wide(_)) => false,
			(ResourceName::Wide(lhs), ResourceName::Wide(rhs)) => lhs == rhs,
			(ResourceName::Wide(_), ResourceName::Id(_)) => false,
			// When comparing against Rust strings
			(ResourceName::Str(lhs), rhs) => rhs.eq_string(lhs),
			(lhs, ResourceName::Str(rhs)) => lhs.eq_string(rhs),
		}
	}
}
impl PartialEq<str> for ResourceName<'_> {
	fn eq(&self, rhs: &str) -> bool {
		self.eq_string(rhs)
	}
}
impl PartialEq<u32> for ResourceName<'_> {
	fn eq(&self, &rhs: &u32) -> bool {
		match self {
			&ResourceName::Id(id) => id == rhs,
			_ => false,
		}
	}
}
impl fmt::Display for ResourceName<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ResourceName::Id(id) => write!(f, "#{}", id),
			ResourceName::Wide(words) => {
				for chr in char::decode_utf16(words.iter().cloned()) {
					let chr = chr.unwrap_or(char::REPLACEMENT_CHARACTER);
					fmt::Write::write_char(f, chr)?;
				}
				Ok(())
			},
			ResourceName::Str(string) => f.write_str(string),
		}
	}
}

//----------------------------------------------------------------

/// Data or directory entry.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum ResourceEntry<'a> {
	Directory(ResourceDirectoryTable<'a>),
	Data(ResourceDataEntry<'a>),
}
impl<'a> ResourceEntry<'a> {
	/// Returns some if the entry is a directory.
	pub fn dir(self) -> Option<ResourceDirectoryTable<'a>> {
		match self {
			ResourceEntry::Directory(dir) => Some(dir),
			ResourceEntry::Data(_) => None,
		}
	}
	/// Returns some if the entry is a data entry.
	pub fn data(self) -> Option<ResourceDataEntry<'a>> {
		match self {
			ResourceEntry::Directory(_) => None,
			ResourceEntry::Data(data) => Some(data),
		}
	}
}

//----------------------------------------------------------------

/// Resource directory child entry.
///
/// Contains a name and a reference to the associated data or directory entry.
#[derive(Copy, Clone)]
pub struct ResourceDirectoryEntry<'a> {
	resources: ResourceDirectory<'a>,
	image: &'a IMAGE_RESOURCE_DIRECTORY_ENTRY,
}
impl<'a> ResourceDirectoryEntry<'a> {
	/// Gets the resources.
	pub fn resources(&self) -> ResourceDirectory<'a> {
		self.resources
	}
	/// Gets the underlying resource directory entry image.
	pub fn image(&self) -> &'a IMAGE_RESOURCE_DIRECTORY_ENTRY {
		self.image
	}
	/// Gets the name for this entry.
	pub fn name(&self) -> Result<ResourceName<'a>> {
		if self.image.Name & 0x80000000 != 0 {
			let offset = self.image.Name & !0x80000000;
			let words = self.resources.slice_ws(offset)?;
			Ok(ResourceName::Wide(words))
		}
		else {
			// TODO: What if this doesn't fit in u16...
			Ok(ResourceName::Id(self.image.Name))
		}
	}
	/// Returns if this entry is a directory.
	pub fn is_dir(&self) -> bool {
		self.image.Offset & 0x80000000 != 0
	}
	/// Returns the directory or data entry for this entry.
	pub fn entry(&self) -> Result<ResourceEntry<'a>> {
		if self.is_dir() {
			let offset = self.image.Offset & !0x80000000;
			ResourceDirectoryTable::try_from(self.resources, offset).map(ResourceEntry::Directory)
		}
		else {
			let offset = self.image.Offset;
			ResourceDataEntry::try_from(self.resources, offset).map(ResourceEntry::Data)
		}
	}
	/// Filesystem consistency check.
	///
	/// Simply walks the filesystem checking all references are valid.
	pub fn fsck(&self) -> Result<()> {
		self.name()?;
		match self.entry()? {
			ResourceEntry::Directory(dir) => dir.fsck(),
			ResourceEntry::Data(data) => data.fsck(),
		}
	}
}
impl<'a> fmt::Debug for ResourceDirectoryEntry<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("ResourceDirectoryEntry")
			.field("name", &format_args!("{:?}", self.name()))
			.field("entry", &if self.is_dir() { "Directory(..)" } else { "Data(..)" })
			.finish()
	}
}

//----------------------------------------------------------------

/// Data entry.
#[derive(Copy, Clone)]
pub struct ResourceDataEntry<'a> {
	resources: ResourceDirectory<'a>,
	image: &'a IMAGE_RESOURCE_DATA_ENTRY,
}
impl<'a> ResourceDataEntry<'a> {
	fn try_from(resources: ResourceDirectory<'a>, offset: u32) -> Result<ResourceDataEntry<'a>> {
		let image = resources.slice(offset)?;
		Ok(ResourceDataEntry { resources, image })
	}
	/// Gets the resources.
	pub fn resources(&self) -> ResourceDirectory<'a> {
		self.resources
	}
	/// Gets the underlying resource data entry image.
	pub fn image(&self) -> &'a IMAGE_RESOURCE_DATA_ENTRY {
		self.image
	}
	/// Gets the actual data.
	pub fn bytes(&self) -> Result<&'a [u8]> {
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
	pub fn fsck(&self) -> Result<()> {
		self.bytes()?;
		Ok(())
	}
}
#[rustfmt::skip]
impl<'a> fmt::Debug for ResourceDataEntry<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("ResourceDataEntry")
			.field("data.len", &self.image.Size)
			.finish()
	}
}

//----------------------------------------------------------------

static RSRC_TYPES: [Option<&str>; 25] = [
	// 0
	None,
	Some("#CURSOR"),
	Some("#BITMAP"),
	Some("#ICON"),
	Some("#MENU"),
	// 5
	Some("#DIALOG"),
	Some("#STRING"),
	Some("#FONTDIR"),
	Some("#FONT"),
	Some("#ACCELERATOR"),
	// 10
	Some("#RCDATA"),
	Some("#MESSAGETABLE"),
	Some("#GROUP_CURSOR"),
	None,
	Some("#GROUP_ICON"),
	// 15
	None,
	Some("#VERSION"),
	Some("#DLGINCLUDE"),
	None,
	Some("#PLUGPLAY"),
	// 20
	Some("#VXD"),
	Some("#ANICURSOR"),
	Some("#ANIICON"),
	Some("#HTML"),
	Some("#MANIFEST"),
];

//----------------------------------------------------------------

/*
	[
		{
			"name": 12,
			"directory": [
				{
					"name": "IMPORTANT",
					"data": {
						address: 0x1000,
						size: 1234,
						code_page: 65001,
					}
				}
			]
		}
	]
*/

serde_impl! {
	use alloc::string::String;

	// Rename the toplevel directory ids to their names
	struct NamedDirectoryEntry<'a>(ResourceDirectoryEntry<'a>);
	impl<'a> Serialize for NamedDirectoryEntry<'a> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("ResourceDirectoryEntry", 2)?;
			state.serialize_field("name", &self.0.name().ok().map(|name| name.rename_id(&super::RSRC_TYPES)))?;
			state.serialize_field(if self.0.is_dir() { "directory" } else { "data" }, &self.0.entry().ok())?;
			state.end()
		}
	}

	impl<'a> Serialize for ResourceDirectory<'a> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			match self.root() {
				Ok(root) => serializer.collect_seq(root.entries().map(NamedDirectoryEntry)),
				Err(_) => serializer.serialize_none(),
			}
		}
	}
	impl<'a> Serialize for ResourceDirectoryTable<'a> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			serializer.collect_seq(self.entries())
		}
	}
	impl<'a> Serialize for ResourceName<'a> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			match self {
				ResourceName::Id(id) => id.serialize(serializer),
				ResourceName::Wide(words) if serializer.is_human_readable() => serializer.serialize_str(&String::from_utf16_lossy(words)),
				ResourceName::Wide(words) => words.serialize(serializer),
				ResourceName::Str(name) => serializer.serialize_str(name),
			}
		}
	}
	impl<'a> Serialize for ResourceDirectoryEntry<'a> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("ResourceDirectoryEntry", 2)?;
			state.serialize_field("name", &self.name().ok())?;
			state.serialize_field(if self.is_dir() { "directory" } else { "data" }, &self.entry().ok())?;
			state.end()
		}
	}
	impl<'a> Serialize for ResourceDataEntry<'a> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let is_human_readable = serializer.is_human_readable();
			let mut state = serializer.serialize_struct("ResourceDataEntry", 4)?;
			state.serialize_field("image", self.image())?;
			state.serialize_field("size", &self.size())?;
			state.serialize_field("code_page", &self.code_page())?;
			if cfg!(feature = "basenc") && is_human_readable {
				#[cfg(feature = "basenc")]
				state.serialize_field("bytes", &self.bytes().ok().map(|data| basenc::Base64Std.encode(data)))?;
			}
			else {
				state.serialize_field("bytes", &self.bytes().ok())?;
			}
			state.end()
		}
	}
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests;

#[cfg(test)]
pub(crate) fn test(resources: ResourceDirectory<'_>) -> Result<()> {
	fn test_dir(dir: ResourceDirectoryTable<'_>) {
		let _ = format!("{}", dir);
		let _ = format!("{:?}", dir);

		for entry in dir.entries() {
			let _ = format!("{:?}\n{:?}", entry.name(), entry);

			// Check consistency in id vs named entries
			if let Ok(name) = entry.name() {
				let mut id_entries;
				let mut named_entries;
				let mut entries: &mut dyn Iterator<Item = _> = match name {
					ResourceName::Id(_) => {
						id_entries = dir.id_entries();
						&mut id_entries
					},
					ResourceName::Wide(_) => {
						named_entries = dir.named_entries();
						&mut named_entries
					},
					ResourceName::Str(_) => unreachable!(),
				};
				assert!((&mut entries).any(|entry| entry.name() == Ok(name)));
			}

			// Inspect the entry recursively
			match entry.entry() {
				Ok(ResourceEntry::Data(data)) => {
					assert!(!entry.is_dir());
					let _ = format!("{:?}", data);
					let _size = data.size();
					let _code_page = data.code_page();
					let _bytes = data.bytes();
				},
				Ok(ResourceEntry::Directory(dir)) => {
					assert!(entry.is_dir());
					let _ = test_dir(dir);
				},
				Err(_) => (),
			}
		}
	}
	let _ = resources.fsck();
	println!("{}", resources);
	if let Ok(version_info) = resources.version_info() {
		self::version_info::test(version_info)
	}
	resources.root().map(test_dir)
}
