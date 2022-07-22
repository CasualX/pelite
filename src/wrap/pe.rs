use super::Wrap;
use crate::*;

/// The specific alignment used by the view.
///
/// See [the module-level documentation](index.html#getting-started) for more information.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Align {
    /// The view uses file alignment, typically 512 bytes.
    File,
    /// The view uses section alignment, typically 4 KiB.
    Section,
}

pub(crate) fn get_section_bytes<'a>(
    image: &'a [u8],
    section_header: &image::IMAGE_SECTION_HEADER,
    align: Align,
) -> Result<&'a [u8]> {
    let (address, size) = match align {
        Align::File => (
            section_header.PointerToRawData,
            section_header.SizeOfRawData,
        ),
        Align::Section => (section_header.VirtualAddress, section_header.VirtualSize),
    };
    if address == 0 {
        return Err(Error::Null);
    }
    let start = address as usize;
    let end = address.wrapping_add(size) as usize;
    image.get(start..end).ok_or(Error::Bounds)
}

impl<'a, Pe32: pe32::Pe<'a>, Pe64: pe64::Pe<'a>> Wrap<Pe32, Pe64> {
    #[inline]
    pub fn image(&self) -> &'a [u8] {
        match self {
            Wrap::T32(pe32) => pe32.image(),
            Wrap::T64(pe64) => pe64.image(),
        }
    }
    #[inline]
    pub fn align(&self) -> Align {
        match self {
            Wrap::T32(pe32) => pe32.align(),
            Wrap::T64(pe64) => pe64.align(),
        }
    }
    #[inline]
    pub fn as_ref(&self) -> Wrap<&Pe32, &Pe64> {
        match self {
            Wrap::T32(pe32) => Wrap::T32(pe32),
            Wrap::T64(pe64) => Wrap::T64(pe64),
        }
    }

    //----------------------------------------------------------------

    #[inline]
    pub fn dos_header(&self) -> &'a image::IMAGE_DOS_HEADER {
        match self {
            Wrap::T32(pe32) => pe32.dos_header(),
            Wrap::T64(pe64) => pe64.dos_header(),
        }
    }
    #[inline]
    pub fn dos_image(&self) -> &'a [u8] {
        match self {
            Wrap::T32(pe32) => pe32.dos_image(),
            Wrap::T64(pe64) => pe64.dos_image(),
        }
    }
    #[inline]
    pub fn nt_headers(&self) -> Wrap<&'a image::IMAGE_NT_HEADERS32, &'a image::IMAGE_NT_HEADERS64> {
        match self {
            Wrap::T32(pe32) => Wrap::T32(pe32.nt_headers()),
            Wrap::T64(pe64) => Wrap::T64(pe64.nt_headers()),
        }
    }
    #[inline]
    pub fn file_header(&self) -> &'a image::IMAGE_FILE_HEADER {
        match self {
            Wrap::T32(pe32) => pe32.file_header(),
            Wrap::T64(pe64) => pe64.file_header(),
        }
    }
    #[inline]
    pub fn optional_header(
        &self,
    ) -> Wrap<&'a image::IMAGE_OPTIONAL_HEADER32, &'a image::IMAGE_OPTIONAL_HEADER64> {
        match self {
            Wrap::T32(pe32) => Wrap::T32(pe32.optional_header()),
            Wrap::T64(pe64) => Wrap::T64(pe64.optional_header()),
        }
    }
    #[inline]
    pub fn data_directory(&self) -> &'a [image::IMAGE_DATA_DIRECTORY] {
        match self {
            Wrap::T32(pe32) => pe32.data_directory(),
            Wrap::T64(pe64) => pe64.data_directory(),
        }
    }
    #[inline]
    pub fn section_headers(&self) -> &'a super::sections::SectionHeaders {
        match self {
            Wrap::T32(pe32) => pe32.section_headers(),
            Wrap::T64(pe64) => pe64.section_headers(),
        }
    }

    //----------------------------------------------------------------

    #[inline]
    pub fn slice(&self, rva: u32, min_size: usize, align: usize) -> Result<&'a [u8]> {
        match self {
            Wrap::T32(pe32) => pe32.slice(rva, min_size, align),
            Wrap::T64(pe64) => pe64.slice(rva, min_size, align),
        }
    }
    #[inline]
    pub fn slice_bytes(&self, rva: u32) -> Result<&'a [u8]> {
        match self {
            Wrap::T32(pe32) => pe32.slice_bytes(rva),
            Wrap::T64(pe64) => pe64.slice_bytes(rva),
        }
    }
    #[inline]
    pub fn get_section_bytes(
        &self,
        section_header: &image::IMAGE_SECTION_HEADER,
    ) -> Result<&'a [u8]> {
        get_section_bytes(self.image(), section_header, self.align())
    }

    //----------------------------------------------------------------

    #[inline]
    pub fn derva<T>(&self, rva: u32) -> Result<&'a T>
    where
        T: Pod,
    {
        match self {
            Wrap::T32(pe32) => pe32.derva(rva),
            Wrap::T64(pe64) => pe64.derva(rva),
        }
    }
    #[inline]
    pub fn derva_copy<T>(&self, rva: u32) -> Result<T>
    where
        T: Copy + Pod,
    {
        match self {
            Wrap::T32(pe32) => pe32.derva_copy(rva),
            Wrap::T64(pe64) => pe64.derva_copy(rva),
        }
    }
    #[inline]
    pub fn derva_into<T>(&self, rva: u32, dest: &mut T) -> Result<()>
    where
        T: ?Sized + Pod,
    {
        match self {
            Wrap::T32(pe32) => pe32.derva_into(rva, dest),
            Wrap::T64(pe64) => pe64.derva_into(rva, dest),
        }
    }
    #[inline]
    pub fn derva_slice<T>(&self, rva: u32, len: usize) -> Result<&'a [T]>
    where
        T: Pod,
    {
        match self {
            Wrap::T32(pe32) => pe32.derva_slice(rva, len),
            Wrap::T64(pe64) => pe64.derva_slice(rva, len),
        }
    }
    #[inline]
    pub fn derva_slice_f<T, F>(&self, rva: u32, f: F) -> Result<&'a [T]>
    where
        T: Pod,
        F: FnMut(&'a T) -> bool,
    {
        match self {
            Wrap::T32(pe32) => pe32.derva_slice_f(rva, f),
            Wrap::T64(pe64) => pe64.derva_slice_f(rva, f),
        }
    }
    #[inline]
    pub fn derva_slice_s<T>(&self, rva: u32, sentinel: T) -> Result<&'a [T]>
    where
        T: PartialEq + Pod,
    {
        match self {
            Wrap::T32(pe32) => pe32.derva_slice_s(rva, sentinel),
            Wrap::T64(pe64) => pe64.derva_slice_s(rva, sentinel),
        }
    }
    #[inline]
    pub fn derva_c_str(&self, rva: u32) -> Result<&'a util::CStr> {
        match self {
            Wrap::T32(pe32) => pe32.derva_c_str(rva),
            Wrap::T64(pe64) => pe64.derva_c_str(rva),
        }
    }
    #[inline]
    pub fn derva_string<T>(&self, rva: u32) -> Result<&'a T>
    where
        T: util::FromBytes + ?Sized,
    {
        match self {
            Wrap::T32(pe32) => pe32.derva_string(rva),
            Wrap::T64(pe64) => pe64.derva_string(rva),
        }
    }

    //----------------------------------------------------------------

    #[inline]
    pub fn headers(&self) -> Wrap<pe32::headers::Headers<Pe32>, pe64::headers::Headers<Pe64>> {
        match self {
            Wrap::T32(pe32) => Wrap::T32(pe32.headers()),
            Wrap::T64(pe64) => Wrap::T64(pe64.headers()),
        }
    }
    #[inline]
    pub fn rich_structure(&self) -> Result<rich_structure::RichStructure<'a>> {
        match self {
            Wrap::T32(pe32) => pe32.rich_structure(),
            Wrap::T64(pe64) => pe64.rich_structure(),
        }
    }
    #[inline]
    pub fn exports(
        &self,
    ) -> Result<Wrap<pe32::exports::Exports<'a, Pe32>, pe64::exports::Exports<'a, Pe64>>> {
        match self {
            Wrap::T32(pe32) => pe32.exports().map(Wrap::T32),
            Wrap::T64(pe64) => pe64.exports().map(Wrap::T64),
        }
    }
    #[inline]
    pub fn imports(
        &self,
    ) -> Result<Wrap<pe32::imports::Imports<'a, Pe32>, pe64::imports::Imports<'a, Pe64>>> {
        match self {
            Wrap::T32(pe32) => pe32.imports().map(Wrap::T32),
            Wrap::T64(pe64) => pe64.imports().map(Wrap::T64),
        }
    }
    #[inline]
    pub fn iat(&self) -> Result<Wrap<pe32::imports::IAT<'a, Pe32>, pe64::imports::IAT<'a, Pe64>>> {
        match self {
            Wrap::T32(pe32) => pe32.iat().map(Wrap::T32),
            Wrap::T64(pe64) => pe64.iat().map(Wrap::T64),
        }
    }
    #[inline]
    pub fn base_relocs(&self) -> Result<crate::base_relocs::BaseRelocs<'a>> {
        match self {
            Wrap::T32(pe32) => pe32.base_relocs(),
            Wrap::T64(pe64) => pe64.base_relocs(),
        }
    }
    #[inline]
    pub fn load_config(
        &self,
    ) -> Result<
        Wrap<pe32::load_config::LoadConfig<'a, Pe32>, pe64::load_config::LoadConfig<'a, Pe64>>,
    > {
        match self {
            Wrap::T32(pe32) => pe32.load_config().map(Wrap::T32),
            Wrap::T64(pe64) => pe64.load_config().map(Wrap::T64),
        }
    }
    #[inline]
    pub fn tls(&self) -> Result<Wrap<pe32::tls::Tls<'a, Pe32>, pe64::tls::Tls<'a, Pe64>>> {
        match self {
            Wrap::T32(pe32) => pe32.tls().map(Wrap::T32),
            Wrap::T64(pe64) => pe64.tls().map(Wrap::T64),
        }
    }
    #[inline]
    pub fn security(&self) -> Result<crate::security::Security<'a>> {
        match self {
            Wrap::T32(pe32) => pe32.security(),
            Wrap::T64(pe64) => pe64.security(),
        }
    }
    #[inline]
    pub fn exception(
        &self,
    ) -> Result<Wrap<pe32::exception::Exception<'a, Pe32>, pe64::exception::Exception<'a, Pe64>>>
    {
        match self {
            Wrap::T32(pe32) => pe32.exception().map(Wrap::T32),
            Wrap::T64(pe64) => pe64.exception().map(Wrap::T64),
        }
    }
    #[inline]
    pub fn debug(
        &self,
    ) -> Result<Wrap<pe32::debug::Debug<'a, Pe32>, pe64::debug::Debug<'a, Pe64>>> {
        match self {
            Wrap::T32(pe32) => pe32.debug().map(Wrap::T32),
            Wrap::T64(pe64) => pe64.debug().map(Wrap::T64),
        }
    }
    #[inline]
    #[cfg(any(feature = "std", feature = "resources_nostd"))]
    pub fn resources(&self) -> Result<crate::resources::Resources<'a>> {
        match self {
            Wrap::T32(pe32) => pe32.resources(),
            Wrap::T64(pe64) => pe64.resources(),
        }
    }
    #[inline]
    pub fn scanner(&self) -> Wrap<pe32::scanner::Scanner<Pe32>, pe64::scanner::Scanner<Pe64>> {
        match self {
            Wrap::T32(pe32) => Wrap::T32(pe32.scanner()),
            Wrap::T64(pe64) => Wrap::T64(pe64.scanner()),
        }
    }
}
