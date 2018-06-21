/*!
Resources.
*/

use std::{fmt, mem, slice};

use error::{Error, Result};
use image::*;
use util::WideStr;
use stringify::RSRC_TYPES;

//----------------------------------------------------------------

mod find;
pub use self::find::FindError;

//----------------------------------------------------------------

/// Resources filesystem.
#[derive(Copy, Clone)]
pub struct Resources<'a> {
	data: &'a [u8],
	base: u32,
}

//----------------------------------------------------------------

impl<'a> Resources<'a> {
	/// Interprets the given bytes as PE resources.
	///
	/// All offsets _except_ the final `IMAGE_RESOURCE_DATA_ENTRY::OffsetToData` are relative to the resource directory.
	/// `base` is subtracted from `OffsetToData` before being used as an offset in this resource directory.
	/// Microsoft... Why would you do this?
	///
	/// # Remarks
	///
	/// No validation is done ahead of time.
	#[inline]
	pub fn new(data: &'a [u8], base: u32) -> Resources<'a> {
		Resources { data, base }
	}
	/// Gets the root directory.
	pub fn root(self) -> Result<Directory<'a>> {
		Directory::from(self, 0)
	}
}
impl<'a> fmt::Debug for Resources<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("Resources { .. }")
	}
}

//----------------------------------------------------------------

/// Directory.
#[derive(Copy, Clone)]
pub struct Directory<'a> {
	resources: Resources<'a>,
	image: &'a IMAGE_RESOURCE_DIRECTORY,
}
impl<'a> Directory<'a> {
	fn from(resources: Resources<'a>, offset: usize) -> Result<Directory<'a>> {
		// Validate the resource directory
		let entries_offset = usize::checked_add(offset, mem::size_of::<IMAGE_RESOURCE_DIRECTORY>()).ok_or(Error::Overflow)?;
		if entries_offset > resources.data.len() {
			return Err(Error::OOB);
		}
		let image = unsafe { &*(resources.data.as_ptr().offset(offset as isize) as *const IMAGE_RESOURCE_DIRECTORY) };
		// Validate number of directory entries
		let len = image.NumberOfNamedEntries as usize + image.NumberOfIdEntries as usize;
		let entries_size = usize::checked_mul(mem::size_of::<IMAGE_RESOURCE_DIRECTORY_ENTRY>(), len).ok_or(Error::Overflow)?;
		if usize::checked_add(entries_offset, entries_size).ok_or(Error::Overflow)? > resources.data.len() {
			return Err(Error::OOB);
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
	pub fn entries(&self) -> Entries<'a> {
		// Validated by constructor
		let slice = unsafe {
			let p = (self.image as *const IMAGE_RESOURCE_DIRECTORY).offset(1) as *const IMAGE_RESOURCE_DIRECTORY_ENTRY;
			let len = self.image.NumberOfNamedEntries as usize + self.image.NumberOfIdEntries as usize;
			slice::from_raw_parts(p, len)
		};
		Entries { resources: self.resources, iter: slice.iter() }
	}
	/// Gets the named entries in this directory.
	///
	/// Note that while it would be a violation of the format spec, there's no strict safety guarantee that these are only named entries.
	pub fn named_entries(&self) -> Entries<'a> {
		// Validated by constructor
		let slice = unsafe {
			// Named entries come first in the array (see chapter "PE File Resources" in "Peering Inside the PE: A Tour of the Win32 Portable Executable File Format")
			let p = (self.image as *const IMAGE_RESOURCE_DIRECTORY).offset(1) as *const IMAGE_RESOURCE_DIRECTORY_ENTRY;
			let len = self.image.NumberOfNamedEntries as usize;
			slice::from_raw_parts(p, len)
		};
		Entries { resources: self.resources, iter: slice.iter() }
	}
	/// Gets the id entries in this directory.
	///
	/// Note that while it would be a violation of the format spec, there's no strict safety guarantee that these are only id entries.
	pub fn id_entries(&self) -> Entries<'a> {
		// Validated by the constructor
		let slice = unsafe {
			// Id entries come last in the array
			let p = (self.image as *const IMAGE_RESOURCE_DIRECTORY).offset(1 + self.image.NumberOfNamedEntries as isize) as *const IMAGE_RESOURCE_DIRECTORY_ENTRY;
			let len = self.image.NumberOfIdEntries as usize;
			slice::from_raw_parts(p, len)
		};
		Entries { resources: self.resources, iter: slice.iter() }
	}
}
impl<'a> fmt::Debug for Directory<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Directory")
			.field("entries", &self.entries())
			.finish()
	}
}

