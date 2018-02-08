/*!
Export Directory.

# Examples

```
# #![allow(unused_variables)]
use pelite::pe64::{Pe, PeFile};

# #[allow(dead_code)]
fn example(file: PeFile) -> pelite::Result<()> {
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
	for export_result in by.iter() {
		if let Ok(export) = export_result {
			println!("export: {:?}", export);
		}
	}

	// For example: iterate over the named exports
	for result in by.iter_names() {
		if let (Ok(export), Ok(name)) = result {
			println!("export {}: {:?}", name, export);
		}
	}

	Ok(())
}
```

*/

use std::{fmt, ops, slice};

use error::{Error, Result};
use util::CStr;

use super::image::*;
use super::imports::Import;
use super::Pe;

//----------------------------------------------------------------

/// Exported symbol.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
		self.pe.derva_str(self.image.Name)
	}
	/// Gets the ordinal base for the exported functions.
	///
	/// Indices in the functions array are hints. A hint is the export's ordinal + `self.ordinal_base()`.
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
	pub fn by(self) -> Result<By<'a, P>> {
		let functions = self.functions().or_else(|e| if e == Error::Null { Ok(&[]) } else { Err(e) })?;
		let names = self.names().or_else(|e| if e == Error::Null { Ok(&[]) } else { Err(e) })?;
		let name_indices = self.name_indices().or_else(|e| if e == Error::Null { Ok(&[]) } else { Err(e) })?;
		Ok(By { exp: self, functions, names, name_indices })
	}
	fn is_forwarded(&self, rva: Rva) -> bool {
		// An export is forward if its rva points within data directory bounds
		rva >= self.datadir.VirtualAddress && rva < self.datadir.VirtualAddress + self.datadir.Size
	}
	fn symbol_from_rva(&self, rva: &'a Rva) -> Result<Export<'a>> {
		if *rva == BADRVA {
			Ok(Export::None)
		}
		else if self.is_forwarded(*rva) {
			let fwd = self.pe.derva_str(*rva)?;
			Ok(Export::Forward(fwd))
		}
		else {
			Ok(Export::Symbol(rva))
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
			let name_it = self.exp.pe.derva_str(name_rva)?.as_ref();
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
				// Try the hint first
				if let Ok(export) = self.hint(hint) {
					// Double check that this is the correct export
					if let Ok(export_name) = self.hint_name(hint) {
						if export_name == name {
							return Ok(export);
						}
					}
				}
				// Otherwise just look up the name
				self.name(name)
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
	/// Looks up the name for a hint.
	pub fn hint_name(&self, hint: usize) -> Result<&'a CStr> {
		let &name_rva = self.names.get(hint).ok_or(Error::OOB)?;
		self.exp.pe.derva_str(name_rva)
	}
	/// Given an index in the functions array, gets the named export.
	///
	/// Note that this does a linear scan to find its name,
	/// if this is called in a loop over all the exported functions you are accidentally quadratic.
	///
	/// See [`iter_names`](#iter_names) to iterate over the exported names in linear time.
	pub fn name_lookup(&self, index: usize) -> Result<Import<'a>> {
		// Lookup the name index, accidentally quadratic :)
		match self.name_indices.iter().position(|&i| i as usize == index) {
			Some(hint) => {
				// Lookup the name
				let name_rva = self.names[hint];
				let name = self.exp.pe.derva_str(name_rva)?;
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
	/// Looking up the exported function's name with [`name_lookup`](#name_lookup) results in quadratic performance.
	/// If the exported function's name is important consider building a cache or using [`iter_names`](#iter_names) instead.
	pub fn iter<'s>(&'s self) -> IterByFn<'s, 'a, P> {
		IterByFn { by: self, fns: self.functions().iter() }
	}
	/// Iterate over functions exported by name.
	pub fn iter_names<'s>(&'s self) -> IterByName<'s, 'a, P> {
		IterByName { by: self, hints: 0..self.image.NumberOfNames }
	}
}

//----------------------------------------------------------------
// Export iteration

/// Exported functions iterator.
///
/// Created with [`By::iter`](struct.By.html#method.iter).
#[derive(Clone)]
pub struct IterByFn<'s, 'a: 's, P: 'a> {
	by: &'s By<'a, P>,
	fns: slice::Iter<'a, Rva>,
}
impl<'s, 'a: 's, P: Pe<'a> + Copy> Iterator for IterByFn<'s, 'a, P> {
	type Item = Result<Export<'a>>;
	fn next(&mut self) -> Option<Self::Item> {
		let by = self.by;
		self.fns.next().map(|rva| by.symbol_from_rva(rva))
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.fns.size_hint()
	}
	fn count(self) -> usize {
		self.fns.count()
	}
	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		let by = self.by;
		self.fns.nth(n).map(|rva| by.symbol_from_rva(rva))
	}
	fn last(self) -> Option<Self::Item> {
		let by = self.by;
		self.fns.last().map(|rva| by.symbol_from_rva(rva))
	}
}
impl<'s, 'a: 's, P: Pe<'a> + Copy> DoubleEndedIterator for IterByFn<'s, 'a, P> {
	fn next_back(&mut self) -> Option<Self::Item> {
		let by = self.by;
		self.fns.next_back().map(|rva| by.symbol_from_rva(rva))
	}
}
impl<'s, 'a: 's, P: Pe<'a> + Copy> ExactSizeIterator for IterByFn<'s, 'a, P> {}

