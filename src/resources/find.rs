/*!
Resources Find API.
*/

use std::{error, fmt, str};
use std::path::Path;

use super::{Resources, Directory, Entry, Name, DataEntry};
use crate::stringify::RSRC_TYPES;

//------------------------------------------------

/// Find error.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FindError {
	/// An error happened when reading the underlying resources.
	///
	/// This error indicates the resources are corrupt.
	Pe(crate::Error),
	/// The resources work with UTF-16 path names.
	///
	/// For this to work the given path must be valid unicode for the path comparison to make sense.
	///
	/// This error means the given path contained non-unicode parts.
	Bad8Path,
	/// The requested data entry or directory doesn't exist.
	NotFound,
	/// Paths from the resources root must start with a `/` or `\`.
	NoRootPath,
	/// Encountered a data entry when expecting a directory.
	///
	/// This error means the given path contained a directory name which is actually a data entry.
	UnDataEntry,
	/// Encountered a directory when expecting a data entry.
	UnDirectory,
}
impl From<crate::Error> for FindError {
	fn from(err: crate::Error) -> FindError {
		FindError::Pe(err)
	}
}
impl From<str::Utf8Error> for FindError {
	fn from(_err: str::Utf8Error) -> FindError {
		FindError::Pe(crate::Error::Encoding)
	}
}
impl fmt::Display for FindError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		error::Error::description(self).fmt(f)
	}
}
impl error::Error for FindError {
	fn description(&self) -> &str {
		match *self {
			FindError::Pe(ref e) => e.description(),
			FindError::Bad8Path => "invalid utf8 path",
			FindError::NotFound => "entry not found",
			FindError::NoRootPath => "missing '/' root",
			FindError::UnDataEntry => "unexpected data entry",
			FindError::UnDirectory => "unexpected directory",
		}
	}
	fn cause(&self) -> Option<&error::Error> {
		match *self {
			FindError::Pe(ref e) => Some(e),
			_ => None,
		}
	}
}

//------------------------------------------------