//----------------------------------------------------------------

/// Iterator over directory entries.
#[derive(Clone)]
pub struct Entries<'a> {
	resources: Resources<'a>,
	iter: slice::Iter<'a, IMAGE_RESOURCE_DIRECTORY_ENTRY>,
}
impl<'a> Entries<'a> {
	/// Gets the underlying resource directory entry images.
	pub fn image(&self) -> &'a [IMAGE_RESOURCE_DIRECTORY_ENTRY] {
		self.iter.as_slice()
	}
}
impl<'a> Iterator for Entries<'a> {
	type Item = DirectoryEntry<'a>;
	fn next(&mut self) -> Option<DirectoryEntry<'a>> {
		self.iter.next().map(|image| DirectoryEntry { resources: self.resources, image })
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
	fn count(self) -> usize {
		self.iter.count()
	}
	fn nth(&mut self, n: usize) -> Option<DirectoryEntry<'a>> {
		self.iter.nth(n).map(|image| DirectoryEntry { resources: self.resources, image })
	}
	fn last(self) -> Option<DirectoryEntry<'a>> {
		let resources = self.resources;
		self.iter.last().map(|image| DirectoryEntry { resources, image })
	}
}
impl<'a> DoubleEndedIterator for Entries<'a> {
	fn next_back(&mut self) -> Option<DirectoryEntry<'a>> {
		self.iter.next_back().map(|image| DirectoryEntry { resources: self.resources, image })
	}
}
impl<'a> ExactSizeIterator for Entries<'a> {}
impl<'a> fmt::Debug for Entries<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_list()
			.entries(self.clone())
			.finish()
	}
}

//----------------------------------------------------------------

/// Represents a resource name.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Name<'a> {
	/// Resource ID.
	///
	/// Technically allows `u32` ids, but some Windows APIs will be unable to use resources with an id which isn't `u16`.
	Id(u32),
	/// UTF-16 named resource.
	Str(&'a WideStr),
}

//----------------------------------------------------------------

/// Data or directory entry.
#[derive(Copy, Clone, Debug)]
pub enum Entry<'a> {
	Directory(Directory<'a>),
	DataEntry(DataEntry<'a>),
}

impl<'a> Entry<'a> {
	/// Returns some if the entry is a directory.
	pub fn dir(self) -> Option<Directory<'a>> {
		match self {
			Entry::Directory(dir) => Some(dir),
			Entry::DataEntry(_) => None,
		}
	}
	/// Returns some if the entry is a data entry.
	pub fn data(self) -> Option<DataEntry<'a>> {
		match self {
			Entry::Directory(_) => None,
			Entry::DataEntry(data) => Some(data),
		}
	}
}

//----------------------------------------------------------------

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
	pub fn name(&self) -> Result<Name<'a>> {
		if self.image.Name & 0x80000000 != 0 {
			let offset = (self.image.Name & !0x80000000) as usize;
			// The name is prefixed by its length in words
			let len = self.resources.data.get(offset..offset + 2).ok_or(Error::OOB)?;
			let len = unsafe { *(len.as_ptr() as *const u16) } as usize;
			// Extract the name given its length
			let name = self.resources.data.get(offset..offset + (len + 1) * 2).ok_or(Error::OOB)?;
			let name = unsafe { slice::from_raw_parts(name.as_ptr() as *const u16, len + 1) };
			let name = unsafe { WideStr::from_words_unchecked(name) };
			Ok(Name::Str(name))
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
	pub fn entry(&self) -> Result<Entry<'a>> {
		if self.is_dir() {
			let offset = (self.image.Offset & !0x80000000) as usize;
			Directory::from(self.resources, offset).map(Entry::Directory)
		}
		else {
			let offset = self.image.Offset as usize;
			DataEntry::from(self.resources, offset).map(Entry::DataEntry)
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
	resources: Resources<'a>,
	image: &'a IMAGE_RESOURCE_DATA_ENTRY,
}
impl<'a> DataEntry<'a> {
	fn from(resources: Resources<'a>, offset: usize) -> Result<DataEntry<'a>> {
		let end = usize::checked_add(offset, mem::size_of::<IMAGE_RESOURCE_DATA_ENTRY>()).ok_or(Error::Overflow)?;
		if end > resources.data.len() {
			return Err(Error::OOB);
		}
		let image = unsafe { &*(resources.data.as_ptr().offset(offset as isize) as *const IMAGE_RESOURCE_DATA_ENTRY) };
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
	pub fn data(&self) -> Result<&'a [u8]> {
		let start = u32::checked_sub(self.image.OffsetToData, self.resources.base).ok_or(Error::Overflow)?;
		let end = u32::checked_add(start, self.image.Size).ok_or(Error::Overflow)?;
		self.resources.data.get(start as usize..end as usize).ok_or(Error::OOB)
	}
}
impl<'a> fmt::Debug for DataEntry<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("DataEntry")
			.field("data.len", &self.image.Size)
			.finish()
	}
}

