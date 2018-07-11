/*!
Export Directory.

The export directory contains a list of symbols, well, exported by this module.
A symbol can refer to a function, static data or a forwarded reference to an exported symbol in another module.

Symbols can be exported by name or by their ordinal number. The ordinal number of an exported function is its index in the exported function list plus the ordinal base.

# Examples

```
# #![allow(unused_variables)]
use pelite::pe64::{Pe, PeFile};
use pelite::pe64::exports::GetProcAddress;

# #[allow(dead_code)]
fn example(file: PeFile<'_>) -> pelite::Result<()> {
	// Most convenient way to get the address of an export
	file.get_proc_address("ThrowException")?;

	// Access the export directory
	let exports = file.exports()?;

	// Print the export DLL name
	let dll_name = exports.dll_name()?;
	println!("dll_name: {}", dll_name);

	// To query the exports
	let by = exports.by()?;

	// For example: query an export by name
	by.name("?__autoclassinit2@Passwds@@QEAAX_K@Z")?;

	// For example: query an export by ordinal
	by.ordinal(6)?;

	// For example: iterate over all the exports.
	for result in by.iter() {
		if let Ok(export) = result {
			println!("export: {:?}", export);
		}
	}

	// For example: iterate over the named exports
	for result in by.iter_names() {
		if let (Ok(name), Ok(export)) = result {
			println!("export {}: {:?}", name, export);
		}
	}

	Ok(())
}
```
*/

use std::{fmt, iter, ops, slice};

use error::{Error, Result};
use util::CStr;

use super::image::*;
use super::imports::Import;
use super::Pe;

//----------------------------------------------------------------

/// Exported symbol.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Export<'a> {
	/// Symbol does not exist.
	///
	/// When symbols exported by ordinal are obseleted and removed, they may leave gaps in the Export Address Table.
	None,
	/// Standard exported symbol.
	Symbol(&'a Rva),
	/// This export is forwarded to another dll.
	///
	/// Format of the string is `"DllName.ExportName"`.
	/// For more information see this [blog post](https://blogs.msdn.microsoft.com/oldnewthing/20060719-24/?p=30473) by Raymond Chen.
	Forward(&'a CStr),
}
impl<'a> Export<'a> {
	/// Returns some if the symbol is exported.
	pub fn symbol(self) -> Option<Rva> {
		match self {
			Export::Symbol(&rva) => Some(rva),
			_ => None,
		}
	}
	/// Returns some if the symbol is forwarded.
	pub fn forward(self) -> Option<&'a CStr> {
		match self {
			Export::Forward(name) => Some(name),
			_ => None,
		}
	}
}

//----------------------------------------------------------------

