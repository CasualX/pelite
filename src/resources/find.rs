/*!
Resources Find API.
*/

use core::{error, fmt, str};

#[cfg(feature = "std")]
use std::path::Path;

use super::{ResourceDataEntry, ResourceDirectory, ResourceDirectoryTable, ResourceEntry, ResourceName};

//------------------------------------------------

/// Find error.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ResourceFindError {
	/// An error happened when reading the underlying resources.
	///
	/// This error indicates the resources are corrupt.
	Pe(crate::Error),
	/// The resources work with UTF-16 path names.
	///
	/// For this to work the given path must be valid unicode for the path comparison to make sense.
	///
	/// This error means the given path contained non-unicode parts.
	InvalidUtf8Path,
	/// The requested data entry or directory doesn't exist.
	NotFound,
	/// Paths from the resources root must start with a `/` or `\`.
	MissingRoot,
	/// Encountered a data entry when expecting a directory.
	///
	/// This error means the given path contained a directory name which is actually a data entry.
	UnexpectedDataEntry,
	/// Encountered a directory when expecting a data entry.
	UnexpectedDirectory,
}
impl ResourceFindError {
	/// Returns a simple string representation of the error.
	pub fn to_str(self) -> &'static str {
		match self {
			ResourceFindError::Pe(err) => err.to_str(),
			ResourceFindError::InvalidUtf8Path => "invalid utf8 path",
			ResourceFindError::NotFound => "entry not found",
			ResourceFindError::MissingRoot => "missing '/' root",
			ResourceFindError::UnexpectedDataEntry => "unexpected data entry",
			ResourceFindError::UnexpectedDirectory => "unexpected directory",
		}
	}
}
impl From<crate::Error> for ResourceFindError {
	fn from(err: crate::Error) -> ResourceFindError {
		ResourceFindError::Pe(err)
	}
}
impl From<str::Utf8Error> for ResourceFindError {
	fn from(_err: str::Utf8Error) -> ResourceFindError {
		ResourceFindError::Pe(crate::Error::Encoding)
	}
}
impl fmt::Display for ResourceFindError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.to_str().fmt(f)
	}
}
impl error::Error for ResourceFindError {
	fn description(&self) -> &str {
		self.to_str()
	}
	fn cause(&self) -> Option<&dyn error::Error> {
		self.source()
	}
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		match self {
			ResourceFindError::Pe(err) => Some(err),
			_ => None,
		}
	}
}

//------------------------------------------------

