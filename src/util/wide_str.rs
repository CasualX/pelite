/*!
Length word prefixed wide string.
*/

use std::prelude::v1::*;
use std::{char, fmt, mem, ops, slice};

use crate::util::FromBytes;

//----------------------------------------------------------------

/// Length word prefixed wide string.
///
/// Used as the string format for file names by the PE resources.
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WideStr {
    words: [u16],
}

impl WideStr {
    pub fn from_str<'a>(s: &str, buffer: &'a mut [u16]) -> &'a WideStr {
        let mut n = 0;
        buffer[0] = n;
        for (p, wc) in buffer[1..].iter_mut().zip(s.encode_utf16()) {
            *p = wc;
            n += 1;
        }
        buffer[0] = n;
        unsafe { WideStr::from_words_unchecked(buffer) }
    }
    /// Constructs the wide string from a length word prefixed word slice.
    pub fn from_words(words: &[u16]) -> Option<&WideStr> {
        let len = *words.get(0)? as usize + 1;
        let words = words.get(0..len)?;
        Some(unsafe { WideStr::from_words_unchecked(words) })
    }
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
    unsafe fn from_bytes(bytes: &[u8]) -> Option<&WideStr> {
        let p = bytes.as_ptr() as *const u16;
        let len = *p as usize + 1;
        if len * 2 > bytes.len() {
            return None;
        }
        Some(WideStr::from_words_unchecked(slice::from_raw_parts(p, len)))
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
        unsafe { self.words.get_unchecked(1..) }
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
        FmtUtf16(self.as_ref()).fmt(f)
    }
}

impl fmt::Debug for WideStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        FmtUtf16(self.as_ref()).fmt(f)
    }
}

pub(crate) struct FmtUtf16<'a>(pub(crate) &'a [u16]);
impl fmt::Display for FmtUtf16<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for chr in char::decode_utf16(self.0.iter().cloned()) {
            let chr = chr.unwrap_or(char::REPLACEMENT_CHARACTER);
            fmt::Write::write_char(f, chr)?;
        }
        Ok(())
    }
}
impl fmt::Debug for FmtUtf16<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("L\"")?;
        for chr in char::decode_utf16(self.0.iter().cloned()) {
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
                }
                Err(e) => write!(f, "\\u{:04x}", e.unpaired_surrogate())?,
            };
        }
        f.write_str("\"")
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for WideStr {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::WideStr;
    use crate::util::FromBytes;
    use std::{char, slice};

    static WIDE_STR: [u16; 7] = [6, 83, 84, 82, 73, 78, 71];
    static INVALID_STR: [u16; 4] = [3, b'a' as u16, 0xd800, b'b' as u16];
    static ESCAPED_STR: [u16; 7] = [
        6,
        b'\0' as u16,
        b'\n' as u16,
        b'\r' as u16,
        b'\t' as u16,
        b'"' as u16,
        b'\\' as u16,
    ];

    #[test]
    fn units() {
        let wide_str = WideStr::from_words(&WIDE_STR).unwrap();
        assert_eq!(wide_str.to_string(), Ok(String::from("STRING")));
        assert_eq!(wide_str.len(), 6);
        assert_eq!(wide_str.as_ref(), &WIDE_STR[1..]);
    }

    #[test]
    fn equality() {
        let wide_str = WideStr::from_words(&WIDE_STR).unwrap();
        assert!(wide_str == "STRING");
    }

    #[test]
    fn from_bytes() {
        let bytes =
            unsafe { slice::from_raw_parts(WIDE_STR.as_ptr() as *const u8, WIDE_STR.len() * 2) };
        let wide_str = unsafe { WideStr::from_bytes(bytes).unwrap() };
        assert_eq!(wide_str, WideStr::from_words(&WIDE_STR).unwrap());
    }

    #[test]
    fn fmt() {
        let wide_str = WideStr::from_words(&WIDE_STR).unwrap();
        assert_eq!(format!("{}", wide_str), "STRING");
        assert_eq!(format!("{:?}", wide_str), r#"L"STRING""#);
        let invalid_str = WideStr::from_words(&INVALID_STR).unwrap();
        assert_eq!(
            format!("{}", invalid_str),
            format!("a{}b", char::REPLACEMENT_CHARACTER)
        );
        assert_eq!(format!("{:?}", invalid_str), "L\"a\\ud800b\"");
        let escaped_str = WideStr::from_words(&ESCAPED_STR).unwrap();
        assert_eq!(format!("{:?}", escaped_str), "L\"\\0\\n\\r\\t\\\"\\\\\"");
    }
}
