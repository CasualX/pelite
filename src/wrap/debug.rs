use super::*;

/// Debug directory.
impl<'a, Pe32: pe32::Pe<'a>, Pe64: pe64::Pe<'a>> Wrap<pe32::DebugDirectory<'a, Pe32>, pe64::DebugDirectory<'a, Pe64>> {
	/// Gets the PE instance.
	#[inline]
	pub fn pe(&self) -> Wrap<Pe32, Pe64> {
		match self {
			Wrap::T32(debug) => Wrap::T32(debug.pe()),
			Wrap::T64(debug) => Wrap::T64(debug.pe()),
		}
	}
	/// Returns the underlying debug directories image.
	#[inline]
	pub fn image(&self) -> &'a [image::IMAGE_DEBUG_DIRECTORY] {
		match self {
			Wrap::T32(debug) => debug.image(),
			Wrap::T64(debug) => debug.image(),
		}
	}
	/// Gets the CodeView PDB file name.
	#[inline]
	pub fn pdb_file_name(&self) -> Option<&'a CStr> {
		match self {
			Wrap::T32(debug) => debug.pdb_file_name(),
			Wrap::T64(debug) => debug.pdb_file_name(),
		}
	}
	/// Iterator over the debug directories.
	#[inline]
	pub fn iter(&self) -> Wrap<pe32::DebugDirectoryIter<'a, Pe32>, pe64::DebugDirectoryIter<'a, Pe64>> {
		match self {
			Wrap::T32(debug) => Wrap::T32(debug.iter()),
			Wrap::T64(debug) => Wrap::T64(debug.iter()),
		}
	}
}

impl<'a, Pe32: pe32::Pe<'a>, Pe64: pe64::Pe<'a>> IntoIterator for Wrap<pe32::DebugDirectory<'a, Pe32>, pe64::DebugDirectory<'a, Pe64>> {
	type Item = Wrap<pe32::DebugDirectoryEntry<'a, Pe32>, pe64::DebugDirectoryEntry<'a, Pe64>>;
	type IntoIter = Wrap<pe32::DebugDirectoryIter<'a, Pe32>, pe64::DebugDirectoryIter<'a, Pe64>>;
	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

/// Debug directory entry.
impl<'a, Pe32: pe32::Pe<'a>, Pe64: pe64::Pe<'a>> Wrap<pe32::DebugDirectoryEntry<'a, Pe32>, pe64::DebugDirectoryEntry<'a, Pe64>> {
	/// Gets the PE instance.
	#[inline]
	pub fn pe(&self) -> Wrap<Pe32, Pe64> {
		match self {
			Wrap::T32(dir) => Wrap::T32(dir.pe()),
			Wrap::T64(dir) => Wrap::T64(dir.pe()),
		}
	}
	/// Gets the underlying debug directory image.
	#[inline]
	pub fn image(&self) -> &'a image::IMAGE_DEBUG_DIRECTORY {
		match self {
			Wrap::T32(dir) => dir.image(),
			Wrap::T64(dir) => dir.image(),
		}
	}
	/// Gets the raw data of this debug directory entry.
	#[inline]
	pub fn data(&self) -> Option<&'a [u8]> {
		match self {
			Wrap::T32(dir) => dir.data(),
			Wrap::T64(dir) => dir.data(),
		}
	}
	/// Interprets the directory entry.
	#[inline]
	pub fn entry(&self) -> Result<DebugData<'a>> {
		match self {
			Wrap::T32(dir) => dir.entry(),
			Wrap::T64(dir) => dir.entry(),
		}
	}
}

//----------------------------------------------------------------

/// Decoded contents of a debug directory entry.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum DebugData<'a> {
	/// CodeView information, usually including a PDB path.
	CodeView(CodeView<'a>),
	/// Miscellaneous debug information.
	Misc(DebugMisc<'a>),
	/// Profile-guided optimization information.
	Pgo(Pgo<'a>),
	/// An unrecognized debug entry and its available raw bytes.
	Unknown(Option<&'a [u8]>),
}
impl<'a> DebugData<'a> {
	/// As a CodeView debug information entry.
	pub fn as_code_view(self) -> Option<CodeView<'a>> {
		match self {
			DebugData::CodeView(cv) => Some(cv),
			_ => None,
		}
	}
	/// As a miscellaneous debug information entry.
	pub fn as_misc(self) -> Option<DebugMisc<'a>> {
		match self {
			DebugData::Misc(misc) => Some(misc),
			_ => None,
		}
	}
	/// As a PGO information entry.
	pub fn as_pgo(self) -> Option<Pgo<'a>> {
		match self {
			DebugData::Pgo(pgo) => Some(pgo),
			_ => None,
		}
	}
	/// Unknown format, return as bytes.
	pub fn as_unknown(self) -> Option<&'a [u8]> {
		match self {
			DebugData::Unknown(data) => data,
			_ => None,
		}
	}
}

//----------------------------------------------------------------

