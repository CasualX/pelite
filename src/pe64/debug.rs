/*!
Debug Directory.

# Examples

```
# #![allow(unused_variables)]
use pelite::pe64::{Pe, PeFile, debug};

# #[allow(dead_code)]
fn example(file: PeFile<'_>) -> pelite::Result<()> {
	// Access the debug directory
	let debug = file.debug()?;

	// Get the CodeView PDB file name
	if let Some(pdb_file_name) = debug.pdb_file_name() {
		println!("PDB: {}", pdb_file_name);
	}

	Ok(())
}
```
*/

use std::{fmt, iter, mem, slice, str};

use error::{Error, Result};
use util::CStr;

use super::image::*;
use super::{Align, Pe};

//----------------------------------------------------------------

/// Debug directory.
///
/// For more information see the [module-level documentation](index.html).
#[derive(Copy, Clone)]
pub struct Debug<'a, P> {
	pe: P,
	image: &'a [IMAGE_DEBUG_DIRECTORY],
}
impl<'a, P: Pe<'a> + Copy> Debug<'a, P> {
	pub(crate) fn try_from(pe: P) -> Result<Debug<'a, P>> {
		let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_DEBUG).ok_or(Error::Bounds)?;
		let (len, rem) = (
			datadir.Size as usize / mem::size_of::<IMAGE_DEBUG_DIRECTORY>(),
			datadir.Size as usize % mem::size_of::<IMAGE_DEBUG_DIRECTORY>(),
		);
		if rem != 0 {
			return Err(Error::Invalid);
		}
		let image = pe.derva_slice(datadir.VirtualAddress, len)?;
		Ok(Debug { pe, image })
	}
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Returns the underlying debug directories image.
	pub fn image(&self) -> &'a [IMAGE_DEBUG_DIRECTORY] {
		self.image
	}
	/// Gets the CodeView PDB file name.
	pub fn pdb_file_name(&self) -> Option<&'a CStr> {
		self.into_iter()
			.filter_map(|dir| dir.entry().ok().and_then(Entry::as_code_view).map(|cv| cv.pdb_file_name()))
			.next()
	}
}
impl<'a, P: Pe<'a> + Copy> IntoIterator for Debug<'a, P> {
	type Item = Dir<'a, P>;
	type IntoIter = Iter<'a, P>;
	fn into_iter(self) -> Iter<'a, P> {
		Iter {
			pe: self.pe,
			iter: self.image.iter(),
		}
	}
}
impl<'a, P: Pe<'a> + Copy> fmt::Debug for Debug<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_list().entries(*self).finish()
	}
}

//----------------------------------------------------------------

#[derive(Clone)]
pub struct Iter<'a, P> {
	pe: P,
	iter: slice::Iter<'a, IMAGE_DEBUG_DIRECTORY>,
}
impl<'a, P: Pe<'a> + Copy> Iter<'a, P> {
	pub fn image(&self) -> &'a [IMAGE_DEBUG_DIRECTORY] {
		self.iter.as_slice()
	}
}
impl<'a, P: Pe<'a> + Copy> Iterator for Iter<'a, P> {
	type Item = Dir<'a, P>;
	fn next(&mut self) -> Option<Dir<'a, P>> {
		self.iter.next().map(|image| Dir { pe: self.pe, image })
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
	fn count(self) -> usize {
		self.iter.count()
	}
	fn nth(&mut self, n: usize) -> Option<Dir<'a, P>> {
		self.iter.nth(n).map(|image| Dir { pe: self.pe, image })
	}
}
impl<'a, P: Pe<'a> + Copy> DoubleEndedIterator for Iter<'a, P> {
	fn next_back(&mut self) -> Option<Dir<'a, P>> {
		self.iter.next_back().map(|image| Dir { pe: self.pe, image })
	}
}
impl<'a, P: Pe<'a> + Copy> ExactSizeIterator for Iter<'a, P> {}
impl<'a, P: Pe<'a> + Copy> iter::FusedIterator for Iter<'a, P> {}

//----------------------------------------------------------------

/// Debug directory entry.
#[derive(Copy, Clone)]
pub struct Dir<'a, P> {
	pe: P,
	image: &'a IMAGE_DEBUG_DIRECTORY,
}
impl<'a, P: Pe<'a> + Copy> Dir<'a, P> {
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Gets the underlying debug directory image.
	pub fn image(&self) -> &'a IMAGE_DEBUG_DIRECTORY {
		self.image
	}
	/// Gets the raw data of this debug directory entry.
	pub fn data(&self) -> Option<&'a [u8]> {
		let image = self.pe.image();
		let size = self.image.SizeOfData as usize;
		let offset = match self.pe.align() {
			Align::File => self.image.PointerToRawData,
			Align::Section => self.image.AddressOfRawData,
		} as usize;
		image.get(offset..offset.wrapping_add(size))
	}
	pub fn entry(&self) -> Result<Entry<'a>> {
		match self.image.Type {
			IMAGE_DEBUG_TYPE_CODEVIEW => Ok(Entry::CodeView(CodeView::new(self.pe, self.image)?)),
			IMAGE_DEBUG_TYPE_MISC => Ok(Entry::Dbg(Dbg::new(self.pe, self.image)?)),
			_ => Ok(Entry::Unknown { data: self.data() })
		}
	}
}
impl<'a, P: Pe<'a> + Copy> fmt::Debug for Dir<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Dir")
			.field("type", &::stringify::debug_type(self.image.Type).ok_or(self.image.Type))
			.field("time_date_stamp", &self.image.TimeDateStamp)
			.field("version", &self.image.Version)
			.field("entry", &self.entry())
			.finish()
	}
}

