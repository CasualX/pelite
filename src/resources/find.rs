/*!
Resources Find API.
*/

use std::{fmt, str};

#[cfg(feature = "std")]
use std::{error, path::Path};

use super::{DataEntry, Directory, Entry, Name, Resources};

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
impl FindError {
    /// Returns a simple string representation of the error.
    pub fn to_str(self) -> &'static str {
        match self {
            FindError::Pe(err) => err.to_str(),
            FindError::Bad8Path => "invalid utf8 path",
            FindError::NotFound => "entry not found",
            FindError::NoRootPath => "missing '/' root",
            FindError::UnDataEntry => "unexpected data entry",
            FindError::UnDirectory => "unexpected directory",
        }
    }
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
        self.to_str().fmt(f)
    }
}
#[cfg(feature = "std")]
impl error::Error for FindError {
    fn description(&self) -> &str {
        self.to_str()
    }
    fn cause(&self) -> Option<&dyn error::Error> {
        self.source()
    }
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            FindError::Pe(err) => Some(err),
            _ => None,
        }
    }
}

//------------------------------------------------

impl<'a> Resources<'a> {
    /// Finds a resource by its type and name.
    pub fn find_resource(&self, path: &[Name<'_>; 2]) -> Result<&'a [u8], FindError> {
        Ok(self
            .root()?
            .get_dir(path[0])?
            .get_dir(path[1])?
            .first_data()?
            .bytes()?)
    }
    /// Finds the language directory for a resource with given type and name.
    pub fn find_resources(&self, path: &[Name<'_>; 2]) -> Result<Directory<'a>, FindError> {
        self.root()?.get_dir(path[0])?.get_dir(path[1])
    }
    /// Finds the resource with specified type, name and language.
    pub fn find_resource_ex(&self, path: &[Name<'_>; 3]) -> Result<&'a [u8], FindError> {
        Ok(self
            .root()?
            .get_dir(path[0])?
            .get_dir(path[1])?
            .get_data(path[2])?
            .bytes()?)
    }
    /// Gets the Version Information.
    pub fn version_info(&self) -> Result<super::version_info::VersionInfo<'a>, FindError> {
        let bytes = self.find_resource(&[Name::VERSION, Name::Id(1)])?;
        let version_info = super::version_info::VersionInfo::try_from(bytes)?;
        Ok(version_info)
    }
    /// Gets the Application Manifest.
    pub fn manifest(&self) -> Result<&'a str, FindError> {
        // Ok, new assumption: just take whatever we can find in the Manifest directory
        let bytes = self
            .root()?
            .get_dir(Name::MANIFEST)?
            .first_dir()?
            .first_data()?
            .bytes()?;
        let manifest = str::from_utf8(bytes)?;
        Ok(manifest)
    }
    /// Gets the icons.
    pub fn icons(
        &self,
    ) -> impl 'a + Iterator<Item = Result<(Name<'a>, super::group::GroupIcon<'a>), FindError>> + Clone
    {
        let resources = *self;
        let icons = self
            .root()
            .map_err(FindError::Pe)
            .and_then(|root| root.get_dir(Name::GROUP_ICON));

        icons.into_iter().flat_map(move |icons| {
            icons.entries().map(move |de| {
                let name = de.name()?;
                // A lot of assumptions being made here...
                let bytes = de
                    .entry()?
                    .dir()
                    .ok_or(FindError::UnDataEntry)?
                    .first_data()?
                    .bytes()?;
                let group_icon = super::group::GroupIcon::new(resources, bytes)?;
                Ok((name, group_icon))
            })
        })
    }
    /// Gets the cursors.
    pub fn cursors(
        &self,
    ) -> impl 'a + Iterator<Item = Result<(Name<'a>, super::group::GroupCursor<'a>), FindError>> + Clone
    {
        let resources = *self;
        let cursors = self
            .root()
            .map_err(FindError::Pe)
            .and_then(|root| root.get_dir(Name::GROUP_CURSOR));

        cursors.into_iter().flat_map(move |cursors| {
            cursors.entries().map(move |de| {
                let name = de.name()?;
                // A lot of assumptions being made here...
                let bytes = de
                    .entry()?
                    .dir()
                    .ok_or(FindError::UnDataEntry)?
                    .first_data()?
                    .bytes()?;
                let group_cursor = super::group::GroupCursor::new(resources, bytes)?;
                Ok((name, group_cursor))
            })
        })
    }
}
impl<'a> Directory<'a> {
    /// Looks up the entry by name.
    pub fn get(&self, name: Name<'_>) -> Result<Entry<'a>, FindError> {
        self.entries()
            .find(|de| de.name() == Ok(name))
            .ok_or(FindError::NotFound)?
            .entry()
            .map_err(FindError::Pe)
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
    /// Looks up the directory by name.
    pub fn get_dir(&self, name: Name<'_>) -> Result<Directory<'a>, FindError> {
        self.entries()
            .find(|de| de.name() == Ok(name))
            .ok_or(FindError::NotFound)?
            .entry()?
            .dir()
            .ok_or(FindError::UnDataEntry)
    }
    /// Gets the first entry.
    pub fn first(&self) -> Result<Entry<'a>, FindError> {
        self.entries()
            .next()
            .ok_or(FindError::NotFound)?
            .entry()
            .map_err(FindError::Pe)
    }
    /// Gets the first data entry.
    pub fn first_data(&self) -> Result<DataEntry<'a>, FindError> {
        self.entries()
            .next()
            .ok_or(FindError::NotFound)?
            .entry()?
            .data()
            .ok_or(FindError::UnDirectory)
    }
    /// Gets the first directory.
    pub fn first_dir(&self) -> Result<Directory<'a>, FindError> {
        self.entries()
            .next()
            .ok_or(FindError::NotFound)?
            .entry()?
            .dir()
            .ok_or(FindError::UnDataEntry)
    }
}

//------------------------------------------------

#[cfg(feature = "std")]
impl<'a> Resources<'a> {
    /// Finds a file or directory by its path.
    pub fn find<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<Entry<'a>, FindError> {
        self.find_internal(path.as_ref())
    }
    /// Finds a file by its path.
    pub fn find_data<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<DataEntry<'a>, FindError> {
        self.find(path)
            .and_then(|e| e.data().ok_or(FindError::UnDirectory))
    }
    /// Finds a directory by its path.
    pub fn find_dir<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<Directory<'a>, FindError> {
        self.find(path)
            .and_then(|e| e.dir().ok_or(FindError::UnDataEntry))
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
        } else {
            // The path is empty
            Err(FindError::NotFound)
        }
    }
}
#[cfg(feature = "std")]
impl<'a> Directory<'a> {
    /// Finds a file or directory by its path.
    pub fn find<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<Entry<'a>, FindError> {
        self.find_internal(path.as_ref())
    }
    /// Finds a file by its path.
    pub fn find_data<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<DataEntry<'a>, FindError> {
        self.find(path)
            .and_then(|e| e.data().ok_or(FindError::UnDirectory))
    }
    /// Finds a directory by its path.
    pub fn find_dir<P: AsRef<Path> + ?Sized>(&self, path: &P) -> Result<Directory<'a>, FindError> {
        self.find(path)
            .and_then(|e| e.dir().ok_or(FindError::UnDataEntry))
    }
    fn find_internal(&self, path: &Path) -> Result<Entry<'a>, FindError> {
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
                }
                Entry::DataEntry(_) => {
                    return Err(FindError::UnDataEntry);
                }
            };
        }
        Ok(entry)
    }
}