impl<'a> ResourceDirectory<'a> {
	/// Finds a resource by its type and name.
	pub fn find_resource(&self, path: &[ResourceName<'_>; 2]) -> Result<&'a [u8], ResourceFindError> {
		Ok(self.root()?.get_dir(path[0])?.get_dir(path[1])?.first_data()?.bytes()?)
	}
	/// Finds the language directory for a resource with given type and name.
	pub fn find_resources(&self, path: &[ResourceName<'_>; 2]) -> Result<ResourceDirectoryTable<'a>, ResourceFindError> {
		self.root()?.get_dir(path[0])?.get_dir(path[1])
	}
	/// Finds the resource with specified type, name and language.
	pub fn find_resource_ex(&self, path: &[ResourceName<'_>; 3]) -> Result<&'a [u8], ResourceFindError> {
		Ok(self.root()?.get_dir(path[0])?.get_dir(path[1])?.get_data(path[2])?.bytes()?)
	}
	/// Gets the Version Information.
	pub fn version_info(&self) -> Result<super::version_info::VersionInfo<'a>, ResourceFindError> {
		let bytes = self.find_resource(&[ResourceName::VERSION, ResourceName::Id(1)])?;
		let version_info = super::version_info::VersionInfo::try_from(bytes)?;
		Ok(version_info)
	}
	/// Gets the Application Manifest.
	pub fn manifest(&self) -> Result<&'a str, ResourceFindError> {
		// Ok, new assumption: just take whatever we can find in the Manifest directory
		let bytes = self.root()?.get_dir(ResourceName::MANIFEST)?.first_dir()?.first_data()?.bytes()?;
		let manifest = str::from_utf8(bytes)?;
		Ok(manifest)
	}
	/// Gets the icons.
	pub fn icons(&self) -> impl 'a + Iterator<Item = Result<(ResourceName<'a>, super::group::GroupIcon<'a>), ResourceFindError>> + Clone {
		let resources = *self;
		let icons = self.root().map_err(ResourceFindError::Pe).and_then(|root| root.get_dir(ResourceName::GROUP_ICON));

		icons.into_iter().flat_map(move |icons| {
			icons.entries().map(move |de| {
				let name = de.name()?;
				// A lot of assumptions being made here...
				let bytes = de.entry()?.dir().ok_or(ResourceFindError::UnexpectedDataEntry)?.first_data()?.bytes()?;
				let group_icon = super::group::GroupIcon::new(resources, bytes)?;
				Ok((name, group_icon))
			})
		})
	}
	/// Gets the cursors.
	pub fn cursors(&self) -> impl 'a + Iterator<Item = Result<(ResourceName<'a>, super::group::GroupCursor<'a>), ResourceFindError>> + Clone {
		let resources = *self;
		let cursors = self.root().map_err(ResourceFindError::Pe).and_then(|root| root.get_dir(ResourceName::GROUP_CURSOR));

		cursors.into_iter().flat_map(move |cursors| {
			cursors.entries().map(move |de| {
				let name = de.name()?;
				// A lot of assumptions being made here...
				let bytes = de.entry()?.dir().ok_or(ResourceFindError::UnexpectedDataEntry)?.first_data()?.bytes()?;
				let group_cursor = super::group::GroupCursor::new(resources, bytes)?;
				Ok((name, group_cursor))
			})
		})
	}
}
impl<'a> ResourceDirectoryTable<'a> {
	/// Looks up the entry by name.
	pub fn get(&self, name: ResourceName<'_>) -> Result<ResourceEntry<'a>, ResourceFindError> {
		self.entries().find(|de| de.name() == Ok(name)).ok_or(ResourceFindError::NotFound)?.entry().map_err(ResourceFindError::Pe)
	}
	/// Looks up the data entry by name.
	pub fn get_data(&self, name: ResourceName<'_>) -> Result<ResourceDataEntry<'a>, ResourceFindError> {
		self.entries()
			.find(|de| de.name() == Ok(name))
			.ok_or(ResourceFindError::NotFound)?
			.entry()?
			.data()
			.ok_or(ResourceFindError::UnexpectedDirectory)
	}
	/// Looks up the directory by name.
	pub fn get_dir(&self, name: ResourceName<'_>) -> Result<ResourceDirectoryTable<'a>, ResourceFindError> {
		self.entries().find(|de| de.name() == Ok(name)).ok_or(ResourceFindError::NotFound)?.entry()?.dir().ok_or(ResourceFindError::UnexpectedDataEntry)
	}
	/// Gets the first entry.
	pub fn first(&self) -> Result<ResourceEntry<'a>, ResourceFindError> {
		self.entries().next().ok_or(ResourceFindError::NotFound)?.entry().map_err(ResourceFindError::Pe)
	}
	/// Gets the first data entry.
	pub fn first_data(&self) -> Result<ResourceDataEntry<'a>, ResourceFindError> {
		self.entries().next().ok_or(ResourceFindError::NotFound)?.entry()?.data().ok_or(ResourceFindError::UnexpectedDirectory)
	}
	/// Gets the first directory.
	pub fn first_dir(&self) -> Result<ResourceDirectoryTable<'a>, ResourceFindError> {
		self.entries().next().ok_or(ResourceFindError::NotFound)?.entry()?.dir().ok_or(ResourceFindError::UnexpectedDataEntry)
	}
}

//------------------------------------------------

#[cfg(feature = "std")]
impl<'a> ResourceDirectory<'a> {
	/// Finds a file or directory by its path.
	pub fn find<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<ResourceEntry<'a>, ResourceFindError> {
		self.find_internal(path.as_ref())
	}
	/// Finds a file by its path.
	pub fn find_data<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<ResourceDataEntry<'a>, ResourceFindError> {
		self.find(path).and_then(|e| e.data().ok_or(ResourceFindError::UnexpectedDirectory))
	}
	/// Finds a directory by its path.
	pub fn find_dir<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<ResourceDirectoryTable<'a>, ResourceFindError> {
		self.find(path).and_then(|e| e.dir().ok_or(ResourceFindError::UnexpectedDataEntry))
	}
	fn find_internal(&self, path: &Path) -> Result<ResourceEntry<'a>, ResourceFindError> {
		let mut iter = path.iter();
		if let Some(slash) = iter.next() {
			// Not an absolute path
			if slash != "/" && slash != "\\" {
				Err(ResourceFindError::MissingRoot)
			}
			// Find the path in the root
			else {
				(*self).root()?.find_internal(iter.as_path())
			}
		}
		else {
			// The path is empty
			Err(ResourceFindError::NotFound)
		}
	}
}
#[cfg(feature = "std")]
impl<'a> ResourceDirectoryTable<'a> {
	/// Finds a file or directory by its path.
	pub fn find<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<ResourceEntry<'a>, ResourceFindError> {
		self.find_internal(path.as_ref())
	}
	/// Finds a file by its path.
	pub fn find_data<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<ResourceDataEntry<'a>, ResourceFindError> {
		self.find(path).and_then(|e| e.data().ok_or(ResourceFindError::UnexpectedDirectory))
	}
	/// Finds a directory by its path.
	pub fn find_dir<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<ResourceDirectoryTable<'a>, ResourceFindError> {
		self.find(path).and_then(|e| e.dir().ok_or(ResourceFindError::UnexpectedDataEntry))
	}
	fn find_internal(&self, path: &Path) -> Result<ResourceEntry<'a>, ResourceFindError> {
		let mut entry = ResourceEntry::Directory(*self);
		'parts: for part in path {
			// The names of resources are UTF16
			let name = ResourceName::Str(part.to_str().ok_or(ResourceFindError::InvalidUtf8Path)?);
			match entry {
				ResourceEntry::Directory(dir) => {
					// Find a child with matching name for this part of the path
					for child in dir.entries() {
						if child.name() == Ok(name) {
							entry = child.entry()?;
							continue 'parts;
						}
					}
					return Err(ResourceFindError::NotFound);
				},
				ResourceEntry::Data(_) => {
					return Err(ResourceFindError::UnexpectedDataEntry);
				},
			};
		}
		Ok(entry)
	}
}
