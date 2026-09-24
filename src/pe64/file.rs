use super::*;

/// View into an unmapped PE file.
#[derive(Copy, Clone)]
pub struct PeFile<'a> {
	image: &'a [u8],
}

impl<'a> PeFile<'a> {
	/// Constructs a file view from a byte slice.
	///
	/// # Errors
	///
	/// * [`Bounds`][crate::Error::Bounds]: The image is too small for its declared headers or directories.
	/// * [`Misaligned`][crate::Error::Misaligned]: The image or its headers are misaligned.
	/// * [`BadMagic`][crate::Error::BadMagic]: The DOS or PE signature is invalid.
	/// * [`PeMagic`][crate::Error::PeMagic]: The image is PE32, but this parser expects PE32+.
	/// * [`Insanity`][crate::Error::Insanity]: Reasonable limits on header fields are exceeded.
	/// * [`Overflow`][crate::Error::Overflow]: A header offset or directory size overflows.
	pub fn from_bytes<T: AsRef<[u8]> + ?Sized>(image: &'a T) -> Result<PeFile<'a>> {
		let image = image.as_ref();
		let _ = validate_headers(image)?;
		Ok(PeFile { image })
	}
	/// Converts the file to section alignment.
	pub fn to_view(self) -> crate::PeMemory {
		let opt = self.optional_header();

		// Zero fill the underlying image
		let mut buf = crate::PeMemory::zeroed(opt.SizeOfImage as usize);

		// Start by copying the headers
		let image = self.image();
		buf.view_mut().write(0, &image[..opt.SizeOfHeaders as usize]);

		// Copy the section file data
		for section in self.section_headers() {
			if !opt.SectionAlignment.is_power_of_two() {
				continue;
			}
			let virtual_size = section.VirtualSize.align_to(opt.SectionAlignment);
			let copy_size = cmp::min(virtual_size, section.SizeOfRawData);
			let dest = buf.get_mut(section.VirtualAddress as usize..u32::wrapping_add(section.VirtualAddress, copy_size) as usize);
			let src = image.get(section.PointerToRawData as usize..u32::wrapping_add(section.PointerToRawData, copy_size) as usize);
			// Skip sections whose declared ranges do not fit...
			if let (Some(dest), Some(src)) = (dest, src) {
				dest.copy_from_slice(src);
			}
		}

		buf
	}
}

impl<'a> PeFile<'a> {
	/// Returns the image as a byte slice.
	pub fn image(&self) -> &'a [u8] {
		self.image
	}

	#[doc = include_str!("../docs/image_base.md")]
	#[inline]
	pub fn image_base(&self) -> Va {
		self.optional_header().ImageBase.into()
	}

	#[doc = include_str!("../docs/get_section_bytes.md")]
	pub fn get_section_bytes(&self, section_header: &IMAGE_SECTION_HEADER) -> Result<&'a [u8]> {
		if section_header.PointerToRawData == 0 {
			return Err(Error::Null);
		}
		let start = section_header.PointerToRawData as usize;
		let end = section_header.PointerToRawData.wrapping_add(section_header.SizeOfRawData) as usize;
		self.image.get(start..end).ok_or(Error::Bounds)
	}

	#[doc = include_str!("../docs/rva_to_va.md")]
	#[inline]
	pub fn rva_to_va(&self, rva: Rva) -> Result<Va> {
		unsafe { pe_impl::rva_to_va(self.image, self.image_base(), rva) }
	}
	#[doc = include_str!("../docs/va_to_rva.md")]
	#[inline]
	pub fn va_to_rva(&self, va: Va) -> Result<Rva> {
		unsafe { pe_impl::va_to_rva(self.image, self.image_base(), va) }
	}

	#[doc = include_str!("../docs/offset_of.md")]
	#[inline]
	#[track_caller]
	pub fn offset_of<T: Pod + ?Sized>(self, symbol: &'a T) -> usize {
		let base = self.image.as_ptr().addr();
		let symbol = (symbol as *const T).addr();

		assert!(
			symbol >= base && symbol <= base + self.image.len(),
			"symbol does not refer to data within this image",
		);

		symbol - base
	}
}

//----------------------------------------------------------------

