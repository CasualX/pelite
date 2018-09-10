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

	// Print the CodeView 7.0 pdb file name
	if let Some(cv) = debug.read_cv70() {
		println!("PDB: {}", cv.pdb_file_name());
	}

	Ok(())
}
```
*/

use std::{fmt, iter, mem, slice};

use error::{Error, Result};
use util::CStr;

use super::image::*;
use super::Pe;

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
	/// Reads the CodeView 2.0 debug information entry if there is any.
	pub fn read_cv20(&self) -> Option<CvNB10<'a, P>> {
		self.find(Dir::read_cv20)
	}
	/// Reads the CodeView 7.0 debug information entry if there is any.
	pub fn read_cv70(&self) -> Option<CvRSDS<'a, P>> {
		self.find(Dir::read_cv70)
	}
	/// Reads the Debug information entry if there is any.
	pub fn read_dbg(&self) -> Option<Dbg<'a, P>> {
		self.find(Dir::read_dbg)
	}
	fn find<T, F: FnMut(&Dir<'a, P>) -> Result<T>>(&self, mut f: F) -> Option<T> {
		for dir in *self {
			if let Ok(item) = f(&dir) {
				return Some(item);
			}
		}
		None
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
	/// Reads as a CodeView 2.0 debug information entry.
	pub fn read_cv20(&self) -> Result<CvNB10<'a, P>> {
		CvNB10::new(self.pe, self.image)
	}
	/// Reads as a CodeView 7.0 debug information entry.
	pub fn read_cv70(&self) -> Result<CvRSDS<'a, P>> {
		CvRSDS::new(self.pe, self.image)
	}
	/// Reads as a Debug information entry.
	pub fn read_dbg(&self) -> Result<Dbg<'a, P>> {
		Dbg::new(self.pe, self.image)
	}
}
impl<'a, P: Pe<'a> + Copy> fmt::Debug for Dir<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut s = f.debug_struct("Dir");
		s.field("type", &::stringify::debug_type(self.image.Type));
		s.field("time_date_stamp", &self.image.TimeDateStamp);
		s.field("version", &self.image.Version);
		if let Ok(cv20) = self.read_cv20() {
			s.field("cv20", &cv20);
		}
		else if let Ok(cv70) = self.read_cv70() {
			s.field("cv70", &cv70);
		}
		else if let Ok(dbg) = self.read_dbg() {
			s.field("dbg", &dbg);
		}
		s.finish()
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
			state.serialize_field("time_date_stamp", &self.image.TimeDateStamp)?;
			state.serialize_field("age", &self.image.Age)?;
			state.serialize_field("pdb_file_name", &self.pdb_file_name())?;
			state.end()
		}
	}
	impl<'a, P: Pe<'a> + Copy> Serialize for CvRSDS<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("CvNB10", 3)?;
			state.serialize_field("signature", &self.image.Signature)?;
			state.serialize_field("age", &self.image.Age)?;
			state.serialize_field("pdb_file_name", &self.pdb_file_name())?;
			state.end()
		}
	}
	impl<'a, P: Pe<'a> + Copy> Serialize for Dbg<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			serializer.serialize_struct("Dbg", 0)?.end()
		}
	}
}
