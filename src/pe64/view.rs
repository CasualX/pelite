/*!
PE view.
*/

use std::prelude::v1::*;

use std::{cmp, slice};

use crate::Result;

use super::image::*;
use super::pe::validate_headers;
use super::{Align, Pe, PeObject};

/// View into a mapped PE image.
#[derive(Copy, Clone)]
pub struct PeView<'a> {
    image: &'a [u8],
}

current_target! {
    impl PeView<'static> {
        /// Constructs a view of the module this code is executing in.
        #[inline]
        pub unsafe fn new() -> PeView<'static> {
            Self::module(image_base() as *const _ as *const u8)
        }
    }
}
impl<'a> PeView<'a> {
    /// Constructs a view from a byte slice.
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
    pub fn from_bytes<T: AsRef<[u8]> + ?Sized>(image: &'a T) -> Result<PeView<'a>> {
        let image = image.as_ref();
        let _ = validate_headers(image)?;
        Ok(PeView { image })
    }
    /// Constructs a new view from module handle.
    ///
    /// # Safety
    ///
    /// The underlying memory is borrowed and an unbounded lifetime is returned.
    /// Ensure the lifetime outlives this view instance!
    ///
    /// No sanity or safety checks are done to make sure this is really PE32(+) image.
    /// When using this with a `HMODULE` from the system the caller must be sure this is a PE32(+) image.
    #[inline]
    pub unsafe fn module(base: *const u8) -> PeView<'a> {
        let dos = &*(base as *const IMAGE_DOS_HEADER);
        let nt = &*(base.offset(dos.e_lfanew as isize) as *const IMAGE_NT_HEADERS);
        PeView {
            image: slice::from_raw_parts(base, nt.OptionalHeader.SizeOfImage as usize),
        }
    }
    /// Converts the view to file alignment.
    pub fn to_file(self) -> Vec<u8> {
        let (sizeof_headers, sizeof_image) = {
            let optional_header = self.optional_header();
            (optional_header.SizeOfHeaders, optional_header.SizeOfImage)
        };

        // Figure out the size of the file image
        let mut file_size = sizeof_headers;
        for section in self.section_headers() {
            file_size = cmp::max(
                file_size,
                u32::wrapping_add(section.PointerToRawData, section.SizeOfRawData),
            );
        }
        // Clamp to the actual image size...
        file_size = cmp::min(file_size, sizeof_image);

        // Zero fill the underlying file
        let mut vec = vec![0u8; file_size as usize];

        // Start by copying the headers
        let image = self.image();
        unsafe {
            // Validated by constructor
            let dest_headers = vec.get_unchecked_mut(..sizeof_headers as usize);
            let src_headers = image.get_unchecked(..sizeof_headers as usize);
            dest_headers.copy_from_slice(src_headers);
        }

        // Copy the section image data
        for section in self.section_headers() {
            let dest = vec.get_mut(
                section.PointerToRawData as usize
                    ..u32::wrapping_add(section.PointerToRawData, section.SizeOfRawData) as usize,
            );
            let src = image.get(
                section.VirtualAddress as usize
                    ..u32::wrapping_add(section.VirtualAddress, section.VirtualSize) as usize,
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

unsafe impl<'a> Pe<'a> for PeView<'a> {}

unsafe impl<'a> PeObject<'a> for PeView<'a> {
    fn image(&self) -> &'a [u8] {
        self.image
    }
    fn align(&self) -> Align {
        Align::Section
    }
    #[cfg(feature = "serde")]
    fn serde_name(&self) -> &'static str {
        "PeView"
    }
}

//----------------------------------------------------------------

#[cfg(feature = "serde")]
impl<'a> serde::Serialize for PeView<'a> {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        super::pe::serialize_pe(*self, serializer)
    }
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::PeView;
    use crate::Error;

    #[test]
    fn from_byte_slice() {
        assert!(match PeView::from_bytes(&[]) {
            Err(Error::Bounds) => true,
            _ => false,
        });
    }
}
