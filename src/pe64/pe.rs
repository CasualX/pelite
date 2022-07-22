/*!
Abstract over mapped images and file binaries.
*/

use dataview::PodMethods;
use std::{cmp, mem, ptr, slice};

use crate::{util::AlignTo, util::CStr, util::FromBytes, Pod};
use crate::{Error, Result};

use super::{image::*, Ptr};

//----------------------------------------------------------------

pub use crate::wrap::Align;

pub unsafe trait PeObject<'a> {
    /// Returns the image as a byte slice.
    fn image(&self) -> &'a [u8];

    /// Returns whether this image uses file alignment or section alignment.
    fn align(&self) -> Align;

    // Give a struct name in Serialize implementation
    #[cfg(feature = "serde")]
    #[doc(hidden)]
    fn serde_name(&self) -> &'static str;
}

pub unsafe trait Pe<'a>: PeObject<'a> + Copy {
    /// Returns the DOS header.
    fn dos_header(self) -> &'a IMAGE_DOS_HEADER {
        unsafe { dos_header(self.image()) }
    }
    /// Returns the DOS image.
    ///
    /// This includes the dos header and everything up to the start of the PE headers but is not guaranteed to actually contain anything valid.
    fn dos_image(self) -> &'a [u8] {
        unsafe { dos_image(self.image()) }
    }
    /// Returns the NT headers.
    fn nt_headers(self) -> &'a IMAGE_NT_HEADERS {
        unsafe { nt_headers(self.image()) }
    }
    /// Returns the file header.
    fn file_header(self) -> &'a IMAGE_FILE_HEADER {
        unsafe { file_header(self.image()) }
    }
    /// Returns the optional header.
    fn optional_header(self) -> &'a IMAGE_OPTIONAL_HEADER {
        unsafe { optional_header(self.image()) }
    }
    /// Returns the data directory.
    fn data_directory(self) -> &'a [IMAGE_DATA_DIRECTORY] {
        unsafe { data_directory(self.image()) }
    }
    /// Returns the section headers.
    fn section_headers(self) -> &'a super::headers::SectionHeaders {
        unsafe { section_headers(self.image()) }
    }

    /// Returns the pe headers together in a single struct.
    fn headers(self) -> super::headers::Headers<Self> {
        super::headers::Headers::new(self)
    }

    //----------------------------------------------------------------

    /// Converts a relative virtual address to file offset.
    ///
    /// # Errors
    ///
    /// * [`Overflow`](../enum.Error.html#variant.Overflow):
    ///   The rva is contained within a corrupt section where the range bounds overflow.
    ///
    /// * [`ZeroFill`](../enum.Error.html#variant.ZeroFill):
    ///   The rva points to part of a section zero filled and is not available on disk.
    ///
    /// * [`Bounds`](../enum.Error.html#variant.Bounds):
    ///   The rva falls outside any valid section or the PE headers.
    fn rva_to_file_offset(self, rva: Rva) -> Result<usize> {
        // Consider rva inside headers to be valid
        if rva < self.optional_header().SizeOfHeaders {
            return Ok(rva as usize);
        }
        // This code has been carefully designed to avoid panicking on overflow
        for it in self.section_headers() {
            // Compare if rva is contained within the virtual address space of a section
            // If the calculating the section end address overflows the corrupt section will be skipped
            #[allow(non_snake_case)]
            let VirtualEnd = it
                .VirtualAddress
                .wrapping_add(cmp::max(it.VirtualSize, it.SizeOfRawData));
            if it.VirtualAddress <= rva && rva < VirtualEnd {
                // $1
                // Check if the raw data reference is sane
                if let None = it.PointerToRawData.checked_add(it.SizeOfRawData) {
                    // $2
                    return Err(Error::Overflow);
                }
                // Calculate the offset in the section. cannot underflow, see $1
                let section_offset = rva - it.VirtualAddress;
                return if section_offset < it.SizeOfRawData {
                    // $3
                    // Calculate the final offset in the file. cannot overflow, see $2 and $3
                    Ok((section_offset + it.PointerToRawData) as usize)
                }
                // Identify the reason the conversion fails
                else if section_offset < it.VirtualSize {
                    Err(Error::ZeroFill)
                } else {
                    Err(Error::Bounds)
                };
            }
        }
        Err(Error::Bounds)
    }
    /// Converts a file offset to relative virtual address.
    ///
    /// # Errors
    ///
    /// * [`Overflow`](../enum.Error.html#variant.Overflow):
    ///   The file offset is contained within a corrupt section where the range bounds overflow.
    ///
    /// * [`Unmapped`](../enum.Error.html#variant.Unmapped):
    ///   The file offset points to part of a section not mapped and is not available in virtual memory.
    ///
    /// * [`Bounds`](../enum.Error.html#variant.Bounds):
    ///   The file offset falls outside any valid section or PE headers.
    fn file_offset_to_rva(self, file_offset: usize) -> Result<Rva> {
        // Consider rva inside headers to be valid
        if file_offset < self.optional_header().SizeOfHeaders as usize {
            return Ok(file_offset as Rva);
        }
        // This code has been carefully designed to avoid panicking on overflow
        for it in self.section_headers() {
            // Compare if file_offset is contained within the raw data of a section
            // If the calculating the section end address overflows the corrupt section will be skipped
            #[allow(non_snake_case)]
            let EndOfRawData = it.PointerToRawData.wrapping_add(it.SizeOfRawData);
            if it.PointerToRawData as usize <= file_offset && file_offset < EndOfRawData as usize {
                // $1
                // Check if the virtual reference is sane
                if let None = it.VirtualAddress.checked_add(it.VirtualSize) {
                    // $2
                    return Err(Error::Overflow);
                }
                // Calculate the offset in the section. cannot underflow, see $1
                let section_offset = file_offset as Rva - it.PointerToRawData;
                return if section_offset < it.VirtualSize {
                    // $3
                    // Calculate the final virtual address. cannot overflow, see $2 and $3
                    Ok(section_offset + it.VirtualAddress)
                }
                // Identify the reason the conversion fails
                else if section_offset < it.SizeOfRawData {
                    Err(Error::Unmapped)
                } else {
                    Err(Error::Bounds)
                };
            }
        }
        Err(Error::Bounds)
    }

    /// Converts from relative virtual address to virtual address.
    ///
    /// # Errors
    ///
    /// * [`Null`](../enum.Error.html#variant.Null):
    ///   The rva is zero.
    ///
    /// * [`Bounds`](../enum.Error.html#variant.Bounds):
    ///   The rva does not fall within the virtual image bounds.
    fn rva_to_va(self, rva: Rva) -> Result<Va> {
        if rva == 0 {
            Err(Error::Null)
        } else {
            let (image_base, size_of_image) = {
                let optional_header = self.optional_header();
                (optional_header.ImageBase, optional_header.SizeOfImage)
            };
            if rva < size_of_image {
                Ok(image_base + rva as Va)
            } else {
                Err(Error::Bounds)
            }
        }
    }
    /// Converts from virtual address to relative virtual address.
    ///
    /// # Errors
    ///
    /// * [`Null`](../enum.Error.html#variant.Null):
    ///   The va is zero.
    ///
    /// * [`Bounds`](../enum.Error.html#variant.Bounds):
    ///   The va does not fall within the virtual image bounds.
    fn va_to_rva(self, va: Va) -> Result<Rva> {
        if va == 0 {
            Err(Error::Null)
        } else {
            let (image_base, size_of_image) = {
                let optional_header = self.optional_header();
                (optional_header.ImageBase, optional_header.SizeOfImage)
            };
            // Carefully avoid panicking overflow
            if va < image_base || va - image_base > size_of_image as Va {
                Err(Error::Bounds)
            } else {
                Ok((va - image_base) as Rva)
            }
        }
    }

    //----------------------------------------------------------------

    /// Slices the image at the specified rva.
    ///
    /// If successful the returned slice's length will be at least the given size but often be quite larger.
    /// This allows to access the image without knowing beforehand how large the structure being accessed will be.
    ///
    /// The length is the largest consecutive number of bytes available until the end.
    /// In case the of PE files on disk, this is limited to the section's size of raw data.
    ///
    /// # Errors
    ///
    /// * [`Null`](../enum.Error.html#variant.Null):
    ///   The rva is zero.
    fn slice(&self, rva: Rva, min_size_of: usize, align: usize) -> Result<&'a [u8]> {
        unsafe {
            match (self.align(), self.image()) {
                (Align::File, image) => slice_file(image, rva, min_size_of, align),
                (Align::Section, image) => slice_section(image, rva, min_size_of, align),
            }
        }
    }

    /// Slices the image at the specified rva returning a byte slice with no alignment or minimum size.
    ///
    /// Shorthand to invoke [`slice(rva, 0, 1)`](#tymethod.slice).
    fn slice_bytes(self, rva: Rva) -> Result<&'a [u8]>
    where
        Self: Sized,
    {
        self.slice(rva, 0, 1)
    }

    /// Gets the bytes defined by a section header in this image.
    ///
    /// # Errors
    ///
    /// * [`Null`](../enum.Error.html#variant.Null):
    ///   The virtual address or pointer to raw data is zero.
    ///
    /// * [`Bounds`](../enum.Error.html#variant.Bounds):
    ///   The data referenced by the section header is out of bounds.
    fn get_section_bytes(self, section_header: &IMAGE_SECTION_HEADER) -> Result<&'a [u8]> {
        crate::wrap::get_section_bytes(self.image(), section_header, self.align())
    }

    /// Reads the image at the specified va.
    ///
    /// If successful the returned slice's length will be at least the given size but often be quite larger.
    /// This allows to access the image without knowing beforehand how large the structure being accessed will be.
    ///
    /// The length is the largest consecutive number of bytes available until the end.
    /// In case the of PE files on disk, this is limited to the section's size of raw data.
    ///
    /// # Errors
    ///
    /// * [`Null`](../enum.Error.html#variant.Null):
    ///   The va is zero.
    fn read(&self, va: Va, min_size_of: usize, align: usize) -> Result<&'a [u8]> {
        unsafe {
            match (self.align(), self.image()) {
                (Align::File, image) => read_file(image, va, min_size_of, align),
                (Align::Section, image) => read_section(image, va, min_size_of, align),
            }
        }
    }

    /// Reads the image at the specified va returning a byte slice with no alignment or minimum size.
    ///
    /// Shorthand to invoke [`read(va, 0, 1)`](#tymethod.read).
    fn read_bytes(self, va: Va) -> Result<&'a [u8]>
    where
        Self: Sized,
    {
        self.read(va, 0, 1)
    }

    //----------------------------------------------------------------

    /// Reads an aligned pod `T`.
    fn derva<T>(self, rva: Rva) -> Result<&'a T>
    where
        T: Pod,
    {
        let align = if cfg!(feature = "unsafe_alignment") {
            1
        } else {
            mem::align_of::<T>()
        };
        let bytes = self.slice(rva, mem::size_of::<T>(), align)?;
        // This is safe as per Pod bound, min_size_of and align
        unsafe {
            let p = &*(bytes.as_ptr() as *const T);
            Ok(p)
        }
    }
    /// Reads an unaligned pod `T`.
    fn derva_copy<T>(self, rva: Rva) -> Result<T>
    where
        T: Copy + Pod,
    {
        let bytes = self.slice(rva, mem::size_of::<T>(), 1)?;
        // This is safe as per Pod bound and min_size_of
        unsafe {
            let p = bytes.as_ptr() as *const T;
            Ok(ptr::read_unaligned(p))
        }
    }
    /// Reads and byte-wise copies the content to the given destination.
    ///
    /// Allows reading of an unaligned array of data.
    fn derva_into<T>(self, rva: Rva, dest: &mut T) -> Result<()>
    where
        T: ?Sized + Pod,
    {
        let len = mem::size_of_val(dest);
        let bytes = self.slice(rva, len, 1)?;
        dest.as_bytes_mut().copy_from_slice(&bytes[..len]);
        Ok(())
    }
    /// Reads an array of pod `T` with given length.
    fn derva_slice<T>(self, rva: Rva, len: usize) -> Result<&'a [T]>
    where
        T: Pod,
    {
        let min_size_of = mem::size_of::<T>()
            .checked_mul(len)
            .ok_or(Error::Overflow)?;
        let align = if cfg!(feature = "unsafe_alignment") {
            1
        } else {
            mem::align_of::<T>()
        };
        let bytes = self.slice(rva, min_size_of, align)?;
        // This is safe as per Pod bound, min_size_of and align
        unsafe { Ok(slice::from_raw_parts(bytes.as_ptr() as *const T, len)) }
    }
    /// Reads an array of pod `T`.
    ///
    /// For every element of the array, starting at the given `rva`, the callable `f` is called with that element.
    /// The length of the array is the index when the callable `f` returns `true`.
    ///
    /// The returned slice contains all `T` up to but not including the element for which the callable returned `true`.
    fn derva_slice_f<T, F>(self, rva: Rva, mut f: F) -> Result<&'a [T]>
    where
        T: Pod,
        F: FnMut(&'a T) -> bool,
    {
        let align = if cfg!(feature = "unsafe_alignment") {
            1
        } else {
            mem::align_of::<T>()
        };
        let bytes = self.slice(rva, 0, align)?;
        let mut len = 0;
        loop {
            // Safety critical OOB check
            // Overflows only if bytes.len() > USIZE_MAX - sizeof(T) which would be ridiculous
            let offset = len * mem::size_of::<T>();
            if offset + mem::size_of::<T>() > bytes.len() {
                return Err(Error::Bounds);
            }
            // Safe because len is checked above and T is Pod
            unsafe {
                let s = bytes.as_ptr().offset(offset as isize) as *const T;
                if f(&*s) {
                    let p = slice::from_raw_parts(bytes.as_ptr() as *const T, len);
                    return Ok(p);
                }
                len += 1;
            }
        }
    }
    /// Reads an array of pod `T`.
    ///
    /// The length of the array is determined by a [sentinel value](https://en.wikipedia.org/wiki/Sentinel_value), a special value of `T` which marks the end of the array.
    ///
    /// The returned slice contains all `T` up to but not including the sentinel value.
    fn derva_slice_s<T>(self, rva: Rva, sentinel: T) -> Result<&'a [T]>
    where
        T: PartialEq + Pod,
    {
        self.derva_slice_f(rva, |tee| *tee == sentinel)
    }
    /// Reads a nul-terminated C string.
    fn derva_c_str(self, rva: Rva) -> Result<&'a CStr> {
        self.derva_string(rva)
    }
    /// Reads a string.
    fn derva_string<T>(self, rva: Rva) -> Result<&'a T>
    where
        T: FromBytes + ?Sized,
    {
        let bytes = self.slice(rva, T::MIN_SIZE_OF, T::ALIGN_OF)?;
        unsafe { T::from_bytes(bytes).ok_or(Error::Encoding) }
    }

    //----------------------------------------------------------------
    // Deref impls for `Ptr`s

    /// Dereferences the pointer to a pod `T`.
    fn deref<T>(self, ptr: Ptr<T>) -> Result<&'a T>
    where
        T: Pod,
    {
        let align = if cfg!(feature = "unsafe_alignment") {
            1
        } else {
            mem::align_of::<T>()
        };
        let bytes = self.read(ptr.into(), mem::size_of::<T>(), align)?;
        // This is safe as per Pod bound, min_size_of and align
        unsafe {
            let p = &*(bytes.as_ptr() as *const T);
            Ok(p)
        }
    }
    /// Dereferences the pointer to an unaligned pod `T`.
    fn deref_copy<T>(self, ptr: Ptr<T>) -> Result<T>
    where
        T: Copy + Pod,
    {
        let bytes = self.read(ptr.into(), mem::size_of::<T>(), 1)?;
        // This is safe as per Pod bound and min_size_of
        unsafe {
            let p = bytes.as_ptr() as *const T;
            Ok(ptr::read_unaligned(p))
        }
    }
    /// Reads and byte-wise copies the content to the given destination.
    ///
    /// Allows reading of an unaligned array of data.
    fn deref_into<T>(self, ptr: Ptr<T>, dest: &mut T) -> Result<()>
    where
        T: ?Sized + Pod,
    {
        let len = mem::size_of_val(dest);
        let bytes = self.read(ptr.into(), len, 1)?;
        dest.as_bytes_mut().copy_from_slice(&bytes[..len]);
        Ok(())
    }
    /// Reads an array of pod `T` with given length.
    fn deref_slice<T>(self, ptr: Ptr<[T]>, len: usize) -> Result<&'a [T]>
    where
        T: Pod,
    {
        let min_size_of = mem::size_of::<T>()
            .checked_mul(len)
            .ok_or(Error::Overflow)?;
        let align = if cfg!(feature = "unsafe_alignment") {
            1
        } else {
            mem::align_of::<T>()
        };
        let bytes = self.read(ptr.into(), min_size_of, align)?;
        // This is safe as per Pod bound, min_size_of and align
        unsafe { Ok(slice::from_raw_parts(bytes.as_ptr() as *const T, len)) }
    }
    /// Reads an array of pod `T`.
    ///
    /// For every element of the array, starting at the given `ptr`, the callable `f` is called with that element.
    /// The length of the array is the index when the callable `f` returns `true`.
    ///
    /// The returned slice contains all `T` up to but not including the element for which the callable returned `true`.
    fn deref_slice_f<T, F>(self, ptr: Ptr<[T]>, mut f: F) -> Result<&'a [T]>
    where
        T: Pod,
        F: FnMut(&'a T) -> bool,
    {
        let align = if cfg!(feature = "unsafe_alignment") {
            1
        } else {
            mem::align_of::<T>()
        };
        let bytes = self.read(ptr.into(), 0, align)?;
        let mut len = 0;
        loop {
            // Safety critical OOB check
            // Overflows only if bytes.len() > USIZE_MAX - sizeof(T) which would be ridiculous
            let offset = len * mem::size_of::<T>();
            if offset + mem::size_of::<T>() > bytes.len() {
                return Err(Error::Bounds);
            }
            // Safe because len is checked above and T is Pod
            unsafe {
                let s = bytes.as_ptr().offset(offset as isize) as *const T;
                if f(&*s) {
                    let p = slice::from_raw_parts(bytes.as_ptr() as *const T, len);
                    return Ok(p);
                }
                len += 1;
            }
        }
    }
    /// Reads an array of pod `T`.
    ///
    /// The length of the array is determined by a [sentinel value](https://en.wikipedia.org/wiki/Sentinel_value), a special value of `T` which marks the end of the array.
    ///
    /// The returned slice contains all `T` up to but not including the sentinel value.
    fn deref_slice_s<T>(self, ptr: Ptr<[T]>, sentinel: T) -> Result<&'a [T]>
    where
        T: PartialEq + Pod,
    {
        self.deref_slice_f(ptr, |tee| *tee == sentinel)
    }
    /// Dereferences the pointer to a nul-terminated C string.
    fn deref_c_str(self, ptr: Ptr<CStr>) -> Result<&'a CStr> {
        self.deref_string(ptr)
    }
    /// Dereferences the pointer to a string.
    fn deref_string<T>(self, ptr: Ptr<T>) -> Result<&'a T>
    where
        T: FromBytes + ?Sized,
    {
        let bytes = self.read(ptr.into(), T::MIN_SIZE_OF, T::ALIGN_OF)?;
        unsafe { T::from_bytes(bytes).ok_or(Error::Encoding) }
    }

    //----------------------------------------------------------------

    /// Returns the Rich structure.
    fn rich_structure(self) -> Result<crate::rich_structure::RichStructure<'a>> {
        let image = self.image();
        let image = unsafe { slice::from_raw_parts(image.as_ptr() as *const u32, image.len() / 4) };
        crate::rich_structure::RichStructure::try_from(image)
    }

    /// Gets the Export Directory.
    ///
    /// See the [exports](exports/index.html) module for more information.
    ///
    /// Returns [`Err(Null)`](../enum.Error.html#variant.Null) if the image has no exports. Any other error indiciates some form of corruption.
    fn exports(self) -> Result<super::exports::Exports<'a, Self>> {
        super::exports::Exports::try_from(self)
    }

    /// Gets the Import Directory.
    ///
    /// See the [imports](imports/index.html) module for more information.
    ///
    /// Returns [`Err(Null)`](../enum.Error.html#variant.Null) if the image has no imports. Any other error indicates some form of corruption.
    fn imports(self) -> Result<super::imports::Imports<'a, Self>> {
        super::imports::Imports::try_from(self)
    }

    /// Gets the Import Address Table.
    ///
    /// See the [imports](imports/index.html) module for more information.
    ///
    /// Returns [`Err(Null)`](../enum.Error.html#variant.Null) if the image has no iat. Any other error indicates some form of corruption.
    fn iat(self) -> Result<super::imports::IAT<'a, Self>> {
        super::imports::IAT::try_from(self)
    }

    /// Gets the Base Relocations Directory.
    ///
    /// See the [base relocations](base_relocs/index.html) module for more information.
    ///
    /// Returns [`Err(Null)`](../enum.Error.html#variant.Null) if the image has no base relocations. Any other error indicates some form of corruption.
    fn base_relocs(self) -> Result<crate::base_relocs::BaseRelocs<'a>> {
        super::base_relocs::try_from(self)
    }

    /// Gets the Load Config Directory.
    ///
    /// See the [load config](load_config/index.html) module for more information.
    ///
    /// Returns [`Err(Null)`](../enum.Error.html#variant.Null) if the image has no load config. Any other error indicates some form of corruption.
    fn load_config(self) -> Result<super::load_config::LoadConfig<'a, Self>> {
        super::load_config::LoadConfig::try_from(self)
    }

    /// Gets the TLS Directory.
    ///
    /// See the [tls](tls/index.html) module for more information.
    ///
    /// Returns [`Err(Null)`](../enum.Error.html#variant.Null) if the image has no tls. Any other error indicates some form of corruption.
    fn tls(self) -> Result<super::tls::Tls<'a, Self>> {
        super::tls::Tls::try_from(self)
    }

    /// Gets the Security Directory.
    ///
    /// See the [security](security/index.html) module for more information.
    ///
    /// Returns [`Err(Null)`](../enum.Error.html#variant.Null) if the image has no security info. Any other error indicates some form of corruption.
    fn security(self) -> Result<crate::security::Security<'a>> {
        super::security::try_from(self)
    }

    /// Gets the Exception Directory.
    ///
    /// See the [exception](exception/index.html) module for more information.
    ///
    /// Returns [`Err(Null)`](../enum.Error.html#variant.Null) if the image has no exception directory. Any other error indicates some form of corruption.
    fn exception(self) -> Result<super::exception::Exception<'a, Self>> {
        super::exception::Exception::try_from(self)
    }

    /// Gets the Debug Directory.
    ///
    /// See the [debug](debug/index.html) module for more information.
    ///
    /// Returns [`Err(Null)`](../enum.Error.html#variant.Null) if the image has no debug info. Any other error indicates some form of corruption.
    fn debug(self) -> Result<super::debug::Debug<'a, Self>> {
        super::debug::Debug::try_from(self)
    }

    /// Gets the Resources.
    ///
    /// See the [resources](resources/index.html) module for more information.
    ///
    /// Returns [`Err(Null)`](../enum.Error.html#variant.Null) if the image has no resources. Any other error indicates some form of corruption.
    #[cfg(any(feature = "std", feature = "resources_nostd"))]
    fn resources(self) -> Result<crate::resources::Resources<'a>>
    where
        Self: Copy,
    {
        let datadir = self
            .data_directory()
            .get(IMAGE_DIRECTORY_ENTRY_RESOURCE)
            .ok_or(Error::Bounds)?;
        let bytes = self.slice_bytes(datadir.VirtualAddress)?;
        let size = cmp::min(datadir.Size as usize, bytes.len());
        Ok(crate::resources::Resources::new(&bytes[..size], datadir))
    }

    /// Gets Scanner access.
    ///
    /// See the [scanner](scanner/index.html) module for more information.
    fn scanner(self) -> super::scanner::Scanner<Self> {
        super::scanner::Scanner::new(self)
    }
}

