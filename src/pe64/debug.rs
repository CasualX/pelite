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

use std::{fmt, iter, mem, slice};

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
			.filter_map(|dir| dir.entry().ok().and_then(Entry::as_cv70).map(|cv| cv.pdb_file_name))
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
	pub fn entry(&self) -> Result<Entry<'a, P>> {
		match self.image.Type {
			IMAGE_DEBUG_TYPE_CODEVIEW => {
				if let Ok(cv20) = CvNB10::new(self.pe, self.image) {
					Ok(Entry::CvNB10(cv20))
				}
				else if let Ok(cv70) = CvRSDS::new(self.pe, self.image) {
					Ok(Entry::CvRSDS(cv70))
				}
				else {
					Err(Error::Null)
				}
			},
			IMAGE_DEBUG_TYPE_MISC => Ok(Entry::Dbg(Dbg::new(self.pe, self.image)?)),
			_ => Ok(Entry::Unknown { data: self.data() })
		}
	}
}
impl<'a, P: Pe<'a> + Copy> fmt::Debug for Dir<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Dir")
			.field("type", &self.image.Type)
			.field("type_name", &::stringify::debug_type(self.image.Type))
			.field("time_date_stamp", &self.image.TimeDateStamp)
			.field("version", &self.image.Version)
			.field("entry", &self.entry())
			.finish()
	}
}

//----------------------------------------------------------------

#[derive(Copy, Clone)]
pub enum Entry<'a, P> {
	CvNB10(CvNB10<'a, P>),
	CvRSDS(CvRSDS<'a, P>),
	Dbg(Dbg<'a, P>),
	Unknown { data: Option<&'a [u8]> },
}
impl<'a, P> Entry<'a, P> {
	/// As a CodeView 2.0 debug information entry.
	pub fn as_cv20(self) -> Option<CvNB10<'a, P>> {
		match self { Entry::CvNB10(cv20) => Some(cv20), _ => None }
	}
	/// As a CodeView 7.0 debug information entry.
	pub fn as_cv70(self) -> Option<CvRSDS<'a, P>> {
		match self { Entry::CvRSDS(cv70) => Some(cv70), _ => None }
	}
	/// As a Dbg information entry.
	pub fn as_dbg(self) -> Option<Dbg<'a, P>> {
		match self { Entry::Dbg(dbg) => Some(dbg), _ => None }
	}
	pub fn as_unknown(self) -> Option<&'a [u8]> {
		match self { Entry::Unknown { data } => data, _ => None }
	}
}
impl<'a, P: Pe<'a> + Copy> fmt::Debug for Entry<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Entry::CvNB10(cv20) => cv20.fmt(f),
			Entry::CvRSDS(cv70) => cv70.fmt(f),
			Entry::Dbg(dbg) => dbg.fmt(f),
			Entry::Unknown { data } => data.fmt(f),
		}
	}
}

//----------------------------------------------------------------

/// CodeView 2.0 debug information.
#[derive(Copy, Clone)]
pub struct CvNB10<'a, P> {
	pe: P,
	image: &'a IMAGE_DEBUG_CV_INFO_PDB20,
	pdb_file_name: &'a CStr,
}
impl<'a, P: Pe<'a> + Copy> CvNB10<'a, P> {
	pub(crate) fn new(pe: P, dir: &IMAGE_DEBUG_DIRECTORY) -> Result<CvNB10<'a, P>> {
		if dir.Type != IMAGE_DEBUG_TYPE_CODEVIEW {
			return Err(Error::BadMagic);
		}
		let bytes = pe.slice(dir.AddressOfRawData, dir.SizeOfData as usize, 4)?;
		if bytes.len() < 16 {
			return Err(Error::Bounds);
		}
		let image = unsafe { &*(bytes.as_ptr() as *const IMAGE_DEBUG_CV_INFO_PDB20) };
		let signature: [u8; 4] = unsafe { mem::transmute(image.CvSignature) };
		if signature != *b"NB10" {
			return Err(Error::BadMagic);
		}
		let pdb_file_name = CStr::from_bytes(&bytes[16..]).ok_or(Error::Encoding)?;
		Ok(CvNB10 { pe, image, pdb_file_name })
	}
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Gets the underlying information image.
	pub fn image(&self) -> &'a IMAGE_DEBUG_CV_INFO_PDB20 {
		self.image
	}
	/// Gets the PDB file name.
	pub fn pdb_file_name(&self) -> &'a CStr {
		self.pdb_file_name
	}
}
impl<'a, P: Pe<'a> + Copy> fmt::Debug for CvNB10<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("CvNB10")
			.field("pdb_file_name", &self.pdb_file_name)
			.field("time_date_stamp", &self.image.TimeDateStamp)
			.field("age", &self.image.Age)
			.finish()
	}
}