impl<'a> Resources<'a> {
	/// Finds a resource by its type and name.
	pub fn find_resource(&self, ty: Name<'_>, name: Name<'_>) -> Result<&'a [u8], FindError> {
		Ok(self.root()?.get_dir(ty)?.get_dir(name)?.first_data()?.bytes()?)
	}
	/// Finds the language directory for a resource with given type and name.
	pub fn find_resources(&self, ty: Name<'_>, name: Name<'_>) -> Result<Directory<'a>, FindError> {
		self.root()?.get_dir(ty)?.get_dir(name)
	}
	/// Finds the resource with specified type, name and language.
	pub fn find_resource_ex(&self, ty: Name<'_>, name: Name<'_>, lang: Name<'_>) -> Result<&'a [u8], FindError> {
		Ok(self.root()?.get_dir(ty)?.get_dir(name)?.get_data(lang)?.bytes()?)
	}
	/// Gets the Version Information.
	pub fn version_info(&self) -> Result<super::version_info::VersionInfo<'a>, FindError> {
		let bytes = self.find_resource(Name::VERSION, 1.into())?;
		let version_info = super::version_info::VersionInfo::try_from(bytes)?;
		Ok(version_info)
	}
	/// Gets the Application Manifest.
	pub fn manifest(&self) -> Result<&'a str, FindError> {
		// Assumption: This appears to be always the same...
		let bytes = self.find_resource_ex(Name::MANIFEST, 2.into(), 1033.into())?;
		let manifest = str::from_utf8(bytes)?;
		Ok(manifest)
	}
	/// Gets the group icons.
	pub fn group_icons(&self) -> impl 'a + Iterator<Item = Result<(Name<'a>, super::group::GroupIcon<'a>), FindError>> + Clone {
		let resources = *self;
		let group_icons = self.root().map_err(FindError::Pe)
			.and_then(|root| root.get_dir(Name::GROUP_ICON));

		group_icons.into_iter().flat_map(move |group_icons| group_icons.entries().map(move |de| {
			let name = de.name()?;
			// A lot of assumptions being made here...
			let bytes = de.entry()?.dir().ok_or(FindError::UnDataEntry)?.first_data()?.bytes()?;
			let group_icon = super::group::GroupIcon::new(resources, bytes)?;
			Ok((name, group_icon))
		}))
	}
	/// Gets the group cursors.
	pub fn group_cursors(&self) -> impl 'a + Iterator<Item = Result<(Name<'a>, super::group::GroupCursor<'a>), FindError>> + Clone {
		let resources = *self;
		let group_cursors = self.root().map_err(FindError::Pe)
			.and_then(|root| root.get_dir(Name::GROUP_CURSOR));

		group_cursors.into_iter().flat_map(move |group_cursors| group_cursors.entries().map(move |de| {
			let name = de.name()?;
			// A lot of assumptions being made here...
			let bytes = de.entry()?.dir().ok_or(FindError::UnDataEntry)?.first_data()?.bytes()?;
			let group_cursor = super::group::GroupCursor::new(resources, bytes)?;
			Ok((name, group_cursor))
		}))
	}
}
impl<'a> Directory<'a> {
	/// Looks up the entry by name.
	pub fn get(&self, name: Name<'_>) -> Result<Entry<'a>, FindError> {
		self.entries().find(|de| de.name() == Ok(name)).ok_or(FindError::NotFound)?.entry().map_err(FindError::Pe)
	}
	/// Looks up the data entry by name.
	pub fn get_data(&self, name: Name<'_>) -> Result<DataEntry<'a>, FindError> {
		self.entries().find(|de| de.name() == Ok(name)).ok_or(FindError::NotFound)?.entry()?.data().ok_or(FindError::UnDirectory)
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
}

//------------------------------------------------

impl<'a> Resources<'a> {
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
	fn find_internal(&self, path: &Path) -> Result<Entry<'a>, FindError> {
		let mut iter = path.iter();
		if let Some(slash) = iter.next() {
			// Not an absolute path
			if slash != "/" && slash != "\\" {
				Err(FindError::NoRootPath)
			}
			// Find the path in the root
			else {
				(*self).root()?.find_internal(iter.as_path(), &RSRC_TYPES)
			}
		}
		else {
			// The path is empty
			Err(FindError::NotFound)
		}
	}
}
impl<'a> Directory<'a> {
	/// Finds a file or directory by its path.
	pub fn find<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<Entry<'a>, FindError> {
		self.find_internal(path.as_ref(), &[])
	}
	/// Finds a file by its path.
	pub fn find_data<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<DataEntry<'a>, FindError> {
		self.find(path).and_then(|e| e.data().ok_or(FindError::UnDirectory))
	}
	/// Finds a directory by its path.
	pub fn find_dir<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<Directory<'a>, FindError> {
		self.find(path).and_then(|e| e.dir().ok_or(FindError::UnDataEntry))
	}
	fn find_internal(&self, path: &Path, id_names: &[Option<&str>]) -> Result<Entry<'a>, FindError> {
		let mut e = Entry::Directory(*self);
		'parts: for part in path {
			// The names of resources are UTF16
			let part_name = part.to_str().ok_or(FindError::Bad8Path)?;
			// For comparison with resource ids
			let part_id = part_name.parse::<u32>().ok();
			match e {
				Entry::Directory(dir) => {
					// Find a child with matching name for this part of the path
					for child in dir.entries() {
						let mut equal = false;
						match child.name()? {
							Name::Id(id) => {
								if Some(id) == part_id {
									equal = true;
								}
								else if let Some(&Some(id_name)) = id_names.get(id as usize) {
									if id_name == part_name {
										equal = true;
									}
								}
							},
							Name::Str(s) => {
								if s == part_name {
									equal = true;
								}
							},
						};
						if equal {
							e = child.entry()?;
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
		Ok(e)
	}
}
