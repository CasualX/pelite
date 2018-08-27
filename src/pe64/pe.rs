/*!
Abstract over mapped images and file binaries.
*/

use std::{cmp, mem, ptr, slice};

use error::{Error, Result};
use util::{CStr, Pod, FromBytes};

use super::image::*;
use super::Ptr;

//----------------------------------------------------------------

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

pub unsafe trait Pe<'a> {
	/// Returns the image as a byte slice.
	fn image(&self) -> &'a [u8];

	/// Returns whether this image uses file alignment or section alignment.
	fn align(&self) -> Align;

	/// Returns the DOS header.
	fn dos_header(self) -> &'a IMAGE_DOS_HEADER where Self: Copy {
		unsafe {
			&*(self.image().as_ptr() as *const IMAGE_DOS_HEADER)
		}
	}
	/// Returns the DOS image.
	///
	/// This includes the dos header and everything up to the start of the PE headers but is not guaranteed to actually contain anything valid.
	fn dos_image(self) -> &'a [u8] where Self: Copy {
		let dos = self.dos_header();
		unsafe {
			self.image().get_unchecked(..dos.e_lfanew as usize)
		}
	}
	/// Returns the NT headers.
	fn nt_headers(self) -> &'a IMAGE_NT_HEADERS where Self: Copy {
		let dos = self.dos_header();
		unsafe {
			&*((dos as *const _ as *const u8).offset(dos.e_lfanew as isize) as *const IMAGE_NT_HEADERS)
		}
	}
	/// Returns the file header.
	fn file_header(self) -> &'a IMAGE_FILE_HEADER where Self: Copy {
		&self.nt_headers().FileHeader
	}
	/// Returns the optional header.
	fn optional_header(self) -> &'a IMAGE_OPTIONAL_HEADER where Self: Copy {
		&self.nt_headers().OptionalHeader
	}
	/// Returns the data directory.
	fn data_directory(self) -> &'a [IMAGE_DATA_DIRECTORY] where Self: Copy {
		let opt = self.optional_header();
		let len = cmp::min(opt.NumberOfRvaAndSizes as usize, IMAGE_NUMBEROF_DIRECTORY_ENTRIES);
		unsafe {
			slice::from_raw_parts(opt.DataDirectory.as_ptr(), len)
		}
	}
	/// Returns the section headers.
	fn section_headers(self) -> &'a [IMAGE_SECTION_HEADER] where Self: Copy {
		let nt = self.nt_headers();
		unsafe {
			let begin = (&nt.OptionalHeader as *const _ as *const u8).offset(nt.FileHeader.SizeOfOptionalHeader as isize) as *const IMAGE_SECTION_HEADER;
			slice::from_raw_parts(begin, nt.FileHeader.NumberOfSections as usize)
		}
	}
	/// Returns the pe headers together in a single struct.
	fn headers(self) -> super::headers::Headers<Self> where Self: Copy {
		super::headers::Headers::new(self)
	}

	// Give a struct name in Serialize implementation
	#[cfg(feature = "serde")]
	#[doc(hidden)]
	const SERDE_NAME: &'static str;

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
	fn rva_to_file_offset(self, rva: Rva) -> Result<usize> where Self: Copy {
		// Consider rva inside headers to be valid
		if rva < self.optional_header().SizeOfHeaders {
			return Ok(rva as usize);
		}
		// This code has been carefully designed to avoid panicking on overflow
		for it in self.section_headers() {
			// Compare if rva is contained within the virtual address space of a section
			// If the calculating the section end address overflows the corrupt section will be skipped
			#[allow(non_snake_case)]
			let VirtualEnd = it.VirtualAddress.wrapping_add(cmp::max(it.VirtualSize, it.SizeOfRawData));
			if it.VirtualAddress <= rva && rva < VirtualEnd { // $1
				// Check if the raw data reference is sane
				if let None = it.PointerToRawData.checked_add(it.SizeOfRawData) { // $2
					return Err(Error::Overflow);
				}
				// Calculate the offset in the section. cannot underflow, see $1
				let section_offset = rva - it.VirtualAddress;
				return if section_offset < it.SizeOfRawData { // $3
					// Calculate the final offset in the file. cannot overflow, see $2 and $3
					Ok((section_offset + it.PointerToRawData) as usize)
				}
				// Identify the reason the conversion fails
				else if section_offset < it.VirtualSize {
					Err(Error::ZeroFill)
				}
				else {
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
	fn file_offset_to_rva(self, file_offset: usize) -> Result<Rva> where Self: Copy {
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
			if it.PointerToRawData as usize <= file_offset && file_offset < EndOfRawData as usize { // $1
				// Check if the virtual reference is sane
				if let None = it.VirtualAddress.checked_add(it.VirtualSize) { // $2
					return Err(Error::Overflow);
				}
				// Calculate the offset in the section. cannot underflow, see $1
				let section_offset = file_offset as Rva - it.PointerToRawData;
				return if section_offset < it.VirtualSize { // $3
					// Calculate the final virtual address. cannot overflow, see $2 and $3
					Ok(section_offset + it.VirtualAddress)
				}
				// Identify the reason the conversion fails
				else if section_offset < it.SizeOfRawData {
					Err(Error::Unmapped)
				}
				else {
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
	fn rva_to_va(self, rva: Rva) -> Result<Va> where Self: Copy {
		if rva == 0 {
			Err(Error::Null)
		}
		else {
			let (image_base, size_of_image) = {
				let optional_header = self.optional_header();
				(optional_header.ImageBase, optional_header.SizeOfImage)
			};
			if rva < size_of_image {
				Ok(image_base + rva as Va)
			}
			else {
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
	fn va_to_rva(self, va: Va) -> Result<Rva> where Self: Copy {
		if va == 0 {
			Err(Error::Null)
		}
		else {
			let (image_base, size_of_image) = {
				let optional_header = self.optional_header();
				(optional_header.ImageBase, optional_header.SizeOfImage)
			};
			// Carefully avoid panicking overflow
			if va < image_base || va - image_base > size_of_image as Va {
				Err(Error::Bounds)
			}
			else {
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
	fn slice(&self, rva: Rva, min_size_of: usize, align: usize) -> Result<&'a [u8]>;

	/// Slices the image at the specified rva returning a byte slice with no alignment or minimum size.
	///
	/// Shorthand to invoke [`slice(rva, 0, 1)`](#tymethod.slice).
	fn slice_bytes(self, rva: Rva) -> Result<&'a [u8]> where Self: Sized {
		self.slice(rva, 0, 1)
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
	fn read(&self, va: Va, min_size_of: usize, align: usize) -> Result<&'a [u8]>;

	/// Reads the image at the specified va returning a byte slice with no alignment or minimum size.
	///
	/// Shorthand to invoke [`read(va, 0, 1)`](#tymethod.read).
	fn read_bytes(self, va: Va) -> Result<&'a [u8]> where Self: Sized {
		self.read(va, 0, 1)
	}

	//----------------------------------------------------------------

	/// Reads an aligned pod `T`.
	fn derva<T>(self, rva: Rva) -> Result<&'a T> where Self: Copy, T: Pod {
		let align = if cfg!(feature = "unsafe_alignment") { 1 } else { mem::align_of::<T>() };
		let bytes = self.slice(rva, mem::size_of::<T>(), align)?;
		// This is safe as per Pod bound, min_size_of and align
		unsafe {
			let p = &*(bytes.as_ptr() as *const T);
			Ok(p)
		}
	}
	/// Reads an unaligned pod `T`.
	fn derva_copy<T>(self, rva: Rva) -> Result<T> where Self: Copy, T: Copy + Pod {
		let bytes = self.slice(rva, mem::size_of::<T>(), 1)?;
		// This is safe as per Pod bound and min_size_of
		unsafe {
			let p = bytes.as_ptr() as *const T;
			Ok(ptr::read_unaligned(p))
		}
	}
	/// Reads an array of pod `T` with given length.
	fn derva_slice<T>(self, rva: Rva, len: usize) -> Result<&'a [T]> where Self: Copy, T: Pod {
		let min_size_of = mem::size_of::<T>().checked_mul(len).ok_or(Error::Overflow)?;
		let align = if cfg!(feature = "unsafe_alignment") { 1 } else { mem::align_of::<T>() };
		let bytes = self.slice(rva, min_size_of, align)?;
		// This is safe as per Pod bound, min_size_of and align
		unsafe {
			Ok(slice::from_raw_parts(bytes.as_ptr() as *const T, len))
		}
	}
	/// Reads an array of pod `T`.
	///
	/// For every element of the array, starting at the given `rva`, the callable `f` is called with that element.
	/// The length of the array is the index when the callable `f` returns `true`.
	///
	/// The returned slice contains all `T` up to but not including the element for which the callable returned `true`.
	fn derva_slice_f<T, F>(self, rva: Rva, mut f: F) -> Result<&'a [T]> where Self: Copy, T: Pod, F: FnMut(&'a T) -> bool {
		let align = if cfg!(feature = "unsafe_alignment") { 1 } else { mem::align_of::<T>() };
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
	fn derva_slice_s<T>(self, rva: Rva, sentinel: T) -> Result<&'a [T]> where Self: Copy, T: PartialEq + Pod {
		self.derva_slice_f(rva, |tee| *tee == sentinel)
	}
	/// Reads a nul-terminated C string.
	fn derva_c_str(self, rva: Rva) -> Result<&'a CStr> where Self: Copy {
		self.derva_string(rva)
	}
	/// Reads a string.
	fn derva_string<T>(self, rva: Rva) -> Result<&'a T> where Self: Copy, T: FromBytes + ?Sized {
		let bytes = self.slice(rva, T::MIN_SIZE_OF, T::ALIGN_OF)?;
		unsafe { T::from_bytes(bytes).ok_or(Error::Encoding) }
	}

	//----------------------------------------------------------------
	// Deref impls for `Ptr`s

	/// Dereferences the pointer to a pod `T`.
	fn deref<T>(self, ptr: Ptr<T>) -> Result<&'a T> where Self: Copy, T: Pod {
		let align = if cfg!(feature = "unsafe_alignment") { 1 } else { mem::align_of::<T>() };
		let bytes = self.read(ptr.into(), mem::size_of::<T>(), align)?;
		// This is safe as per Pod bound, min_size_of and align
		unsafe {
			let p = &*(bytes.as_ptr() as *const T);
			Ok(p)
		}
	}
	/// Dereferences the pointer to an unaligned pod `T`.
	fn deref_copy<T>(self, ptr: Ptr<T>) -> Result<T> where Self: Copy, T: Copy + Pod {
		let bytes = self.read(ptr.into(), mem::size_of::<T>(), 1)?;
		// This is safe as per Pod bound and min_size_of
		unsafe {
			let p = bytes.as_ptr() as *const T;
			Ok(ptr::read_unaligned(p))
		}
	}
	/// Reads an array of pod `T` with given length.
	fn deref_slice<T>(self, ptr: Ptr<[T]>, len: usize) -> Result<&'a [T]> where Self: Copy, T: Pod {
		let min_size_of = mem::size_of::<T>().checked_mul(len).ok_or(Error::Overflow)?;
		let align = if cfg!(feature = "unsafe_alignment") { 1 } else { mem::align_of::<T>() };
		let bytes = self.read(ptr.into(), min_size_of, align)?;
		// This is safe as per Pod bound, min_size_of and align
		unsafe {
			Ok(slice::from_raw_parts(bytes.as_ptr() as *const T, len))
		}
	}
	/// Reads an array of pod `T`.
	///
	/// For every element of the array, starting at the given `ptr`, the callable `f` is called with that element.
	/// The length of the array is the index when the callable `f` returns `true`.
	///
	/// The returned slice contains all `T` up to but not including the element for which the callable returned `true`.
	fn deref_slice_f<T, F>(self, ptr: Ptr<[T]>, mut f: F) -> Result<&'a [T]> where Self: Copy, T: Pod, F: FnMut(&'a T) -> bool {
		let align = if cfg!(feature = "unsafe_alignment") { 1 } else { mem::align_of::<T>() };
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
	fn deref_slice_s<T>(self, ptr: Ptr<[T]>, sentinel: T) -> Result<&'a [T]> where Self: Copy, T: PartialEq + Pod {
		self.deref_slice_f(ptr, |tee| *tee == sentinel)
	}
	/// Dereferences the pointer to a nul-terminated C string.
	fn deref_c_str(self, ptr: Ptr<CStr>) -> Result<&'a CStr> where Self: Copy {
		self.deref_string(ptr)
	}
	/// Dereferences the pointer to a string.
	fn deref_string<T>(self, ptr: Ptr<T>) -> Result<&'a T> where Self: Copy, T: FromBytes + ?Sized {
		let bytes = self.read(ptr.into(), T::MIN_SIZE_OF, T::ALIGN_OF)?;
		unsafe { T::from_bytes(bytes).ok_or(Error::Encoding) }
	}

	//----------------------------------------------------------------

	/// Gets the Export Directory.
	///
	/// See the [exports](exports/index.html) module for more information.
	///
	/// Returns [`Err(Null)`](../enum.Error.html#variant.Null) if the image has no exports. Any other error indiciates some form of corruption.
	fn exports(self) -> Result<super::exports::Exports<'a, Self>> where Self: Copy {
		super::exports::Exports::new(self)
	}

	/// Gets the Import Directory.
	///
	/// See the [imports](imports/index.html) module for more information.
	///
	/// Returns [`Err(Null)`](../enum.Error.html#variant.Null) if the image has no imports. Any other error indicates some form of corruption.
	fn imports(self) -> Result<super::imports::Imports<'a, Self>> where Self: Copy {
		super::imports::Imports::new(self)
	}

	/// Gets the Import Address Table.
	///
	/// See the [imports](imports/index.html) module for more information.
	///
	/// Returns [`Err(Null)`](../enum.Error.html#variant.Null) if the image has no iat. Any other error indicates some form of corruption.
	fn iat(self) -> Result<super::imports::IAT<'a, Self>> where Self: Copy {
		super::imports::IAT::new(self)
	}

	/// Gets the Base Relocations Directory.
	///
	/// See the [base relocations](base_relocs/index.html) module for more information.
	///
	/// Returns [`Err(Null)`](../enum.Error.html#variant.Null) if the image has no base relocations. Any other error indicates some form of corruption.
	fn base_relocs(self) -> Result<super::base_relocs::BaseRelocs<'a, Self>> where Self: Copy {
		super::base_relocs::BaseRelocs::new(self)
	}

	/// Gets the Load Config Directory.
	///
	/// See the [load config](load_config/index.html) module for more information.
	///
	/// Returns [`Err(Null)`](../enum.Error.html#variant.Null) if the image has no load config. Any other error indicates some form of corruption.
	fn load_config(self) -> Result<super::load_config::LoadConfig<'a, Self>> where Self: Copy {
		super::load_config::LoadConfig::new(self)
	}

	/// Gets the TLS Directory.
	///
	/// See the [tls](tls/index.html) module for more information.
	///
	/// Returns [`Err(Null)`](../enum.Error.html#variant.Null) if the image has no tls. Any other error indicates some form of corruption.
	fn tls(self) -> Result<super::tls::Tls<'a, Self>> where Self: Copy {
		super::tls::Tls::new(self)
	}

	/// Gets the Security Directory.
	///
	/// See the [security](security/index.html) module for more information.
	///
	/// Returns [`Err(Null)`](../enum.Error.html#variant.Null) if the image has no security info. Any other error indicates some form of corruption.
	fn security(self) -> Result<super::security::Security<'a, Self>> where Self: Copy {
		super::security::Security::new(self)
	}

	/// Gets the Exception Directory.
	///
	/// See the [exception](exception/index.html) module for more information.
	///
	/// Returns [`Err(Null)`](../enum.Error.html#variant.Null) if the image has no exception directory. Any other error indicates some form of corruption.
	fn exception(self) -> Result<super::exception::Exception<'a, Self>> where Self: Copy {
		super::exception::Exception::new(self)
	}

	/// Gets the Debug Directory.
	///
	/// See the [debug](debug/index.html) module for more information.
	///
	/// Returns [`Err(Null)`](../enum.Error.html#variant.Null) if the image has no debug info. Any other error indicates some form of corruption.
	fn debug(self) -> Result<super::debug::Debug<'a, Self>> where Self: Copy {
		super::debug::Debug::new(self)
	}

	/// Gets the Resources.
	///
	/// See the [resources](resources/index.html) module for more information.
	///
	/// Returns [`Err(Null)`](../enum.Error.html#variant.Null) if the image has no resources. Any other error indicates some form of corruption.
	fn resources(self) -> Result<::resources::Resources<'a>> where Self: Copy {
		let datadir = self.data_directory().get(IMAGE_DIRECTORY_ENTRY_RESOURCE).ok_or(Error::Bounds)?;
		let data = self.derva_slice(datadir.VirtualAddress, datadir.Size as usize)?;
		Ok(::resources::Resources::new(data, datadir))
	}

	/// Gets Scanner access.
	///
	/// See the [scanner](scanner/index.html) module for more information.
	fn scanner(self) -> super::scanner::Scanner<Self> where Self: Copy {
		super::scanner::Scanner::new(self)
	}
}

