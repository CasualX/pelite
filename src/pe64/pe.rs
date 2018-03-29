/*!
Abstract over mapped images and file binaries.
*/

use std::{cmp, mem, ptr, slice};

use error::{Error, Result};
use util::{CStr, Pod};

use super::image::*;
use super::ptr::Ptr;

//----------------------------------------------------------------

/// The specific alignment used by the view.
///
/// See [the module-level documentation](index.html#getting-started) for more information.
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
	/// Returns the section headers.
	fn section_headers(self) -> &'a [IMAGE_SECTION_HEADER] where Self: Copy {
		let nt = self.nt_headers();
		unsafe {
			let begin = (&nt.OptionalHeader as *const _ as *const u8).offset(nt.FileHeader.SizeOfOptionalHeader as isize) as *const IMAGE_SECTION_HEADER;
			slice::from_raw_parts(begin, nt.FileHeader.NumberOfSections as usize)
		}
	}
	/// Returns the data directory.
	fn data_directory(self) -> &'a [IMAGE_DATA_DIRECTORY] where Self: Copy {
		let opt = self.optional_header();
		unsafe {
			slice::from_raw_parts(opt.DataDirectory.as_ptr(), opt.NumberOfRvaAndSizes as usize)
		}
	}

	//----------------------------------------------------------------

	/// Converts an `Rva` to file offset.
	///
	/// # Errors
	///
	/// * [`Err(ZeroFill)`](../enum.Error.html#variant.ZeroFill) if the rva points to the part of a section zero filled.
	///   This happens when the size of raw data is shorter than the virtual size. Windows fills the remaining of the section with zeroes.
	///
	/// * [`Err(OOB)`](../enum.Error.html#variant.OOB) if the rva does not point within any section. This includes the headers.
	fn rva_to_file_offset(self, rva: Rva) -> Result<usize> where Self: Copy {
		for it in self.section_headers() {
			#[allow(non_snake_case)]
			let VirtualEnd = it.VirtualAddress + cmp::max(it.VirtualSize, it.SizeOfRawData);
			if rva >= it.VirtualAddress && rva < VirtualEnd {
				if rva < (it.VirtualAddress + it.SizeOfRawData) {
					return Ok((rva - it.VirtualAddress + it.PointerToRawData) as usize);
				}
				return Err(Error::ZeroFill);
			}
		}
		Err(Error::OOB)
	}
	/// Converts a file offset to `Rva`.
	///
	/// # Errors
	///
	/// * [`Err(OOB)`](../enum.Error.html#variant.OOB) if the file offset points within the headers or part of a section which isn't mapped.
	///   This happens when the virtual size is shorter than the size of raw data.
	fn file_offset_to_rva(self, file_offset: usize) -> Result<Rva> where Self: Copy {
		for it in self.section_headers() {
			if file_offset >= it.PointerToRawData as usize && file_offset < (it.PointerToRawData as usize + it.SizeOfRawData as usize) {
				if file_offset < (it.PointerToRawData as usize + it.VirtualSize as usize) {
					return Ok(file_offset as Rva - it.PointerToRawData + it.VirtualAddress);
				}
				return Err(Error::OOB);
			}
		}
		Err(Error::OOB)
	}

	/// Converts from `Rva` to `Va`.
	///
	/// # Errors
	///
	/// * [`Err(Null)`](../enum.Error.html#variant.Null) given a null rva.
	///
	/// * [`Err(OOB)`](../enum.Error.html#variant.OOB) if the rva is out of bounds.
	fn rva_to_va(self, rva: Rva) -> Result<Va> where Self: Copy {
		if rva == BADRVA {
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
				Err(Error::OOB)
			}
		}
	}
	/// Converts from `Va` to `Rva`.
	///
	/// # Errors
	///
	/// * [`Err(Null)`](../enum.Error.html#variant.Null) given a null va.
	///
	/// * [`Err(OOB)`](../enum.Error.html#variant.OOB) if the va is out of bounds.
	fn va_to_rva(self, va: Va) -> Result<Rva> where Self: Copy {
		if va == BADVA {
			Err(Error::Null)
		}
		else {
			let (image_base, size_of_image) = {
				let optional_header = self.optional_header();
				(optional_header.ImageBase, optional_header.SizeOfImage)
			};
			if va < image_base || va - image_base > size_of_image as Va {
				Err(Error::OOB)
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
	/// Returns [`Err(Null)`](../enum.Error.html#variant.Null) given a null rva.
	fn slice(&self, rva: Rva, min_size: usize, align: usize) -> Result<&'a [u8]>;

	/// Slices the image at the specified rva returning a byte slice with no alignment or minimum size.
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
	/// Returns [`Err(Null)`](../enum.Error.html#variant.Null) given a null va.
	fn read(&self, va: Va, min_size: usize, align: usize) -> Result<&'a [u8]>;

	/// Reads the image at the specified va returning a byte slice with no alignment or minimum size.
	fn read_bytes(self, va: Va) -> Result<&'a [u8]> where Self: Sized {
		self.read(va, 0, 1)
	}

	//----------------------------------------------------------------

	/// Reads an aligned pod `T`.
	fn derva<T>(self, rva: Rva) -> Result<&'a T> where Self: Copy, T: Pod {
		let align = if cfg!(feature = "unsafe_alignment") { 1 } else { mem::align_of::<T>() };
		let bytes = self.slice(rva, mem::size_of::<T>(), align)?;
		// This is safe as per Pod bound, min_size and align
		unsafe {
			let p = &*(bytes.as_ptr() as *const T);
			Ok(p)
		}
	}
	/// Reads an unaligned pod `T`.
	fn derva_copy<T>(self, rva: Rva) -> Result<T> where Self: Copy, T: Copy + Pod {
		let bytes = self.slice(rva, mem::size_of::<T>(), 1)?;
		// This is safe as per Pod bound and min_size
		unsafe {
			let p = bytes.as_ptr() as *const T;
			Ok(ptr::read_unaligned(p))
		}
	}
	/// Reads an array of pod `T` with given length.
	fn derva_slice<T>(self, rva: Rva, len: usize) -> Result<&'a [T]> where Self: Copy, T: Pod {
		let min_size = mem::size_of::<T>().checked_mul(len).ok_or(Error::Overflow)?;
		let align = if cfg!(feature = "unsafe_alignment") { 1 } else { mem::align_of::<T>() };
		let bytes = self.slice(rva, min_size, align)?;
		// This is safe as per Pod bound, min_size and align
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
				return Err(Error::OOB);
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
	fn derva_str(self, rva: Rva) -> Result<&'a CStr> where Self: Copy {
		self.slice_bytes(rva).and_then(CStr::from_bytes)
	}

	//----------------------------------------------------------------
	// Deref impls for `Ptr`s

	/// Dereferences the pointer to a pod `T`.
	fn deref<T, P>(self, ptr: P) -> Result<&'a T> where Self: Copy, T: Pod, P: Into<Ptr<T>> {
		let align = if cfg!(feature = "unsafe_alignment") { 1 } else { mem::align_of::<T>() };
		let bytes = self.read(ptr.into().into(), mem::size_of::<T>(), align)?;
		// This is safe as per Pod bound, min_size and align
		unsafe {
			let p = &*(bytes.as_ptr() as *const T);
			Ok(p)
		}
	}
	/// Dereferences the pointer to an unaligned pod `T`.
	fn deref_copy<T, P>(self, ptr: P) -> Result<T> where Self: Copy, T: Copy + Pod, P: Into<Ptr<T>> {
		let bytes = self.read(ptr.into().into(), mem::size_of::<T>(), 1)?;
		// This is safe as per Pod bound and min_size
		unsafe {
			let p = bytes.as_ptr() as *const T;
			Ok(ptr::read_unaligned(p))
		}
	}
	/// Reads an array of pod `T` with given length.
	fn deref_slice<T, P>(self, ptr: P, len: usize) -> Result<&'a [T]> where Self: Copy, T: Pod, P: Into<Ptr<[T]>> {
		let min_size = mem::size_of::<T>().checked_mul(len).ok_or(Error::Overflow)?;
		let align = if cfg!(feature = "unsafe_alignment") { 1 } else { mem::align_of::<T>() };
		let bytes = self.read(ptr.into().into(), min_size, align)?;
		// This is safe as per Pod bound, min_size and align
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
	fn deref_slice_f<T, P, F>(self, ptr: P, mut f: F) -> Result<&'a [T]> where Self: Copy, T: Pod, P: Into<Ptr<[T]>>, F: FnMut(&'a T) -> bool {
		let align = if cfg!(feature = "unsafe_alignment") { 1 } else { mem::align_of::<T>() };
		let bytes = self.read(ptr.into().into(), 0, align)?;
		let mut len = 0;
		loop {
			// Safety critical OOB check
			// Overflows only if bytes.len() > USIZE_MAX - sizeof(T) which would be ridiculous
			let offset = len * mem::size_of::<T>();
			if offset + mem::size_of::<T>() > bytes.len() {
				return Err(Error::OOB);
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
	fn deref_slice_s<T, P>(self, ptr: P, sentinel: T) -> Result<&'a [T]> where Self: Copy, T: PartialEq + Pod, P: Into<Ptr<[T]>> {
		self.deref_slice_f(ptr, |tee| *tee == sentinel)
	}
	/// Dereferences the pointer to a nul-terminated C string.
	fn deref_str<P>(self, ptr: P) -> Result<&'a CStr> where Self: Copy, P: Into<Ptr<CStr>> {
		self.read_bytes(ptr.into().into()).and_then(CStr::from_bytes)
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

	/// Gets the Base Relocations Directory.
	///
	/// See the [base relocations](base_relocs/index.html) module for more information.
	///
	/// Returns [`Err(Null)`](../enum.Error.html#variant.Null) if the image has no base relocations. Any other error indicates some form of corruption.
	fn base_relocs(self) -> Result<super::base_relocs::BaseRelocs<'a, Self>> where Self: Copy {
		super::base_relocs::BaseRelocs::new(self)
	}

	/// Gets the TLS Directory.
	///
	/// See the [tls](tls/index.html) module for more information.
	///
	/// Returns [`Err(Null)`](../enum.Error.html#variant.Null) if the image has no tls. Any other error indicates some form of corruption.
	fn tls(self) -> Result<super::tls::Tls<'a, Self>> where Self: Copy {
		super::tls::Tls::new(self)
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
		let datadir = self.data_directory().get(IMAGE_DIRECTORY_ENTRY_RESOURCE).ok_or(Error::OOB)?;
		let data = self.derva_slice(datadir.VirtualAddress, datadir.Size as usize)?;
		Ok(::resources::Resources::new(data, datadir.VirtualAddress))
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
	fn slice(&self, rva: Rva, min_size: usize, align: usize) -> Result<&'a [u8]> {
		P::slice(*self, rva, min_size, align)
	}
	fn read(&self, va: Va, min_size: usize, align: usize) -> Result<&'a [u8]> {
		P::read(*self, va, min_size, align)
	}
}