//----------------------------------------------------------------

struct Art {
	margin: &'static str,
	dir: &'static str,
	file: &'static str,
}
/// Art used to format a directory tree.
pub struct TreeArt {
	it: Art,
	en: Art,
}
/// Uses [box-drawing characters](https://en.wikipedia.org/wiki/Box-drawing_character) to draw the tree art.
pub static TREE_ART_U: TreeArt = TreeArt {
	it: Art { margin: "│ ", dir: "├─┬ ", file: "│   " },
	en: Art { margin: "  ", dir: "└─┬ ", file: "╵   " },
};
/// Uses ascii to draw the tree art.
pub static TREE_ART_A: TreeArt = TreeArt {
	it: Art { margin: "| ", dir: "+-. ", file: "|   " },
	en: Art { margin: "  ", dir: "`-. ", file: "`   " },
};

/// Format the directory tree.
///
/// For the art use [`&TREE_ART_U`](static.TREE_ART_U.html) or [`&TREE_ART_A`](static.TREE_ART_A.html) for unicode and ascii based tree art respectively.
///
/// Specify if this is the root directory to have its children ids printed with their names instead.
pub fn tree_fmt(f: &mut fmt::Formatter, dir: Directory, art: &TreeArt, root: bool) -> fmt::Result {
	let mut margin = [false; 32];
	tree_fmt_rec(f, &mut margin, if root { !0 } else { 0 }, art, dir)
}
fn tree_fmt_rec(f: &mut fmt::Formatter, margin: &mut [bool; 32], depth: u32, art: &TreeArt, dir: Directory) -> fmt::Result {
	// Encode if root in depth
	let (root, depth) = if depth == !0 { (true, 0) } else { (false, depth) };

	// Quiet failsafe, should never happen
	if depth as usize >= margin.len() {
		return Ok(());
	}

	let mut entries = dir.entries();
	while let Some(e) = entries.next() {
		// Print the margin
		for &is_last in &margin[0..depth as usize] {
			f.write_str(if is_last { &art.en } else { &art.it }.margin)?;
		}
		// Write the prefix
		let is_last = entries.len() == 0;
		let strings = if is_last { &art.en } else { &art.it };
		let prefix = if e.is_dir() { strings.dir } else { strings.file };
		f.write_str(prefix)?;
		// Print the file_name
		match e.name() {
			Ok(Name::Id(id)) => {
				// At root level some resource ids have special names
				let get_rsrc_name = || {
					if root { RSRC_TYPES.get(id as usize).and_then(|&a| a) }
					else { None }
				};
				if let Some(name) = get_rsrc_name() {
					write!(f, "{}\n", name)
				}
				else {
					write!(f, "{:?}\n", id)
				}
			},
			Ok(Name::Str(s)) => write!(f, "{:?}\n", s),
			e @ Err(_) => write!(f, "{:?}\n", e),
		}?;
		// If it's a directory, print it recursively
		if let Ok(Entry::Directory(dir)) = e.entry() {
			margin[depth as usize] = is_last;
			tree_fmt_rec(f, margin, depth + 1, art, dir)?;
		}
	}
	Ok(())
}
