/*!
Import Directory.

# Examples

```
# #![allow(unused_variables)]
use pelite::pe64::{Pe, PeFile};

# #[allow(dead_code)]
fn example(file: PeFile) -> pelite::Result<()> {
	// Access the import directory
	let imports = file.imports()?;

	// Iterate over the import descriptors
	for desc in imports {
		// DLL being imported from
		let dll_name = desc.dll_name()?;

		// Import Address Table
		let iat = desc.iat()?;

		// Iterate over the imported functions from this DLL
		let int = desc.int()?;
		for (va, import) in Iterator::zip(iat, int) {}
	}

	Ok(())
}
```
*/

use std::{fmt, slice};

use error::{Error, Result};
use util::CStr;

use super::image::*;
use super::Pe;

//----------------------------------------------------------------

/// Imported symbol.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Import<'a> {
	/// Imported by name.
	///
	/// The hint is an index in the export names table that may contain the desired symbol.
	/// For more information see this [blog post](https://blogs.msdn.microsoft.com/oldnewthing/20100317-00/?p=14573) by Raymond Chen.
	ByName { hint: usize, name: &'a CStr },
	/// Imported by ordinal.
	ByOrdinal { ord: Ordinal }
}

//----------------------------------------------------------------

/// Import directory.
#[derive(Copy, Clone)]
pub struct Imports<'a, P> {
	pe: P,
	image: &'a [IMAGE_IMPORT_DESCRIPTOR],
}
impl<'a, P: Pe<'a> + Copy> Imports<'a, P> {
	pub(crate) fn new(pe: P) -> Result<Imports<'a, P>> {
		let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_IMPORT).ok_or(Error::OOB)?;
		let image = pe.derva_slice_f(datadir.VirtualAddress, |image: &IMAGE_IMPORT_DESCRIPTOR| image.is_null())?;
		Ok(Imports { pe, image })
	}
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Returns the underlying import directory image array.
	pub fn image(&self) -> &'a [IMAGE_IMPORT_DESCRIPTOR] {
		self.image
	}
}
impl<'a, P: Pe<'a> + Copy> IntoIterator for Imports<'a, P> {
	type Item = Desc<'a, P>;
	type IntoIter = DescIter<'a, P>;
	fn into_iter(self) -> DescIter<'a, P> {
		DescIter {
			pe: self.pe,
			iter: self.image.iter(),
		}
	}
}
impl<'a, P: Pe<'a> + Copy> fmt::Debug for Imports<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_list()
			.entries(self.into_iter())
			.finish()
	}
}

//----------------------------------------------------------------

#[derive(Clone)]
pub struct DescIter<'a, P> {
	pe: P,
	iter: slice::Iter<'a, IMAGE_IMPORT_DESCRIPTOR>,
}
def_iter!(struct DescIter -> IMAGE_IMPORT_DESCRIPTOR, Desc<'a, P>; this |image| Desc { pe: this.pe, image });

//----------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct Desc<'a, P> {
	pe: P,
	image: &'a IMAGE_IMPORT_DESCRIPTOR,
}
impl<'a, P: Pe<'a> + Copy> Desc<'a, P> {
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Returns the underlying import descriptor image.
	pub fn image(&self) -> &'a IMAGE_IMPORT_DESCRIPTOR {
		self.image
	}
	/// Gets the name of the DLL imported from.
	pub fn dll_name(&self) -> Result<&'a CStr> {
		self.pe.derva_c_str(self.image.Name)
	}
	/// Gets the import address table.
	///
	/// After being loaded as a library their values are resolved to the addresses of the imported functions.
	///
	/// Otherwise these contain references to the imported functions.
	/// See [`import_from_va`](struct.Desc.html#import_from_va) to get their names.
	pub fn iat(&self) -> Result<slice::Iter<'a, Va>> {
		self.pe.derva_slice_s(self.image.FirstThunk, 0).map(|iat| iat.iter())
	}
	/// Gets the import name table.
	pub fn int(self) -> Result<IntIter<'a, P>> {
		let slice = self.pe.derva_slice_s(self.image.OriginalFirstThunk, 0)?;
		Ok(IntIter {
			pe: self.pe,
			iter: slice.iter(),
		})
	}
}
impl<'a, P: Pe<'a> + Copy> fmt::Debug for Desc<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Imports")
			.field("dll_name", &format_args!("{:?}", self.dll_name()))
			.field("time_date_stamp", &self.image.TimeDateStamp)
			.field("iat.len", &format_args!("{:?}", &self.iat().map(|iter| iter.len())))
			.field("int.len", &format_args!("{:?}", &self.int().map(|iter| iter.as_slice().len())))
			.finish()
	}
}

//----------------------------------------------------------------

/// Gets the import from the import name table.
///
/// These aren't actually virtual addresses.
/// This function will decode them to get the import.
fn import_from_va<'a, P: Pe<'a> + Copy>(pe: P, va: Va) -> Result<Import<'a>> {
	if va & IMAGE_ORDINAL_FLAG == 0 {
		// TODO! Validate that this really is an Rva in PE32+?
		let rva = va as Rva;
		let hint = pe.derva::<u16>(rva)?;
		let name = pe.derva_c_str(rva + 2)?;
		Ok(Import::ByName { hint: *hint as usize, name })
	}
	else {
		Ok(Import::ByOrdinal { ord: va as Ordinal })
	}
}

#[derive(Clone)]
pub struct IntIter<'a, P> {
	pe: P,
	iter: slice::Iter<'a, Va>
}
def_iter!(struct IntIter -> Va, Result<Import<'a>>; this |&va| import_from_va(this.pe, va));
