/*!
Resources.
*/

use std::prelude::v1::*;

use std::{char, fmt, iter, mem, slice};

use crate::image::*;
use crate::{Error, Pod, Result};

//----------------------------------------------------------------

mod find;
pub use self::find::FindError;

mod art;

pub mod group;
pub mod version_info;

//----------------------------------------------------------------

/// Resources filesystem.
#[derive(Copy, Clone)]
pub struct Resources<'a> {
    section: &'a [u8],
    dir: &'a IMAGE_DATA_DIRECTORY,
}
impl<'a> Resources<'a> {
    /// Parse the bytes as PE resources.
    ///
    /// No validation or integrity checking is done ahead of time.
    pub fn new(section: &'a [u8], dir: &'a IMAGE_DATA_DIRECTORY) -> Resources<'a> {
        // All offsets _except_ the data entry offsets are relative to the resource directory.
        // Data entry offsets are relative virtual addresses from the PE image.
        // Microsoft... Why would you do this?
        Resources { section, dir }
    }
    /// Gets the root directory.
    pub fn root(&self) -> Result<Directory<'a>> {
        Directory::try_from(*self, 0)
    }
    /// Filesystem consistency check.
    ///
    /// Simply walks the filesystem checking all references are valid.
    pub fn fsck(&self) -> Result<()> {
        self.root()?.fsck()
    }

    #[inline]
    fn slice<T>(&self, offset: u32) -> Result<&'a T>
    where
        T: Pod,
    {
        let start = offset as usize;
        let end = mem::size_of::<T>().wrapping_add(start);
        // Alignment checking
        if !cfg!(feature = "unsafe_alignment") && start & (mem::align_of::<T>() - 1) != 0 {
            return Err(Error::Misaligned);
        }
        // Range checking done by the indexing operator
        let bytes = self.section.get(start..end).ok_or(Error::Bounds)?;
        // Safe because size and alignment are checked and T is Pod
        Ok(unsafe { &*(bytes.as_ptr() as *const T) })
    }
    #[inline]
    #[allow(dead_code)] // unused for now...
    fn slice_len<T>(&self, offset: u32, len: usize) -> Result<&'a [T]>
    where
        T: Pod,
    {
        let start = offset as usize;
        let size_of = mem::size_of::<T>()
            .checked_mul(len)
            .ok_or(Error::Overflow)?;
        let end = start.wrapping_add(size_of);
        // Alignment checking
        if !cfg!(feature = "unsafe_alignment") && start & (mem::align_of::<T>() - 1) != 0 {
            return Err(Error::Misaligned);
        }
        // Range checking done by the indexing operator
        let bytes = self.section.get(start..end).ok_or(Error::Bounds)?;
        Ok(unsafe { slice::from_raw_parts(bytes.as_ptr() as *const T, len) })
    }
    #[inline]
    fn slice_ws(&self, offset: u32) -> Result<&'a [u16]> {
        let offset = offset as usize;
        // Alignment checking
        if !cfg!(feature = "unsafe_alignment") && offset & 1 != 0 {
            return Err(Error::Misaligned);
        }
        // The name is prefixed by its length in words
        let len = self.section.get(offset..offset + 2).ok_or(Error::Bounds)?;
        let len = unsafe { *(len.as_ptr() as *const u16) } as usize;
        // Extract the name given its length
        let name = self
            .section
            .get(offset + 2..offset + 2 + len * 2)
            .ok_or(Error::Bounds)?;
        let name = unsafe { slice::from_raw_parts(name.as_ptr() as *const u16, len) };
        Ok(name)
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
    fn try_from(resources: Resources<'a>, offset: u32) -> Result<Directory<'a>> {
        let image: &IMAGE_RESOURCE_DIRECTORY = resources.slice(offset)?;
        // Validate the number of directory entries
        // This code has been carefully written to avoid panicking on overflow
        // It also validates the unsafe blocks below cf. size and alignment
        let entries_size = (image.NumberOfNamedEntries as usize + image.NumberOfIdEntries as usize)
            * mem::size_of::<IMAGE_RESOURCE_DIRECTORY_ENTRY>();
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
    pub fn entries(
        &self,
    ) -> Entries<'a, impl Clone + FnMut(&'a IMAGE_RESOURCE_DIRECTORY_ENTRY) -> DirectoryEntry<'a>>
    {
        // Validated by constructor
        let slice = unsafe {
            let p = (self.image as *const IMAGE_RESOURCE_DIRECTORY).offset(1)
                as *const IMAGE_RESOURCE_DIRECTORY_ENTRY;
            let len =
                self.image.NumberOfNamedEntries as usize + self.image.NumberOfIdEntries as usize;
            slice::from_raw_parts(p, len)
        };
        let resources = self.resources;
        slice
            .iter()
            .map(move |image| DirectoryEntry { resources, image })
    }
    /// Gets the named entries in this directory.
    ///
    /// Note that while it would be a violation of the format spec, there's no strict safety guarantee that these are only named entries.
    pub fn named_entries(
        &self,
    ) -> Entries<'a, impl Clone + FnMut(&'a IMAGE_RESOURCE_DIRECTORY_ENTRY) -> DirectoryEntry<'a>>
    {
        // Validated by constructor
        let slice = unsafe {
            // Named entries come first in the array (see chapter "PE File Resources" in "Peering Inside the PE: A Tour of the Win32 Portable Executable File Format")
            let p = (self.image as *const IMAGE_RESOURCE_DIRECTORY).offset(1)
                as *const IMAGE_RESOURCE_DIRECTORY_ENTRY;
            let len = self.image.NumberOfNamedEntries as usize;
            slice::from_raw_parts(p, len)
        };
        let resources = self.resources;
        slice
            .iter()
            .map(move |image| DirectoryEntry { resources, image })
    }
    /// Gets the id entries in this directory.
    ///
    /// Note that while it would be a violation of the format spec, there's no strict safety guarantee that these are only id entries.
    pub fn id_entries(
        &self,
    ) -> Entries<'a, impl Clone + FnMut(&'a IMAGE_RESOURCE_DIRECTORY_ENTRY) -> DirectoryEntry<'a>>
    {
        // Validated by the constructor
        let slice = unsafe {
            // Id entries come last in the array
            let p = ((self.image as *const IMAGE_RESOURCE_DIRECTORY).offset(1)
                as *const IMAGE_RESOURCE_DIRECTORY_ENTRY)
                .offset(self.image.NumberOfNamedEntries as isize);
            let len = self.image.NumberOfIdEntries as usize;
            slice::from_raw_parts(p, len)
        };
        let resources = self.resources;
        slice
            .iter()
            .map(move |image| DirectoryEntry { resources, image })
    }
    /// Filesystem consistency check.
    ///
    /// Simply walks the filesystem checking all references are valid.
    pub fn fsck(&self) -> Result<()> {
        self.entries().try_for_each(|e| e.fsck())
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

/// Iterator over entries in a directory.
pub type Entries<'a, F> = iter::Map<slice::Iter<'a, IMAGE_RESOURCE_DIRECTORY_ENTRY>, F>;

//----------------------------------------------------------------

/// Represents a resource name.
#[derive(Copy, Clone, Debug, Eq)]
pub enum Name<'a> {
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
impl<'a> Name<'a> {
    pub const MANIFEST: Name<'a> = Name::Id(crate::image::RT_MANIFEST as u32);
    pub const VERSION: Name<'a> = Name::Id(crate::image::RT_VERSION as u32);
    pub const GROUP_ICON: Name<'a> = Name::Id(crate::image::RT_GROUP_ICON as u32);
    pub const GROUP_CURSOR: Name<'a> = Name::Id(crate::image::RT_GROUP_CURSOR as u32);
}
impl<'a> Name<'a> {
    #[inline(never)]
    fn eq_string(&self, string: &str) -> bool {
        match self {
            &Name::Id(id) => {
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
            }
            &Name::Wide(words) => {
                char::decode_utf16(words.iter().cloned()).eq(string.chars().map(Ok))
            }
            &Name::Str(name) => string == name,
        }
    }
    fn rename_id(self, names: &[Option<&'a str>]) -> Name<'a> {
        if let Name::Id(id) = self {
            if let Some(&Some(name)) = names.get(id as usize) {
                return Name::Str(name);
            }
        }
        self
    }
}
impl<'a> From<u16> for Name<'a> {
    fn from(id: u16) -> Name<'a> {
        Name::Id(id as u32)
    }
}
impl<'a> From<&'a [u16]> for Name<'a> {
    fn from(words: &'a [u16]) -> Name<'a> {
        Name::Wide(words)
    }
}
impl<'a> From<&'a str> for Name<'a> {
    fn from(name: &'a str) -> Name<'a> {
        Name::Str(name)
    }
}
impl PartialEq for Name<'_> {
    #[inline(never)]
    fn eq(&self, rhs: &Name<'_>) -> bool {
        match (*self, *rhs) {
            // Strict checking between ids and wide strings
            (Name::Id(lhs), Name::Id(rhs)) => lhs == rhs,
            (Name::Id(_), Name::Wide(_)) => false,
            (Name::Wide(lhs), Name::Wide(rhs)) => lhs == rhs,
            (Name::Wide(_), Name::Id(_)) => false,
            // When comparing against Rust strings
            (Name::Str(lhs), rhs) => rhs.eq_string(lhs),
            (lhs, Name::Str(rhs)) => lhs.eq_string(rhs),
        }
    }
}
impl PartialEq<str> for Name<'_> {
    fn eq(&self, rhs: &str) -> bool {
        self.eq_string(rhs)
    }
}
impl PartialEq<u32> for Name<'_> {
    fn eq(&self, &rhs: &u32) -> bool {
        match self {
            &Name::Id(id) => id == rhs,
            _ => false,
        }
    }
}
impl fmt::Display for Name<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Name::Id(id) => write!(f, "#{}", id),
            Name::Wide(words) => {
                for chr in char::decode_utf16(words.iter().cloned()) {
                    let chr = chr.unwrap_or(char::REPLACEMENT_CHARACTER);
                    fmt::Write::write_char(f, chr)?;
                }
                Ok(())
            }
            Name::Str(string) => f.write_str(string),
        }
    }
}

