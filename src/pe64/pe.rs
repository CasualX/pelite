use super::*;

//----------------------------------------------------------------

pub use crate::wrap::PeLayout;

/// Read-only access to a validated PE image.
///
/// Basic properties shared by file-aligned and mapped PE images.
///
/// # Safety
///
/// Implementors must return a valid PE image for the full lifetime `'a`,
/// and report layout and base-address information matching that image.
///
/// The image supplied by must contain valid headers for this
/// architecture so the unchecked header accessors are sound.
pub unsafe trait Pe<'a> {
	/// Returns the image as a byte slice.
	fn image(&self) -> &'a [u8];

	/// Returns whether the image uses file or mapped section layout.
	fn layout(&self) -> PeLayout;

	#[doc = include_str!("../docs/image_base.md")]
	fn image_base(&self) -> Va;

	// Give a struct name in Serialize implementation
	#[cfg(feature = "serde")]
	#[doc(hidden)]
	fn serde_name(&self) -> &'static str;

	#[doc = include_str!("../docs/get_section_bytes.md")]
	fn get_section_bytes(&self, section_header: &IMAGE_SECTION_HEADER) -> Result<&'a [u8]>;

	#[doc = include_str!("../docs/slice.md")]
	fn slice(&self, rva: Rva, min_size_of: usize, align: usize) -> Result<&'a [u8]>;

	#[doc = include_str!("../docs/read.md")]
	fn read(&self, va: Va, min_size_of: usize, align: usize) -> Result<&'a [u8]>;

	#[doc = include_str!("../docs/rva_to_va.md")]
	fn rva_to_va(&self, rva: Rva) -> Result<Va>;

	#[doc = include_str!("../docs/va_to_rva.md")]
	fn va_to_rva(&self, va: Va) -> Result<Rva>;

	//----------------------------------------------------------------

	/// Returns the DOS header.
	fn dos_header(self) -> &'a IMAGE_DOS_HEADER where Self: Copy {
		unsafe { pe_impl::dos_header(self.image()) }
	}
	#[doc = include_str!("../docs/dos_image.md")]
	fn dos_image(self) -> &'a [u8] where Self: Copy {
		unsafe { pe_impl::dos_image(self.image()) }
	}
	/// Returns the NT headers.
	fn nt_headers(self) -> &'a IMAGE_NT_HEADERS where Self: Copy {
		unsafe { pe_impl::nt_headers(self.image()) }
	}
	/// Returns the file header.
	fn file_header(self) -> &'a IMAGE_FILE_HEADER where Self: Copy {
		unsafe { pe_impl::file_header(self.image()) }
	}
	/// Returns the optional header.
	fn optional_header(self) -> &'a IMAGE_OPTIONAL_HEADER where Self: Copy {
		unsafe { pe_impl::optional_header(self.image()) }
	}
	/// Returns the data directory.
	fn data_directory(self) -> &'a [IMAGE_DATA_DIRECTORY] where Self: Copy {
		unsafe { pe_impl::data_directory(self.image()) }
	}
	/// Returns the section headers.
	fn section_headers(self) -> &'a PeSectionHeaders where Self: Copy {
		unsafe { pe_impl::section_headers(self.image()) }
	}
	/// Returns the headers.
	fn headers(self) -> PeHeaders<Self> where Self: Copy {
		PeHeaders::new(self)
	}

	//----------------------------------------------------------------

	#[doc = include_str!("../docs/offset_of.md")]
	#[inline]
	#[track_caller]
	fn offset_of<T: Pod + ?Sized>(self, symbol: &'a T) -> usize where Self: Copy {
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

	#[doc = include_str!("../docs/slice_bytes.md")]
	#[inline]
	fn slice_bytes(self, rva: Rva) -> Result<&'a [u8]> where Self: Copy {
		self.slice(rva, 0, 1)
	}

	#[doc = include_str!("../docs/read_bytes.md")]
	#[inline]
	fn read_bytes(self, va: Va) -> Result<&'a [u8]> where Self: Copy {
		self.read(va, 0, 1)
	}

	//----------------------------------------------------------------

	#[doc = include_str!("../docs/derva.md")]
	fn derva<T: Pod>(self, rva: Rva) -> Result<&'a T> where Self: Copy {
		let align = mem::align_of::<T>();
		let bytes = self.slice(rva, mem::size_of::<T>(), align)?;
		// This is safe as per Pod bound, min_size_of and align
		unsafe {
			let p = &*(bytes.as_ptr() as *const T);
			Ok(p)
		}
	}
	#[doc = include_str!("../docs/derva_copy.md")]
	fn derva_copy<T: Copy + Pod>(self, rva: Rva) -> Result<T> where Self: Copy {
		let bytes = self.slice(rva, mem::size_of::<T>(), 1)?;
		// This is safe as per Pod bound and min_size_of
		unsafe {
			let p = bytes.as_ptr() as *const T;
			Ok(ptr::read_unaligned(p))
		}
	}
	#[doc = include_str!("../docs/derva_into.md")]
	fn derva_into<T: ?Sized + Pod>(self, rva: Rva, dest: &mut T) -> Result<()> where Self: Copy {
		let len = mem::size_of_val(dest);
		let bytes = self.slice(rva, len, 1)?;
		dataview::bytes_mut(dest).copy_from_slice(&bytes[..len]);
		Ok(())
	}
	#[doc = include_str!("../docs/derva_slice.md")]
	fn derva_slice<T: Pod>(self, rva: Rva, len: usize) -> Result<&'a [T]> where Self: Copy {
		let min_size_of = mem::size_of::<T>().checked_mul(len).ok_or(Error::Overflow)?;
		let align = mem::align_of::<T>();
		let bytes = self.slice(rva, min_size_of, align)?;
		// This is safe as per Pod bound, min_size_of and align
		unsafe { Ok(slice::from_raw_parts(bytes.as_ptr() as *const T, len)) }
	}
	#[doc = include_str!("../docs/derva_slice_f.md")]
	fn derva_slice_f<T: Pod, F: FnMut(&'a T) -> bool>(self, rva: Rva, mut f: F) -> Result<&'a [T]> where Self: Copy {
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
	#[doc = include_str!("../docs/derva_slice_s.md")]
	#[inline]
	fn derva_slice_s<T: PartialEq + Pod>(self, rva: Rva, sentinel: T) -> Result<&'a [T]> where Self: Copy {
		self.derva_slice_f(rva, |tee| *tee == sentinel)
	}
	#[doc = include_str!("../docs/derva_c_str.md")]
	fn derva_c_str(self, rva: Rva) -> Result<&'a CStr> where Self: Copy {
		self.derva_string(rva)
	}
	#[doc = include_str!("../docs/derva_string.md")]
	fn derva_string<T: FromBytes + ?Sized>(self, rva: Rva) -> Result<&'a T> where Self: Copy {
		let bytes = self.slice(rva, T::MIN_SIZE_OF, T::ALIGN_OF)?;
		unsafe { T::from_bytes(bytes).ok_or(Error::Encoding) }
	}

	//----------------------------------------------------------------
	// Deref impls for `Ptr`s

	#[doc = include_str!("../docs/deref.md")]
	fn deref<T: Pod>(self, ptr: Ptr<T>) -> Result<&'a T> where Self: Copy {
		let align = mem::align_of::<T>();
		let bytes = self.read(ptr.into(), mem::size_of::<T>(), align)?;
		// This is safe as per Pod bound, min_size_of and align
		unsafe {
			let p = &*(bytes.as_ptr() as *const T);
			Ok(p)
		}
	}
	#[doc = include_str!("../docs/deref_copy.md")]
	fn deref_copy<T: Copy + Pod>(self, ptr: Ptr<T>) -> Result<T> where Self: Copy {
		let bytes = self.read(ptr.into(), mem::size_of::<T>(), 1)?;
		// This is safe as per Pod bound and min_size_of
		unsafe {
			let p = bytes.as_ptr() as *const T;
			Ok(ptr::read_unaligned(p))
		}
	}
	#[doc = include_str!("../docs/deref_into.md")]
	fn deref_into<T: ?Sized + Pod>(self, ptr: Ptr<T>, dest: &mut T) -> Result<()> where Self: Copy {
		let len = mem::size_of_val(dest);
		let bytes = self.read(ptr.into(), len, 1)?;
		dataview::bytes_mut(dest).copy_from_slice(&bytes[..len]);
		Ok(())
	}
	#[doc = include_str!("../docs/deref_slice.md")]
	fn deref_slice<T: Pod>(self, ptr: Ptr<[T]>, len: usize) -> Result<&'a [T]> where Self: Copy {
		let min_size_of = mem::size_of::<T>().checked_mul(len).ok_or(Error::Overflow)?;
		let align = mem::align_of::<T>();
		let bytes = self.read(ptr.into(), min_size_of, align)?;
		// This is safe as per Pod bound, min_size_of and align
		unsafe { Ok(slice::from_raw_parts(bytes.as_ptr() as *const T, len)) }
	}
	#[doc = include_str!("../docs/deref_slice_f.md")]
	fn deref_slice_f<T: Pod, F: FnMut(&'a T) -> bool>(self, ptr: Ptr<[T]>, mut f: F) -> Result<&'a [T]> where Self: Copy {
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
	#[doc = include_str!("../docs/deref_slice_s.md")]
	#[inline]
	fn deref_slice_s<T: PartialEq + Pod>(self, ptr: Ptr<[T]>, sentinel: T) -> Result<&'a [T]> where Self: Copy {
		self.deref_slice_f(ptr, |tee| *tee == sentinel)
	}
	#[doc = include_str!("../docs/deref_c_str.md")]
	fn deref_c_str(self, ptr: Ptr<CStr>) -> Result<&'a CStr> where Self: Copy {
		self.deref_string(ptr)
	}
	#[doc = include_str!("../docs/deref_string.md")]
	fn deref_string<T: FromBytes + ?Sized>(self, ptr: Ptr<T>) -> Result<&'a T> where Self: Copy {
		let bytes = self.read(ptr.into(), T::MIN_SIZE_OF, T::ALIGN_OF)?;
		unsafe { T::from_bytes(bytes).ok_or(Error::Encoding) }
	}

	//----------------------------------------------------------------

	#[doc = include_str!("../docs/rich_structure.md")]
	fn rich_structure(self) -> Result<crate::rich_structure::RichStructure<'a>> where Self: Copy {
		let image = self.image();
		let image = unsafe { slice::from_raw_parts(image.as_ptr() as *const u32, image.len() / 4) };
		crate::rich_structure::RichStructure::try_from(image)
	}

	#[doc = include_str!("../docs/exports.md")]
	#[inline]
	fn exports(self) -> Result<ExportDirectory<'a, Self>> where Self: Copy {
		ExportDirectory::try_from(self)
	}

	#[doc = include_str!("../docs/get_proc_address.md")]
	#[inline(never)]
	fn get_proc_address<T>(self, name: T) -> Result<Va> where Self: Copy + GetProcAddress<'a, T> {
		let export = <Self as GetProcAddress<'a, T>>::get_export(self, name)?;
		self.rva_to_va(export.symbol().ok_or(Error::Null)?)
	}

	#[doc = include_str!("../docs/imports.md")]
	#[inline]
	fn imports(self) -> Result<ImportDirectory<'a, Self>> where Self: Copy {
		ImportDirectory::try_from(self)
	}

	#[doc = include_str!("../docs/iat.md")]
	#[inline]
	fn iat(self) -> Result<ImportAddressTable<'a, Self>> where Self: Copy {
		ImportAddressTable::try_from(self)
	}

	#[doc = include_str!("../docs/base_relocs.md")]
	#[inline]
	fn base_relocs(self) -> Result<crate::base_relocs::BaseRelocationDirectory<'a>> where Self: Copy {
		base_relocs::try_from(self)
	}

	#[doc = include_str!("../docs/load_config.md")]
	#[inline]
	fn load_config(self) -> Result<LoadConfigDirectory<'a, Self>> where Self: Copy {
		LoadConfigDirectory::try_from(self)
	}

	#[doc = include_str!("../docs/tls.md")]
	#[inline]
	fn tls(self) -> Result<TlsDirectory<'a, Self>> where Self: Copy {
		TlsDirectory::try_from(self)
	}

	#[doc = include_str!("../docs/security.md")]
	#[inline]
	fn security(self) -> Result<crate::security::SecurityDirectory<'a>> where Self: Copy {
		security::try_from(self)
	}

	branch! {
		pe32 {}
		pe64 {
			#[doc = include_str!("../docs/exception_x64.md")]
			fn exception_x64(self) -> Result<X64ExceptionDirectory<'a, Self>> where Self: Copy {
				X64ExceptionDirectory::try_from(self)
			}

			#[doc = include_str!("../docs/exception_arm64.md")]
			fn exception_arm64(self) -> Result<Arm64ExceptionDirectory<'a, Self>> where Self: Copy {
				Arm64ExceptionDirectory::try_from(self)
			}
		}
	}

	#[doc = include_str!("../docs/debug.md")]
	#[inline]
	fn debug(self) -> Result<crate::debug::DebugDirectory<'a>> where Self: Copy {
		debug::try_from(self)
	}

	#[doc = include_str!("../docs/resources.md")]
	fn resources(self) -> Result<crate::resources::ResourceDirectory<'a>> where Self: Copy {
		let datadir = self.data_directory().get(IMAGE_DIRECTORY_ENTRY_RESOURCE).ok_or(Error::Bounds)?;
		if datadir.VirtualAddress == 0 {
			return Err(Error::Null);
		}
		let bytes = self.slice_bytes(datadir.VirtualAddress)?;
		let size = cmp::min(datadir.Size as usize, bytes.len());
		Ok(crate::resources::ResourceDirectory::new(&bytes[..size], datadir))
	}

	/// Returns Scanner access.
	fn scanner(self) -> Scanner<Self> where Self: Copy {
		Scanner::new(self)
	}
}

