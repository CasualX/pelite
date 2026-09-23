/*!
PE file.
*/

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
	/// * [`Bounds`][crate::Error::Bounds]:
	///   The byte slice is too small to fit the PE headers.
	///
	/// * [`Misaligned`][crate::Error::Misaligned]:
	///   The minimum alignment of 4 is not satisfied.
	///
	/// * [`BadMagic`][crate::Error::BadMagic]:
	///   This is not a PE file.
	///
	/// * [`PeMagic`][crate::Error::PeMagic]:
	///   Trying to parse a PE32 file with the PE32+ parser and vice versa.
	///
	/// * [`Insanity`][crate::Error::Insanity]:
	///   Reasonable limits on `e_lfanew`, `SizeOfHeaders` or `NumberOfSections` are exceeded.
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

//----------------------------------------------------------------

unsafe impl<'a> Pe<'a> for PeFile<'a> {}

unsafe impl<'a> PeObject<'a> for PeFile<'a> {
	fn image(&self) -> &'a [u8] {
		self.image
	}
	fn layout(&self) -> PeLayout {
		PeLayout::File
	}

	fn image_base(&self) -> super::Va {
		self.optional_header().ImageBase.into()
	}

	#[cfg(feature = "serde")]
	fn serde_name(&self) -> &'static str {
		"PeFile"
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
