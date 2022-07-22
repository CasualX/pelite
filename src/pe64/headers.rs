/*!
PE headers.
 */

use dataview::PodMethods;
use std::ops::Range;
use std::slice;

use super::image::*;
use super::Pe;

pub use crate::wrap::sections::*;

/// Describes the PE headers.
#[derive(Copy, Clone)]
pub struct Headers<P> {
    pe: P,
}

impl<'a, P: Pe<'a>> Headers<P> {
    pub(crate) fn new(pe: P) -> Headers<P> {
        Headers { pe }
    }
    /// Gets the PE instance.
    pub fn pe(&self) -> P {
        self.pe
    }
    /// Gets the PE headers as a byte slice.
    pub fn image(&self) -> &'a [u8] {
        unsafe {
            self.pe
                .image()
                .get_unchecked(..self.pe.optional_header().SizeOfHeaders as usize)
        }
    }
    /// Calculates the optional header's CheckSum.
    pub fn check_sum(&self) -> u32 {
        let image = self.pe.image();
        let opt_header_checksum_offset = {
            let value = IMAGE_NT_HEADERS::zeroed();
            &value.OptionalHeader.CheckSum as *const _ as usize - &value as *const _ as usize
        };
        let check_sum_position =
            (self.pe.dos_header().e_lfanew as usize + opt_header_checksum_offset) / 4;
        let dwords =
            unsafe { slice::from_raw_parts(image.as_ptr() as *const u32, image.len() / 4) };
        let mut check_sum = 0u64;
        for i in 0..dwords.len() {
            if i == check_sum_position {
                continue;
            }
            let dw = dwords[i];
            check_sum = (check_sum & 0xffffffff) + dw as u64 + (check_sum >> 32);
            if check_sum > 0xffffffff {
                check_sum = (check_sum & 0xffffffff) + (check_sum >> 32);
            }
        }
        check_sum = (check_sum & 0xffff) + (check_sum >> 16);
        check_sum = check_sum + (check_sum >> 16);
        check_sum = check_sum & 0xffff;

        check_sum += image.len() as u64;

        check_sum as u32
    }
    /// Gets the code range from the optional header.
    pub fn code_range(&self) -> Range<Rva> {
        let optional_header = self.pe.optional_header();
        optional_header.BaseOfCode
            ..u32::wrapping_add(optional_header.BaseOfCode, optional_header.SizeOfCode)
    }
    /// Gets the full image range excluding the PE headers.
    pub fn image_range(&self) -> Range<Rva> {
        let optional_header = self.pe.optional_header();
        optional_header.SizeOfHeaders..optional_header.SizeOfImage
    }
}

/*
    "headers": {
        "DosHeader": { .. }
        "NtHeaders": {
            "Signature": ..,
            "FileHeader": { .. }
            "OptionalHeader": { .. }
        }
        "DataDirectory": [ .. ]
        "SectionHeaders": [ .. ]
        "details": {
            "DosHeader.e_magic": "MZ",
            "NtHeaders.Signature": "PE",
            "FileHeader.Machine": "AMD",
            "FileHeader.Characteristics": [],
            "OptionalHeader.Magic": "PE32+",
            "DataDirectory.Names": ["Exports", "Imports", ..],
            "DataDirectory.Sections": [1, 1, ..],
            "SectionHeaders.Characteristics": [["executable", "read", "write"], ["read"], ["read", "write"]],
        }
    }
*/

#[cfg(feature = "serde")]
mod serde {
    use super::{Headers, Pe};
    use crate::stringify;
    use crate::util::serde_helper::*;

    impl<'a, P: Pe<'a>> Serialize for Headers<P> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut state = serializer.serialize_struct("Headers", 5)?;
            state.serialize_field("DosHeader", self.pe.dos_header())?;
            state.serialize_field("NtHeaders", self.pe.nt_headers())?;
            state.serialize_field("DataDirectory", self.pe.data_directory())?;
            state.serialize_field("SectionHeaders", self.pe.section_headers())?;
            state.serialize_field("details", &Details { pe: self.pe })?;
            state.end()
        }
    }

    struct Details<P> {
        pe: P,
    }
    impl<'a, P: Pe<'a>> Serialize for Details<P> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut state = serializer.serialize_struct("Details", 11)?;

            state.serialize_field("DosHeader.e_magic", "MZ")?;
            state.serialize_field("NtHeaders.Signature", "PE")?;

            let file_header = self.pe.file_header();
            state.serialize_field(
                "FileHeader.Machine",
                &stringify::Machine(file_header.Machine).to_str(),
            )?;
            state.serialize_field(
                "FileHeader.Characteristics",
                &SerdeIter(stringify::FileChars(file_header.Characteristics).to_strs()),
            )?;

            let optional_header = self.pe.optional_header();
            state.serialize_field(
                "OptionalHeader.Magic",
                &stringify::OptionalMagic(optional_header.Magic).to_str(),
            )?;
            state.serialize_field(
                "OptionalHeader.CheckSum",
                &Headers { pe: self.pe }.check_sum(),
            )?;
            state.serialize_field(
                "OptionalHeader.Subsystem",
                &stringify::Subsystem(optional_header.Subsystem).to_str(),
            )?;
            state.serialize_field(
                "OptionalHeader.DllCharacteristics",
                &SerdeIter(stringify::DllChars(optional_header.DllCharacteristics).to_strs()),
            )?;

            let data_directory_names = (0..self.pe.data_directory().len())
                .map(stringify::DirectoryEntry)
                .map(stringify::DirectoryEntry::to_str);
            state.serialize_field("DataDirectory.Names", &SerdeIter(data_directory_names))?;

            let data_directory_sects = self.pe.data_directory().iter().map(|dd| {
                self.pe.section_headers().iter().position(|&sect| {
                    dd.VirtualAddress >= sect.VirtualAddress
                        && dd.VirtualAddress < sect.VirtualAddress + sect.VirtualSize
                })
            });
            state.serialize_field("DataDirectory.Sections", &SerdeIter(data_directory_sects))?;

            let sections_chars = self.pe.section_headers().iter().map(|sect| {
                let section_chars = sect.Characteristics;
                SerdeIter(stringify::SectionChars(section_chars).to_strs())
            });
            state.serialize_field("SectionHeaders.Characteristics", &SerdeIter(sections_chars))?;

            state.end()
        }
    }
}