//----------------------------------------------------------------

pub(crate) struct VH {
	pub image_base: Va,
	pub size_of_image: u32,
}
// TODO: This code needs to be audited...
// The safety of `Pe` relies on it.
pub(crate) fn validate_headers(image: &[u8]) -> Result<VH> {
	// Grab the DOS header
	if mem::size_of::<IMAGE_DOS_HEADER>() > image.len() {
		return Err(Error::OOB);
	}
	let dos = unsafe { &*(image.as_ptr() as *const IMAGE_DOS_HEADER) };
	// Verify the DOS header
	if dos.e_magic != IMAGE_DOS_SIGNATURE {
		return Err(Error::BadMagic);
	}
	// Grab the NT headers
	if usize::checked_add(dos.e_lfanew as usize, mem::size_of::<IMAGE_NT_HEADERS>()).ok_or(Error::Overflow)? > image.len() {
		return Err(Error::OOB);
	}
	let nt = unsafe { &*(image.as_ptr().offset(dos.e_lfanew as isize) as *const IMAGE_NT_HEADERS) };
	// Verify the NT headers
	if nt.Signature != IMAGE_NT_HEADERS_SIGNATURE || nt.OptionalHeader.Magic != IMAGE_NT_OPTIONAL_HDR_MAGIC {
		return Err(Error::BadMagic);
	}
	if nt.OptionalHeader.SizeOfHeaders as usize > image.len() {
		return Err(Error::Corrupt);
	}
	// Verify the data directory with some arbtirary chosen limits
	if nt.OptionalHeader.NumberOfRvaAndSizes < IMAGE_NUMBEROF_DIRECTORY_ENTRIES as u32 || nt.OptionalHeader.NumberOfRvaAndSizes > 100 {
		return Err(Error::Insanity);
	}
	let size_of_data_dir = mem::size_of::<IMAGE_DATA_DIRECTORY>() * nt.OptionalHeader.NumberOfRvaAndSizes as usize;
	let size_of_opt_header = mem::size_of::<IMAGE_OPTIONAL_HEADER>() - mem::size_of::<[IMAGE_DATA_DIRECTORY; IMAGE_NUMBEROF_DIRECTORY_ENTRIES]>() + size_of_data_dir;
	if size_of_opt_header > nt.FileHeader.SizeOfOptionalHeader as usize {
		return Err(Error::Corrupt);
	}
	// Verify the section headers
	let sec_begin = dos.e_lfanew as usize + (mem::size_of::<IMAGE_NT_HEADERS>() - mem::size_of::<IMAGE_OPTIONAL_HEADER>()) + nt.FileHeader.SizeOfOptionalHeader as usize;
	let sec_end = sec_begin + nt.FileHeader.NumberOfSections as usize * mem::size_of::<IMAGE_SECTION_HEADER>();
	if sec_end > nt.OptionalHeader.SizeOfHeaders as usize {
		return Err(Error::Corrupt);
	}
	Ok(VH {
		image_base: nt.OptionalHeader.ImageBase,
		size_of_image: nt.OptionalHeader.SizeOfImage,
	})
}
