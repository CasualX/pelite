/*!
Nul-terminated C string.
*/

use std::{cmp, fmt, mem, ops, str};

use crate::util::{split_f, FromBytes};

//----------------------------------------------------------------

/// Nul-terminated C string.
#[derive(Eq, Ord, Hash)]
pub struct CStr {
    bytes: [u8],
}

impl CStr {
    /// Returns the empty nul-terminated C string.
    pub fn empty() -> &'static CStr {
        unsafe { CStr::from_bytes_unchecked(&[0]) }
    }
    /// Scans the byte slice for a nul-terminated C string.
    ///
    /// Returns `None` if no nul byte was found.
    ///
    /// # Examples
    ///
    /// ```
    /// use pelite::util::CStr;
    ///
    /// let c_str = CStr::from_bytes(b"Hello\0World\0").unwrap();
    /// assert_eq!(c_str.to_str(), Ok("Hello"));
    /// assert_eq!(c_str.c_str(), b"Hello\0");
    /// assert_eq!(c_str.len(), 5);
    ///
    /// let no_nul = CStr::from_bytes(b"not nul terminated");
    /// assert_eq!(no_nul, None);
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Option<&CStr> {
        let len = bytes.iter().position(|&byte| byte == 0)?;
        Some(unsafe { CStr::from_bytes_unchecked(bytes.get_unchecked(..len + 1)) })
    }
    /// Interprets a byte slice as a C string.
    ///
    /// # Safety
    ///
    /// Ensure that the byte slice ends with the only nul byte.
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &CStr {
        mem::transmute(bytes)
    }
    /// Gets the C string as a nul terminated byte slice.
    pub fn c_str(&self) -> &[u8] {
        &self.bytes
    }
    /// Casts the C string to an UTF8 validated `str`.
    pub fn to_str(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(self.as_ref())
    }
}

impl FromBytes for CStr {
    const MIN_SIZE_OF: usize = 0;
    const ALIGN_OF: usize = 1;
    unsafe fn from_bytes(bytes: &[u8]) -> Option<&CStr> {
        CStr::from_bytes(bytes)
    }
}

//----------------------------------------------------------------

impl<T: AsRef<[u8]> + ?Sized> PartialEq<T> for CStr {
    fn eq(&self, rhs: &T) -> bool {
        self.as_ref() == rhs.as_ref()
    }
}
impl<T: AsRef<[u8]> + ?Sized> PartialOrd<T> for CStr {
    fn partial_cmp(&self, rhs: &T) -> Option<cmp::Ordering> {
        self.as_ref().partial_cmp(rhs.as_ref())
    }
}

//----------------------------------------------------------------

impl ops::Deref for CStr {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        self.as_ref()
    }
}
impl AsRef<[u8]> for CStr {
    fn as_ref(&self) -> &[u8] {
        // Strip the nul byte
        let len = self.bytes.len() - 1;
        unsafe { self.bytes.get_unchecked(..len) }
    }
}

//----------------------------------------------------------------
// Formatting

impl fmt::Debug for CStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("\"")?;
        let mut bytes = self.as_ref();
        while bytes.len() > 0 {
            let byte = bytes[0];
            match byte {
                b'\0' => {
                    bytes = &bytes[1..];
                    f.write_str("\\0")?;
                }
                b'\n' => {
                    bytes = &bytes[1..];
                    f.write_str("\\n")?;
                }
                b'\r' => {
                    bytes = &bytes[1..];
                    f.write_str("\\r")?;
                }
                b'\t' => {
                    bytes = &bytes[1..];
                    f.write_str("\\t")?;
                }
                b'"' => {
                    bytes = &bytes[1..];
                    f.write_str("\\\"")?;
                }
                b'\\' => {
                    bytes = &bytes[1..];
                    f.write_str("\\\\")?;
                }
                0x20...0x7E => {
                    let (s, tail) = split_f(bytes, |&byte| {
                        byte < 0x20 || byte >= 0x80 || byte == b'"' || byte == b'\\'
                    });
                    bytes = tail;
                    let s = unsafe { str::from_utf8_unchecked(s) };
                    f.write_str(s)?;
                }
                _ => {
                    let (s, tail) = split_f(bytes, |&byte| byte >= 0x20 && byte < 0x80);
                    bytes = tail;
                    for &byte in s {
                        write!(f, "\\x{:02X}", byte)?;
                    }
                }
            };
        }
        f.write_str("\"")
    }
}
impl fmt::Display for CStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut bytes = self.as_ref();
        while bytes.len() > 0 {
            let byte = bytes[0];
            // Display ascii as expected
            if byte < 0x80 {
                let (s, tail) = split_f(bytes, |&byte| byte >= 0x80);
                bytes = tail;
                let s = unsafe { str::from_utf8_unchecked(s) };
                f.write_str(s)?;
            }
            // Escape all other bytes
            else {
                let (s, tail) = split_f(bytes, |&byte| byte < 0x80);
                bytes = tail;
                for &byte in s {
                    write!(f, "\\x{:02X}", byte)?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for CStr {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::CStr;

    #[test]
    fn from_bytes() {
        assert_eq!(
            CStr::from_bytes(b"this is a c str\0").unwrap().c_str(),
            b"this is a c str\0"
        );
        assert_eq!(CStr::from_bytes(b"no nul terminator"), None);
        assert_eq!(
            CStr::from_bytes(b"valid utf8\0").unwrap().to_str(),
            Ok("valid utf8")
        );
        assert_eq!(CStr::from_bytes(b"length is eighteen\0").unwrap().len(), 18);
        assert_eq!(CStr::from_bytes(b"length is eighteen\0").unwrap().len(), 18);
    }

    #[test]
    fn fmt() {
        assert_eq!(
            format!("{}", unsafe {
                CStr::from_bytes_unchecked(b"\tabc\n\xFFhello\x80world\0")
            }),
            "\tabc\n\\xFFhello\\x80world"
        );
        assert_eq!(
            format!("{:?}", unsafe {
                CStr::from_bytes_unchecked(b"\tabc\n\xFFhello\x80world\0")
            }),
            r#""\tabc\n\xFFhello\x80world""#
        );
    }
}
