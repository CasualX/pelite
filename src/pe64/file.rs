/*!
PE file.
*/

use std::prelude::v1::*;

use crate::Result;

use super::pe::validate_headers;
use super::{Align, Pe, PeObject};

/// View into an unmapped PE file.
#[derive(Copy, Clone)]
pub struct PeFile<'a> {
    image: &'a [u8],
}

impl<'a> PeFile<'a> {
    /// Constructs a file view from a byte slice.
    ///
    /// # Errors
    ///
    /// * [`Bounds`](../enum.Error.html#variant.Bounds):
    ///   The byte slice is too small to fit the PE headers.
    ///
    /// * [`Misaligned`](../enum.Error.html#variant.Misaligned):
    ///   The minimum alignment of 4 is not satisfied.
    ///
    /// * [`BadMagic`](../enum.Error.html#variant.BadMagic):
    ///   This is not a PE file.
    ///
    /// * [`PeMagic`](../enum.Error.html#variant.PeMagic):
    ///   Trying to parse a PE32 file with the PE32+ parser and vice versa.
    ///
    /// * [`Insanity`](../enum.Error.html#variant.Insanity):
    ///   Reasonable limits on `e_lfanew`, `SizeOfHeaders` or `NumberOfSections` are exceeded.
    pub fn from_bytes<T: AsRef<[u8]> + ?Sized>(image: &'a T) -> Result<PeFile<'a>> {
        let image = image.as_ref();
        let _ = validate_headers(image)?;
        Ok(PeFile { image })
    }
    /// Converts the file to section alignment.
    pub fn to_view(self) -> Vec<u8> {
        let (sizeof_headers, sizeof_image) = {
            let optional_header = self.optional_header();
            (optional_header.SizeOfHeaders, optional_header.SizeOfImage)
        };

        // Zero fill the underlying image
        let mut vec = vec![0u8; sizeof_image as usize];

        // Start by copying the headers
        let image = self.image();
        unsafe {
            // Validated by constructor
            let dest_headers = vec.get_unchecked_mut(..sizeof_headers as usize);
            let src_headers = image.get_unchecked(..sizeof_headers as usize);
            dest_headers.copy_from_slice(src_headers);
        }

        // Copy the section file data
        for section in self.section_headers() {
            let dest = vec.get_mut(
                section.VirtualAddress as usize
                    ..u32::wrapping_add(section.VirtualAddress, section.VirtualSize) as usize,
            );
            let src = image.get(
                section.PointerToRawData as usize
                    ..u32::wrapping_add(section.PointerToRawData, section.SizeOfRawData) as usize,
            );
            // Skip invalid sections...
            if let (Some(dest), Some(src)) = (dest, src) {
                dest.copy_from_slice(src);
            }
        }

        vec
    }
}

//----------------------------------------------------------------

unsafe impl<'a> Pe<'a> for PeFile<'a> {}

unsafe impl<'a> PeObject<'a> for PeFile<'a> {
    fn image(&self) -> &'a [u8] {
        self.image
    }
    fn align(&self) -> Align {
        Align::File
    }
    #[cfg(feature = "serde")]
    fn serde_name(&self) -> &'static str {
        "PeFile"
    }
}

//----------------------------------------------------------------

#[cfg(feature = "serde")]
impl<'a> serde::Serialize for PeFile<'a> {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        super::pe::serialize_pe(*self, serializer)
    }
}

//----------------------------------------------------------------

#[test]
fn from_byte_slice() {
    assert!(match PeFile::from_bytes(&[]) {
        Err(crate::Error::Bounds) => true,
        _ => false,
    });
}
