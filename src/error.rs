/*!
Errors and Results.
*/

use std::{error, fmt, result};

/// Errors while parsing the PE binary.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
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
	/// Data is not mapped.
	Unmapped,
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

impl Error {
	/// Returns if the error variant is Null.
	///
	/// Useful in match guards where `Null` should be handled as a non-error case.
	///
	/// ```
	/// fn with_default(result: pelite::Result<i32>) -> pelite::Result<i32> {
	/// 	let i = match result {
	/// 		Ok(i) => i,
	/// 		// Avoids a more verbose comparison with pelite::Error::Null
	/// 		Err(err) if err.is_null() => 0,
	/// 		Err(err) => return Err(err),
	/// 	};
	/// 	Ok(i)
	/// }
	///
	/// assert_eq!(with_default(Err(pelite::Error::Null)), Ok(0));
	/// ```
	pub fn is_null(self) -> bool {
		self == Error::Null
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(error::Error::description(self))
	}
}

impl error::Error for Error {
	fn description(&self) -> &str {
		match self {
			Error::Null => "null",
			Error::OOB => "out of bounds",
			Error::ZeroFill => "zero fill",
			Error::Unmapped => "unmapped",
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
