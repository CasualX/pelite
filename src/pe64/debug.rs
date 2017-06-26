/*!
Debug Directory.

# Examples

```
# #![allow(unused_variables)]
use pelite::pe64::{Pe, PeFile, debug};

# #[allow(dead_code)]
fn example(file: PeFile) -> pelite::Result<()> {
	// Access the debug directory
	let debug = file.debug()?;

	// Iterate over the entries
	for dir in debug {

		// Interpret this debug info
		match dir.info()? {

			// And if it matches CodeView RSDS debug format
			debug::Info::CvRSDS { image, pdb_file_name } => {

				// Print the PDB file name path
				println!("PDB: {}", pdb_file_name);
			},
			_ => (),
		};
	}

	Ok(())
}
```
*/

use ::std::{slice, fmt, mem};

use super::image::*;
use super::{Pe};
use ::{Error, Result};
use ::util::CStr;

//----------------------------------------------------------------

/// Debug directory.
#[derive(Copy, Clone)]
pub struct Debug<'a, P> {
	pe: P,
	image: &'a [IMAGE_DEBUG_DIRECTORY],
}
impl<'a, P: Pe<'a> + Copy> Debug<'a, P> {
	#[doc(hidden)]
	pub fn new(pe: P) -> Result<Debug<'a, P>> {
		let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_DEBUG).ok_or(Error::OOB)?;
		let (len, rem) = (
			datadir.Size as usize / mem::size_of::<IMAGE_DEBUG_DIRECTORY>(),
			datadir.Size as usize % mem::size_of::<IMAGE_DEBUG_DIRECTORY>(),
		);
		if rem != 0 {
			return Err(Error::Corrupt);
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

//----------------------------------------------------------------

#[derive(Clone)]
pub struct Iter<'a, P> {
	pe: P,
	iter: slice::Iter<'a, IMAGE_DEBUG_DIRECTORY>,
}
def_iter!(struct Iter -> IMAGE_DEBUG_DIRECTORY, Dir<'a, P>; this |image| Dir { pe: this.pe, image });

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
	/// Gets the referenced debug info.
	pub fn info(&self) -> Result<Info<'a>> {
		let bytes = self.pe.slice_rva(self.image.AddressOfRawData, self.image.SizeOfData as usize, 4)?;
		match self.image.Type {
			IMAGE_DEBUG_TYPE_CODEVIEW => {
				if bytes.len() >= 4 {
					let cv_sig = &bytes[..4];
					// CodeView "NB10"
					if cv_sig == &[0x4E, 0x42, 0x31, 0x30] {
						if bytes.len() >= 16 {
							let image = unsafe { &*(bytes.as_ptr() as *const IMAGE_DEBUG_CV_INFO_PDB20) };
							let pdb_file_name = CStr::from_bytes(&bytes[16..])?;
							Ok(Info::CvNB10 { image, pdb_file_name })
						}
						else {
							Err(Error::Corrupt)
						}
					}
					// CodeView "RSDS"
					else if cv_sig == &[0x52, 0x53, 0x44, 0x53] {
						if bytes.len() >= 24 {
							let image = unsafe { &*(bytes.as_ptr() as *const IMAGE_DEBUG_CV_INFO_PDB70) };
							let pdb_file_name = CStr::from_bytes(&bytes[24..])?;
							Ok(Info::CvRSDS { image, pdb_file_name })
						}
						else {
							Err(Error::Corrupt)
						}
					}
					else {
						Ok(Info::Unknown)
					}
				}
				else {
					Err(Error::Corrupt)
				}
			},
			IMAGE_DEBUG_TYPE_MISC => {
				if bytes.len() >= 12 {
					let image = unsafe { &*(bytes.as_ptr() as *const IMAGE_DEBUG_MISC) };
					Ok(Info::Dbg { image })
				}
				else {
					Err(Error::Corrupt)
				}
			},
			_ => Ok(Info::Unknown),
		}
	}
}

//----------------------------------------------------------------

#[derive(Copy, Clone)]
pub enum Info<'a> {
	Unknown,
	CvNB10 { image: &'a IMAGE_DEBUG_CV_INFO_PDB20, pdb_file_name: &'a CStr },
	CvRSDS { image: &'a IMAGE_DEBUG_CV_INFO_PDB70, pdb_file_name: &'a CStr },
	Dbg { image: &'a IMAGE_DEBUG_MISC },
}

//----------------------------------------------------------------
// Formatting

impl<'a, P: Pe<'a> + Copy> fmt::Debug for Debug<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for dir in *self {
			dir.fmt(f)?;
		}
		Ok(())
	}
}

impl<'a, P: Pe<'a> + Copy> fmt::Debug for Dir<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.image.fmt(f)?;
		match self.info() {
			Ok(info) => info.fmt(f)?,
			e @ Err(_) => e.fmt(f)?,
		};
		f.write_str("\n")
	}
}

impl<'a> fmt::Debug for Info<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Info::Unknown => write!(f, "Unknown\n"),
			Info::CvNB10 { image, pdb_file_name } => {
				write!(f, "{:?}  PdbFileName:      {}\n", image, pdb_file_name)
			},
			Info::CvRSDS { image, pdb_file_name } => {
				write!(f, "{:?}  PdbFileName:      {}\n", image, pdb_file_name)
			},
			Info::Dbg { image } => {
				write!(f, "{:?}", image)
			},
		}
	}
}
