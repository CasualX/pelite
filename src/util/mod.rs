/*!
Utilities and other tidbits.
*/

// For testing, assert that structs have specified ABI size
#[cfg(test)]
macro_rules! assert_size_of {
	($size_of:expr, $ident:ident) => (assert_eq!($size_of, ::std::mem::size_of::<$ident>(),
		concat!("Unexpected sizeof(", stringify!($ident), ")")));
}

#[test]
#[should_panic]
fn assert_size_of() {
	struct Foo(i32);
	assert_size_of!(8, Foo);
}

mod c_str;
mod wide_str;
mod pod;
mod guid;

#[cfg(feature = "serde")]
pub(crate) mod serde_helper;

pub use self::c_str::CStr;
pub use self::wide_str::WideStr;
pub use self::pod::Pod;

/// Converts from a byte slice to a string.
pub trait FromBytes {
	/// Minimum size argument.
	const MIN_SIZE_OF: usize;
	/// Alignment argument.
	const ALIGN_OF: usize;
	/// Converts from a byte slice to a string.
	///
	/// # Safety
	///
	/// The given byte slice must have the minimum size and alignment as specified.
	unsafe fn from_bytes(bytes: &[u8]) -> Option<&Self>;
}

/// Splits a slice at the point defined by the callback.
#[inline]
pub(crate) fn split_f<T, F: FnMut(&T) -> bool>(slice: &[T], f: F) -> (&[T], &[T]) {
	let i = slice.iter().position(f).unwrap_or(slice.len());
	(&slice[..i], &slice[i..])
}

/// Reads an optionally nul-terminated string from byte buffer.
///
/// Returns the slice split before the nul byte and the whole slice if no nul byte is found.
///
/// Analog to the `strn*` family of C string functions.
///
/// # Examples
///
/// ```
/// use pelite::util::strn;
///
/// let buf: &[u8; 8] = b"STRING\0\0";
/// assert_eq!(strn(buf), b"STRING");
///
/// let buf: &[u8; 4] = b"FOUR";
/// assert_eq!(strn(buf), b"FOUR");
/// ```
#[inline]
pub fn strn(buf: &[u8]) -> &[u8] {
	split_f(buf, |&byte| byte == 0).0
}

/// Reads an optionally nul-terminated wide char string from buffer.
///
/// Returns the slice split before the nul word and the whole slice if no nul word is found.
///
/// Analog to the `wcsn*` family of C string functions.
///
/// # Examples
///
/// ```
/// use pelite::util::wstrn;
///
/// let buf: [u16; 8] = [83, 84, 82, 73, 78, 71, 0, 0];
/// assert_eq!(wstrn(&buf), &[83, 84, 82, 73, 78, 71]);
///
/// let buf: [u16; 4] = [70, 79, 85, 82];
/// assert_eq!(wstrn(&buf), &[70, 79, 85, 82]);
/// ```
#[inline]
pub fn wstrn(buf: &[u16]) -> &[u16] {
	split_f(buf, |&word| word == 0).0
}