/// CodeView information.
#[derive(Copy, Clone)]
pub enum CodeView<'a> {
	/// CodeView 2.0 debug information.
	Cv20 {
		/// Underlying CodeView 2.0 record.
		image: &'a image::IMAGE_DEBUG_CV_INFO_PDB20,
		/// Nul-terminated PDB path following the record.
		pdb_file_name: &'a CStr,
	},
	/// CodeView 7.0 debug information.
	Cv70 {
		/// Underlying CodeView 7.0 record.
		image: &'a image::IMAGE_DEBUG_CV_INFO_PDB70,
		/// Nul-terminated PDB path following the record.
		pdb_file_name: &'a CStr,
	},
}
impl<'a> CodeView<'a> {
	/// Returns the four-byte CodeView format signature as a string.
	pub fn format(&self) -> &'a str {
		let cv_signature = match self {
			CodeView::Cv20 { image, .. } => &image.CvSignature,
			CodeView::Cv70 { image, .. } => &image.CvSignature,
		} as *const _ as *const [u8; 4];
		unsafe { str::from_utf8_unchecked(&*cv_signature) }
	}
	/// Returns the PDB age used to match the image with its symbols.
	pub fn age(&self) -> u32 {
		match self {
			CodeView::Cv20 { image, .. } => image.Age,
			CodeView::Cv70 { image, .. } => image.Age,
		}
	}
	/// Returns the recorded PDB path.
	pub fn pdb_file_name(&self) -> &'a CStr {
		match self {
			CodeView::Cv20 { pdb_file_name, .. } => pdb_file_name,
			CodeView::Cv70 { pdb_file_name, .. } => pdb_file_name,
		}
	}
}
impl<'a> fmt::Debug for CodeView<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut stru = f.debug_struct("CodeView");
		stru.field("format", &self.format());
		stru.field("pdb_file_name", &self.pdb_file_name());
		match self {
			CodeView::Cv20 { image, .. } => {
				stru.field("time_date_stamp", &image.TimeDateStamp);
				stru.field("age", &image.Age);
			},
			CodeView::Cv70 { image, .. } => {
				stru.field("signature", &image.Signature);
				stru.field("age", &image.Age);
			},
		}
		stru.finish()
	}
}

//----------------------------------------------------------------

/// Debug information.
#[derive(Copy, Clone)]
pub struct DebugMisc<'a> {
	pub(crate) image: &'a image::IMAGE_DEBUG_MISC,
}
impl<'a> DebugMisc<'a> {
	/// Gets the underlying information image.
	pub fn image(&self) -> &'a image::IMAGE_DEBUG_MISC {
		self.image
	}
}
impl<'a> fmt::Debug for DebugMisc<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("DebugMisc")
			.field("data_type", &self.image.DataType)
			.field("length", &self.image.Length)
			.field("unicode", &self.image.Unicode)
			.finish()
	}
}

//----------------------------------------------------------------

/// PGO information.
#[derive(Copy, Clone)]
pub struct Pgo<'a> {
	pub(crate) image: &'a [u32],
}
impl<'a> Pgo<'a> {
	/// Gets the underlying image.
	pub fn image(&self) -> &'a [u32] {
		self.image
	}
	/// Iterator over the PGO sections.
	pub fn iter(&self) -> PgoIter<'a> {
		let image = if self.image.len() >= 1 { &self.image[1..] } else { self.image };
		PgoIter { image }
	}
}
impl<'a> IntoIterator for Pgo<'a> {
	type Item = PgoItem<'a>;
	type IntoIter = PgoIter<'a>;
	fn into_iter(self) -> PgoIter<'a> {
		self.iter()
	}
}
impl<'a> fmt::Debug for Pgo<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Pgo")
			.field("sections", &crate::util::DebugList(self.iter()))
			.finish()
	}
}
/// Iterator over PGO sections.
#[derive(Clone)]
pub struct PgoIter<'a> {
	image: &'a [u32],
}
impl<'a> Iterator for PgoIter<'a> {
	type Item = PgoItem<'a>;
	fn next(&mut self) -> Option<PgoItem<'a>> {
		if self.image.len() >= 3 {
			let rva = self.image[0];
			let size = self.image[1];
			let name = CStr::from_bytes(dataview::bytes(&self.image[2..]))?;
			let len = name.len() >> 2;
			self.image = &self.image[2 + len + 1..];
			Some(PgoItem { rva, size, name })
		}
		else {
			None
		}
	}
}
/// Describes a PGO section.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PgoItem<'a> {
	/// Relative virtual address of the section.
	pub rva: u32,
	/// Size of the section in bytes.
	pub size: u32,
	/// Nul-terminated section name.
	pub name: &'a CStr,
}

//----------------------------------------------------------------

#[cfg(feature = "serde")]
mod serde2 {
	use crate::util::serde_helper::*;

	use super::{CodeView, DebugMisc, Pgo};

	impl<'a> Serialize for CodeView<'a> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("CodeView", 3)?;
			match self {
				CodeView::Cv20 { image, .. } => state.serialize_field("image", image)?,
				CodeView::Cv70 { image, .. } => state.serialize_field("image", image)?,
			}
			state.serialize_field("format", &self.format())?;
			state.serialize_field("pdb_file_name", &self.pdb_file_name())?;
			state.end()
		}
	}
	impl<'a> Serialize for DebugMisc<'a> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("DebugMisc", 1)?;
			state.serialize_field("image", self.image)?;
			state.end()
		}
	}
	impl<'a> Serialize for Pgo<'a> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			serializer.collect_seq(self.iter())
		}
	}
}
