/*!
Abstract over mapped images and file binaries.
*/

use super::*;

//----------------------------------------------------------------

pub use crate::wrap::PeLayout;

/// Basic properties shared by file-aligned and mapped PE images.
///
/// # Safety
///
/// Implementors must return a valid PE image for the full lifetime `'a`,
/// and report layout and base-address information matching that image.
pub unsafe trait PeObject<'a> {
	/// Returns the image as a byte slice.
	fn image(&self) -> &'a [u8];

	/// Returns whether the image uses file or mapped section layout.
	fn layout(&self) -> PeLayout;

	/// Returns the base virtual address of this image.
	///
	/// For an image on disk, this should be the preferred virtual address. For a memory mapped image, this should be the actual virtual address.
	fn image_base(&self) -> Va;

	// Give a struct name in Serialize implementation
	#[cfg(feature = "serde")]
	#[doc(hidden)]
	fn serde_name(&self) -> &'static str;
}

/// Read-only access to a validated PE image.
///
/// # Safety
///
/// The image supplied by [`PeObject`] must contain valid headers for this
/// architecture so the unchecked header accessors are sound.
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
	fn section_headers(self) -> &'a super::PeSectionHeaders {
		unsafe { section_headers(self.image()) }
	}

	/// Returns the pe headers together in a single struct.
	fn headers(self) -> super::PeHeaders<Self> {
		super::PeHeaders::new(self)
	}

	//----------------------------------------------------------------

	/// Converts a relative virtual address to file offset.
	///
	/// # Errors
	///
	/// * [`Overflow`][crate::Error::Overflow]:
	///   The rva is contained within a corrupt section where the range bounds overflow.
	///
	/// * [`ZeroFill`][crate::Error::ZeroFill]:
	///   The rva points to part of a section zero filled and is not available on disk.
	///
	/// * [`Bounds`][crate::Error::Bounds]:
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
			let VirtualEnd = it.VirtualAddress.wrapping_add(cmp::max(it.VirtualSize, it.SizeOfRawData));
			// $1
			if it.VirtualAddress <= rva && rva < VirtualEnd {
				// Check if the raw data reference is sane
				// $2
				if let None = it.PointerToRawData.checked_add(it.SizeOfRawData) {
					return Err(Error::Overflow);
				}
				// Calculate the offset in the section. cannot underflow, see $1
				let section_offset = rva - it.VirtualAddress;
				// $3
				return if section_offset < it.SizeOfRawData {
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
	/// * [`Overflow`][crate::Error::Overflow]:
	///   The file offset is contained within a corrupt section where the range bounds overflow.
	///
	/// * [`Unmapped`][crate::Error::Unmapped]:
	///   The file offset points to part of a section not mapped and is not available in virtual memory.
	///
	/// * [`Bounds`][crate::Error::Bounds]:
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
			// $1
			if it.PointerToRawData as usize <= file_offset && file_offset < EndOfRawData as usize {
				// Check if the virtual reference is sane
				// $2
				if let None = it.VirtualAddress.checked_add(it.VirtualSize) {
					return Err(Error::Overflow);
				}
				// Calculate the offset in the section. cannot underflow, see $1
				let section_offset = file_offset as Rva - it.PointerToRawData;
				// $3
				return if section_offset < it.VirtualSize {
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
	/// * [`Null`][crate::Error::Null]:
	///   The rva is zero.
	///
	/// * [`Bounds`][crate::Error::Bounds]:
	///   The rva does not fall within the virtual image bounds.
	fn rva_to_va(self, rva: Rva) -> Result<Va> {
		if rva == 0 {
			Err(Error::Null)
		}
		else {
			let image_base = self.image_base();
			let size_of_image = self.optional_header().SizeOfImage;

			if rva <= size_of_image {
				image_base.checked_add(rva as Va).ok_or(Error::Overflow)
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
	/// * [`Null`][crate::Error::Null]:
	///   The va is zero.
	///
	/// * [`Bounds`][crate::Error::Bounds]:
	///   The va does not fall within the virtual image bounds.
	fn va_to_rva(self, va: Va) -> Result<Rva> {
		if va == 0 {
			Err(Error::Null)
		}
		else {
			let image_base = self.image_base();
			let size_of_image = self.optional_header().SizeOfImage;

			// Carefully avoid panicking overflow
			if va < image_base || va - image_base > size_of_image as Va {
				Err(Error::Bounds)
			}
			else {
				Ok((va - image_base) as Rva)
			}
		}
	}

	/// Returns the byte offset of `symbol` from the start of the image.
	///
	/// `symbol` must refer to data within `self.image()`.
	/// Be careful not to pass a reference to a copy of that data, such as one stored in a temporary buffer.
	///
	/// # Panics
	///
	/// Panics if `symbol` does not refer to data within this image.
	/// Rust's lifetime system cannot express this relationship, so this requirement is enforced at runtime.
	#[inline]
	#[track_caller]
	fn offset_of<T: Pod + ?Sized>(self, symbol: &'a T) -> usize {
		let image = self.image();
		let base = image.as_ptr().addr();
		let symbol = (symbol as *const T).addr();

		assert!(
			symbol >= base && symbol <= base + image.len(),
			"symbol does not refer to data within this image",
		);

		symbol - base
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
	/// * [`Null`][crate::Error::Null]:
	///   The rva is zero.
	fn slice(&self, rva: Rva, min_size_of: usize, align: usize) -> Result<&'a [u8]> {
		unsafe {
			match (self.layout(), self.image()) {
				(PeLayout::File, image) => slice_file(image, rva, min_size_of, align),
				(PeLayout::Section, image) => slice_section(image, rva, min_size_of, align),
			}
		}
	}

	/// Slices the image at the specified rva returning a byte slice with no alignment or minimum size.
	///
	/// Shorthand to invoke [`slice(rva, 0, 1)`](#tymethod.slice).
	fn slice_bytes(self, rva: Rva) -> Result<&'a [u8]> where Self: Sized {
		self.slice(rva, 0, 1)
	}

	/// Gets the bytes defined by a section header in this image.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]:
	///   The virtual address or pointer to raw data is zero.
	///
	/// * [`Bounds`][crate::Error::Bounds]:
	///   The data referenced by the section header is out of bounds.
	fn get_section_bytes(self, section_header: &IMAGE_SECTION_HEADER) -> Result<&'a [u8]> {
		crate::wrap::get_section_bytes(self.image(), section_header, self.layout())
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
	/// * [`Null`][crate::Error::Null]:
	///   The va is zero.
	fn read(&self, va: Va, min_size_of: usize, align: usize) -> Result<&'a [u8]> {
		unsafe {
			match (self.layout(), self.image()) {
				(PeLayout::File, image) => read_file(image, self.image_base(), va, min_size_of, align),
				(PeLayout::Section, image) => read_section(image, self.image_base(), va, min_size_of, align),
			}
		}
	}

	/// Reads the image at the specified va returning a byte slice with no alignment or minimum size.
	///
	/// Shorthand to invoke [`read(va, 0, 1)`](#tymethod.read).
	fn read_bytes(self, va: Va) -> Result<&'a [u8]> where Self: Sized {
		self.read(va, 0, 1)
	}

	//----------------------------------------------------------------

	/// Reads an aligned pod `T`.
	fn derva<T: Pod>(self, rva: Rva) -> Result<&'a T> {
		let align = mem::align_of::<T>();
		let bytes = self.slice(rva, mem::size_of::<T>(), align)?;
		// This is safe as per Pod bound, min_size_of and align
		unsafe {
			let p = &*(bytes.as_ptr() as *const T);
			Ok(p)
		}
	}
	/// Reads an unaligned pod `T`.
	fn derva_copy<T: Copy + Pod>(self, rva: Rva) -> Result<T> {
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
	fn derva_into<T: ?Sized + Pod>(self, rva: Rva, dest: &mut T) -> Result<()> {
		let len = mem::size_of_val(dest);
		let bytes = self.slice(rva, len, 1)?;
		dataview::bytes_mut(dest).copy_from_slice(&bytes[..len]);
		Ok(())
	}
	/// Reads an array of pod `T` with given length.
	fn derva_slice<T: Pod>(self, rva: Rva, len: usize) -> Result<&'a [T]> {
		let min_size_of = mem::size_of::<T>().checked_mul(len).ok_or(Error::Overflow)?;
		let align = mem::align_of::<T>();
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
	fn derva_slice_f<T: Pod, F: FnMut(&'a T) -> bool>(self, rva: Rva, mut f: F) -> Result<&'a [T]> {
		const { assert!(mem::size_of::<T>() > 0, "cannot scan an array of zero-sized elements") };
		let element_size = mem::size_of::<T>();
		let align = mem::align_of::<T>();
		let bytes = self.slice(rva, 0, align)?;
		let mut len = 0;
		loop {
			// Safety critical OOB check
			// Overflows only if bytes.len() > USIZE_MAX - sizeof(T) which would be ridiculous
			let offset = len * element_size;
			if offset + element_size > bytes.len() {
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
	fn derva_slice_s<T: PartialEq + Pod>(self, rva: Rva, sentinel: T) -> Result<&'a [T]> {
		self.derva_slice_f(rva, |tee| *tee == sentinel)
	}
	/// Reads a nul-terminated C string.
	fn derva_c_str(self, rva: Rva) -> Result<&'a CStr> {
		self.derva_string(rva)
	}
	/// Reads a string.
	fn derva_string<T: FromBytes + ?Sized>(self, rva: Rva) -> Result<&'a T> {
		let bytes = self.slice(rva, T::MIN_SIZE_OF, T::ALIGN_OF)?;
		unsafe { T::from_bytes(bytes).ok_or(Error::Encoding) }
	}

	//----------------------------------------------------------------
	// Deref impls for `Ptr`s

	/// Dereferences the pointer to a pod `T`.
	fn deref<T: Pod>(self, ptr: Ptr<T>) -> Result<&'a T> {
		let align = mem::align_of::<T>();
		let bytes = self.read(ptr.into(), mem::size_of::<T>(), align)?;
		// This is safe as per Pod bound, min_size_of and align
		unsafe {
			let p = &*(bytes.as_ptr() as *const T);
			Ok(p)
		}
	}
	/// Dereferences the pointer to an unaligned pod `T`.
	fn deref_copy<T: Copy + Pod>(self, ptr: Ptr<T>) -> Result<T> {
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
	fn deref_into<T: ?Sized + Pod>(self, ptr: Ptr<T>, dest: &mut T) -> Result<()> {
		let len = mem::size_of_val(dest);
		let bytes = self.read(ptr.into(), len, 1)?;
		dataview::bytes_mut(dest).copy_from_slice(&bytes[..len]);
		Ok(())
	}
	/// Reads an array of pod `T` with given length.
	fn deref_slice<T: Pod>(self, ptr: Ptr<[T]>, len: usize) -> Result<&'a [T]> {
		let min_size_of = mem::size_of::<T>().checked_mul(len).ok_or(Error::Overflow)?;
		let align = mem::align_of::<T>();
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
	fn deref_slice_f<T: Pod, F: FnMut(&'a T) -> bool>(self, ptr: Ptr<[T]>, mut f: F) -> Result<&'a [T]> {
		const { assert!(mem::size_of::<T>() > 0, "cannot scan an array of zero-sized elements") };
		let element_size = mem::size_of::<T>();
		let align = mem::align_of::<T>();
		let bytes = self.read(ptr.into(), 0, align)?;
		let mut len = 0;
		loop {
			// Safety critical OOB check
			// Overflows only if bytes.len() > USIZE_MAX - sizeof(T) which would be ridiculous
			let offset = len * element_size;
			if offset + element_size > bytes.len() {
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
	fn deref_slice_s<T: PartialEq + Pod>(self, ptr: Ptr<[T]>, sentinel: T) -> Result<&'a [T]> {
		self.deref_slice_f(ptr, |tee| *tee == sentinel)
	}
	/// Dereferences the pointer to a nul-terminated C string.
	fn deref_c_str(self, ptr: Ptr<CStr>) -> Result<&'a CStr> {
		self.deref_string(ptr)
	}
	/// Dereferences the pointer to a string.
	fn deref_string<T: FromBytes + ?Sized>(self, ptr: Ptr<T>) -> Result<&'a T> {
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
	/// Returns the [`ExportDirectory`](super::ExportDirectory).
	///
	/// Returns [`Err(Null)`][crate::Error::Null] if the image has no exports. Any other error indiciates some form of corruption.
	fn exports(self) -> Result<super::ExportDirectory<'a, Self>> {
		super::ExportDirectory::try_from(self)
	}

	/// Conveniently gets the address of an exported function.
	///
	/// This method does not support forwarded exports and returns [`Err(Null)`][crate::Error::Null] for them.
	/// Calling it repeatedly is less efficient than caching an [`ExportBy`](super::ExportBy) instance.
	#[inline(never)]
	fn get_proc_address<T>(self, name: T) -> Result<Va> where Self: super::GetProcAddress<'a, T> {
		let export = <Self as super::GetProcAddress<'a, T>>::get_export(self, name)?;
		self.rva_to_va(export.symbol().ok_or(Error::Null)?)
	}

	/// Gets the Import Directory.
	///
	/// Returns the [`ImportDirectory`](super::ImportDirectory).
	///
	/// Returns [`Err(Null)`][crate::Error::Null] if the image has no imports. Any other error indicates some form of corruption.
	fn imports(self) -> Result<super::ImportDirectory<'a, Self>> {
		super::ImportDirectory::try_from(self)
	}

	/// Gets the Import Address Table.
	///
	/// Returns the [`ImportAddressTable`](super::ImportAddressTable).
	///
	/// Returns [`Err(Null)`][crate::Error::Null] if the image has no iat. Any other error indicates some form of corruption.
	fn iat(self) -> Result<super::ImportAddressTable<'a, Self>> {
		super::ImportAddressTable::try_from(self)
	}

	/// Gets the Base Relocations Directory.
	///
	/// See the [base relocations][crate::base_relocs] module for more information.
	///
	/// Returns [`Err(Null)`][crate::Error::Null] if the image has no base relocations. Any other error indicates some form of corruption.
	fn base_relocs(self) -> Result<crate::base_relocs::BaseRelocationDirectory<'a>> {
		super::base_relocs::try_from(self)
	}

	/// Gets the Load Config Directory.
	///
	/// Returns the [`LoadConfigDirectory`](super::LoadConfigDirectory).
	///
	/// Returns [`Err(Null)`][crate::Error::Null] if the image has no load config. Any other error indicates some form of corruption.
	fn load_config(self) -> Result<super::LoadConfigDirectory<'a, Self>> {
		super::LoadConfigDirectory::try_from(self)
	}

	/// Gets the TLS Directory.
	///
	/// Returns the [`TlsDirectory`](super::TlsDirectory).
	///
	/// Returns [`Err(Null)`][crate::Error::Null] if the image has no tls. Any other error indicates some form of corruption.
	fn tls(self) -> Result<super::TlsDirectory<'a, Self>> {
		super::TlsDirectory::try_from(self)
	}

	/// Gets the Security Directory.
	///
	/// See the [security][crate::security] module for more information.
	///
	/// Returns [`Err(Null)`][crate::Error::Null] if the image has no security info. Any other error indicates some form of corruption.
	fn security(self) -> Result<crate::security::SecurityDirectory<'a>> {
		super::security::try_from(self)
	}

	branch! {
		pe32 {}
		pe64 {
			/// Gets the x64 Exception Directory.
			///
			/// Returns the [`X64ExceptionDirectory`](super::X64ExceptionDirectory).
			///
			/// Returns [`Err(Null)`][crate::Error::Null] if the image has no exception directory.
			/// Any other error indicates an unsupported machine type or some form of corruption.
			fn exception_x64(self) -> Result<super::X64ExceptionDirectory<'a, Self>> {
				super::X64ExceptionDirectory::try_from(self)
			}

			/// Gets the ARM64 Exception Directory.
			///
			/// Returns the [`Arm64ExceptionDirectory`](super::Arm64ExceptionDirectory).
			///
			/// Returns [`Err(Null)`][crate::Error::Null] if the image has no exception directory.
			/// Any other error indicates an unsupported machine type or some form of corruption.
			fn exception_arm64(self) -> Result<super::Arm64ExceptionDirectory<'a, Self>> {
				super::Arm64ExceptionDirectory::try_from(self)
			}
		}
	}

	/// Gets the Debug Directory.
	///
	/// Returns the [`DebugDirectory`](super::DebugDirectory).
	///
	/// Returns [`Err(Null)`][crate::Error::Null] if the image has no debug info. Any other error indicates some form of corruption.
	fn debug(self) -> Result<crate::debug::DebugDirectory<'a>> {
		super::debug::try_from(self)
	}

	/// Gets the Resource Directory.
	///
	/// Returns the resource directory. See [`crate::resources`] for its API.
	///
	/// Returns [`Err(Null)`][crate::Error::Null] if the image has no resources. Any other error indicates some form of corruption.
	///
	/// # Examples
	///
	/// ```
	/// use pelite::pe64::{Pe, PeFile};
	/// use pelite::resources::ResourceFindError;
	///
	/// # #[allow(dead_code)]
	/// fn manifest<'a>(file: PeFile<'a>) -> Result<&'a [u8], ResourceFindError> {
	/// 	// Access the resource directory
	/// 	let resources = file.resources()?;
	///
	/// 	// Find the manifest resource and return its bytes
	/// 	Ok(resources.find_data("/Manifest/2/1033")?.bytes()?)
	/// }
	/// ```
	fn resources(self) -> Result<crate::resources::ResourceDirectory<'a>> where Self: Copy {
		let datadir = self.data_directory().get(IMAGE_DIRECTORY_ENTRY_RESOURCE).ok_or(Error::Bounds)?;
		if datadir.VirtualAddress == 0 {
			return Err(Error::Null);
		}
		let bytes = self.slice_bytes(datadir.VirtualAddress)?;
		let size = cmp::min(datadir.Size as usize, bytes.len());
		Ok(crate::resources::ResourceDirectory::new(&bytes[..size], datadir))
	}

	/// Gets Scanner access.
	///
	/// Creates a [`Scanner`](super::Scanner) for this image.
	fn scanner(self) -> super::Scanner<Self> {
		super::Scanner::new(self)
	}
}