//----------------------------------------------------------------

/// CodeView 7.0 debug information.
#[derive(Copy, Clone)]
pub struct CvRSDS<'a, P> {
	pe: P,
	image: &'a IMAGE_DEBUG_CV_INFO_PDB70,
	pdb_file_name: &'a CStr,
}
impl<'a, P: Pe<'a> + Copy> CvRSDS<'a, P> {
	pub(crate) fn new(pe: P, dir: &IMAGE_DEBUG_DIRECTORY) -> Result<CvRSDS<'a, P>> {
		if dir.Type != IMAGE_DEBUG_TYPE_CODEVIEW {
			return Err(Error::BadMagic);
		}
		let bytes = pe.slice(dir.AddressOfRawData, dir.SizeOfData as usize, 4)?;
		if bytes.len() < 24 {
			return Err(Error::Bounds);
		}
		let image = unsafe { &*(bytes.as_ptr() as *const IMAGE_DEBUG_CV_INFO_PDB70) };
		let signature: [u8; 4] = unsafe { mem::transmute(image.CvSignature) };
		if signature != *b"RSDS" {
			return Err(Error::BadMagic);
		}
		let pdb_file_name = CStr::from_bytes(&bytes[24..]).ok_or(Error::Encoding)?;
		Ok(CvRSDS { pe, image, pdb_file_name })
	}
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Gets the underlying information image.
	pub fn image(&self) -> &'a IMAGE_DEBUG_CV_INFO_PDB70 {
		self.image
	}
	/// Gets the PDB file name.
	pub fn pdb_file_name(&self) -> &'a CStr {
		self.pdb_file_name
	}
}
impl<'a, P: Pe<'a> + Copy> fmt::Debug for CvRSDS<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("CvRSDS")
			.field("pdb_file_name", &self.pdb_file_name)
			.field("signature", &self.image.Signature)
			.field("age", &self.image.Age)
			.finish()
	}
}

//----------------------------------------------------------------

/// Debug information.
#[derive(Copy, Clone)]
pub struct Dbg<'a, P> {
	pe: P,
	image: &'a IMAGE_DEBUG_MISC,
}
impl<'a, P: Pe<'a> + Copy> Dbg<'a, P> {
	pub(crate) fn new(pe: P, dir: &IMAGE_DEBUG_DIRECTORY) -> Result<Dbg<'a, P>> {
		if dir.Type != IMAGE_DEBUG_TYPE_MISC {
			return Err(Error::Null);
		}
		let image = pe.derva(dir.AddressOfRawData)?;
		Ok(Dbg { pe, image })
	}
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Gets the underlying information image.
	pub fn image(&self) -> &'a IMAGE_DEBUG_MISC {
		self.image
	}
}
impl<'a, P: Pe<'a> + Copy> fmt::Debug for Dbg<'a, P> {
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
			"cv70": {
				pdb_file_name: "",
				...
			},
		},
	],
*/

#[cfg(feature = "serde")]
mod serde {
	use util::serde_helper::*;
	use super::{Pe, Debug, Dir, CvNB10, CvRSDS, Dbg};

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
			if let Ok(cv20) = self.read_cv20() {
				state.serialize_field("cv20", &cv20)?;
			}
			else if let Ok(cv70) = self.read_cv70() {
				state.serialize_field("cv70", &cv70)?;
			}
			else if let Ok(dbg) = self.read_dbg() {
				state.serialize_field("dbg", &dbg)?;
			}
			else {
				state.serialize_field("unknown", &(None as Option<i32>))?;
			}
			state.end()
		}
	}
	impl<'a, P: Pe<'a> + Copy> Serialize for CvNB10<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("CvNB10", 3)?;
			state.serialize_field("pdb_file_name", &self.pdb_file_name())?;
			state.serialize_field("time_date_stamp", &self.image.TimeDateStamp)?;
			state.serialize_field("age", &self.image.Age)?;
			state.end()
		}
	}
	impl<'a, P: Pe<'a> + Copy> Serialize for CvRSDS<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("CvNB10", 3)?;
			state.serialize_field("pdb_file_name", &self.pdb_file_name())?;
			state.serialize_field("signature", &self.image.Signature)?;
			state.serialize_field("age", &self.image.Age)?;
			state.end()
		}
	}
	impl<'a, P: Pe<'a> + Copy> Serialize for Dbg<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			serializer.serialize_struct("Dbg", 0)?.end()
		}
	}
}