//----------------------------------------------------------------
// Make `&Pe<'a>` trait objects work seamlessly.

unsafe impl<'s, 'a> Pe<'a> for &'s dyn Pe<'a> {
	#[inline]
	fn image(&self) -> &'a [u8] {
		Pe::image(*self)
	}
	#[inline]
	fn layout(&self) -> PeLayout {
		Pe::layout(*self)
	}
	#[inline]
	fn image_base(&self) -> Va {
		Pe::image_base(*self)
	}

	#[cfg(feature = "serde")]
	#[inline]
	fn serde_name(&self) -> &'static str {
		Pe::serde_name(*self)
	}

	#[inline]
	fn get_section_bytes(&self, section_header: &IMAGE_SECTION_HEADER) -> Result<&'a [u8]> {
		Pe::get_section_bytes(*self, section_header)
	}
	#[inline]
	fn slice(&self, rva: Rva, min_size_of: usize, align: usize) -> Result<&'a [u8]> {
		Pe::slice(*self, rva, min_size_of, align)
	}
	#[inline]
	fn read(&self, va: Va, min_size_of: usize, align: usize) -> Result<&'a [u8]> {
		Pe::read(*self, va, min_size_of, align)
	}
	#[inline]
	fn rva_to_va(&self, rva: Rva) -> Result<Va> {
		Pe::rva_to_va(*self, rva)
	}
	#[inline]
	fn va_to_rva(&self, va: Va) -> Result<Rva> {
		Pe::va_to_rva(*self, va)
	}
}

//----------------------------------------------------------------

#[cfg(feature = "serde")]
pub(crate) fn serialize_pe<'a, P: Copy + Pe<'a>, S: serde::Serializer>(pe: P, serializer: S) -> core::result::Result<S::Ok, S::Error> {
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