//----------------------------------------------------------------
// Make `&PeObject<'a>` trait objects work seamlessly.

unsafe impl<'s, 'a> PeObject<'a> for &'s dyn PeObject<'a> {
	fn image(&self) -> &'a [u8] {
		PeObject::image(*self)
	}
	fn layout(&self) -> PeLayout {
		PeObject::layout(*self)
	}

	fn image_base(&self) -> Va {
		PeObject::image_base(*self)
	}

	#[cfg(feature = "serde")]
	fn serde_name(&self) -> &'static str {
		PeObject::serde_name(*self)
	}
}

unsafe impl<'s, 'a> Pe<'a> for &'s dyn PeObject<'a> {}

//----------------------------------------------------------------

#[cfg(feature = "serde")]
pub(crate) fn serialize_pe<'a, P: Pe<'a>, S: serde::Serializer>(pe: P, serializer: S) -> core::result::Result<S::Ok, S::Error> {
	use crate::util::serde_helper::*;

	let fields = branch! { pe32 { 11 } pe64 { 12 } };
	let mut state = serializer.serialize_struct(pe.serde_name(), fields)?;
	state.serialize_field("headers", &pe.headers())?;
	state.serialize_field("rich_structure", &pe.rich_structure().ok())?;
	state.serialize_field("exports", &pe.exports().ok())?;
	state.serialize_field("imports", &pe.imports().ok())?;
	state.serialize_field("iat", &pe.iat().ok())?;
	state.serialize_field("base_relocs", &pe.base_relocs().ok())?;
	state.serialize_field("debug", &pe.debug().ok())?;
	state.serialize_field("tls", &pe.tls().ok())?;
	state.serialize_field("load_config", &pe.load_config().ok())?;
	state.serialize_field("security", &pe.security().ok())?;
	state.serialize_field("resources", &pe.resources().ok())?;
	branch! {
		pe32 {}
		pe64 {
			if let Ok(exception) = pe.exception_x64() {
				state.serialize_field("exception", &exception)?;
			}
			else if let Ok(exception) = pe.exception_arm64() {
				state.serialize_field("exception", &exception)?;
			}
			else {
				state.serialize_field("exception", &Option::<()>::None)?;
			}
		}
	}
	state.end()
}

