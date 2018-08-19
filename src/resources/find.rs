/*!
Resources Find API.
*/

use std::{error, fmt, str};
use std::path::Path;

use super::{Resources, Directory, Entry, Name, DataEntry};
use stringify::RSRC_TYPES;

//------------------------------------------------

/// Find error.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FindError {
	/// An error happened when reading the underlying resources.
	///
	/// This error indicates the resources are corrupt.
	Pe(::Error),
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
impl From<::Error> for FindError {
	fn from(err: ::Error) -> FindError {
		FindError::Pe(err)
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
	/// Finds a resource by its type, name and language.
	pub fn find_resource<'n>(&self, ty: impl Into<Name<'n>>, name: impl Into<Name<'n>>, lang: impl Into<Name<'n>>) -> Result<&'a [u8], FindError> {
		self.find_resource_internal(ty.into(), name.into(), lang.into())
	}
	/// Gets the Version Information.
	pub fn version_info(&self) -> Result<super::version_info::VersionInfo<'a>, FindError> {
		self.find_resource(::image::RT_VERSION, 1, 1033)
			.and_then(|bytes| super::version_info::VersionInfo::try_from(bytes).map_err(FindError::Pe))
	}
	#[inline(never)]
	fn find_resource_internal(&self, ty: Name<'_>, name: Name<'_>, lang: Name<'_>) -> Result<&'a [u8], FindError> {
		let path = [ty, name, lang];
		path.iter().try_fold(Entry::Directory(self.root()?), |e, &name| Ok(
			e.dir()
				.ok_or(FindError::UnDataEntry)?
				.entries()
				.find(|child| child.name() == Ok(name))
				.ok_or(FindError::NotFound)?
				.entry()?
		)).and_then(|e| Ok(e.data().ok_or(FindError::UnDirectory)?.bytes()?))
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
