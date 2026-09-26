use super::*;

//----------------------------------------------------------------

/// Section headers.
#[repr(transparent)]
pub struct PeSectionHeaders([image::IMAGE_SECTION_HEADER]);

impl PeSectionHeaders {
	pub(crate) fn new(image: &[image::IMAGE_SECTION_HEADER]) -> &PeSectionHeaders {
		unsafe { mem::transmute(image) }
	}
	/// Returns the underlying slice of section headers.
	#[inline]
	pub fn image(&self) -> &[image::IMAGE_SECTION_HEADER] {
		&self.0
	}
	/// Returns an iterator over the [`image::IMAGE_SECTION_HEADER`] elements.
	#[inline]
	pub fn iter(&self) -> slice::Iter<'_, image::IMAGE_SECTION_HEADER> {
		self.image().iter()
	}
	/// Finds a section header by its name.
	#[inline]
	pub fn by_name<S: ?Sized + AsRef<[u8]>>(&self, name: &S) -> Option<&image::IMAGE_SECTION_HEADER> {
		// Names have a max length, if larger they will never match
		let name = name.as_ref();
		if name.len() > image::IMAGE_SIZEOF_SHORT_NAME {
			return None;
		}
		// Copy the prefix into a new buffer for easy comparison
		let mut name_buf = [0u8; image::IMAGE_SIZEOF_SHORT_NAME];
		for i in 0..name.len() {
			name_buf[i] = name[i];
		}
		for sect in self.iter() {
			if sect.Name == name_buf {
				return Some(sect);
			}
		}
		None
	}
	/// Finds a section header by its RVA.
	#[inline]
	pub fn by_rva(&self, rva: u32) -> Option<&image::IMAGE_SECTION_HEADER> {
		for sect in self.iter() {
			// FIXME! Should this round up the VirtualSize to the next virtual section alignment?
			if rva >= sect.VirtualAddress && rva < u32::wrapping_add(sect.VirtualAddress, sect.VirtualSize) {
				return Some(sect);
			}
		}
		None
	}
}

unsafe impl Pod for PeSectionHeaders {}

impl<'a> IntoIterator for &'a PeSectionHeaders {
	type Item = &'a image::IMAGE_SECTION_HEADER;
	type IntoIter = slice::Iter<'a, image::IMAGE_SECTION_HEADER>;
	fn into_iter(self) -> Self::IntoIter {
		self.image().into_iter()
	}
}

impl fmt::Debug for PeSectionHeaders {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.image().fmt(f)
	}
}

//----------------------------------------------------------------

#[cfg(feature = "serde")]
pub(crate) fn serialize_name<S: serde::ser::Serializer>(name: &[u8; image::IMAGE_SIZEOF_SHORT_NAME], serializer: S) -> core::result::Result<S::Ok, S::Error> {
	if !serializer.is_human_readable() {
		return serializer.serialize_bytes(name);
	}
	match crate::util::parsen(name) {
		Ok(name) => serializer.serialize_str(name),
		Err(name) => serializer.serialize_bytes(name),
	}
}

serde_impl! {
	impl serde::Serialize for PeSectionHeaders {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			serializer.collect_seq(self.iter())
		}
	}
}