//----------------------------------------------------------------
// Make `&PeObject<'a>` trait objects work seamlessly.

unsafe impl<'s, 'a> PeObject<'a> for &'s dyn PeObject<'a> {
    fn image(&self) -> &'a [u8] {
        PeObject::image(*self)
    }
    fn align(&self) -> Align {
        PeObject::align(*self)
    }
    #[cfg(feature = "serde")]
    fn serde_name(&self) -> &'static str {
        PeObject::serde_name(*self)
    }
}

unsafe impl<'s, 'a> Pe<'a> for &'s dyn PeObject<'a> {}

//----------------------------------------------------------------

#[cfg(feature = "serde")]
pub(crate) fn serialize_pe<'a, P: Pe<'a>, S: serde::Serializer>(
    pe: P,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error> {
    use crate::util::serde_helper::*;

    let mut state = serializer.serialize_struct(pe.serde_name(), 10)?;
    state.serialize_field("headers", &pe.headers())?;
    state.serialize_field("rich_structure", &pe.rich_structure().ok())?;
    state.serialize_field("exports", &pe.exports().ok())?;
    state.serialize_field("imports", &pe.imports().ok())?;
    state.serialize_field("base_relocs", &pe.base_relocs().ok())?;
    state.serialize_field("debug", &pe.debug().ok())?;
    state.serialize_field("tls", &pe.tls().ok())?;
    state.serialize_field("load_config", &pe.load_config().ok())?;
    state.serialize_field("security", &pe.security().ok())?;
    state.serialize_field("resources", &pe.resources().ok())?;
    state.end()
}

//----------------------------------------------------------------
// Implementation helpers

unsafe fn dos_header(image: &[u8]) -> &IMAGE_DOS_HEADER {
    &*(image.as_ptr() as *const IMAGE_DOS_HEADER)
}
unsafe fn dos_image(image: &[u8]) -> &[u8] {
    image.get_unchecked(..dos_header(image).e_lfanew as usize)
}
unsafe fn nt_headers(image: &[u8]) -> &IMAGE_NT_HEADERS {
    &*(image.as_ptr().offset(dos_header(image).e_lfanew as isize) as *const IMAGE_NT_HEADERS)
}
unsafe fn file_header(image: &[u8]) -> &IMAGE_FILE_HEADER {
    &nt_headers(image).FileHeader
}
unsafe fn optional_header(image: &[u8]) -> &IMAGE_OPTIONAL_HEADER {
    &nt_headers(image).OptionalHeader
}
unsafe fn data_directory(image: &[u8]) -> &[IMAGE_DATA_DIRECTORY] {
    let opt = optional_header(image);
    let len = cmp::min(
        opt.NumberOfRvaAndSizes as usize,
        IMAGE_NUMBEROF_DIRECTORY_ENTRIES,
    );
    slice::from_raw_parts(opt.DataDirectory.as_ptr(), len)
}
unsafe fn section_headers(image: &[u8]) -> &super::headers::SectionHeaders {
    let nt = nt_headers(image);
    let data = (&nt.OptionalHeader as *const _ as *const u8)
        .offset(nt.FileHeader.SizeOfOptionalHeader as isize)
        as *const IMAGE_SECTION_HEADER;
    let raw = slice::from_raw_parts(data, nt.FileHeader.NumberOfSections as usize);
    super::headers::SectionHeaders::new(raw)
}

unsafe fn slice_section(
    image: &[u8],
    rva: Rva,
    min_size_of: usize,
    align_of: usize,
) -> Result<&[u8]> {
    let start = rva as usize;
    if rva == 0 {
        Err(Error::Null)
    } else if !usize::wrapping_add(image.as_ptr() as usize, start).aligned_to(align_of) {
        Err(Error::Misaligned)
    } else {
        match image.get(start..) {
            Some(bytes) if bytes.len() >= min_size_of => Ok(bytes),
            _ => Err(Error::Bounds),
        }
    }
}
unsafe fn read_section(image: &[u8], va: Va, min_size_of: usize, align_of: usize) -> Result<&[u8]> {
    let (image_base, image_size) = {
        let optional_header = optional_header(image);
        (optional_header.ImageBase, optional_header.SizeOfImage)
    };
    if va == 0 {
        Err(Error::Null)
    } else if va < image_base || va - image_base > image_size as Va {
        Err(Error::Bounds)
    } else {
        let start = (va - image_base) as usize;
        if !usize::wrapping_add(image.as_ptr() as usize, start).aligned_to(align_of) {
            Err(Error::Misaligned)
        } else {
            match image.get(start..) {
                Some(bytes) if bytes.len() >= min_size_of => Ok(bytes),
                _ => Err(Error::Bounds),
            }
        }
    }
}

unsafe fn range_file(image: &[u8], rva: Rva, min_size_of: usize) -> Result<&[u8]> {
    // This code has been carefully designed to avoid panicking on overflow
    for it in section_headers(image) {
        // Compare if rva is contained within the virtual address space of a section
        // If the calculating the section end address overflows the corrupt section will be skipped
        #[allow(non_snake_case)]
        let VirtualEnd = it
            .VirtualAddress
            .wrapping_add(cmp::max(it.VirtualSize, it.SizeOfRawData));
        if it.VirtualAddress <= rva && rva < VirtualEnd {
            // $1
            // Isolate and range check the pointer and size of raw data
            // If this fails immediately abort and return an error
            let section_range = it.PointerToRawData as usize
                ..it.PointerToRawData.wrapping_add(it.SizeOfRawData) as usize;
            let section_bytes = image.get(section_range).ok_or(Error::Invalid)?;
            // Calculate the offset in the section requested. cannot underflow, see $1
            let section_offset = (rva - it.VirtualAddress) as usize;
            return match section_bytes.get(section_offset..) {
                Some(bytes) if bytes.len() >= min_size_of => Ok(bytes),
                // Identify the reason the slice fails. cannot underflow, see $1
                _ => Err(if min_size_of > (VirtualEnd - rva) as usize {
                    Error::Bounds
                } else {
                    Error::ZeroFill
                }),
            };
        }
    }
    Err(Error::Bounds)
}
#[inline(never)]
unsafe fn slice_file(image: &[u8], rva: Rva, min_size_of: usize, align_of: usize) -> Result<&[u8]> {
    if rva == 0 {
        Err(Error::Null)
    } else if !usize::wrapping_add(image.as_ptr() as usize, rva as usize).aligned_to(align_of) {
        Err(Error::Misaligned)
    } else {
        range_file(image, rva, min_size_of)
    }
}
#[inline(never)]
unsafe fn read_file(image: &[u8], va: Va, min_size_of: usize, align_of: usize) -> Result<&[u8]> {
    let (image_base, size_of_image) = {
        let optional_header = optional_header(image);
        (optional_header.ImageBase, optional_header.SizeOfImage)
    };
    if va == 0 {
        Err(Error::Null)
    } else if va < image_base || va - image_base > size_of_image as Va {
        Err(Error::Bounds)
    } else {
        let rva = (va - image_base) as Rva;
        if !usize::wrapping_add(image.as_ptr() as usize, rva as usize).aligned_to(align_of) {
            Err(Error::Misaligned)
        } else {
            range_file(image, rva, min_size_of)
        }
    }
}

//----------------------------------------------------------------

// TODO: This code needs to be audited...
// The safety of `Pe` relies on it.
pub(crate) fn validate_headers(image: &[u8]) -> Result<u32> {
    // Grab the DOS header
    if mem::size_of::<IMAGE_DOS_HEADER>() > image.len() {
        return Err(Error::Bounds);
    }
    // Check basic alignment of the image bytes
    if !image.as_ptr().aligned_to(4) {
        return Err(Error::Misaligned);
    }
    let dos = unsafe { &*(image.as_ptr() as *const IMAGE_DOS_HEADER) };
    // Verify the DOS header
    if dos.e_magic != IMAGE_DOS_SIGNATURE {
        return Err(Error::BadMagic);
    }
    // "According to the PE specification, the PE header must be aligned on a 8 byte boundary, but the Windows loader requires only a 4 byte alignment."
    if !dos.e_lfanew.aligned_to(4) {
        return Err(Error::Misaligned);
    }
    // Prevent overflow the easy way...
    // When changing, take care of overflow in later offset calculations!
    if dos.e_lfanew > 0x01000000 {
        return Err(Error::Insanity);
    }

    // Grab the NT headers
    let nt_end = dos.e_lfanew as usize + mem::size_of::<IMAGE_NT_HEADERS>();
    if nt_end > image.len() {
        return Err(Error::Bounds);
    }
    let nt = unsafe { &*(image.as_ptr().offset(dos.e_lfanew as isize) as *const IMAGE_NT_HEADERS) };
    // Verify the NT headers
    if nt.Signature != IMAGE_NT_HEADERS_SIGNATURE
        || !(nt.OptionalHeader.Magic == IMAGE_NT_OPTIONAL_HDR32_MAGIC
            || nt.OptionalHeader.Magic == IMAGE_NT_OPTIONAL_HDR64_MAGIC)
    {
        return Err(Error::BadMagic);
    }
    if nt.OptionalHeader.SizeOfHeaders as usize > image.len() {
        return Err(Error::Bounds);
    }
    if nt.OptionalHeader.SizeOfHeaders > nt.OptionalHeader.SizeOfImage {
        return Err(Error::Insanity);
    }
    // Give the caller a chance to retry with the correct parser
    if nt.OptionalHeader.Magic != IMAGE_NT_OPTIONAL_HDR_MAGIC {
        return Err(Error::PeMagic);
    }

    // Verify the data directory
    let num_rva_sizes = cmp::min(
        nt.OptionalHeader.NumberOfRvaAndSizes as usize,
        IMAGE_NUMBEROF_DIRECTORY_ENTRIES,
    );
    let size_of_data_dir = num_rva_sizes * mem::size_of::<IMAGE_DATA_DIRECTORY>();
    if nt_end + size_of_data_dir > image.len() {
        return Err(Error::Bounds);
    }

    // Verify the section headers
    if nt.FileHeader.NumberOfSections > 96 {
        return Err(Error::Insanity);
    }
    // u16 * sizeof(T) casted to usize, cannot reasonably overflow
    let size_of_sections =
        nt.FileHeader.NumberOfSections as usize * mem::size_of::<IMAGE_SECTION_HEADER>();
    // e_lfanew is checked for reasonable values, the others then cannot reasonably cause overflow
    let start_of_sections = dos.e_lfanew as usize
        + (mem::size_of::<IMAGE_NT_HEADERS>() - mem::size_of::<IMAGE_OPTIONAL_HEADER>())
        + nt.FileHeader.SizeOfOptionalHeader as usize;
    // then the sum of these cannot reasonably overflow
    if size_of_sections + start_of_sections > image.len() {
        return Err(Error::Bounds);
    }
    Ok(nt.OptionalHeader.SizeOfImage)
}

/// Returns the PE headers as mutable borrows.
///
/// # Safety
///
/// No checks of any kind are performed, before calling this function ensure the byte slice points to a valid PE image by running it through the `PeFile::from_bytes` constructor.
#[cfg(feature = "unstable")]
pub unsafe fn headers_mut(
    image: &mut [u8],
) -> (
    &mut IMAGE_DOS_HEADER,
    &mut IMAGE_NT_HEADERS,
    &mut [IMAGE_DATA_DIRECTORY],
    &mut [IMAGE_SECTION_HEADER],
) {
    let dos = &mut *(image.as_mut_ptr() as *mut IMAGE_DOS_HEADER);
    let nt = &mut *(image.as_mut_ptr().offset(dos.e_lfanew as isize) as *mut IMAGE_NT_HEADERS);
    let dd_ptr = nt.OptionalHeader.DataDirectory.as_mut_ptr();
    let dd_len = cmp::min(
        nt.OptionalHeader.NumberOfRvaAndSizes as usize,
        IMAGE_NUMBEROF_DIRECTORY_ENTRIES,
    );
    let dd = slice::from_raw_parts_mut(dd_ptr, dd_len);
    let sections_ptr = (&mut nt.OptionalHeader as *mut _ as *mut u8)
        .offset(nt.FileHeader.SizeOfOptionalHeader as isize)
        as *mut IMAGE_SECTION_HEADER;
    let sections_len = nt.FileHeader.NumberOfSections as usize;
    let sections = slice::from_raw_parts_mut(sections_ptr, sections_len);
    (dos, nt, dd, sections)
}