//----------------------------------------------------------------

/// Data or directory entry.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize), serde(untagged))]
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
            let offset = self.image.Name & !0x80000000;
            let words = self.resources.slice_ws(offset)?;
            Ok(Name::Wide(words))
        } else {
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
            let offset = self.image.Offset & !0x80000000;
            Directory::try_from(self.resources, offset).map(Entry::Directory)
        } else {
            let offset = self.image.Offset;
            DataEntry::try_from(self.resources, offset).map(Entry::DataEntry)
        }
    }
    /// Filesystem consistency check.
    ///
    /// Simply walks the filesystem checking all references are valid.
    pub fn fsck(&self) -> Result<()> {
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
            .field(
                "entry",
                &if self.is_dir() {
                    "Directory(..)"
                } else {
                    "DataEntry(..)"
                },
            )
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
    fn try_from(resources: Resources<'a>, offset: u32) -> Result<DataEntry<'a>> {
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
    pub fn bytes(&self) -> Result<&'a [u8]> {
        let start = u32::checked_sub(self.image.OffsetToData, self.resources.dir.VirtualAddress)
            .ok_or(Error::Overflow)?;
        let end = u32::checked_add(start, self.image.Size).ok_or(Error::Overflow)?;
        self.resources
            .section
            .get(start as usize..end as usize)
            .ok_or(Error::Bounds)
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
impl<'a> fmt::Debug for DataEntry<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DataEntry")
            .field("data.len", &self.image.Size)
            .finish()
    }
}

//----------------------------------------------------------------

static RSRC_TYPES: [Option<&str>; 25] = [
    /* 0*/ None,
    Some("#CURSOR"),
    Some("#BITMAP"),
    Some("#ICON"),
    Some("#MENU"),
    /* 5*/ Some("#DIALOG"),
    Some("#STRING"),
    Some("#FONTDIR"),
    Some("#FONT"),
    Some("#ACCELERATOR"),
    /*10*/ Some("#RCDATA"),
    Some("#MESSAGETABLE"),
    Some("#GROUP_CURSOR"),
    None,
    Some("#GROUP_ICON"),
    /*15*/ None,
    Some("#VERSION"),
    Some("#DLGINCLUDE"),
    None,
    Some("#PLUGPLAY"),
    /*20*/ Some("#VXD"),
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

#[cfg(feature = "serde")]
mod serde {
    use super::{DataEntry, Directory, DirectoryEntry, Name, Resources};
    use crate::util::serde_helper::*;

    // Rename the toplevel directory ids to their names
    struct NamedDirectoryEntry<'a>(DirectoryEntry<'a>);
    impl<'a> Serialize for NamedDirectoryEntry<'a> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut state = serializer.serialize_struct("DirectoryEntry", 2)?;
            state.serialize_field(
                "name",
                &self
                    .0
                    .name()
                    .ok()
                    .map(|name| name.rename_id(&super::RSRC_TYPES)),
            )?;
            state.serialize_field(
                if self.0.is_dir() { "directory" } else { "data" },
                &self.0.entry().ok(),
            )?;
            state.end()
        }
    }

    impl<'a> Serialize for Resources<'a> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match self.root() {
                Ok(root) => serializer.collect_seq(root.entries().map(NamedDirectoryEntry)),
                Err(_) => serializer.serialize_none(),
            }
        }
    }
    impl<'a> Serialize for Directory<'a> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.collect_seq(self.entries())
        }
    }
    impl<'a> Serialize for Name<'a> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            match self {
                Name::Id(id) => id.serialize(serializer),
                Name::Wide(words) => serializer.serialize_str(&String::from_utf16_lossy(words)),
                Name::Str(name) => serializer.serialize_str(name),
            }
        }
    }
    impl<'a> Serialize for DirectoryEntry<'a> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut state = serializer.serialize_struct("DirectoryEntry", 2)?;
            state.serialize_field("name", &self.name().ok())?;
            state.serialize_field(
                if self.is_dir() { "directory" } else { "data" },
                &self.entry().ok(),
            )?;
            state.end()
        }
    }
    impl<'a> Serialize for DataEntry<'a> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut state = serializer.serialize_struct("DataEntry", 3)?;
            state.serialize_field("address", &self.image().OffsetToData)?;
            state.serialize_field("size", &self.size())?;
            state.serialize_field("code_page", &self.code_page())?;
            state.end()
        }
    }
}

