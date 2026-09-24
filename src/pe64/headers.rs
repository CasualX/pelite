use super::*;

impl<'a> PeFile<'a> {
	/// Returns the DOS header.
	pub fn dos_header(self) -> &'a IMAGE_DOS_HEADER {
		unsafe { pe_impl::dos_header(self.image()) }
	}
	#[doc = include_str!("../docs/dos_image.md")]
	pub fn dos_image(self) -> &'a [u8] {
		unsafe { pe_impl::dos_image(self.image()) }
	}
	/// Returns the NT headers.
	pub fn nt_headers(self) -> &'a IMAGE_NT_HEADERS {
		unsafe { pe_impl::nt_headers(self.image()) }
	}
	/// Returns the file header.
	pub fn file_header(self) -> &'a IMAGE_FILE_HEADER {
		unsafe { pe_impl::file_header(self.image()) }
	}
	/// Returns the optional header.
	pub fn optional_header(self) -> &'a IMAGE_OPTIONAL_HEADER {
		unsafe { pe_impl::optional_header(self.image()) }
	}
	/// Returns the data directory.
	pub fn data_directory(self) -> &'a [IMAGE_DATA_DIRECTORY] {
		unsafe { pe_impl::data_directory(self.image()) }
	}
	/// Returns the section headers.
	pub fn section_headers(self) -> &'a PeSectionHeaders {
		unsafe { pe_impl::section_headers(self.image()) }
	}
	/// Returns the headers.
	pub fn headers(self) -> PeHeaders<Self> {
		PeHeaders { pe: self }
	}
}

impl<'a> PeView<'a> {
	/// Returns the DOS header.
	pub fn dos_header(self) -> &'a IMAGE_DOS_HEADER {
		unsafe { pe_impl::dos_header(self.image()) }
	}
	#[doc = include_str!("../docs/dos_image.md")]
	pub fn dos_image(self) -> &'a [u8] {
		unsafe { pe_impl::dos_image(self.image()) }
	}
	/// Returns the NT headers.
	pub fn nt_headers(self) -> &'a IMAGE_NT_HEADERS {
		unsafe { pe_impl::nt_headers(self.image()) }
	}
	/// Returns the file header.
	pub fn file_header(self) -> &'a IMAGE_FILE_HEADER {
		unsafe { pe_impl::file_header(self.image()) }
	}
	/// Returns the optional header.
	pub fn optional_header(self) -> &'a IMAGE_OPTIONAL_HEADER {
		unsafe { pe_impl::optional_header(self.image()) }
	}
	/// Returns the data directory.
	pub fn data_directory(self) -> &'a [IMAGE_DATA_DIRECTORY] {
		unsafe { pe_impl::data_directory(self.image()) }
	}
	/// Returns the section headers.
	pub fn section_headers(self) -> &'a PeSectionHeaders {
		unsafe { pe_impl::section_headers(self.image()) }
	}
	/// Returns the headers.
	pub fn headers(self) -> PeHeaders<Self> {
		PeHeaders { pe: self }
	}
}

pub use crate::wrap::sections::*;

/// Describes the PE headers.
#[derive(Copy, Clone)]
pub struct PeHeaders<P> {
	pe: P,
}

impl<'a, P: Copy + Pe<'a>> PeHeaders<P> {
	pub(crate) fn new(pe: P) -> PeHeaders<P> {
		PeHeaders { pe }
	}
	/// Returns the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Returns the PE headers as a byte slice.
	pub fn image(&self) -> &'a [u8] {
		unsafe { self.pe.image().get_unchecked(..self.pe.optional_header().SizeOfHeaders as usize) }
	}
	/// Calculates the optional header's CheckSum.
	pub fn check_sum(&self) -> u32 {
		let image = self.pe.image();
		#[rustfmt::skip]
		let check_sum_position = (
			self.pe.dos_header().e_lfanew as usize +
			dataview::offset_of!(IMAGE_NT_HEADERS.OptionalHeader) +
			dataview::offset_of!(IMAGE_OPTIONAL_HEADER.CheckSum)) / 4;
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

impl<'a, P: Copy + Pe<'a>> PeHeaders<P> {
	#[doc = include_str!("../docs/rva_to_file_offset.md")]
	#[inline]
	pub fn rva_to_file_offset(self, rva: Rva) -> Result<usize> {
		unsafe { pe_impl::rva_to_file_offset(self.image(), rva) }
	}
	#[doc = include_str!("../docs/file_offset_to_rva.md")]
	#[inline]
	pub fn file_offset_to_rva(self, file_offset: usize) -> Result<Rva> {
		unsafe { pe_impl::file_offset_to_rva(self.image(), file_offset) }
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

serde_impl! {
	use crate::stringify;

	impl<'a, P: Copy + Pe<'a>> Serialize for PeHeaders<P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("PeHeaders", 5)?;
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
	impl<'a, P: Copy + Pe<'a>> Serialize for Details<P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("Details", 11)?;

			state.serialize_field("DosHeader.e_magic", "MZ")?;
			state.serialize_field("NtHeaders.Signature", "PE")?;

			let file_header = self.pe.file_header();
			state.serialize_field("FileHeader.Machine", &stringify::Machine(file_header.Machine).to_str())?;
			state.serialize_field("FileHeader.Characteristics", &SerdeIter(stringify::FileChars(file_header.Characteristics).to_strs()))?;

			let optional_header = self.pe.optional_header();
			state.serialize_field("OptionalHeader.Magic", &stringify::OptionalMagic(optional_header.Magic).to_str())?;
			state.serialize_field("OptionalHeader.CheckSum", &PeHeaders { pe: self.pe }.check_sum())?;
			state.serialize_field("OptionalHeader.Subsystem", &stringify::Subsystem(optional_header.Subsystem).to_str())?;
			state.serialize_field("OptionalHeader.DllCharacteristics", &SerdeIter(stringify::DllChars(optional_header.DllCharacteristics).to_strs()))?;

			let data_directory_names = (0..self.pe.data_directory().len()).map(stringify::DirectoryEntry).map(stringify::DirectoryEntry::to_str);
			state.serialize_field("DataDirectory.Names", &SerdeIter(data_directory_names))?;

			let data_directory_sects = self.pe.data_directory().iter().map(|dd| {
				self.pe.section_headers().iter()
					.position(|&sect| dd.VirtualAddress >= sect.VirtualAddress && dd.VirtualAddress < sect.VirtualAddress.wrapping_add(sect.VirtualSize))
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
