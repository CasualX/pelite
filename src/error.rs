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
	/// Out of bounds.
	///
	/// Catch-all for bounds check errors.
	Bounds,
	/// Data is not available.
	///
	/// Can happen when referencing data in `PeFile` instances.
	///
	/// Sections can be shorter than stored on disk, the remaining bytes will default to zeroes when loaded by the system.
	/// Since these zeroes would just be a waste of space, they are not present in the binaries on disk.
	/// This error happens when attempting to get a reference to such zero filled data.
	ZeroFill,
	/// Data is not available.
	///
	/// Can happen when referencing data in `PeView` instances.
	///
	/// Sections can have excess in their raw data which won't be mapped when loaded by the system.
	/// This error happens when attempting to get a reference to such unmapped raw data.
	/// Sometimes this kind of excess is called the PE file having an overlay.
	Unmapped,
	/// Address is misaligned.
	Misaligned,
	/// Magic number does not match the expected value.
	///
	/// Most commonly indicate non-PE files or trying to load a PE32 file with a PE32+ parsers or vice versa.
	BadMagic,
	/// Sanity check failed.
	///
	/// Some value was so far outside its typical range, while not technically incorrect, probably indicating something went wrong.
	/// If this error is encountered legitimately, create an issue or file a PR to relax the artificial restrictions.
	Insanity,
	/// Invalid data.
	///
	/// Structured data was found which simply isn't valid.
	/// Catch-all for errors which don't fall under other errors.
	Invalid,
	/// Overflow error.
	///
	/// Catch-all for overflow and underflow errors.
	Overflow,
	/// Encoding error.
	///
	/// Catch-all for string related errors such as lacking a nul terminator.
	Encoding,
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
		match self {
			Error::Null => f.write_str("Null address reference"),
			Error::Bounds => f.write_str("Bounds check failed"),
			Error::ZeroFill => f.write_str("Zero filled data reference"),
			Error::Unmapped => f.write_str("Overlay data reference"),
			Error::Misaligned => f.write_str("Address misaligned"),
			Error::BadMagic => f.write_str("Unknown magic number"),
			Error::Insanity => f.write_str("Data insanity"),
			Error::Invalid => f.write_str("Invalid data"),
			Error::Overflow => f.write_str("Overflow error"),
			Error::Encoding => f.write_str("Encoding error"),
		}
	}
}

impl error::Error for Error {
	fn description(&self) -> &str {
		match self {
			Error::Null => "null address",
			Error::Bounds => "out of bounds",
			Error::ZeroFill => "zero fill",
			Error::Unmapped => "unmapped",
			Error::Misaligned => "misaligned",
			Error::BadMagic => "bad magic",
			Error::Insanity => "insanity",
			Error::Invalid => "invalid data",
			Error::Overflow => "overflow error",
			Error::Encoding => "encoding error",
		}
	}
}

/// Specialized `Result` type for PE errors.
pub type Result<T> = result::Result<T, Error>;
