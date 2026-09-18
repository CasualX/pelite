//! Debug directory parsing.

use core::{fmt, iter, mem, slice, str};

use crate::image;
use crate::util::{AlignTo, CStr};
use crate::{Error, PeLayout, Result};

/// Debug directory.
///
/// # Examples
///
/// ```
/// use pelite::pe64::{Pe, PeFile};
///
/// # #[allow(dead_code)]
/// fn example(file: PeFile<'_>) -> pelite::Result<()> {
/// 	let debug = file.debug()?;
/// 	if let Some(pdb_file_name) = debug.pdb_file_name()? {
/// 		println!("PDB: {}", pdb_file_name);
/// 	}
/// 	Ok(())
/// }
/// ```
#[derive(Copy, Clone)]
pub struct DebugDirectory<'a> {
	data: &'a [u8],
	layout: PeLayout,
	image: &'a [image::IMAGE_DEBUG_DIRECTORY],
}
impl<'a> DebugDirectory<'a> {
	pub(crate) fn new(data: &'a [u8], layout: PeLayout, image: &'a [image::IMAGE_DEBUG_DIRECTORY]) -> Self {
		Self { data, layout, image }
	}
	/// Returns the underlying debug directories image.
	pub fn image(&self) -> &'a [image::IMAGE_DEBUG_DIRECTORY] {
		self.image
	}
	/// Gets the CodeView PDB file name.
	pub fn pdb_file_name(&self) -> Result<Option<&'a CStr>> {
		for dir in self.iter() {
			if dir.image.Type == image::IMAGE_DEBUG_TYPE_CODEVIEW {
				return Ok(dir.entry()?.and_then(DebugData::as_code_view).map(|cv| cv.pdb_file_name()));
			}
		}
		Ok(None)
	}
	/// Iterator over the debug directories.
	pub fn iter(&self) -> DebugDirectoryIter<'a> {
		DebugDirectoryIter { data: self.data, layout: self.layout, iter: self.image.iter() }
	}
}
impl<'a> IntoIterator for DebugDirectory<'a> {
	type Item = DebugDirectoryEntry<'a>;
	type IntoIter = DebugDirectoryIter<'a>;
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}
impl fmt::Debug for DebugDirectory<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("DebugDirectory")
			.field("entries", &crate::util::DebugList(self.iter()))
			.finish()
	}
}

/// Iterator over debug directory entries.
#[derive(Clone)]
pub struct DebugDirectoryIter<'a> {
	data: &'a [u8],
	layout: PeLayout,
	iter: slice::Iter<'a, image::IMAGE_DEBUG_DIRECTORY>,
}
impl<'a> DebugDirectoryIter<'a> {
	/// Returns the unconsumed debug directory entries.
	pub fn image(&self) -> &'a [image::IMAGE_DEBUG_DIRECTORY] {
		self.iter.as_slice()
	}
	fn make_entry(&self, image: &'a image::IMAGE_DEBUG_DIRECTORY) -> DebugDirectoryEntry<'a> {
		DebugDirectoryEntry { data: debug_data(self.data, self.layout, image), image }
	}
}
impl<'a> Iterator for DebugDirectoryIter<'a> {
	type Item = DebugDirectoryEntry<'a>;
	fn next(&mut self) -> Option<Self::Item> {
		let image = self.iter.next()?;
		Some(self.make_entry(image))
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
	fn count(self) -> usize {
		self.iter.count()
	}
	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		let image = self.iter.nth(n)?;
		Some(DebugDirectoryEntry { data: debug_data(self.data, self.layout, image), image })
	}
}
impl<'a> DoubleEndedIterator for DebugDirectoryIter<'a> {
	fn next_back(&mut self) -> Option<Self::Item> {
		let image = self.iter.next_back()?;
		Some(self.make_entry(image))
	}
}
impl ExactSizeIterator for DebugDirectoryIter<'_> {}
impl iter::FusedIterator for DebugDirectoryIter<'_> {}

