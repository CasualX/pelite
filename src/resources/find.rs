/*!
Resources Find API.
*/

use core::{fmt, str};

use std::{error, path::Path};

use crate::resources::FindError;

use super::{DataEntry, Directory, Entry, Name, Resources};

//------------------------------------------------

impl<'a> Resources<'a> {
	/// Finds a resource by its type and name.
	pub fn find_resource(&self, path: &[Name<'_>; 2]) -> Result<&'a [u8], FindError> {
		Ok(self.root()?.get_dir(path[0])?.get_dir(path[1])?.first_data()?.bytes()?)
	}
	
	/// Finds the language directory for a resource with given type and name.
	pub fn find_resources(&self, path: &[Name<'_>; 2]) -> Result<Directory<'a>, FindError> {
		self.root()?.get_dir(path[0])?.get_dir(path[1])
	}
	
	/// Finds the resource with specified type, name and language.
	pub fn find_resource_ex(&self, path: &[Name<'_>; 3]) -> Result<&'a [u8], FindError> {
		Ok(self.root()?.get_dir(path[0])?.get_dir(path[1])?.get_data(path[2])?.bytes()?)
	}
	
	/// Gets the Version Information.
	#[cfg(all(feature = "alloc", feature = "std"))]
	pub fn version_info(&self) -> Result<super::version_info::VersionInfo<'a>, FindError> {
		let bytes = self.find_resource(&[Name::VERSION, Name::Id(1)])?;
		let version_info = super::version_info::VersionInfo::try_from(bytes)?;
		Ok(version_info)
	}

	/// Gets the Application Manifest.
	pub fn manifest(&self) -> Result<&'a str, FindError> {
		// Ok, new assumption: just take whatever we can find in the Manifest directory
		let bytes = self.root()?.get_dir(Name::MANIFEST)?.first_dir()?.first_data()?.bytes()?;
		let manifest = str::from_utf8(bytes)?;
		Ok(manifest)
	}

	/// Gets the icons.
	pub fn icons(&self) -> impl 'a + Iterator<Item = Result<(Name<'a>, super::group::GroupIcon<'a>), FindError>> + Clone {
		let resources = *self;
		let icons = self.root().map_err(FindError::Pe).and_then(|root| root.get_dir(Name::GROUP_ICON));

		icons.into_iter().flat_map(move |icons| {
			icons.entries().map(move |de| {
				let name = de.name()?;
				// A lot of assumptions being made here...
				let bytes = de.entry()?.dir().ok_or(FindError::UnDataEntry)?.first_data()?.bytes()?;
				let group_icon = super::group::GroupIcon::new(resources, bytes)?;
				Ok((name, group_icon))
			})
		})
	}

	/// Gets the cursors.
	pub fn cursors(&self) -> impl 'a + Iterator<Item = Result<(Name<'a>, super::group::GroupCursor<'a>), FindError>> + Clone {
		let resources = *self;
		let cursors = self.root().map_err(FindError::Pe).and_then(|root| root.get_dir(Name::GROUP_CURSOR));

		cursors.into_iter().flat_map(move |cursors| {
			cursors.entries().map(move |de| {
				let name = de.name()?;
				// A lot of assumptions being made here...
				let bytes = de.entry()?.dir().ok_or(FindError::UnDataEntry)?.first_data()?.bytes()?;
				let group_cursor = super::group::GroupCursor::new(resources, bytes)?;
				Ok((name, group_cursor))
			})
		})
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
				(*self).root()?.find_internal(iter.as_path())
			}
		}
		else {
			// The path is empty
			Err(FindError::NotFound)
		}
	}
}