//----------------------------------------------------------------
// Implementation helpers

unsafe fn dos_header(image: &[u8]) -> &IMAGE_DOS_HEADER { unsafe {
	&*(image.as_ptr() as *const IMAGE_DOS_HEADER)
}}
unsafe fn dos_image(image: &[u8]) -> &[u8] { unsafe {
	image.get_unchecked(..dos_header(image).e_lfanew as usize)
}}
unsafe fn nt_headers(image: &[u8]) -> &IMAGE_NT_HEADERS { unsafe {
	&*(image.as_ptr().offset(dos_header(image).e_lfanew as isize) as *const IMAGE_NT_HEADERS)
}}
unsafe fn file_header(image: &[u8]) -> &IMAGE_FILE_HEADER { unsafe {
	&nt_headers(image).FileHeader
}}
pub(crate) unsafe fn optional_header(image: &[u8]) -> &IMAGE_OPTIONAL_HEADER { unsafe {
	&nt_headers(image).OptionalHeader
}}
unsafe fn data_directory(image: &[u8]) -> &[IMAGE_DATA_DIRECTORY] { unsafe {
	let opt = optional_header(image);
	let len = cmp::min(opt.NumberOfRvaAndSizes as usize, IMAGE_NUMBEROF_DIRECTORY_ENTRIES);
	let offset = dos_header(image).e_lfanew as usize
		+ mem::offset_of!(IMAGE_NT_HEADERS, OptionalHeader)
		+ mem::offset_of!(IMAGE_OPTIONAL_HEADER, DataDirectory);
	let data = image.as_ptr().add(offset).cast();
	slice::from_raw_parts(data, len)
}}
unsafe fn section_headers(image: &[u8]) -> &super::PeSectionHeaders { unsafe {
	let nt = nt_headers(image);
	let offset = dos_header(image).e_lfanew as usize
		+ mem::offset_of!(IMAGE_NT_HEADERS, OptionalHeader)
		+ nt.FileHeader.SizeOfOptionalHeader as usize;
	let data = image.as_ptr().add(offset).cast();
	let raw = slice::from_raw_parts(data, nt.FileHeader.NumberOfSections as usize);
	super::PeSectionHeaders::new(raw)
}}

