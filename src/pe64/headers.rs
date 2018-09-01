/*!
PE headers.
 */

use std::{slice};

use super::Pe;
use super::image::*;

/// Describes the PE headers.
#[derive(Copy, Clone)]
pub struct Headers<P> {
	pe: P,
}

impl<'a, P: Pe<'a> + Copy> Headers<P> {
	pub(crate) fn new(pe: P) -> Headers<P> {
		Headers { pe }
	}
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Gets the PE headers as a byte slice.
	pub fn image(&self) -> &'a [u8] {
		unsafe { self.pe.image().get_unchecked(..self.pe.optional_header().SizeOfHeaders as usize) }
	}
	/// Calculates the optional header's check_sum.
	pub fn check_sum(&self) -> u32 {
		let image = self.pe.image();
		let check_sum_position = (self.pe.dos_header().e_lfanew as usize +
			offset_of!(IMAGE_NT_HEADERS, OptionalHeader.CheckSum)) / 4;
		let dwords = unsafe { slice::from_raw_parts(image.as_ptr() as *const u32, image.len() / 4) };
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
}

/*
	"headers": {
		"DosHeader": { .. }
		"DosImage": "base64encoded=",
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
	use util::serde_helper::*;
	use stringify;
	use super::{Pe, Headers};

	impl<'a, P: Pe<'a> + Copy> Serialize for Headers<P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let is_human_readable = serializer.is_human_readable();
			let mut state = serializer.serialize_struct("Headers", 6)?;
			state.serialize_field("DosHeader", self.pe.dos_header())?;
			#[cfg(feature = "data-encoding")]
			(if is_human_readable {
				let string = ::data_encoding::BASE64.encode(self.pe.dos_image());
				state.serialize_field("DosImage", &string)
			}
			else {
				state.serialize_field("DosImage", self.pe.dos_image())
			})?;
			#[cfg(not(feature = "data-encoding"))]
			state.serialize_field("DosImage", self.pe.dos_image())?;
			state.serialize_field("NtHeaders", self.pe.nt_headers())?;
			state.serialize_field("DataDirectory", self.pe.data_directory())?;
			state.serialize_field("SectionHeaders", self.pe.section_headers())?;
			state.serialize_field("details", &Details { pe: self.pe })?;
			state.end()
		}
	}

	struct Details<P> { pe: P }
	impl<'a, P: Pe<'a> + Copy> Serialize for Details<P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("Details", 11)?;

			state.serialize_field("DosHeader.e_magic", "MZ")?;
			state.serialize_field("NtHeaders.Signature", "PE")?;

			let file_header = self.pe.file_header();
			state.serialize_field("FileHeader.Machine", &stringify::machine(file_header.Machine))?;
			let file_chars = (0..16).filter(|&i| file_header.Characteristics & (1 << i) != 0).map(stringify::file_chars);
			state.serialize_field("FileHeader.Characteristics", &SerdeIter(file_chars))?;

			let optional_header = self.pe.optional_header();
			state.serialize_field("OptionalHeader.Magic", &stringify::optional_magic(optional_header.Magic))?;
			state.serialize_field("OptionalHeader.CheckSum", &Headers { pe: self.pe }.check_sum())?;
			state.serialize_field("OptionalHeader.Subsystem", &stringify::subsystem(optional_header.Subsystem))?;
			let dll_chars = (0..16).filter(|&i| optional_header.DllCharacteristics & (1 << i) != 0).map(stringify::dll_chars);
			state.serialize_field("OptionalHeader.DllCharacteristics", &SerdeIter(dll_chars))?;

			let data_directory_names = (0..self.pe.data_directory().len()).map(stringify::directory_entry);
			state.serialize_field("DataDirectory.Names", &SerdeIter(data_directory_names))?;

			let data_directory_sects = self.pe.data_directory().iter().map(|dd| {
				self.pe.section_headers().iter().position(|&sect| dd.VirtualAddress >= sect.VirtualAddress && dd.VirtualAddress < sect.VirtualAddress + sect.VirtualSize)
			});
			state.serialize_field("DataDirectory.Sections", &SerdeIter(data_directory_sects))?;

			let sect_chars = self.pe.section_headers().iter().map(|sect| {
				let chars = sect.Characteristics;
				SerdeIter((0..32).filter(move |&i| chars & (1 << i) != 0).map(stringify::section_chars))
			});
			state.serialize_field("SectionHeaders.Characteristics", &SerdeIter(sect_chars))?;

			state.end()
		}
	}
}