//----------------------------------------------------------------

#[cfg(test)]
pub(crate) fn test(resources: Resources<'_>) -> Result<()> {
    fn test_dir(dir: Directory<'_>) {
        let _ = format!("{}", dir);
        let _ = format!("{:?}", dir);

        for entry in dir.entries() {
            let _ = format!("{:?}\n{:?}", entry.name(), entry);

            // Check consistency in id vs named entries
            if let Ok(name) = entry.name() {
                let mut id_entries;
                let mut named_entries;
                let mut entries: &mut dyn Iterator<Item = _> = match name {
                    Name::Id(_) => {
                        id_entries = dir.id_entries();
                        &mut id_entries
                    }
                    Name::Wide(_) => {
                        named_entries = dir.named_entries();
                        &mut named_entries
                    }
                    Name::Str(_) => unreachable!(),
                };
                assert!((&mut entries).any(|entry| entry.name() == Ok(name)));
            }

            // Inspect the entry recursively
            match entry.entry() {
                Ok(Entry::DataEntry(data)) => {
                    assert!(!entry.is_dir());
                    let _ = format!("{:?}", data);
                    let _size = data.size();
                    let _code_page = data.code_page();
                    let _bytes = data.bytes();
                }
                Ok(Entry::Directory(dir)) => {
                    assert!(entry.is_dir());
                    let _ = test_dir(dir);
                }
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