unsafe fn slice_section(image: &[u8], rva: Rva, min_size_of: usize, align_of: usize) -> Result<&[u8]> {
	let start = rva as usize;
	if rva == 0 {
		Err(Error::Null)
	}
	else if start > image.len() {
		Err(Error::Bounds)
	}
	else if !usize::wrapping_add(image.as_ptr() as usize, start).aligned_to(align_of) {
		Err(Error::Misaligned)
	}
	else {
		match image.get(start..) {
			Some(bytes) if bytes.len() >= min_size_of => Ok(bytes),
			_ => Err(Error::Bounds),
		}
	}
}
unsafe fn read_section(image: &[u8], image_base: Va, va: Va, min_size_of: usize, align_of: usize) -> Result<&[u8]> { unsafe {
	let image_size = optional_header(image).SizeOfImage;

	if va == 0 {
		Err(Error::Null)
	}
	else if va < image_base || va - image_base > image_size as Va {
		Err(Error::Bounds)
	}
	else {
		let start = (va - image_base) as usize;
		if !usize::wrapping_add(image.as_ptr() as usize, start).aligned_to(align_of) {
			Err(Error::Misaligned)
		}
		else {
			match image.get(start..) {
				Some(bytes) if bytes.len() >= min_size_of => Ok(bytes),
				_ => Err(Error::Bounds),
			}
		}
	}
}}

