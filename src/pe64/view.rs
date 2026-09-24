use super::*;

/// View into a mapped PE image.
#[derive(Copy, Clone)]
pub struct PeView<'a> {
	image: &'a [u8],
	base_address: Va,
}

current_target! {
	impl PeView<'static> {
		/// Constructs a view of the module this code is executing in.
		#[inline]
		pub unsafe fn new() -> PeView<'static> {
			unsafe { Self::module(image_base() as *const _ as *const u8) }
		}
	}
}
impl<'a> PeView<'a> {
	/// Constructs a view from a byte slice.
	///
	/// The base address initially comes from the optional header.
	/// For a relocated image, use [`PeView::set_base_address`] or construct the view with [`PeView::module`].
	///
	/// # Errors
	///
	/// * [`Bounds`][crate::Error::Bounds]: The image is too small for its declared headers or directories.
	/// * [`Misaligned`][crate::Error::Misaligned]: The image or its headers are misaligned.
	/// * [`BadMagic`][crate::Error::BadMagic]: The DOS or PE signature is invalid.
	/// * [`PeMagic`][crate::Error::PeMagic]: The image is PE32, but this parser expects PE32+.
	/// * [`Insanity`][crate::Error::Insanity]: Reasonable limits on header fields are exceeded.
	/// * [`Overflow`][crate::Error::Overflow]: A header offset or directory size overflows.
	#[inline]
	pub fn from_bytes<T: AsRef<[u8]> + ?Sized>(image: &'a T) -> Result<PeView<'a>> {
		let image = image.as_ref();
		let _ = validate_headers(image)?;
		let base_address = unsafe { optional_header(image).ImageBase.into() };
		Ok(PeView { image, base_address })
	}

	/// Returns a new `PeView` instance with the provided base address.
	#[inline]
	#[must_use]
	pub fn set_base_address(self, base_address: Va) -> PeView<'a> {
		let PeView { image, .. } = self;
		PeView { image, base_address }
	}

	/// Constructs a new view from module handle.
	///
	/// # Safety
	///
	/// The underlying memory is borrowed and an unbounded lifetime is returned.
	/// Ensure the lifetime outlives this view instance!
	///
	/// No sanity or safety checks are done to make sure this is really PE32(+) image.
	/// When using this with a `HMODULE` from the system the caller must be sure this is a PE32(+) image.
	#[inline]
	pub unsafe fn module(base: *const u8) -> PeView<'a> { unsafe {
		let dos = &*(base as *const IMAGE_DOS_HEADER);
		let nt = &*(base.offset(dos.e_lfanew as isize) as *const IMAGE_NT_HEADERS);
		PeView {
			image: slice::from_raw_parts(base, nt.OptionalHeader.SizeOfImage as usize),
			base_address: base as Va,
		}}
	}
	/// Converts the view to file alignment.
	pub fn to_file(self) -> crate::PeMemory {
		let opt = self.optional_header();

		// Figure out the size of the file image
		let mut file_size = opt.SizeOfHeaders;
		for section in self.section_headers() {
			file_size = cmp::max(file_size, u32::wrapping_add(section.PointerToRawData, section.SizeOfRawData));
		}
		// Clamp to the actual image size...
		file_size = cmp::min(file_size, opt.SizeOfImage);

		// Zero fill the underlying file
		let mut buf = crate::PeMemory::zeroed(file_size as usize);

		// Start by copying the headers
		let image = self.image();
		buf.view_mut().write(0, &image[..opt.SizeOfHeaders as usize]);

		// Copy the section image data
		for section in self.section_headers() {
			let dest = buf.get_mut(section.PointerToRawData as usize..u32::wrapping_add(section.PointerToRawData, section.SizeOfRawData) as usize);
			let src = image.get(section.VirtualAddress as usize..u32::wrapping_add(section.VirtualAddress, section.SizeOfRawData) as usize);
			// Skip invalid sections...
			if let (Some(dest), Some(src)) = (dest, src) {
				dest.copy_from_slice(src);
			}
		}

		buf
	}
}

impl<'a> PeView<'a> {
	/// Returns the image as a byte slice.
	#[inline]
	pub fn image(&self) -> &'a [u8] {
		return self.image
	}

	#[doc = include_str!("../docs/image_base.md")]
	#[inline]
	pub fn image_base(&self) -> Va {
		self.base_address
	}

	#[doc = include_str!("../docs/get_section_bytes.md")]
	fn get_section_bytes(&self, section_header: &IMAGE_SECTION_HEADER) -> Result<&'a [u8]> {
		if section_header.VirtualAddress == 0 {
			return Err(Error::Null);
		}
		let start = section_header.VirtualAddress as usize;
		let end = section_header.VirtualAddress.wrapping_add(section_header.VirtualSize) as usize;
		self.image.get(start..end).ok_or(Error::Bounds)
	}

	#[doc = include_str!("../docs/rva_to_va.md")]
	#[inline]
	pub fn rva_to_va(&self, rva: Rva) -> Result<Va> {
		unsafe { pe_impl::rva_to_va(self.image, self.base_address, rva) }
	}
	#[doc = include_str!("../docs/va_to_rva.md")]
	#[inline]
	pub fn va_to_rva(&self, va: Va) -> Result<Rva> {
		unsafe { pe_impl::va_to_rva(self.image, self.base_address, va) }
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

impl<'a> PeView<'a> {
	#[doc = include_str!("../docs/slice.md")]
	#[inline]
	pub fn slice(&self, rva: Rva, min_size_of: usize, align: usize) -> Result<&'a [u8]> {
		unsafe { pe_impl::slice_section(self.image, rva, min_size_of, align) }
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

impl<'a> PeView<'a> {
	#[doc = include_str!("../docs/read.md")]
	#[inline]
	pub fn read(&self, va: Va, min_size_of: usize, align: usize) -> Result<&'a [u8]> {
		unsafe { pe_impl::read_section(self.image, self.base_address, va, min_size_of, align) }
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

unsafe impl<'a> Pe<'a> for PeView<'a> {
	#[inline]
	fn image(&self) -> &'a [u8] {
		self.image
	}
	#[inline]
	fn layout(&self) -> PeLayout {
		PeLayout::Section
	}
	#[inline]
	fn image_base(&self) -> Va {
		self.base_address
	}

	#[cfg(feature = "serde")]
	fn serde_name(&self) -> &'static str {
		"PeView"
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
impl<'a> serde::Serialize for PeView<'a> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error> {
		super::pe::serialize_pe(*self, serializer)
	}
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use core::mem;

	use crate::Error;

	use super::{IMAGE_DOS_SIGNATURE, IMAGE_NT_HEADERS, PeView};

	#[test]
	fn from_byte_slice() {
		assert!(matches!(PeView::from_bytes(&[]), Err(Error::Bounds)));
	}

	#[test]
	fn misaligned_nt_headers() {
		if mem::align_of::<IMAGE_NT_HEADERS>() <= 4 {
			return;
		}

		#[repr(align(8))]
		struct Aligned([u8; 256]);

		let mut image = Aligned([0; 256]);
		image.0[..2].copy_from_slice(&IMAGE_DOS_SIGNATURE.to_le_bytes());
		image.0[60..64].copy_from_slice(&68u32.to_le_bytes());

		assert!(matches!(PeView::from_bytes(&image.0), Err(Error::Misaligned)));
	}
}
