
use crate::resources::Resources;
use crate::{image::IMAGE_RESOURCE_DIRECTORY_ENTRY, resources::{DataEntry, Directory}};
use core::fmt::{self, *};

pub static RSRC_TYPES: [Option<&str>; 25] = [
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


/// Iterator over entries in a directory.
pub type Entries<'a, F> = core::iter::Map<core::slice::Iter<'a, IMAGE_RESOURCE_DIRECTORY_ENTRY>, F>;

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
			},
			&Name::Wide(words) => char::decode_utf16(words.iter().cloned()).eq(string.chars().map(Ok)),
			&Name::Str(name) => string == name,
		}
	}
	
	pub fn rename_id(self, names: &[Option<&'a str>]) -> Name<'a> {
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
			},
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
	use crate::{resources::{DirectoryEntry, Resources}, util::serde_helper::*};

	use super::{DataEntry, Directory, Name};

	// Rename the toplevel directory ids to their names
	struct NamedDirectoryEntry<'a>(DirectoryEntry<'a>);
	impl<'a> Serialize for NamedDirectoryEntry<'a> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("DirectoryEntry", 2)?;
			state.serialize_field("name", &self.0.name().ok().map(|name| name.rename_id(&super::RSRC_TYPES)))?;
			state.serialize_field(if self.0.is_dir() { "directory" } else { "data" }, &self.0.entry().ok())?;
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
				Name::Wide(words) => {
					// TO-DO
					let str = heapless::String::<128>::from_utf16(words).unwrap();
					serializer.serialize_str(&str)
				},
				Name::Str(name) => serializer.serialize_str(name),
			}
		}
	}

	impl<'a> Serialize for DirectoryEntry<'a> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("DirectoryEntry", 2)?;
			state.serialize_field("name", &self.name().ok())?;
			state.serialize_field(if self.is_dir() { "directory" } else { "data" }, &self.entry().ok())?;
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
#[cfg(all(feature = "alloc", feature = "std"))]
#[cfg(test)]
pub(crate) fn test(resources: Resources<'_>) -> super::super::Result<()> {
	fn test_dir(dir: Directory<'_>) {
    	use alloc::format;

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
					},
					Name::Wide(_) => {
						named_entries = dir.named_entries();
						&mut named_entries
					},
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
				},
				Ok(Entry::Directory(dir)) => {
					assert!(entry.is_dir());
					let _ = test_dir(dir);
				},
				Err(_) => (),
			}
		}
	}

	let _ = resources.fsck();

	#[cfg(all(feature = "alloc", feature = "std"))]
	if let Ok(version_info) = resources.version_info() {

		crate::resources::version_info::test(version_info);
	}

	resources.root().map(test_dir)
}
