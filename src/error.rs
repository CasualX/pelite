/*!
Errors and Results.
*/

use std::{error, fmt, result};

/// Errors while parsing the PE binary.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Error {
	/// Null address.
	Null,
	/// Index was out of bounds.
	OOB,
	/// Sections can be shorter than stored on disk, the remaining bytes will default to zeroes when loaded by the system.
	/// Since these zeroes would just be a waste of space, they are not present in the binaries on disk.
	///
	/// This error happens when attempting to get a reference to such zero filled data when working with `PeFile` contents.
	ZeroFill,
	/// Address is misaligned.
	Misalign,
	/// Magic number does not match the expected value.
	BadMagic,
	/// Sanity check failed.
	///
	/// Some value was so far outside its typical range, while not technically incorrect, probably indicating something went wrong.
	Insanity,
	/// Data corruption.
	///
	/// Structured data was found which simply isn't valid.
	///
	/// Catch all for errors which don't fall under other errors.
	Corrupt,
	/// Address calculation overflow error.
	Overflow,
	/// No nul byte found when reading a C string.
	CStr,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(error::Error::description(self))
	}
}

impl error::Error for Error {
	fn description(&self) -> &str {
		match *self {
			Error::Null => "null",
			Error::OOB => "out of bounds",
			Error::ZeroFill => "zero fill",
			Error::Misalign => "misalign",
			Error::BadMagic => "bad magic",
			Error::Insanity => "insanity",
			Error::Corrupt => "corrupt",
			Error::Overflow => "overflow",
			Error::CStr => "c str",
		}
	}
}

/// Specialized `Result` type for PE errors.
pub type Result<T> = result::Result<T, Error>;
