/*!
Import Directory.

The import directory lists all the module dependencies and their imported symbols by this module.

# Examples

```
# #![allow(unused_variables)]
use pelite::pe64::{Pe, PeFile};

# #[allow(dead_code)]
fn example(file: PeFile<'_>) -> pelite::Result<()> {
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

use std::{fmt, iter, slice};

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
///
/// For more information see the [module-level documentation](index.html).
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
	type IntoIter = Iter<'a, P>;
	fn into_iter(self) -> Iter<'a, P> {
		Iter {
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
pub struct Iter<'a, P> {
	pe: P,
	iter: slice::Iter<'a, IMAGE_IMPORT_DESCRIPTOR>,
}
impl<'a, P: Pe<'a> + Copy> Iter<'a, P> {
	pub fn image(&self) -> &'a [IMAGE_IMPORT_DESCRIPTOR] {
		self.iter.as_slice()
	}
}
impl<'a, P: Pe<'a> + Copy> Iterator for Iter<'a, P> {
	type Item = Desc<'a, P>;
	fn next(&mut self) -> Option<Desc<'a, P>> {
		self.iter.next().map(|image| Desc { pe: self.pe, image })
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
	fn count(self) -> usize {
		self.iter.count()
	}
	fn nth(&mut self, n: usize) -> Option<Desc<'a, P>> {
		self.iter.nth(n).map(|image| Desc { pe: self.pe, image })
	}
}
impl<'a, P: Pe<'a> + Copy> DoubleEndedIterator for Iter<'a, P> {
	fn next_back(&mut self) -> Option<Desc<'a, P>> {
		self.iter.next_back().map(|image| Desc { pe: self.pe, image })
	}
}
impl<'a, P: Pe<'a> + Copy> ExactSizeIterator for Iter<'a, P> {}
impl<'a, P: Pe<'a> + Copy> iter::FusedIterator for Iter<'a, P> {}

//----------------------------------------------------------------

/// Import library descriptor.
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
	/// Gets the import from the import name table.
	///
	/// These aren't actually virtual addresses.
	/// This function will decode them to get the import.
	fn import_from_va(&self, va: Va) -> Result<Import<'a>> {
		if va & IMAGE_ORDINAL_FLAG == 0 {
			// TODO! Validate that this really is an Rva in PE32+?
			let rva = va as Rva;
			let hint = self.pe.derva::<u16>(rva)?;
			let name = self.pe.derva_c_str(rva + 2)?;
			Ok(Import::ByName { hint: *hint as usize, name })
		}
		else {
			Ok(Import::ByOrdinal { ord: va as Ordinal })
		}
	}
	/// Gets the import address table.
	///
	/// After being loaded as a library their values are resolved to the addresses of the imported functions.
	///
	/// Otherwise these contain references to the imported functions.
	/// See [`import_from_va`](struct.Desc.html#import_from_va) to get their names.
	pub fn iat(&self) -> Result<slice::Iter<'a, Va>> {
		let slice = self.pe.derva_slice_s(self.image.FirstThunk, 0)?;
		Ok(slice.iter())
	}
	/// Gets the import name table.
	pub fn int(self) -> Result<iter::Map<slice::Iter<'a, Va>, impl Clone + FnMut(&'a Va) -> Result<Import<'a>>>> {
		let slice = self.pe.derva_slice_s(self.image.OriginalFirstThunk, 0)?;
		Ok(slice.iter().map(move |&va| self.import_from_va(va)))
	}
}
impl<'a, P: Pe<'a> + Copy> fmt::Debug for Desc<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Imports")
			.field("dll_name", &format_args!("{:?}", self.dll_name()))
			.field("iat.len", &format_args!("{:?}", &self.iat().map(|iter| iter.len())))
			.field("int.len", &format_args!("{:?}", &self.int().map(|iter| iter.len())))
			.finish()
	}
}