/// Debug directory entry.
#[derive(Copy, Clone)]
pub struct DebugDirectoryEntry<'a> {
	data: Option<&'a [u8]>,
	image: &'a image::IMAGE_DEBUG_DIRECTORY,
}
impl<'a> DebugDirectoryEntry<'a> {
	/// Gets the underlying debug directory image.
	pub fn image(&self) -> &'a image::IMAGE_DEBUG_DIRECTORY {
		self.image
	}
	/// Gets the raw data of this debug directory entry.
	pub fn data(&self) -> Option<&'a [u8]> {
		self.data
	}
	/// Interprets the directory entry.
	///
	/// Returns `Ok(None)` when the debug type is not supported. An error means
	/// that a supported entry is malformed or its data is unavailable.
	pub fn entry(&self) -> Result<Option<DebugData<'a>>> {
		match self.image.Type {
			image::IMAGE_DEBUG_TYPE_CODEVIEW => Ok(Some(DebugData::CodeView(code_view(self)?))),
			image::IMAGE_DEBUG_TYPE_MISC => Ok(Some(DebugData::Misc(debug_misc(self)?))),
			image::IMAGE_DEBUG_TYPE_POGO => Ok(Some(DebugData::Pgo(pgo(self)?))),
			_ => Ok(None),
		}
	}
}

fn debug_data<'a>(data: &'a [u8], layout: PeLayout, image: &image::IMAGE_DEBUG_DIRECTORY) -> Option<&'a [u8]> {
	let size = image.SizeOfData as usize;
	if size == 0 {
		return Some(&data[..0]);
	}
	let offset = match layout {
		PeLayout::File => image.PointerToRawData,
		PeLayout::Section => image.AddressOfRawData,
	} as usize;
	if offset == 0 {
		return None;
	}
	let end = offset.checked_add(size)?;
	data.get(offset..end)
}
impl fmt::Debug for DebugDirectoryEntry<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("DebugDirectoryEntry")
			.field("type", &crate::stringify::DebugType(self.image.Type).to_str().ok_or(self.image.Type))
			.field("time_date_stamp", &self.image.TimeDateStamp)
			.field("version", &self.image.Version)
			.field("entry", &self.entry())
			.finish()
	}
}

fn code_view<'a>(dir: &DebugDirectoryEntry<'a>) -> Result<CodeView<'a>> {
	let bytes = dir.data().ok_or(Error::Bounds)?;
	if bytes.len() < 16 {
		return Err(Error::Bounds);
	}
	if !bytes.as_ptr().aligned_to(4) {
		return Err(Error::Misaligned);
	}
	let cv_signature = unsafe { &*(bytes.as_ptr() as *const [u8; 4]) };
	match cv_signature {
		b"NB10" => {
			let image = unsafe { &*(bytes.as_ptr() as *const image::IMAGE_DEBUG_CV_INFO_PDB20) };
			let pdb_file_name = CStr::from_bytes(&bytes[16..]).ok_or(Error::Encoding)?;
			Ok(CodeView::Cv20 { image, pdb_file_name })
		},
		b"RSDS" => {
			if bytes.len() < 24 {
				return Err(Error::Bounds);
			}
			let image = unsafe { &*(bytes.as_ptr() as *const image::IMAGE_DEBUG_CV_INFO_PDB70) };
			let pdb_file_name = CStr::from_bytes(&bytes[24..]).ok_or(Error::Encoding)?;
			Ok(CodeView::Cv70 { image, pdb_file_name })
		},
		_ => Err(Error::BadMagic),
	}
}

fn debug_misc<'a>(dir: &DebugDirectoryEntry<'a>) -> Result<DebugMisc<'a>> {
	let data = dir.data().ok_or(Error::Bounds)?;
	if data.len() < mem::size_of::<image::IMAGE_DEBUG_MISC>() {
		return Err(Error::Bounds);
	}
	if !data.as_ptr().aligned_to(4) {
		return Err(Error::Misaligned);
	}
	let image = unsafe { &*(data.as_ptr() as *const image::IMAGE_DEBUG_MISC) };
	let length = image.Length as usize;
	if length < mem::size_of::<image::IMAGE_DEBUG_MISC>() || length > data.len() {
		return Err(Error::Bounds);
	}
	let name = debug_misc_name(&data[mem::size_of::<image::IMAGE_DEBUG_MISC>()..length], image.Unicode != 0)?;
	Ok(DebugMisc { image, name })
}