impl<'a> PeFile<'a> {
	#[doc = include_str!("../docs/slice.md")]
	#[inline]
	pub fn slice(&self, rva: Rva, min_size_of: usize, align: usize) -> Result<&'a [u8]> {
		unsafe { pe_impl::slice_file(self.image, rva, min_size_of, align) }
	}

	#[doc = include_str!("../docs/slice_bytes.md")]
	#[inline]
	pub fn slice_bytes(self, rva: Rva) -> Result<&'a [u8]> {
		self.slice(rva, 0, 1)
	}

	#[doc = include_str!("../docs/derva.md")]
	pub fn derva<T: Pod>(self, rva: Rva) -> Result<&'a T> {
		let align = mem::align_of::<T>();
		let bytes = self.slice(rva, mem::size_of::<T>(), align)?;
		// This is safe as per Pod bound, min_size_of and align
		unsafe {
			let p = &*(bytes.as_ptr() as *const T);
			Ok(p)
		}
	}
	#[doc = include_str!("../docs/derva_copy.md")]
	pub fn derva_copy<T: Copy + Pod>(self, rva: Rva) -> Result<T> {
		let bytes = self.slice(rva, mem::size_of::<T>(), 1)?;
		// This is safe as per Pod bound and min_size_of
		unsafe {
			let p = bytes.as_ptr() as *const T;
			Ok(ptr::read_unaligned(p))
		}
	}
	#[doc = include_str!("../docs/derva_into.md")]
	pub fn derva_into<T: ?Sized + Pod>(self, rva: Rva, dest: &mut T) -> Result<()> {
		let len = mem::size_of_val(dest);
		let bytes = self.slice(rva, len, 1)?;
		dataview::bytes_mut(dest).copy_from_slice(&bytes[..len]);
		Ok(())
	}
	#[doc = include_str!("../docs/derva_slice.md")]
	pub fn derva_slice<T: Pod>(self, rva: Rva, len: usize) -> Result<&'a [T]> {
		let min_size_of = mem::size_of::<T>().checked_mul(len).ok_or(Error::Overflow)?;
		let align = mem::align_of::<T>();
		let bytes = self.slice(rva, min_size_of, align)?;
		// This is safe as per Pod bound, min_size_of and align
		unsafe { Ok(slice::from_raw_parts(bytes.as_ptr() as *const T, len)) }
	}
	#[doc = include_str!("../docs/derva_slice_f.md")]
	pub fn derva_slice_f<T: Pod, F: FnMut(&'a T) -> bool>(self, rva: Rva, mut f: F) -> Result<&'a [T]> {
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
	pub fn derva_slice_s<T: PartialEq + Pod>(self, rva: Rva, sentinel: T) -> Result<&'a [T]> {
		self.derva_slice_f(rva, |tee| *tee == sentinel)
	}
	#[doc = include_str!("../docs/derva_c_str.md")]
	pub fn derva_c_str(self, rva: Rva) -> Result<&'a CStr> {
		self.derva_string(rva)
	}
	#[doc = include_str!("../docs/derva_string.md")]
	pub fn derva_string<T: FromBytes + ?Sized>(self, rva: Rva) -> Result<&'a T> {
		let bytes = self.slice(rva, T::MIN_SIZE_OF, T::ALIGN_OF)?;
		unsafe { T::from_bytes(bytes).ok_or(Error::Encoding) }
	}
}

//----------------------------------------------------------------
// Deref impls for `Ptr`s