unsafe fn range_file(image: &[u8], rva: Rva, min_size_of: usize) -> Result<&[u8]> { unsafe {
	// This code has been carefully designed to avoid panicking on overflow
	for it in section_headers(image) {
		// Compare if rva is contained within the virtual address space of a section
		// If the calculating the section end address overflows the corrupt section will be skipped
		#[allow(non_snake_case)]
		let VirtualEnd = it.VirtualAddress.wrapping_add(cmp::max(it.VirtualSize, it.SizeOfRawData));
		// $1
		if it.VirtualAddress <= rva && rva < VirtualEnd {
			// Isolate and range check the pointer and size of raw data
			// If this fails immediately abort and return an error
			let section_range = it.PointerToRawData as usize..it.PointerToRawData.wrapping_add(it.SizeOfRawData) as usize;
			let section_bytes = image.get(section_range).ok_or(Error::Invalid)?;
			// Calculate the offset in the section requested. cannot underflow, see $1
			let section_offset = (rva - it.VirtualAddress) as usize;
			return match section_bytes.get(section_offset..) {
				Some(bytes) if bytes.len() >= min_size_of => Ok(bytes),
				// Identify the reason the slice fails. cannot underflow, see $1
				_ => Err(if min_size_of > (VirtualEnd - rva) as usize { Error::Bounds } else { Error::ZeroFill }),
			};
		}
	}
	Err(Error::Bounds)
}}
#[inline(never)]
unsafe fn slice_file(image: &[u8], rva: Rva, min_size_of: usize, align_of: usize) -> Result<&[u8]> { unsafe {
	if rva == 0 {
		return Err(Error::Null);
	}

	let bytes = range_file(image, rva, min_size_of)?;

	if !(bytes.as_ptr() as usize).aligned_to(align_of) {
		return Err(Error::Misaligned);
	}

	Ok(bytes)
}}
#[inline(never)]
unsafe fn read_file(image: &[u8], image_base: Va, va: Va, min_size_of: usize, align_of: usize) -> Result<&[u8]> { unsafe {
	let size_of_image = optional_header(image).SizeOfImage;

	if va == 0 {
		return Err(Error::Null);
	}
	if va < image_base || va - image_base >= size_of_image as Va {
		return Err(Error::Bounds);
	}

	let rva = (va - image_base) as Rva;
	let bytes = range_file(image, rva, min_size_of)?;

	if !(bytes.as_ptr() as usize).aligned_to(align_of) {
		return Err(Error::Misaligned);
	}

	Ok(bytes)
}}

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
	if !image.as_ptr().wrapping_add(dos.e_lfanew as usize).aligned_to(mem::align_of::<IMAGE_NT_HEADERS>()) {
		return Err(Error::Misaligned);
	}
	let nt = unsafe { &*(image.as_ptr().offset(dos.e_lfanew as isize) as *const IMAGE_NT_HEADERS) };
	// Verify the NT headers
	if nt.Signature != IMAGE_NT_HEADERS_SIGNATURE || !(nt.OptionalHeader.Magic == IMAGE_NT_OPTIONAL_HDR32_MAGIC || nt.OptionalHeader.Magic == IMAGE_NT_OPTIONAL_HDR64_MAGIC) {
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

	// Verify that the standard data directories exposed by this parser are physically present.
	// Pelite follows the standard table size here; malformed but loadable images may advertise
	// a larger count without storing those extra entries.
	let num_rva_sizes = cmp::min(nt.OptionalHeader.NumberOfRvaAndSizes as usize, IMAGE_NUMBEROF_DIRECTORY_ENTRIES);
	let size_of_data_dir = mem::size_of::<IMAGE_DATA_DIRECTORY>().checked_mul(num_rva_sizes).ok_or(Error::Overflow)?;
	if nt_end.checked_add(size_of_data_dir).ok_or(Error::Overflow)? > image.len() {
		return Err(Error::Bounds);
	}

	// Verify the section headers
	if nt.FileHeader.NumberOfSections > 96 {
		return Err(Error::Insanity);
	}
	// u16 * sizeof(T) casted to usize, cannot reasonably overflow
	let size_of_sections = nt.FileHeader.NumberOfSections as usize * mem::size_of::<IMAGE_SECTION_HEADER>();
	// e_lfanew is checked for reasonable values, the others then cannot reasonably cause overflow
	#[rustfmt::skip]
	let start_of_sections = dos.e_lfanew as usize
		+ (mem::size_of::<IMAGE_NT_HEADERS>() - mem::size_of::<IMAGE_OPTIONAL_HEADER>())
		+ nt.FileHeader.SizeOfOptionalHeader as usize;
	if !image.as_ptr().wrapping_add(start_of_sections).aligned_to(mem::align_of::<IMAGE_SECTION_HEADER>()) {
		return Err(Error::Misaligned);
	}
	// then the sum of these cannot reasonably overflow
	if size_of_sections + start_of_sections > image.len() {
		return Err(Error::Bounds);
	}
	Ok(nt.OptionalHeader.SizeOfImage)
}