fn debug_misc_name(data: &[u8], unicode: bool) -> Result<DebugMiscName<'_>> {
	if !unicode {
		return CStr::from_bytes(data).map(DebugMiscName::Ansi).ok_or(Error::Encoding);
	}
	if data.len() % mem::size_of::<u16>() != 0 {
		return Err(Error::Invalid);
	}
	if !data.as_ptr().aligned_to(mem::align_of::<u16>()) {
		return Err(Error::Misaligned);
	}
	let words = unsafe { slice::from_raw_parts(data.as_ptr() as *const u16, data.len() / mem::size_of::<u16>()) };
	let len = words.iter().position(|&word| word == 0).ok_or(Error::Encoding)?;
	Ok(DebugMiscName::Unicode(&words[..len]))
}

fn pgo<'a>(dir: &DebugDirectoryEntry<'a>) -> Result<Pgo<'a>> {
	let data = dir.data().ok_or(Error::Bounds)?;
	if data.len() < 4 {
		return Err(Error::Bounds);
	}
	if data.len() % mem::size_of::<u32>() != 0 {
		return Err(Error::Invalid);
	}
	if !data.as_ptr().aligned_to(4) {
		return Err(Error::Misaligned);
	}
	let image = unsafe { slice::from_raw_parts(data.as_ptr() as *const u32, data.len() / 4) };
	let pgo = Pgo { image };
	for item in pgo {
		item?;
	}
	Ok(pgo)
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
	pub(crate) name: DebugMiscName<'a>,
}
impl<'a> DebugMisc<'a> {
	/// Gets the underlying information image.
	pub fn image(&self) -> &'a image::IMAGE_DEBUG_MISC {
		self.image
	}
	/// Gets the name of the separate debug file.
	pub fn name(&self) -> DebugMiscName<'a> {
		self.name
	}
}
impl<'a> fmt::Debug for DebugMisc<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("DebugMisc")
			.field("data_type", &self.image.DataType)
			.field("length", &self.image.Length)
			.field("unicode", &self.image.Unicode)
			.field("name", &self.name)
			.finish()
	}
}

/// Name stored in an [`IMAGE_DEBUG_MISC`](image::IMAGE_DEBUG_MISC) record.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum DebugMiscName<'a> {
	/// Nul-terminated byte string.
	Ansi(&'a CStr),
	/// Nul-terminated UTF-16 string, without the terminator.
	Unicode(&'a [u16]),
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
	/// Gets the PGO format signature.
	pub fn signature(&self) -> PgoSignature {
		PgoSignature::from(self.image.first().copied().unwrap_or_default())
	}
	/// Iterator over the PGO sections.
	pub fn iter(&self) -> PgoIter<'a> {
		let image = if self.image.len() >= 1 { &self.image[1..] } else { self.image };
		PgoIter { image }
	}
}
impl<'a> IntoIterator for Pgo<'a> {
	type Item = Result<PgoItem<'a>>;
	type IntoIter = PgoIter<'a>;
	fn into_iter(self) -> PgoIter<'a> {
		self.iter()
	}
}
impl<'a> fmt::Debug for Pgo<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Pgo")
			.field("signature", &self.signature())
			.field("sections", &crate::util::DebugList(self.iter()))
			.finish()
	}
}

