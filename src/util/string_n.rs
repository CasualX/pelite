use std::{fmt, str};

use crate::Pod;

/// Represents a fixed size string buffer.
///
/// The string is terminated by the first nul character or the end of the buffer otherwise.
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct StringN<A>(pub A);

impl<A: Pod> StringN<A> {
	/// Gets the byte string.
	///
	/// Includes all bytes up to the first nul character or the whole buffer otherwise.
	pub fn to_byte_str(&self) -> &[u8] {
		super::strn(self.0.as_bytes())
	}
	/// Gets the string.
	///
	/// Wraps `to_byte_str` and converts to `str`.
	pub fn to_str(&self) -> Result<&str, str::Utf8Error> {
		str::from_utf8(self.to_byte_str())
	}
}

unsafe impl<A: Pod> Pod for StringN<A> {}

impl<A> From<A> for StringN<A> {
	fn from(array: A) -> StringN<A> {
		StringN(array)
	}
}
impl<A> AsRef<A> for StringN<A> {
	fn as_ref(&self) -> &A {
		&self.0
	}
}
impl<A> AsMut<A> for StringN<A> {
	fn as_mut(&mut self) -> &mut A {
		&mut self.0
	}
}
impl<A: PartialEq> PartialEq<A> for StringN<A> {
	fn eq(&self, other: &A) -> bool {
		self.0.eq(other)
	}
}

impl<A: Pod> PartialEq<[u8]> for StringN<A> {
	fn eq(&self, other: &[u8]) -> bool {
		let bytes = self.0.as_bytes();
		// Dont bother if other larger than our fixed size
		if other.len() > bytes.len() {
			return false;
		}
		// Only compute strn if lengths are not equal
		let against = if other.len() == bytes.len() { bytes } else { super::strn(bytes) };
		return other == against;
	}
}
impl<A: Pod> PartialEq<str> for StringN<A> {
	fn eq(&self, other: &str) -> bool {
		self.eq(other.as_bytes())
	}
}

impl<A: Pod> fmt::Display for StringN<A> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.to_str() {
			Ok(s) => s.fmt(f),
			Err(err) => err.fmt(f),
		}
	}
}
impl<A: Pod> fmt::Debug for StringN<A> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.to_str() {
			Ok(s) => s.fmt(f),
			Err(err) => err.fmt(f),
		}
	}
}

#[cfg(feature = "serde")]
impl<A: Pod> serde::Serialize for StringN<A> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		let bytes = self.to_byte_str();
		match str::from_utf8(bytes) {
			Ok(s) => serializer.collect_str(s),
			Err(_) => serializer.collect_seq(bytes),
		}
	}
}