/// Export directory.
///
/// For more information see the [module-level documentation](index.html).
#[derive(Copy, Clone)]
pub struct Exports<'a, P> {
	pe: P,
	datadir: &'a IMAGE_DATA_DIRECTORY,
	image: &'a IMAGE_EXPORT_DIRECTORY,
}
impl<'a, P: Pe<'a> + Copy> Exports<'a, P> {
	pub(crate) fn new(pe: P) -> Result<Exports<'a, P>> {
		let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_EXPORT).ok_or(Error::OOB)?;
		let image = pe.derva(datadir.VirtualAddress)?;
		Ok(Exports { pe, datadir, image })
	}
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Returns the underlying export directory image.
	pub fn image(&self) -> &'a IMAGE_EXPORT_DIRECTORY {
		self.image
	}
	/// Gets the export directory's name for this library.
	pub fn dll_name(&self) -> Result<&'a CStr> {
		self.pe.derva_c_str(self.image.Name)
	}
	/// Gets the ordinal base for the exported functions.
	pub fn ordinal_base(&self) -> Ordinal {
		self.image.Base as Ordinal
	}
	/// Gets the export address table.
	pub fn functions(&self) -> Result<&'a [Rva]> {
		self.pe.derva_slice(self.image.AddressOfFunctions, self.image.NumberOfFunctions as usize)
	}
	/// Gets the name address table.
	///
	/// The values are RVAs to the exported function's name, to find its export look at the name index table with the same index.
	///
	/// The names are sorted allowing binary search lookup.
	pub fn names(&self) -> Result<&'a [Rva]> {
		self.pe.derva_slice(self.image.AddressOfNames, self.image.NumberOfNames as usize)
	}
	/// Gets the name index table.
	///
	/// The values are indices (not ordinals!) into the export address table matching name with the same index in the name address table.
	pub fn name_indices(&self) -> Result<&'a [u16]> {
		self.pe.derva_slice(self.image.AddressOfNameOrdinals, self.image.NumberOfNames as usize)
	}
	/// Query the exports.
	///
	/// This specifically validates whether the functions, names and name indices are valid.
	pub fn by(&self) -> Result<By<'a, P>> {
		let functions = match self.functions() {
			Ok(functions) => functions,
			Err(Error::Null) => &[],
			Err(e) => return Err(e),
		};
		let names = match self.names() {
			Ok(names) => names,
			Err(Error::Null) => &[],
			Err(e) => return Err(e),
		};
		let name_indices = match self.name_indices() {
			Ok(name_indices) => name_indices,
			Err(Error::Null) => &[],
			Err(e) => return Err(e),
		};
		Ok(By { exp: *self, functions, names, name_indices })
	}
	fn is_forwarded(&self, rva: Rva) -> bool {
		// An export is forward if its rva points within data directory bounds
		rva >= self.datadir.VirtualAddress && rva < self.datadir.VirtualAddress + self.datadir.Size
	}
	fn symbol_from_rva(&self, rva: &'a Rva) -> Result<Export<'a>> {
		if *rva == 0 {
			Ok(Export::None)
		}
		else if self.is_forwarded(*rva) {
			let fwd = self.pe.derva_c_str(*rva)?;
			Ok(Export::Forward(fwd))
		}
		else {
			Ok(Export::Symbol(rva))
		}
	}
}
impl<'a, P: 'a + Pe<'a> + Copy> fmt::Debug for Exports<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.by() {
			Ok(by) => by.fmt(f),
			Err(err) => err.fmt(f),
		}
	}
}

//----------------------------------------------------------------