//----------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Entry<'a> {
	CodeView(CodeView<'a>),
	Dbg(Dbg<'a>),
	Unknown { data: Option<&'a [u8]> },
}
impl<'a> Entry<'a> {
	/// As a CodeView debug information entry.
	pub fn as_code_view(self) -> Option<CodeView<'a>> {
		match self { Entry::CodeView(cv) => Some(cv), _ => None }
	}
	/// As a Dbg information entry.
	pub fn as_dbg(self) -> Option<Dbg<'a>> {
		match self { Entry::Dbg(dbg) => Some(dbg), _ => None }
	}
	/// Unknown format, return as bytes.
	pub fn as_unknown(self) -> Option<&'a [u8]> {
		match self { Entry::Unknown { data } => data, _ => None }
	}
}

//----------------------------------------------------------------

#[derive(Copy, Clone)]
pub enum CodeView<'a> {
	/// CodeView 2.0 debug information.
	Cv20 { image: &'a IMAGE_DEBUG_CV_INFO_PDB20, pdb_file_name: &'a CStr },
	/// CodeView 7.0 debug information.
	Cv70 { image: &'a IMAGE_DEBUG_CV_INFO_PDB70, pdb_file_name: &'a CStr },
}
impl<'a> CodeView<'a> {
	pub(crate) fn new<P: Pe<'a> + Copy>(pe: P, dir: &IMAGE_DEBUG_DIRECTORY) -> Result<CodeView<'a>> {
		if dir.Type != IMAGE_DEBUG_TYPE_CODEVIEW {
			return Err(Error::Null);
		}
		let bytes = pe.slice(dir.AddressOfRawData, dir.SizeOfData as usize, 4)?;
		match unsafe { &*(bytes.as_ptr() as *const [u8; 4]) } {
			b"NB10" => {
				if bytes.len() < 16 {
					return Err(Error::Bounds);
				}
				let image = unsafe { &*(bytes.as_ptr() as *const IMAGE_DEBUG_CV_INFO_PDB20) };
				let pdb_file_name = CStr::from_bytes(&bytes[16..]).ok_or(Error::Encoding)?;
				Ok(CodeView::Cv20 { image, pdb_file_name })
			},
			b"RSDS" => {
				if bytes.len() < 24 {
					return Err(Error::Bounds);
				}
				let image = unsafe { &*(bytes.as_ptr() as *const IMAGE_DEBUG_CV_INFO_PDB70) };
				let pdb_file_name = CStr::from_bytes(&bytes[24..]).ok_or(Error::Encoding)?;
				Ok(CodeView::Cv70 { image, pdb_file_name })
			},
			_ => Err(Error::BadMagic),
		}
	}
	pub fn format(&self) -> &'a str {
		let cv_signature = match self {
			CodeView::Cv20 { image, .. } => &image.CvSignature,
			CodeView::Cv70 { image, .. } => &image.CvSignature,
		};
		unsafe { str::from_utf8_unchecked(&*(cv_signature as *const _ as *const [u8; 4])) }
	}
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
pub struct Dbg<'a> {
	image: &'a IMAGE_DEBUG_MISC,
}
impl<'a> Dbg<'a> {
	pub(crate) fn new<P: Pe<'a> + Copy>(pe: P, dir: &IMAGE_DEBUG_DIRECTORY) -> Result<Dbg<'a>> {
		if dir.Type != IMAGE_DEBUG_TYPE_MISC {
			return Err(Error::Null);
		}
		let image = pe.derva(dir.AddressOfRawData)?;
		Ok(Dbg { image })
	}
	/// Gets the underlying information image.
	pub fn image(&self) -> &'a IMAGE_DEBUG_MISC {
		self.image
	}
}
impl<'a> fmt::Debug for Dbg<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Dbg").finish()
	}
}

//----------------------------------------------------------------

/*
	"debug": [
		{
			"type": "CodeView",
			"time_date_stamp": 0,
			"version": "1.0",
			"entry": {
				format: "RSDS",
				pdb_file_name: "",
				...
			},
		},
	],
*/

#[cfg(feature = "serde")]
mod serde {
	use util::serde_helper::*;
	use super::{Pe, Debug, Dir, CodeView, Dbg};

	impl<'a, P: Pe<'a> + Copy> Serialize for Debug<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			serializer.collect_seq(self.into_iter())
		}
	}
	impl<'a, P: Pe<'a> + Copy> Serialize for Dir<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let is_human_readable = serializer.is_human_readable();
			let mut state = serializer.serialize_struct("Dir", 4)?;
			if is_human_readable {
				state.serialize_field("type", &::stringify::debug_type(self.image.Type))?;
			}
			else {
				state.serialize_field("type", &self.image.Type)?;
			}
			state.serialize_field("time_date_stamp", &self.image.TimeDateStamp)?;
			state.serialize_field("version", &self.image.Version)?;
			state.serialize_field("entry", &self.entry().ok())?;
			state.end()
		}
	}
	impl<'a> Serialize for CodeView<'a> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("CvNB10", 4)?;
			state.serialize_field("format", &self.format())?;
			state.serialize_field("pdb_file_name", &self.pdb_file_name())?;
			match self {
				CodeView::Cv20 { image, .. } => {
					state.serialize_field("time_date_stamp", &image.TimeDateStamp)?;
					state.serialize_field("age", &image.Age)?;
				},
				CodeView::Cv70 { image, .. } => {
					state.serialize_field("signature", &image.Signature)?;
					state.serialize_field("age", &image.Age)?;
				},
			}
			state.end()
		}
	}
	impl<'a> Serialize for Dbg<'a> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			serializer.serialize_struct("Dbg", 0)?.end()
		}
	}
}