/// Four-byte format signature at the start of a PGO debug record.
///
/// The bytes use the conventional spelling displayed by Microsoft tools. The
/// PE stores the signature as a little-endian integer, so [`PgoSignature::LTCG`]
/// corresponds to the on-disk bytes `GCTL`.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct PgoSignature(pub [u8; 4]);
impl PgoSignature {
	/// PGO data without a format signature.
	pub const ZERO: Self = Self([0; 4]);
	/// Link-time code generation data.
	pub const LTCG: Self = Self(*b"LTCG");
	/// Profile-guided instrumentation data.
	pub const PGI: Self = Self(*b"PGI\0");
	/// Profile-guided optimization data.
	pub const PGO: Self = Self(*b"PGO\0");
	/// Profile-guided optimization-use data.
	pub const PGU: Self = Self(*b"PGU\0");
	/// Static profile-guided optimization data.
	pub const SPGO: Self = Self(*b"SPGO");

	/// Gets the conventionally ordered signature bytes.
	pub fn as_bytes(&self) -> &[u8; 4] {
		&self.0
	}
	/// Returns the conventionally ordered signature bytes.
	pub fn into_bytes(self) -> [u8; 4] {
		self.0
	}
	/// Gets the raw integer magic value stored in the record.
	pub fn raw(self) -> u32 {
		u32::from_be_bytes(self.0)
	}
}
impl fmt::Display for PgoSignature {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let end = self.0.iter().rposition(|&byte| byte != 0).map_or(0, |index| index + 1);
		for &byte in &self.0[..end] {
			match byte {
				b'\\' => f.write_str("\\\\")?,
				0x20..=0x7e => fmt::Write::write_char(f, byte as char)?,
				_ => write!(f, "\\x{byte:02X}")?,
			}
		}
		Ok(())
	}
}
impl From<u32> for PgoSignature {
	fn from(raw: u32) -> Self {
		Self(raw.to_be_bytes())
	}
}
impl From<[u8; 4]> for PgoSignature {
	fn from(bytes: [u8; 4]) -> Self {
		Self(bytes)
	}
}

/// Iterator over PGO sections.
#[derive(Clone)]
pub struct PgoIter<'a> {
	image: &'a [u32],
}
impl<'a> Iterator for PgoIter<'a> {
	type Item = Result<PgoItem<'a>>;
	fn next(&mut self) -> Option<Self::Item> {
		if self.image.is_empty() {
			None
		}
		else if self.image.len() >= 3 {
			let rva = self.image[0];
			let size = self.image[1];
			let name = match CStr::from_bytes(dataview::bytes(&self.image[2..])) {
				Some(name) => name,
				None => {
					self.image = &[];
					return Some(Err(Error::Encoding));
				},
			};
			let len = name.len() >> 2;
			self.image = &self.image[2 + len + 1..];
			Some(Ok(PgoItem { rva, size, name }))
		}
		else {
			self.image = &[];
			Some(Err(Error::Bounds))
		}
	}
}
impl<'a> core::iter::FusedIterator for PgoIter<'a> {}

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

serde_impl! {
	impl Serialize for DebugDirectory<'_> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			serializer.collect_seq(self.into_iter())
		}
	}
	impl Serialize for DebugDirectoryEntry<'_> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let is_human_readable = serializer.is_human_readable();
			let mut state = serializer.serialize_struct("DebugDirectoryEntry", 3)?;
			state.serialize_field("image", self.image)?;
			if is_human_readable {
				state.serialize_field("type", &crate::stringify::DebugType(self.image.Type).to_str())?;
			}
			else {
				state.serialize_field("type", &self.image.Type)?;
			}
			state.serialize_field("entry", &self.entry().ok().flatten())?;
			state.end()
		}
	}

	impl Serialize for PgoSignature {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			if serializer.is_human_readable() {
				serializer.collect_str(self)
			}
			else {
				serializer.serialize_bytes(&self.0)
			}
		}
	}

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
			let mut state = serializer.serialize_struct("DebugMisc", 2)?;
			state.serialize_field("image", self.image)?;
			state.serialize_field("name", &self.name)?;
			state.end()
		}
	}
	impl<'a> Serialize for Pgo<'a> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("Pgo", 2)?;
			state.serialize_field("signature", &self.signature())?;
			state.serialize_field("sections", &SerdeIter(self.iter()))?;
			state.end()
		}
	}
}
