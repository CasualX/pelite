use crate::image::*;
use crate::Pod;
use std::{fmt, mem, ops, slice};

//----------------------------------------------------------------

/// Section header.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct SectionHeader(IMAGE_SECTION_HEADER);

impl SectionHeader {
    /// Returns the name as a byte slice.
    #[inline]
    pub fn name_bytes(&self) -> &[u8] {
        crate::util::trimn(&self.0.Name)
    }
    /// Returns the name.
    pub fn name(&self) -> Result<&str, &[u8]> {
        crate::util::parsen(&self.0.Name)
    }
    /// Returns the virtual range.
    #[inline]
    pub fn virtual_range(&self) -> std::ops::Range<u32> {
        let start = self.0.VirtualAddress;
        let end = u32::wrapping_add(self.0.VirtualAddress, self.0.VirtualSize);
        start..end
    }
    /// Returns the file range.
    #[inline]
    pub fn file_range(&self) -> std::ops::Range<u32> {
        let start = self.0.PointerToRawData;
        let end = u32::wrapping_add(self.0.PointerToRawData, self.0.SizeOfRawData);
        start..end
    }
}

unsafe impl Pod for SectionHeader {}

impl ops::Deref for SectionHeader {
    type Target = IMAGE_SECTION_HEADER;
    #[inline]
    fn deref(&self) -> &IMAGE_SECTION_HEADER {
        &self.0
    }
}

impl fmt::Debug for SectionHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = self.name();
        let name = match &name {
            Ok(name) => name as &dyn fmt::Debug,
            Err(name) => name as &dyn fmt::Debug,
        };
        f.debug_struct("SectionHeader")
            .field("Name", name)
            .field(
                "VirtualAddress",
                &format_args!("{:#x}", self.0.VirtualAddress),
            )
            .field("VirtualSize", &format_args!("{:#x}", self.0.VirtualSize))
            .field(
                "PointerToRawData",
                &format_args!("{:#x}", self.0.PointerToRawData),
            )
            .field(
                "SizeOfRawData",
                &format_args!("{:#x}", self.0.SizeOfRawData),
            )
            .field(
                "Characteristics",
                &format_args!("{:#x}", self.0.Characteristics),
            )
            .finish()
    }
}

//----------------------------------------------------------------

/// Section headers.
#[repr(transparent)]
pub struct SectionHeaders([IMAGE_SECTION_HEADER]);

impl SectionHeaders {
    pub(crate) fn new(image: &[IMAGE_SECTION_HEADER]) -> &SectionHeaders {
        unsafe { mem::transmute(image) }
    }
    /// Returns the underlying slice of section headers.
    #[inline]
    pub fn image(&self) -> &[IMAGE_SECTION_HEADER] {
        &self.0
    }
    /// Gets the section headers as a slice of `SectionHeader`.
    #[inline]
    pub fn as_slice(&self) -> &[SectionHeader] {
        unsafe { mem::transmute(self) }
    }
    /// Returns an iterator over the `SectionHeader` elements.
    #[inline]
    pub fn iter(&self) -> slice::Iter<'_, SectionHeader> {
        self.as_slice().iter()
    }
    /// Finds a section header by its name.
    #[inline]
    pub fn by_name<S: ?Sized + AsRef<[u8]>>(&self, name: &S) -> Option<&SectionHeader> {
        // Names have a max length, if larger they will never match
        let name = name.as_ref();
        if name.len() > IMAGE_SIZEOF_SHORT_NAME {
            return None;
        }
        // Copy the prefix into a new buffer for easy comparison
        let mut name_buf = [0u8; IMAGE_SIZEOF_SHORT_NAME];
        for i in 0..name.len() {
            name_buf[i] = name[i];
        }
        for sect in self.iter() {
            if sect.0.Name == name_buf {
                return Some(sect);
            }
        }
        None
    }
    /// Finds a section header by its RVA.
    #[inline]
    pub fn by_rva(&self, rva: u32) -> Option<&SectionHeader> {
        for sect in self.iter() {
            // FIXME! Should this round up the VirtualSize to the next virtual section alignment?
            if rva >= sect.VirtualAddress
                && rva < u32::wrapping_add(sect.VirtualAddress, sect.VirtualSize)
            {
                return Some(sect);
            }
        }
        None
    }
}

unsafe impl Pod for SectionHeaders {}

impl<'a> IntoIterator for &'a SectionHeaders {
    type Item = &'a SectionHeader;
    type IntoIter = slice::Iter<'a, SectionHeader>;
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().into_iter()
    }
}

impl fmt::Debug for SectionHeaders {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}

//----------------------------------------------------------------

#[cfg(feature = "serde")]
pub(crate) fn serialize_name<S: ::serde::ser::Serializer>(
    name: &[u8; IMAGE_SIZEOF_SHORT_NAME],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match crate::util::parsen(name) {
        Ok(name) => serializer.serialize_str(name),
        Err(name) => serializer.serialize_bytes(name),
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::{SectionHeader, SectionHeaders};
    use crate::util::serde_helper::*;

    impl serde::Serialize for SectionHeaders {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.collect_seq(self.iter())
        }
    }

    impl serde::Serialize for SectionHeader {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.0.serialize(serializer)
        }
    }
}
