use super::*;

//----------------------------------------------------------------

/**
Debug directory.

# Examples

```
# #![allow(unused_variables)]
use pelite::pe64::{Pe, PeFile};

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
#[derive(Copy, Clone)]
pub struct DebugDirectory<'a, P> {
	pe: P,
	image: &'a [IMAGE_DEBUG_DIRECTORY],
}
impl<'a, P: Pe<'a>> DebugDirectory<'a, P> {
	pub(crate) fn try_from(pe: P) -> Result<DebugDirectory<'a, P>> {
		let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_DEBUG).ok_or(Error::Bounds)?;
		let (len, rem) = (
			datadir.Size as usize / mem::size_of::<IMAGE_DEBUG_DIRECTORY>(),
			datadir.Size as usize % mem::size_of::<IMAGE_DEBUG_DIRECTORY>(),
		);
		if rem != 0 {
			return Err(Error::Invalid);
		}
		let image = pe.derva_slice(datadir.VirtualAddress, len)?;
		Ok(DebugDirectory { pe, image })
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
		self.into_iter().find_map(|dir| dir.entry().ok().and_then(DebugData::as_code_view).map(|cv| cv.pdb_file_name()))
	}
	/// Iterator over the debug directories.
	pub fn iter(&self) -> DebugDirectoryIter<'a, P> {
		DebugDirectoryIter { pe: self.pe, iter: self.image.iter() }
	}
}
impl<'a, P: Pe<'a>> IntoIterator for DebugDirectory<'a, P> {
	type Item = DebugDirectoryEntry<'a, P>;
	type IntoIter = DebugDirectoryIter<'a, P>;
	fn into_iter(self) -> DebugDirectoryIter<'a, P> {
		self.iter()
	}
}
impl<'a, P: Pe<'a>> fmt::Debug for DebugDirectory<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("DebugDirectory")
			.field("entries", &crate::util::DebugList(self.iter()))
			.finish()
	}
}

//----------------------------------------------------------------

/// Iterator over DebugDirectoryEntry entries.
#[derive(Clone)]
pub struct DebugDirectoryIter<'a, P> {
	pe: P,
	iter: slice::Iter<'a, IMAGE_DEBUG_DIRECTORY>,
}
impl<'a, P: Pe<'a>> DebugDirectoryIter<'a, P> {
	/// Returns the unconsumed debug directory entries.
	pub fn image(&self) -> &'a [IMAGE_DEBUG_DIRECTORY] {
		self.iter.as_slice()
	}
}
impl<'a, P: Pe<'a>> Iterator for DebugDirectoryIter<'a, P> {
	type Item = DebugDirectoryEntry<'a, P>;
	fn next(&mut self) -> Option<DebugDirectoryEntry<'a, P>> {
		self.iter.next().map(|image| DebugDirectoryEntry { pe: self.pe, image })
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
	fn count(self) -> usize {
		self.iter.count()
	}
	fn nth(&mut self, n: usize) -> Option<DebugDirectoryEntry<'a, P>> {
		self.iter.nth(n).map(|image| DebugDirectoryEntry { pe: self.pe, image })
	}
}
impl<'a, P: Pe<'a>> DoubleEndedIterator for DebugDirectoryIter<'a, P> {
	fn next_back(&mut self) -> Option<DebugDirectoryEntry<'a, P>> {
		self.iter.next_back().map(|image| DebugDirectoryEntry { pe: self.pe, image })
	}
}
impl<'a, P: Pe<'a>> ExactSizeIterator for DebugDirectoryIter<'a, P> {}
impl<'a, P: Pe<'a>> iter::FusedIterator for DebugDirectoryIter<'a, P> {}

//----------------------------------------------------------------

/// Debug directory entry.
#[derive(Copy, Clone)]
pub struct DebugDirectoryEntry<'a, P> {
	pe: P,
	image: &'a IMAGE_DEBUG_DIRECTORY,
}
impl<'a, P: Pe<'a>> DebugDirectoryEntry<'a, P> {
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
		let offset = match self.pe.layout() {
			PeLayout::File => self.image.PointerToRawData,
			PeLayout::Section => self.image.AddressOfRawData,
		} as usize;
		image.get(offset..offset.wrapping_add(size))
	}
	/// Interprets the directory entry.
	pub fn entry(&self) -> Result<DebugData<'a>> {
		match self.image.Type {
			IMAGE_DEBUG_TYPE_CODEVIEW => Ok(DebugData::CodeView(code_view(&self)?)),
			IMAGE_DEBUG_TYPE_MISC => Ok(DebugData::Misc(dbg(&self)?)),
			IMAGE_DEBUG_TYPE_POGO => Ok(DebugData::Pgo(pgo(&self)?)),
			_ => Ok(DebugData::Unknown(self.data())),
		}
	}
}
impl<'a, P: Pe<'a>> fmt::Debug for DebugDirectoryEntry<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("DebugDirectoryEntry")
			.field("type", &crate::stringify::DebugType(self.image.Type).to_str().ok_or(self.image.Type))
			.field("time_date_stamp", &self.image.TimeDateStamp)
			.field("version", &self.image.Version)
			.field("entry", &self.entry())
			.finish()
	}
}

//----------------------------------------------------------------

pub use crate::wrap::debug::*;

fn code_view<'a, P: Pe<'a>>(dir: &DebugDirectoryEntry<'a, P>) -> Result<CodeView<'a>> {
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

fn dbg<'a, P: Pe<'a>>(dir: &DebugDirectoryEntry<'a, P>) -> Result<DebugMisc<'a>> {
	let data = dir.data().ok_or(Error::Bounds)?;
	if data.len() < mem::size_of::<IMAGE_DEBUG_MISC>() {
		return Err(Error::Bounds);
	}
	if !data.as_ptr().aligned_to(4) {
		return Err(Error::Misaligned);
	}
	let image = unsafe { &*(data.as_ptr() as *const IMAGE_DEBUG_MISC) };
	Ok(DebugMisc { image })
}

fn pgo<'a, P: Pe<'a>>(dir: &DebugDirectoryEntry<'a, P>) -> Result<Pgo<'a>> {
	let data = dir.data().ok_or(Error::Bounds)?;
	if data.len() < 4 {
		return Err(Error::Bounds);
	}
	if !data.as_ptr().aligned_to(4) {
		return Err(Error::Misaligned);
	}
	let len = data.len() / 4;
	let image = unsafe { slice::from_raw_parts(data.as_ptr() as *const u32, len) };
	Ok(Pgo { image })
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
	use crate::util::serde_helper::*;

	use super::{DebugDirectory, DebugDirectoryEntry, Pe};

	impl<'a, P: Pe<'a>> Serialize for DebugDirectory<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			serializer.collect_seq(self.into_iter())
		}
	}
	impl<'a, P: Pe<'a>> Serialize for DebugDirectoryEntry<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let is_human_readable = serializer.is_human_readable();
			let mut state = serializer.serialize_struct("DebugDirectoryEntry", 4)?;
			if is_human_readable {
				state.serialize_field("type", &crate::stringify::DebugType(self.image.Type).to_str())?;
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
}

//----------------------------------------------------------------

#[cfg(test)]
pub(crate) fn test_debug<'a, P: Pe<'a>>(pe: P) -> Result<()> {
	let debug = pe.debug()?;
	for dir in debug {
		let _data = dir.data();
		match dir.entry() {
			Ok(DebugData::CodeView(cv)) => {
				let _format = cv.format();
				let _pdb_file_name = cv.pdb_file_name();
			},
			Ok(DebugData::Misc(_misc)) => (),
			Ok(DebugData::Pgo(pgo)) => for _sec in pgo {},
			Ok(DebugData::Unknown(_data)) => (),
			Err(_) => (),
		}
	}
	Ok(())
}