// Make `&Pe<'a>` trait objects work seamlessly.
unsafe impl<'s, 'a, P: Pe<'a> + ?Sized> Pe<'a> for &'s P {
	fn image(&self) -> &'a [u8] {
		P::image(*self)
	}
	fn align(&self) -> Align {
		P::align(*self)
	}
	fn slice(&self, rva: Rva, min_size_of: usize, align: usize) -> Result<&'a [u8]> {
		P::slice(*self, rva, min_size_of, align)
	}
	fn read(&self, va: Va, min_size_of: usize, align: usize) -> Result<&'a [u8]> {
		P::read(*self, va, min_size_of, align)
	}
	#[cfg(feature = "serde")]
	const SERDE_NAME: &'static str = P::SERDE_NAME;
}

//----------------------------------------------------------------

#[cfg(feature = "serde")]
pub(crate) fn serialize_pe<'a, P: 'a + Pe<'a> + Copy, S: ::serde::Serializer>(pe: P, serializer: S) -> ::std::result::Result<S::Ok, S::Error> {
	use util::serde_helper::*;

	let mut state = serializer.serialize_struct(P::SERDE_NAME, 8)?;
	state.serialize_field("headers", &pe.headers())?;
	state.serialize_field("exports", &pe.exports().ok())?;
	state.serialize_field("imports", &pe.imports().ok())?;
	state.serialize_field("base_relocs", &pe.base_relocs().ok())?;
	state.serialize_field("tls", &pe.tls().ok())?;
	state.serialize_field("load_config", &pe.load_config().ok())?;
	state.serialize_field("security", &pe.security().ok())?;
	state.serialize_field("resources", &pe.resources().ok())?;
	state.end()
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
	if image.as_ptr() as usize % 4 != 0 {
		return Err(Error::Misaligned);
	}
	let dos = unsafe { &*(image.as_ptr() as *const IMAGE_DOS_HEADER) };
	// Verify the DOS header
	if dos.e_magic != IMAGE_DOS_SIGNATURE {
		return Err(Error::BadMagic);
	}
	// "According to the PE specification, the PE header must be aligned on a 8 byte boundary, but the Windows loader requires only a 4 byte alignment."
	if dos.e_lfanew % 4 != 0 {
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
	if nt.Signature != IMAGE_NT_HEADERS_SIGNATURE ||
		!(nt.OptionalHeader.Magic == IMAGE_NT_OPTIONAL_HDR32_MAGIC || nt.OptionalHeader.Magic == IMAGE_NT_OPTIONAL_HDR64_MAGIC)
	{
		return Err(Error::BadMagic);
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
		IMAGE_NUMBEROF_DIRECTORY_ENTRIES);
	let size_of_data_dir = num_rva_sizes * mem::size_of::<IMAGE_DATA_DIRECTORY>();
	if nt_end + size_of_data_dir > image.len() {
		return Err(Error::Bounds);
	}

	// Verify the section headers
	if nt.FileHeader.NumberOfSections > 96 {
		return Err(Error::Insanity);
	}
	// u16 * sizeof(T) casted to usize, cannot reasonably overflow
	let size_of_sections = nt.FileHeader.NumberOfSections as usize * mem::size_of::<IMAGE_SECTION_HEADER>();
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