impl<'a> PeFile<'a> {
	#[doc = include_str!("../docs/read.md")]
	#[inline]
	pub fn read(&self, va: Va, min_size_of: usize, align: usize) -> Result<&'a [u8]> {
		unsafe { pe_impl::read_file(self.image, self.image_base(), va, min_size_of, align) }
	}

	#[doc = include_str!("../docs/read_bytes.md")]
	#[inline]
	pub fn read_bytes(self, va: Va) -> Result<&'a [u8]> {
		self.read(va, 0, 1)
	}

	#[doc = include_str!("../docs/deref.md")]
	pub fn deref<T: Pod>(self, ptr: Ptr<T>) -> Result<&'a T> {
		let align = mem::align_of::<T>();
		let bytes = self.read(ptr.into(), mem::size_of::<T>(), align)?;
		// This is safe as per Pod bound, min_size_of and align
		unsafe {
			let p = &*(bytes.as_ptr() as *const T);
			Ok(p)
		}
	}
	#[doc = include_str!("../docs/deref_copy.md")]
	pub fn deref_copy<T: Copy + Pod>(self, ptr: Ptr<T>) -> Result<T> {
		let bytes = self.read(ptr.into(), mem::size_of::<T>(), 1)?;
		// This is safe as per Pod bound and min_size_of
		unsafe {
			let p = bytes.as_ptr() as *const T;
			Ok(ptr::read_unaligned(p))
		}
	}
	#[doc = include_str!("../docs/deref_into.md")]
	pub fn deref_into<T: ?Sized + Pod>(self, ptr: Ptr<T>, dest: &mut T) -> Result<()> {
		let len = mem::size_of_val(dest);
		let bytes = self.read(ptr.into(), len, 1)?;
		dataview::bytes_mut(dest).copy_from_slice(&bytes[..len]);
		Ok(())
	}
	#[doc = include_str!("../docs/deref_slice.md")]
	pub fn deref_slice<T: Pod>(self, ptr: Ptr<[T]>, len: usize) -> Result<&'a [T]> {
		let min_size_of = mem::size_of::<T>().checked_mul(len).ok_or(Error::Overflow)?;
		let align = mem::align_of::<T>();
		let bytes = self.read(ptr.into(), min_size_of, align)?;
		// This is safe as per Pod bound, min_size_of and align
		unsafe { Ok(slice::from_raw_parts(bytes.as_ptr() as *const T, len)) }
	}
	#[doc = include_str!("../docs/deref_slice_f.md")]
	pub fn deref_slice_f<T: Pod, F: FnMut(&'a T) -> bool>(self, ptr: Ptr<[T]>, mut f: F) -> Result<&'a [T]> {
		const {
			assert!(mem::size_of::<T>() > 0, "cannot scan an array of zero-sized elements");
		};

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
	pub fn deref_slice_s<T: PartialEq + Pod>(self, ptr: Ptr<[T]>, sentinel: T) -> Result<&'a [T]> {
		self.deref_slice_f(ptr, |tee| *tee == sentinel)
	}
	#[doc = include_str!("../docs/deref_c_str.md")]
	pub fn deref_c_str(self, ptr: Ptr<CStr>) -> Result<&'a CStr> {
		self.deref_string(ptr)
	}
	#[doc = include_str!("../docs/deref_string.md")]
	pub fn deref_string<T: FromBytes + ?Sized>(self, ptr: Ptr<T>) -> Result<&'a T> {
		let bytes = self.read(ptr.into(), T::MIN_SIZE_OF, T::ALIGN_OF)?;
		unsafe { T::from_bytes(bytes).ok_or(Error::Encoding) }
	}
}

//----------------------------------------------------------------

unsafe impl<'a> Pe<'a> for PeFile<'a> {
	#[inline]
	fn image(&self) -> &'a [u8] {
		self.image
	}
	#[inline]
	fn layout(&self) -> PeLayout {
		PeLayout::File
	}

	#[inline]
	fn image_base(&self) -> super::Va {
		self.image_base()
	}

	#[cfg(feature = "serde")]
	fn serde_name(&self) -> &'static str {
		"PeFile"
	}

	#[inline]
	fn get_section_bytes(&self, section_header: &IMAGE_SECTION_HEADER) -> Result<&'a [u8]> {
		self.get_section_bytes(section_header)
	}
	#[inline]
	fn slice(&self, rva: Rva, min_size_of: usize, align: usize) -> Result<&'a [u8]> {
		self.slice(rva, min_size_of, align)
	}
	#[inline]
	fn read(&self, va: Va, min_size_of: usize, align: usize) -> Result<&'a [u8]> {
		self.read(va, min_size_of, align)
	}
	#[inline]
	fn rva_to_va(&self, rva: Rva) -> Result<Va> {
		self.rva_to_va(rva)
	}
	#[inline]
	fn va_to_rva(&self, va: Va) -> Result<Rva> {
		self.va_to_rva(va)
	}
}

//----------------------------------------------------------------

#[cfg(feature = "serde")]
impl<'a> serde::Serialize for PeFile<'a> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> {
		super::pe::serialize_pe(*self, serializer)
	}
}

//----------------------------------------------------------------

#[test]
fn from_byte_slice() {
	assert!(matches!(PeFile::from_bytes(&[]), Err(crate::Error::Bounds)));
}
