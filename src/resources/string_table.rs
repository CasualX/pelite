/*!
Windows string-table resources.

Each resource block contains exactly 16 length-prefixed UTF-16LE strings.
The resource name identifies the block; string IDs within block `n` range from
`(n - 1) * 16` through `n * 16 - 1`.
*/

use alloc::string::String;
use core::{char, fmt, iter};

use crate::{Error, Result};

const STRINGS_PER_BLOCK: usize = 16;

/// A parsed string-table resource block.
#[derive(Copy, Clone)]
pub struct StringTable<'a> {
	bytes: &'a [u8],
}

impl<'a> StringTable<'a> {
	/// Parses a string-table resource block.
	///
	/// The block must contain exactly 16 length-prefixed UTF-16LE strings and no
	/// trailing data. Empty strings are valid entries.
	pub fn new(bytes: &'a [u8]) -> Result<StringTable<'a>> {
		let mut remaining = bytes;
		for _ in 0..STRINGS_PER_BLOCK {
			let (_, tail) = split_string(remaining)?;
			remaining = tail;
		}
		if !remaining.is_empty() {
			return Err(Error::Invalid);
		}
		Ok(StringTable { bytes })
	}

	/// Returns the encoded resource bytes.
	pub fn bytes(&self) -> &'a [u8] {
		self.bytes
	}

	/// Returns the string at the given block-relative index.
	pub fn get(&self, index: usize) -> Option<StringTableString<'a>> {
		self.iter().nth(index)
	}

	/// Iterates over all 16 strings in the block.
	pub fn iter(&self) -> StringTableIter<'a> {
		StringTableIter { remaining: self.bytes, len: STRINGS_PER_BLOCK }
	}
}

impl<'a> IntoIterator for StringTable<'a> {
	type Item = StringTableString<'a>;
	type IntoIter = StringTableIter<'a>;

	fn into_iter(self) -> StringTableIter<'a> {
		self.iter()
	}
}

impl fmt::Debug for StringTable<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_list().entries(self.iter()).finish()
	}
}

/// A UTF-16LE string borrowed from a string-table resource.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StringTableString<'a> {
	bytes: &'a [u8],
}

impl StringTableString<'_> {
	/// Returns the number of UTF-16 code units in the string.
	pub fn len(&self) -> usize {
		self.bytes.len() / 2
	}

	/// Returns whether the string is empty.
	pub fn is_empty(&self) -> bool {
		self.bytes.is_empty()
	}

	/// Iterates over the UTF-16 code units.
	pub fn encode_utf16(&self) -> impl Clone + ExactSizeIterator<Item = u16> + use<'_> {
		self.bytes.as_chunks::<2>().0.iter().map(|&word| u16::from_le_bytes(word))
	}

	/// Decodes the string, returning an error for an unpaired surrogate.
	pub fn to_string(&self) -> core::result::Result<String, char::DecodeUtf16Error> {
		char::decode_utf16(self.encode_utf16()).collect()
	}
}

impl fmt::Display for StringTableString<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for chr in char::decode_utf16(self.encode_utf16()) {
			fmt::Write::write_char(f, chr.unwrap_or(char::REPLACEMENT_CHARACTER))?;
		}
		Ok(())
	}
}

impl fmt::Debug for StringTableString<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_tuple("StringTableString").field(&format_args!("{}", self)).finish()
	}
}

/// Iterator over the strings in a string-table resource block.
#[derive(Clone)]
pub struct StringTableIter<'a> {
	remaining: &'a [u8],
	len: usize,
}

impl<'a> Iterator for StringTableIter<'a> {
	type Item = StringTableString<'a>;

	fn next(&mut self) -> Option<StringTableString<'a>> {
		if self.len == 0 {
			return None;
		}
		// StringTable validates every entry before constructing the iterator.
		let (string, remaining) = split_string(self.remaining).ok()?;
		self.remaining = remaining;
		self.len -= 1;
		Some(string)
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.len, Some(self.len))
	}
}

impl ExactSizeIterator for StringTableIter<'_> {}
impl iter::FusedIterator for StringTableIter<'_> {}

fn split_string(bytes: &[u8]) -> Result<(StringTableString<'_>, &[u8])> {
	let len = match bytes {
		&[lo, hi, ..] => u16::from_le_bytes([lo, hi]) as usize,
		_ => return Err(Error::Bounds),
	};
	let byte_len = len.checked_mul(2).ok_or(Error::Overflow)?;
	let end = 2usize.checked_add(byte_len).ok_or(Error::Overflow)?;
	let string = bytes.get(2..end).ok_or(Error::Bounds)?;
	let remaining = bytes.get(end..).ok_or(Error::Bounds)?;
	Ok((StringTableString { bytes: string }, remaining))
}

serde_impl! {
	impl Serialize for StringTable<'_> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			serializer.collect_seq(self.iter())
		}
	}

	impl Serialize for StringTableString<'_> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			serializer.collect_str(self)
		}
	}
}

#[cfg(test)]
mod tests {
	use alloc::string::String;
	use alloc::vec::Vec;

	use super::*;

	fn table(strings: &[&[u16]]) -> Vec<u8> {
		assert!(strings.len() <= STRINGS_PER_BLOCK);
		let mut bytes = Vec::new();
		for string in strings.iter().copied().chain(iter::repeat(&[][..])).take(STRINGS_PER_BLOCK) {
			bytes.extend_from_slice(&(string.len() as u16).to_le_bytes());
			for word in string {
				bytes.extend_from_slice(&word.to_le_bytes());
			}
		}
		bytes
	}

	#[test]
	fn parses_sixteen_strings() {
		let bytes = table(&[&[b'H' as u16, b'i' as u16], &[], &[0xd83e, 0xdd80]]);
		let table = StringTable::new(&bytes).unwrap();
		assert_eq!(table.bytes(), bytes);
		assert_eq!(table.iter().len(), STRINGS_PER_BLOCK);
		assert_eq!(table.get(0).unwrap().to_string(), Ok(String::from("Hi")));
		assert!(table.get(1).unwrap().is_empty());
		assert_eq!(table.get(2).unwrap().to_string(), Ok(String::from("🦀")));
		assert_eq!(table.get(15).unwrap().to_string(), Ok(String::new()));
		assert!(table.get(16).is_none());
	}

	#[test]
	fn rejects_truncated_or_trailing_data() {
		let bytes = table(&[]);
		assert_eq!(StringTable::new(&bytes[..bytes.len() - 1]).unwrap_err(), Error::Bounds);

		let mut trailing = bytes;
		trailing.push(0);
		assert_eq!(StringTable::new(&trailing).unwrap_err(), Error::Invalid);
	}

	#[test]
	fn rejects_truncated_string() {
		let mut bytes = table(&[]);
		bytes[0..2].copy_from_slice(&1u16.to_le_bytes());
		assert_eq!(StringTable::new(&bytes).unwrap_err(), Error::Bounds);
	}
}
