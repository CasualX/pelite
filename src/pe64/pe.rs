/*!
Abstract over mapped images and file binaries.
*/

use ::std::{mem, slice};

use super::image::*;
use super::ptr::Ptr;
use ::{Error, Result};
use ::util::{CStr, Pod};

//----------------------------------------------------------------

pub trait Pe<'a> {
	/// Returns the image as a byte slice.
	fn image(&self) -> &'a [u8];

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

	/// Converts an `Rva` to `FileOffset`.
	///
	/// Returns [`Err(ZeroFill)`](../enum.Error.html#variant.ZeroFill) if the rva points to the part of a section zero filled.
	/// This happens when the size of raw data is shorter than the virtual size. Windows fills the remaining of the section with zeroes.
	///
	/// Returns [`Err(OOB)`](../enum.Error.html#variant.OOB) if the rva does not point within any section. This includes the headers.
	fn rva_to_file_offset(self, rva: Rva) -> Result<FileOffset> where Self: Copy {
		for it in self.section_headers() {
			if rva >= it.VirtualAddress && rva < (it.VirtualAddress + it.VirtualSize) {
				if rva < (it.VirtualAddress + it.SizeOfRawData) {
					return Ok((rva - it.VirtualAddress + it.PointerToRawData) as FileOffset);
				}
				return Err(Error::ZeroFill);
			}
		}
		Err(Error::OOB)
	}
	/// Converts a `FileOffset` to `Rva`.
	///
	/// Returns [`Err(OOB)`](../enum.Error.html#variant.OOB) if the file offset points within the headers or part of a section which isn't mapped.
	/// This happens when the virtual size is shorter than the size of raw data.
	fn file_offset_to_rva(self, file_offset: FileOffset) -> Result<Rva> where Self: Copy {
		for it in self.section_headers() {
			if file_offset >= it.PointerToRawData as FileOffset && file_offset < (it.PointerToRawData as FileOffset + it.SizeOfRawData as FileOffset) {
				if file_offset < (it.PointerToRawData as FileOffset + it.VirtualSize as FileOffset) {
					return Ok(file_offset as Rva - it.PointerToRawData + it.VirtualAddress);
				}
				return Err(Error::OOB);
			}
		}
		Err(Error::OOB)
	}

	/// Converts from `Rva` to `Va`.
	///
	/// Returns [`Err(Null)`](../enum.Error.html#variant.Null) given a null rva or [`Err(OOB)`](../enum.Error.html#variant.OOB) if the rva is out of bounds.
	fn rva_to_va(self, rva: Rva) -> Result<Va> where Self: Copy {
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
	/// Converts from `Va` to `Rva`.
	///
	/// Returns [`Err(Null)`](../enum.Error.html#variant.Null) given a null va or [`Err(OOB)`](../enum.Error.html#variant.OOB) if the va is out of bounds.
	fn va_to_rva(self, va: Va) -> Result<Rva> where Self: Copy {
		let (image_base, size_of_image) = {
			let optional_header = self.optional_header();
			(optional_header.ImageBase, optional_header.SizeOfImage)
		};
		let rva = va.checked_sub(image_base).ok_or(Error::OOB)?;
		if rva < size_of_image as Va {
			Ok(rva as Rva)
		}
		else {
			Err(Error::OOB)
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
	fn slice_rva(&self, rva: Rva, size: usize, align: usize) -> Result<&'a [u8]>;

	//----------------------------------------------------------------

	/// Reads a pod `T`.
	fn derva<T>(self, rva: Rva) -> Result<&'a T> where Self: Copy, T: Pod {
		// This is safe as per `Pod` bound
		unsafe {
			let bytes = self.slice_rva(rva, mem::size_of::<T>(), 1)?;
			let p = &*(bytes.as_ptr() as *const T);
			Ok(p)
		}
	}
	/// Reads an array of pod `T` with known length.
	fn derva_slice<T>(self, rva: Rva, len: usize) -> Result<&'a [T]> where Self: Copy, T: Pod {
		// This is safe as per `Pod` bound
		unsafe {
			let size = mem::size_of::<T>().checked_mul(len).ok_or(Error::OOB)?;
			let bytes = self.slice_rva(rva, size, 1)?;
			let p = slice::from_raw_parts(bytes.as_ptr() as *const T, len);
			Ok(p)
		}
	}
	/// Reads an array of pod `T` guarded by a sentinel value.
	///
	/// The callback is called for each `T` found, return `true` to indicate this is the sentinel value.
	///
	/// The returned slice contains all `T` up to but not including the sentinel value.
	///
	/// Returns [`Err(OOB)`](../enum.Error.html#variant.OOB) if no sentinel value was found before reaching the end of the image.
	fn derva_slice_f<T, F>(self, rva: Rva, mut f: F) -> Result<&'a [T]> where Self: Copy, T: Pod, F: FnMut(&T) -> bool {
		let bytes = self.slice_rva(rva, 0, 1)?;
		let size_of_t = mem::size_of::<T>();
		let mut len = 0;
		let mut size = 0;
		loop {
			// Safety critical OOB check
			if size + size_of_t >= bytes.len() {
				return Err(Error::OOB);
			}
			// Safe because size is checked above and T is Pod
			unsafe {
				let s = bytes.as_ptr().offset(size as isize) as *const T;
				if f(&*s) {
					let p = slice::from_raw_parts(bytes.as_ptr() as *const T, len);
					return Ok(p);
				}
			}
			size += size_of_t;
			len += 1;
		}
	}
	/// Reads a nul-terminated C string.
	fn derva_c_str(self, rva: Rva) -> Result<&'a CStr> where Self: Copy {
		self.slice_rva(rva, 0, 1).and_then(CStr::from_bytes)
	}

	//----------------------------------------------------------------
	// Deref impls for `Ptr`s

	/// Dereferences the pointer to a pod `T`.
	fn deref<T, P: Into<Ptr<T>>>(self, ptr: P) -> Result<&'a T> where Self: Copy, T: Pod {
		let ptr = ptr.into();
		self.derva(self.va_to_rva(ptr.into())?)
	}
	/// Dereferences the pointer to an array of pod `T` with known length.
	fn deref_slice<T, P: Into<Ptr<[T]>>>(self, ptr: P, len: usize) -> Result<&'a [T]> where Self: Copy, T: Pod {
		let ptr = ptr.into();
		self.derva_slice(self.va_to_rva(ptr.into())?, len)
	}
	/// Dereferences the pointer to an array of pod `T` guarded by a sentinel value.
	///
	/// The callback is called for each `T` found, return `true` to indicate this is the sentinel value.
	///
	/// The returned slice contains all `T` up to but not including the sentinel value.
	///
	/// Returns [`Err(OOB)`](../enum.Error.html#variant.OOB) if no sentinel value was found before reaching the end of the image.
	fn deref_slice_f<T, P: Into<Ptr<[T]>>, F>(self, ptr: P, f: F) -> Result<&'a [T]> where Self: Copy, T: Pod, F: FnMut(&T) -> bool {
		let ptr = ptr.into();
		self.derva_slice_f(self.va_to_rva(ptr.into())?, f)
	}
	/// Dereferences the pointer to a nul-terminated C string.
	fn deref_c_str<P: Into<Ptr<CStr>>>(self, ptr: P) -> Result<&'a CStr> where Self: Copy {
		let ptr = ptr.into();
		self.derva_c_str(self.va_to_rva(ptr.into())?)
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
}

// Make `&Pe<'a>` trait objects work seamlessly.
impl<'s, 'a, P: Pe<'a> + ?Sized> Pe<'a> for &'s P {
	fn image(&self) -> &'a [u8] {
		P::image(*self)
	}
	fn slice_rva(&self, rva: Rva, size: usize, align: usize) -> Result<&'a [u8]> {
		P::slice_rva(*self, rva, size, align)
	}
}

//----------------------------------------------------------------

pub(crate) struct VH {
	pub image_base: Va,
	pub size_of_image: u32,
}
// TODO: This code needs to be audited...
// The safety of `Pe` relies on it.
pub(crate) fn validate_headers(image: &[u8]) -> ::Result<VH> {
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