/// Export directory symbol lookup.
#[derive(Copy, Clone)]
pub struct By<'a, P> {
	exp: Exports<'a, P>,
	functions: &'a [Rva],
	names: &'a [Rva],
	name_indices: &'a [u16],
}
impl<'a, P: Pe<'a>> ops::Deref for By<'a, P> {
	type Target = Exports<'a, P>;
	fn deref(&self) -> &Exports<'a, P> {
		&self.exp
	}
}
impl<'a, P: Pe<'a> + Copy> By<'a, P> {
	/// Gets the export address table.
	pub fn functions(&self) -> &'a [Rva] {
		self.functions
	}
	/// Gets the name address table.
	///
	/// The values are RVAs to the exported function's name, to find its export look at the name index table with the same index.
	///
	/// The names are sorted allowing binary search lookup.
	pub fn names(&self) -> &'a [Rva] {
		self.names
	}
	/// Gets the name index table.
	///
	/// The values are indices (not ordinals!) into the export address table matching name with the same index in the name address table.
	pub fn name_indices(&self) -> &'a [u16] {
		self.name_indices
	}
	/// Looks up an `Export` by its ordinal.
	pub fn ordinal(&self, ordinal: Ordinal) -> Result<Export<'a>> {
		let base = self.exp.image.Base;
		if (ordinal as u32) < base {
			Err(Error::OOB)
		}
		else {
			let index = (ordinal as u32 - base) as usize;
			self.index(index)
		}
	}
	/// Looks up an `Export` by its name.
	pub fn name<S: AsRef<[u8]> + ?Sized>(&self, name: &S) -> Result<Export<'a>> {
		self.name_(name.as_ref())
	}
	fn name_(&self, name: &[u8]) -> Result<Export<'a>> {
		// Binary search for the name
		let mut lower_bound = 0;
		let mut upper_bound = self.names.len();
		while lower_bound != upper_bound {
			let i = lower_bound + (upper_bound - lower_bound) / 2;
			let name_rva = self.names[i];
			let name_it = self.exp.pe.derva_c_str(name_rva)?.as_ref();
			use std::cmp::Ordering::*;
			match name.cmp(name_it) {
				Less => upper_bound = i,
				Greater => lower_bound = i + 1,
				Equal => {
					let &index = self.name_indices.get(i).ok_or(Error::OOB)?;
					return self.index(index as usize);
				},
			};
		}
		Ok(Export::None)
	}
	/// Looks up an `Export` by its import.
	pub fn import(&self, import: Import) -> Result<Export<'a>> {
		match import {
			Import::ByName { hint, name } => {
				self.hint_name_(hint, name)
			},
			Import::ByOrdinal { ord } => {
				self.ordinal(ord)
			}
		}
	}
	/// Looks up an export by its index.
	pub fn index(&self, index: usize) -> Result<Export<'a>> {
		let rva = self.functions.get(index).ok_or(Error::OOB)?;
		self.exp.symbol_from_rva(rva)
	}
	/// Looks up an export by its hint.
	pub fn hint(&self, hint: usize) -> Result<Export<'a>> {
		let &index = self.name_indices.get(hint).ok_or(Error::OOB)?;
		self.index(index as usize)
	}
	/// Looks up an export by its hint and falls back to the name if the hint is incorrect.
	pub fn hint_name<S: AsRef<[u8]> + ?Sized>(&self, hint: usize, name: &S) -> Result<Export<'a>> {
		self.hint_name_(hint, name.as_ref())
	}
	fn hint_name_(&self, hint: usize, name: &[u8]) -> Result<Export<'a>> {
		// Try the hint first
		if let Ok(export) = self.hint(hint) {
			// Double check that this is the correct export
			if let Ok(export_name) = self.name_of_hint(hint) {
				if export_name == name {
					return Ok(export);
				}
			}
		}
		// Otherwise fallback to the name
		self.name(name)
	}
	/// Looks up the name for a hint.
	pub fn name_of_hint(&self, hint: usize) -> Result<&'a CStr> {
		let &name_rva = self.names.get(hint).ok_or(Error::OOB)?;
		self.exp.pe.derva_c_str(name_rva)
	}
	/// Given an index in the functions array, gets the named export.
	///
	/// Note that this does a linear scan to find its name,
	/// if this is called in a loop over all the exported functions you are accidentally quadratic.
	///
	/// See [`iter_names`](#method.iter_names) to iterate over the exported names in linear time.
	pub fn name_lookup(&self, index: usize) -> Result<Import<'a>> {
		// Lookup the name index, accidentally quadratic :)
		match self.name_indices.iter().position(|&i| i as usize == index) {
			Some(hint) => {
				// Lookup the name
				let name_rva = self.names[hint];
				let name = self.exp.pe.derva_c_str(name_rva)?;
				Ok(Import::ByName { hint, name })
			},
			None => {
				// Name not found
				let ord = (index as u32 + self.exp.image.Base) as Ordinal;
				Ok(Import::ByOrdinal { ord })
			},
		}
	}
	/// Iterate over exported functions.
	///
	/// Not every exported function has a name, some are exported by ordinal.
	/// Looking up the exported function's name with [`name_lookup`](#method.name_lookup) results in quadratic performance.
	/// If the exported function's name is important consider building a cache or using [`iter_names`](#method.iter_names) instead.
	pub fn iter<'s>(&'s self)
		-> iter::Map<slice::Iter<'a, Rva>, impl 's + Clone + FnMut(&'a Rva) -> Result<Export<'a>>>
	{
		self.functions.iter()
			.map(move |rva| self.symbol_from_rva(rva))
	}
	/// Iterate over functions exported by name.
	pub fn iter_names<'s>(&'s self)
		-> iter::Map<ops::Range<u32>, impl 's + Clone + FnMut(u32) -> (Result<&'a CStr>, Result<Export<'a>>)>
	{
		(0..self.names().len() as u32)
			.map(move |hint| (
				self.name_of_hint(hint as usize),
				self.hint(hint as usize),
			))
	}
	/// Iterate over functions exported by name, returning their name and index in the functions table.
	pub fn iter_name_indices<'s>(&'s self)
		-> iter::Map<ops::Range<u32>, impl 's + Clone + FnMut(u32) -> (Result<&'a CStr>, usize)>
	{
		(0..self.names().len() as u32)
			.map(move |hint| (
				self.name_of_hint(hint as usize),
				self.name_indices[hint as usize] as usize,
			))
	}
}
impl<'a, P: 'a + Pe<'a> + Copy> fmt::Debug for By<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Exports")
			.field("dll_name", &format_args!("{:?}", self.dll_name()))
			.field("time_date_stamp", &self.image.TimeDateStamp)
			.field("version", &format_args!("{}.{}", self.image.MajorVersion, self.image.MinorVersion))
			.field("ordinal_base", &self.ordinal_base())
			.field("functions.len", &self.functions().len())
			.field("names.len", &self.names.len())
			.finish()
	}
}