/// Exported names iterator.
///
/// Created with [`By::iter_names`](struct.By.html#method.iter_names).
#[derive(Clone)]
pub struct IterByName<'s, 'a: 's, P: 'a> {
	by: &'s By<'a, P>,
	hints: ops::Range<u32>,
}
impl<'s, 'a: 's, P: Pe<'a> + Copy> Iterator for IterByName<'s, 'a, P> {
	type Item = (Result<Export<'a>>, Result<&'a CStr>);
	fn next(&mut self) -> Option<Self::Item> {
		let by = self.by;
		self.hints.next().map(|hint| (by.hint(hint as usize), by.hint_name(hint as usize)))
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.hints.size_hint()
	}
	fn count(self) -> usize {
		self.hints.count()
	}
	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		let by = self.by;
		self.hints.nth(n).map(|hint| (by.hint(hint as usize), by.hint_name(hint as usize)))
	}
	fn last(self) -> Option<Self::Item> {
		let by = self.by;
		self.hints.last().map(|hint| (by.hint(hint as usize), by.hint_name(hint as usize)))
	}
}
impl<'s, 'a: 's, P: Pe<'a> + Copy> DoubleEndedIterator for IterByName<'s, 'a, P> {
	fn next_back(&mut self) -> Option<Self::Item> {
		let by = self.by;
		self.hints.next_back().map(|hint| (by.hint(hint as usize), by.hint_name(hint as usize)))
	}
}
impl<'s, 'a: 's, P: Pe<'a> + Copy> ExactSizeIterator for IterByName<'s, 'a, P> {}

//----------------------------------------------------------------
// Formatting

use strings::Fmt;

impl<'a> fmt::Display for Export<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Export::None => f.write_str("None"),
			Export::Symbol(rva) => write!(f, "{:·>8X}", rva),
			Export::Forward(fwd) => write!(f, "{}", fwd),
		}
	}
}

impl<'a, P: Pe<'a> + Copy> fmt::Debug for Exports<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.by() {
			Ok(by) => by.fmt(f),
			Err(err) => err.fmt(f),
		}
	}
}

fn export_fmt<'a, P: Pe<'a> + Copy>(f: &mut fmt::Formatter, by: &By<'a, P>, index: usize) -> fmt::Result {
	match by.index(index) {
		Ok(Export::None) => Ok(()),
		Ok(export) => {
			let ord = (index as u32 + by.image.Base) as Ordinal;
			write!(f, "\n{:>5} ", ord)?;
			match export {
				Export::None => Ok(()),
				Export::Symbol(rva) => {
					match by.name_lookup(index) {
						Ok(Import::ByName { hint, name }) => {
							write!(f, "{:·>4X} {:·>8X} {}", hint, rva, name)
						},
						Ok(Import::ByOrdinal { .. }) => {
							write!(f, "     {:·>8X}", rva)
						},
						err @ Err(_) => write!(f, "{:?}", err),
					}
				},
				Export::Forward(fwd) => {
					match by.name_lookup(index) {
						Ok(Import::ByName { hint, name }) => {
							write!(f, "{:·>4X}          {} (forwarded to {})", hint, name, fwd)
						},
						Ok(Import::ByOrdinal { .. }) => {
							write!(f, "              (forwarded to {})", fwd)
						},
						err @ Err(_) => write!(f, "{:?}", err),
					}
				}
			}
		},
		err @ Err(_) => write!(f, "{:?}", err),
	}
}

impl<'a, P: Pe<'a> + Copy> fmt::Debug for By<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			#"Exports for {}\n", Fmt(|f| {
				match self.dll_name() {
					Ok(name) => fmt::Display::fmt(name, f),
					err @ Err(_) => err.fmt(f),
				}
			}),
			#"  TimeDateStamp:   {}\n", self.image.TimeDateStamp,
			#"  Version:         {}.{}\n", self.image.MajorVersion, self.image.MinorVersion,
			#"  OrdinalBase:     {}\n", self.image.Base,
			#"  # of functions:  {}\n", self.functions.len(),
			#"  # of names:      {}\n\n", self.names.len(),
			#"  ord hint RVA      name{}\n", Fmt(|f| {
				for index in 0..self.functions.len() {
					export_fmt(f, self, index)?;
				}
				Ok(())
			}),
		)
	}
}
