/*!
Resources.
*/

use ::std::{fmt, slice, mem};

use super::image::*;
use super::error::{Error, Result};
use super::util::WideStr;

//----------------------------------------------------------------

/// Resources filesystem.
#[derive(Copy, Clone)]
pub struct Resources<'a> {
	data: &'a [u8],
	vbase: u32,
}

//----------------------------------------------------------------

impl<'a> Resources<'a> {
	/// Interprets the given bytes as PE resources.
	///
	/// All offsets _except_ the final `IMAGE_RESOURCE_DATA_ENTRY::OffsetToData` are relative to the resource directory.
	/// `vbase` is subtracted from `OffsetToData` before being used as an offset in this resource directory.
	/// Microsoft... Why would you do this?
	///
	/// # Remarks
	///
	/// No validation is done ahead of time.
	#[inline]
	pub fn new(data: &'a [u8], vbase: u32) -> Resources<'a> {
		Resources {
			data: data,
			vbase: vbase,
		}
	}
	/// Gets the root directory.
	pub fn root(self) -> Result<Directory<'a>> {
		Directory::from(self, 0)
	}
}

//----------------------------------------------------------------

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
	pub fn resources(&self) -> Resources<'a> {
		self.resources
	}
	pub fn image(&self) -> &'a IMAGE_RESOURCE_DIRECTORY {
		self.image
	}
	pub fn entries(&self) -> Entries<'a> {
		// Validated by constructor
		let slice = unsafe {
			let p = (self.image as *const _).offset(1) as *const IMAGE_RESOURCE_DIRECTORY_ENTRY;
			let len = self.image.NumberOfNamedEntries as usize + self.image.NumberOfIdEntries as usize;
			slice::from_raw_parts(p, len)
		};
		Entries { resources: self.resources, iter: slice.iter() }
	}
	pub fn named_entries(&self) -> Entries<'a> {
		// Validated by constructor
		let slice = unsafe {
			// Named entries come first in the array (see chapter "PE File Resources" in "Peering Inside the PE: A Tour of the Win32 Portable Executable File Format")
			let p = (self.image as *const _).offset(1) as *const IMAGE_RESOURCE_DIRECTORY_ENTRY;
			let len = self.image.NumberOfNamedEntries as usize;
			slice::from_raw_parts(p, len)
		};
		Entries { resources: self.resources, iter: slice.iter() }
	}
	pub fn id_entries(&self) -> Entries<'a> {
		// Validated by the constructor
		let slice = unsafe {
			// Id entries come last in the array
			let p = (self.image as *const _).offset(1 + self.image.NumberOfNamedEntries as isize) as *const IMAGE_RESOURCE_DIRECTORY_ENTRY;
			let len = self.image.NumberOfIdEntries as usize;
			slice::from_raw_parts(p, len)
		};
		Entries { resources: self.resources, iter: slice.iter() }
	}
}

//----------------------------------------------------------------

#[derive(Clone)]
pub struct Entries<'a> {
	resources: Resources<'a>,
	iter: slice::Iter<'a, IMAGE_RESOURCE_DIRECTORY_ENTRY>,
}
impl<'a> Entries<'a> {
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

//----------------------------------------------------------------

/// Represents a resource name.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Name<'a> {
	/// A u16 resource ID.
	Id(u32),
	/// UTF-16 named resource.
	Str(&'a WideStr),
}

//----------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
pub enum Entry<'a> {
	Directory(Directory<'a>),
	DataEntry(DataEntry<'a>),
}

impl<'a> Entry<'a> {
	pub fn dir(self) -> Option<Directory<'a>> {
		match self {
			Entry::Directory(dir) => Some(dir),
			Entry::DataEntry(_) => None,
		}
	}
	pub fn data(self) -> Option<DataEntry<'a>> {
		match self {
			Entry::Directory(_) => None,
			Entry::DataEntry(data) => Some(data),
		}
	}
}

//----------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct DirectoryEntry<'a> {
	resources: Resources<'a>,
	image: &'a IMAGE_RESOURCE_DIRECTORY_ENTRY,
}
impl<'a> DirectoryEntry<'a> {
	pub fn resources(&self) -> Resources<'a> {
		self.resources
	}
	pub fn image(&self) -> &'a IMAGE_RESOURCE_DIRECTORY_ENTRY {
		self.image
	}
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
	pub fn is_dir(&self) -> bool {
		self.image.Offset & 0x80000000 != 0
	}
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

//----------------------------------------------------------------

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
	pub fn resources(&self) -> Resources<'a> {
		self.resources
	}
	pub fn image(&self) -> &'a IMAGE_RESOURCE_DATA_ENTRY {
		self.image
	}
	pub fn data(&self) -> Result<&'a [u8]> {
		let start = u32::checked_sub(self.image.OffsetToData, self.resources.vbase).ok_or(Error::Overflow)?;
		let end = u32::checked_add(start, self.image.Size).ok_or(Error::Overflow)?;
		self.resources.data.get(start as usize..end as usize).ok_or(Error::OOB)
	}
}

//----------------------------------------------------------------

struct Art {
	margin: &'static str,
	dir: &'static str,
	file: &'static str,
}
pub struct TreeArt {
	it: Art,
	en: Art,
}
pub static TREE_ART_U: TreeArt = TreeArt {
	it: Art { margin: "│ ", dir: "├─┬ ", file: "│   " },
	en: Art { margin: "  ", dir: "└─┬ ", file: "╵   " },
};
pub static TREE_ART_A: TreeArt = TreeArt {
	it: Art { margin: "| ", dir: "+-. ", file: "|   " },
	en: Art { margin: "  ", dir: "`-. ", file: "`   " },
};

/// Tree formatting.
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
					use ::strings::RSRC_TYPES;
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

//----------------------------------------------------------------
// Formatting

impl<'a> fmt::Debug for Resources<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("Resources\n")?;
		match self.root() {
			Ok(dir) => {
				tree_fmt(f, dir, &TREE_ART_A, true)
			},
			e @ Err(_) => {
				write!(f, "{:?}\n", e)
			}
		}
	}
}

impl<'a> fmt::Debug for Directory<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.image.fmt(f)?;
		for entry in self.entries() {
			write!(f, "{:?}", entry)?;
			match entry.entry() {
				Ok(Entry::DataEntry(data_entry)) => write!(f, "{:?}", data_entry),
				Ok(Entry::Directory(directory)) => write!(f, "{:?}", directory),
				e @ Err(_) => write!(f, "{:?}", e),
			}?;
		}
		Ok(())
	}
}

impl<'a> fmt::Debug for DirectoryEntry<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let a = if self.is_dir() { "<DIR>" } else { "<FILE>" };
		write!(f, "{}: {:?}\n", a, self.name().unwrap())
	}
}

impl<'a> fmt::Debug for DataEntry<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.image.fmt(f)
	}
}
