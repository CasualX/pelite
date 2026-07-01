use core::fmt;
use core::error::Error;

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

impl From<core::str::Utf8Error> for FindError {
	fn from(_err: core::str::Utf8Error) -> FindError {
		FindError::Pe(crate::Error::Encoding)
	}
}

impl fmt::Display for FindError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.to_str().fmt(f)
	}
}

impl Error for FindError {
	
	fn description(&self) -> &str {
		self.to_str()
	}
	
	fn cause(&self) -> Option<&dyn Error> {
		self.source()
	}

	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			FindError::Pe(err) => Some(err),
			_ => None,
		}
	}
}