//----------------------------------------------------------------

/// Convenient way to get an exported address.
pub trait GetProcAddress<'a, T>: Pe<'a> + Copy {
	/// Convenient method to get an exported function.
	///
	/// Note that calling this method many times is less efficient than caching a [`By`](struct.By.html) instance, such is the trade-off for convenience.
	fn get_export(self, name: T) -> Result<Export<'a>>;
	/// Convenient method to get the address of an exported function.
	///
	/// Note that this method does not support forwarded exports and will return `Err(Null)` instead.
	///
	/// Note that calling this method many times is less efficient than caching a [`By`](struct.By.html) instance, such is the trade-off for convenience.
	#[inline(never)]
	fn get_proc_address(self, name: T) -> Result<Va> {
		self.rva_to_va(self.get_export(name)?.symbol().ok_or(Error::Null)?)
	}
}
impl<'a, P: Pe<'a> + Copy> GetProcAddress<'a, Ordinal> for P {
	fn get_export(self, name: Ordinal) -> Result<Export<'a>> {
		self.exports()?.by()?.ordinal(name)
	}
}
impl<'b, 'a, P: Pe<'a> + Copy> GetProcAddress<'a, Import<'b>> for P {
	fn get_export(self, name: Import<'b>) -> Result<Export<'a>> {
		self.exports()?.by()?.import(name)
	}
}
impl<'b, 'a, P: Pe<'a> + Copy, S: AsRef<[u8]> + ?Sized> GetProcAddress<'a, &'b S> for P {
	fn get_export(self, name: &'b S) -> Result<Export<'a>> {
		self.exports()?.by()?.name(name)
	}
}

//----------------------------------------------------------------

/*
	"exports": {
		"dll_name": "Demo.dll",
		"time_date_stamp": 0,
		"version": "0.0",
		"ordinal_base": 1,
		"functions": [ .. ],
		"names": {
			"__autoclassinit": 5,
			..
		}
	}
*/

#[cfg(feature = "serde")]
mod serde {
	use util::serde_helper::*;
	use super::{Pe, Exports, By};

	#[cfg(feature = "serde")]
	impl<'a, P: 'a + Pe<'a> + Copy> Serialize for Exports<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			self.by().ok().serialize(serializer)
		}
	}
	#[cfg(feature = "serde")]
	impl<'a, P: 'a + Pe<'a> + Copy> Serialize for By<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("Exports", 6)?;
			state.serialize_field("dll_name", &self.dll_name().ok())?;
			state.serialize_field("time_date_stamp", &self.image.TimeDateStamp)?;
			state.serialize_field("version", &format_args!("{}.{}", self.image.MajorVersion, self.image.MinorVersion))?;
			state.serialize_field("ordinal_base", &self.ordinal_base())?;
			state.serialize_field("functions", &self.functions())?;
			let names = self.iter_name_indices().filter_map(|(name, index)| {
				name.ok().and_then(|name| name.to_str().ok()).map(|name| (name, index))
			});
			state.serialize_field("names", &SerdeKV(names))?;
			state.end()
		}
	}
}
