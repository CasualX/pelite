/*!
Length word prefixed wide string.
*/

use std::{char, fmt, mem, slice, ops};

use util::FromBytes;

//----------------------------------------------------------------

/// Length word prefixed wide string.
///
/// Used as the string format for file names by the PE resources.
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WideStr {
	words: [u16],
}

impl WideStr {
	/// Interprets a word slice as a length word prefixed wide string.
	///
	/// # Safety
	///
	/// Ensure the slice's first word equals the length of the slice + 1.
	pub unsafe fn from_words_unchecked(words: &[u16]) -> &WideStr {
		mem::transmute(words)
	}
	/// Encodes the string as an UTF8 validated `String`.
	pub fn to_string(&self) -> Result<String, char::DecodeUtf16Error> {
		char::decode_utf16(self.as_ref().iter().cloned()).collect()
	}
}

impl FromBytes for WideStr {
	const MIN_SIZE_OF: usize = 2;
	const ALIGN_OF: usize = 2;
	unsafe fn from_bytes(bytes: &[u8]) -> ::Result<&WideStr> {
		let p = bytes.as_ptr() as *const u16;
		let len = *p as usize;
		if len * 2 > bytes.len() {
			return Err(::Error::CStr);
		}
		Ok(WideStr::from_words_unchecked(slice::from_raw_parts(p, len)))
	}
}

impl PartialEq<str> for WideStr {
	fn eq(&self, rhs: &str) -> bool {
		let decoder = char::decode_utf16(self.as_ref().iter().cloned());
		let chars = rhs.chars().map(Ok);
		decoder.eq(chars)
	}
}

impl ops::Deref for WideStr {
	type Target = [u16];
	#[inline]
	fn deref(&self) -> &[u16] {
		self.as_ref()
	}
}
impl AsRef<[u16]> for WideStr {
	#[inline]
	fn as_ref(&self) -> &[u16] {
		unsafe { self.words.get_unchecked(1..) }
	}
}

//----------------------------------------------------------------
// Formatting

impl fmt::Display for WideStr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for chr in char::decode_utf16(self.as_ref().iter().cloned()) {
			let chr = chr.unwrap_or(char::REPLACEMENT_CHARACTER);
			fmt::Write::write_char(f, chr)?;
		}
		Ok(())
	}
}

impl fmt::Debug for WideStr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("L\"")?;
		for chr in char::decode_utf16(self.as_ref().iter().cloned()) {
			match chr {
				Ok(chr) => {
					match chr {
						'\0' => f.write_str("\\0")?,
						'\n' => f.write_str("\\n")?,
						'\r' => f.write_str("\\r")?,
						'\t' => f.write_str("\\t")?,
						'"' => f.write_str("\\\"")?,
						'\\' => f.write_str("\\\\")?,
						_ => fmt::Write::write_char(f, chr)?,
					};
				},
				Err(e) => write!(f, "\\u{:04X}", e.unpaired_surrogate())?,
			};
		}
		f.write_str("\"")
	}
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::WideStr;

	#[test]
	fn units() {
		static WIDE_STR: [u16; 7] = [6, 83, 84, 82, 73, 78, 71];
		let wide_str = unsafe { WideStr::from_words_unchecked(&WIDE_STR) };
		assert_eq!(wide_str.to_string(), Ok(String::from("STRING")));
		assert_eq!(wide_str.len(), 6);
		assert_eq!(wide_str.as_ref(), &WIDE_STR[1..]);
	}
}
