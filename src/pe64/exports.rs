use super::*;

impl<'a> PeFile<'a> {
	#[doc = include_str!("../docs/exports.md")]
	#[inline]
	pub fn exports(self) -> Result<ExportDirectory<'a, Self>> {
		ExportDirectory::try_from(self)
	}

	#[doc = include_str!("../docs/get_proc_address.md")]
	#[inline(never)]
	pub fn get_proc_address<T>(self, name: T) -> Result<Va> where Self: GetProcAddress<'a, T> {
		let export = <Self as GetProcAddress<'a, T>>::get_export(self, name)?;
		self.rva_to_va(export.symbol().ok_or(Error::Null)?)
	}
}

impl<'a> PeView<'a> {
	#[doc = include_str!("../docs/exports.md")]
	#[inline]
	pub fn exports(self) -> Result<ExportDirectory<'a, Self>> {
		ExportDirectory::try_from(self)
	}

	#[doc = include_str!("../docs/get_proc_address.md")]
	#[inline(never)]
	pub fn get_proc_address<T>(self, name: T) -> Result<Va> where Self: GetProcAddress<'a, T> {
		let export = <Self as GetProcAddress<'a, T>>::get_export(self, name)?;
		self.rva_to_va(export.symbol().ok_or(Error::Null)?)
	}
}

//----------------------------------------------------------------

#[doc(inline)]
pub use crate::Export;

//----------------------------------------------------------------

/// Export directory.
///
/// The export directory contains the symbols exported by an image. A symbol can
/// refer to a function, static data, or a forwarded export in another module.
///
/// Symbols can be exported by name or ordinal. An exported function's ordinal is
/// its index in the export address table plus the ordinal base.
///
/// # Examples
///
/// ```
/// # #![allow(unused_variables)]
/// use pelite::pe64::{Pe, PeFile};
///
/// # #[allow(dead_code)]
/// fn example(file: PeFile<'_>) -> pelite::Result<()> {
/// 	// Most convenient way to get the address of an export
/// 	file.get_proc_address("ThrowException")?;
///
/// 	// Access the export directory
/// 	let exports = file.exports()?;
///
/// 	// Print the export DLL name
/// 	let dll_name = exports.dll_name()?;
/// 	println!("dll_name: {}", dll_name);
///
/// 	// Build the validated lookup tables used to query exports
/// 	let by = exports.by()?;
///
/// 	// Query an export by name
/// 	by.name("?__autoclassinit2@Passwds@@QEAAX_K@Z")?;
///
/// 	// Query an export by ordinal
/// 	by.ordinal(6)?;
///
/// 	// Iterate over all exports, including ordinal-only exports
/// 	for result in by.iter() {
/// 		if let Ok(export) = result {
/// 			println!("export: {:?}", export);
/// 		}
/// 	}
///
/// 	// Iterate over named exports
/// 	for result in by.iter_names() {
/// 		if let (Ok(name), Ok(export)) = result {
/// 			println!("export {}: {:?}", name, export);
/// 		}
/// 	}
///
/// 	Ok(())
/// }
/// ```
#[derive(Copy, Clone)]
pub struct ExportDirectory<'a, P> {
	pe: P,
	datadir: &'a IMAGE_DATA_DIRECTORY,
	image: &'a IMAGE_EXPORT_DIRECTORY,
}
impl<'a, P: Copy + Pe<'a>> ExportDirectory<'a, P> {
	pub(crate) fn try_from(pe: P) -> Result<ExportDirectory<'a, P>> {
		let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_EXPORT).ok_or(Error::Bounds)?;
		if datadir.VirtualAddress == 0 {
			return Err(Error::Null);
		}
		let image = pe.derva(datadir.VirtualAddress)?;
		Ok(ExportDirectory { pe, datadir, image })
	}
	/// Returns the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Returns the underlying export directory image.
	pub fn image(&self) -> &'a IMAGE_EXPORT_DIRECTORY {
		self.image
	}
	/// Returns the export directory's name for this library.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The name RVA is zero.
	/// * [`Encoding`][crate::Error::Encoding]: The name has no null terminator.
	/// * [`Bounds`][crate::Error::Bounds], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The name cannot be read from the image.
	pub fn dll_name(&self) -> Result<&'a CStr> {
		self.pe.derva_c_str(self.image.Name)
	}
	/// Returns the ordinal base for the exported functions.
	pub fn ordinal_base(&self) -> Ordinal {
		self.image.Base as Ordinal
	}
	/// Returns the export address table.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The function table RVA is zero.
	/// * [`Overflow`][crate::Error::Overflow]: The declared table length overflows.
	/// * [`Bounds`][crate::Error::Bounds], [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The function table cannot be read from the image.
	pub fn functions(&self) -> Result<&'a [Rva]> {
		self.pe.derva_slice(self.image.AddressOfFunctions, self.image.NumberOfFunctions as usize)
	}
	/// Returns the name address table.
	///
	/// The values are RVAs to the exported function's name, to find its export look at the name index table with the same index.
	///
	/// The names are sorted allowing binary search lookup.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The name table RVA is zero.
	/// * [`Overflow`][crate::Error::Overflow]: The declared table length overflows.
	/// * [`Bounds`][crate::Error::Bounds], [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The name table cannot be read from the image.
	pub fn names(&self) -> Result<&'a [Rva]> {
		self.pe.derva_slice(self.image.AddressOfNames, self.image.NumberOfNames as usize)
	}
	/// Returns the name index table.
	///
	/// The values are indices (not ordinals!) into the export address table matching name with the same index in the name address table.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The name index table RVA is zero.
	/// * [`Overflow`][crate::Error::Overflow]: The declared table length overflows.
	/// * [`Bounds`][crate::Error::Bounds], [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The name index table cannot be read from the image.
	pub fn name_indices(&self) -> Result<&'a [u16]> {
		self.pe.derva_slice(self.image.AddressOfNameOrdinals, self.image.NumberOfNames as usize)
	}
	/// Returns validated export lookup tables.
	///
	/// This specifically validates whether the functions, names and name indices are valid.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: A nonempty table has a zero RVA.
	/// * [`Overflow`][crate::Error::Overflow]: A declared table length overflows.
	/// * [`Bounds`][crate::Error::Bounds], [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: A declared export table cannot be read.
	pub fn by(&self) -> Result<ExportBy<'a, P>> {
		// A null table RVA is valid only when the corresponding table is empty.
		// Keeping the declared lengths intact here is important:
		// methods such as `iter_name_indices` rely on the two name tables having equal lengths.
		let functions = if self.image.NumberOfFunctions == 0 { &[] } else { self.functions()? };
		let names = if self.image.NumberOfNames == 0 { &[] } else { self.names()? };
		let name_indices = if self.image.NumberOfNames == 0 { &[] } else { self.name_indices()? };
		Ok(ExportBy {
			exp: *self,
			functions,
			names,
			name_indices,
		})
	}
	fn is_forwarded(&self, rva: Rva) -> bool {
		// An export is forward if its rva points within data directory bounds
		rva.wrapping_sub(self.datadir.VirtualAddress) < self.datadir.Size
	}
	pub(crate) fn symbol_from_rva(&self, rva: &'a Rva) -> Result<Export<'a>> {
		if *rva == 0 {
			Err(Error::Null)
		}
		else if self.is_forwarded(*rva) {
			// Forwarder strings must be wholly contained in the export directory.
			// Besides enforcing the PE format, bounding the scan prevents a corrupt
			// entry from borrowing a terminator from unrelated section data.
			let offset = rva.wrapping_sub(self.datadir.VirtualAddress) as usize;
			let remaining = self.datadir.Size as usize - offset;
			let bytes = self.pe.slice_bytes(*rva)?;
			let bytes = &bytes[..cmp::min(bytes.len(), remaining)];
			let fwd = CStr::from_bytes(bytes).ok_or(Error::Encoding)?;
			Ok(Export::Forward(fwd))
		}
		else {
			Ok(Export::Symbol(rva))
		}
	}
}
impl<'a, P: Copy + Pe<'a>> fmt::Debug for ExportDirectory<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("ExportDirectory")
			.field("dll_name", &format_args!("{:?}", self.dll_name()))
			.field("time_date_stamp", &self.image.TimeDateStamp)
			.field("version", &self.image.Version)
			.field("ordinal_base", &self.ordinal_base())
			.field("functions.len", &format_args!("{:?}", self.functions().map(<[_]>::len)))
			.field("names.len", &format_args!("{:?}", self.names().map(<[_]>::len)))
			.finish()
	}
}

//----------------------------------------------------------------

/// Export directory symbol lookup.
#[derive(Copy, Clone)]
pub struct ExportBy<'a, P> {
	exp: ExportDirectory<'a, P>,
	functions: &'a [Rva],
	names: &'a [Rva],
	name_indices: &'a [u16],
}
impl<'a, P: Copy + Pe<'a>> ops::Deref for ExportBy<'a, P> {
	type Target = ExportDirectory<'a, P>;
	fn deref(&self) -> &ExportDirectory<'a, P> {
		&self.exp
	}
}
impl<'a, P: Copy + Pe<'a>> ExportBy<'a, P> {
	/// Returns the export address table.
	pub fn functions(&self) -> &'a [Rva] {
		self.functions
	}
	/// Returns the name address table.
	///
	/// The values are RVAs to the exported function's name, to find its export look at the name index table with the same index.
	///
	/// The names are sorted allowing binary search lookup.
	pub fn names(&self) -> &'a [Rva] {
		self.names
	}
	/// Returns the name index table.
	///
	/// The values are indices (not ordinals!) into the export address table matching name with the same index in the name address table.
	pub fn name_indices(&self) -> &'a [u16] {
		self.name_indices
	}
	/// Checks whether export names are sorted.
	///
	/// The PE specification says that the list of names should be sorted to allow binary search.
	/// This function checks if the names table is actually sorted, if not then various name based lookup functions may fail to find certain exports.
	///
	/// Returns an error if a name entry cannot be read or is otherwise corrupt.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null], [`Bounds`][crate::Error::Bounds], [`Encoding`][crate::Error::Encoding], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: An export name cannot be read.
	pub fn check_sorted(&self) -> Result<bool> {
		let mut last = CStr::empty();
		for (name, _export) in self.iter_names() {
			let name = name?;
			if last > name {
				return Ok(false);
			}
			last = name;
		}
		Ok(true)
	}
	/// Looks up an export by its ordinal.
	///
	/// # Errors
	///
	/// * [`Bounds`][crate::Error::Bounds]: The ordinal is outside the export address table.
	/// * [`Null`][crate::Error::Null]: The selected export slot is empty.
	/// * [`Encoding`][crate::Error::Encoding], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: A forwarded export cannot be read.
	pub fn ordinal(&self, ordinal: Ordinal) -> Result<Export<'a>> {
		let base = self.exp.image.Base;
		if (ordinal as u32) < base {
			Err(Error::Bounds)
		}
		else {
			let index = (ordinal as u32 - base) as usize;
			self.index(index)
		}
	}
	/// Looks up an export by its name.
	///
	/// Does a linear scan over the name table.
	/// If the name table isn't sorted this will still be able to find exported functions by name.
	///
	/// Gracefully handles corrupted name entries by ignoring them.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The name is absent or its export slot is empty.
	/// * [`Bounds`][crate::Error::Bounds]: A matching name has an invalid table index.
	/// * [`Encoding`][crate::Error::Encoding], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: A matching forwarded export cannot be read.
	pub fn name_linear<S: AsRef<[u8]> + ?Sized>(&self, name: &S) -> Result<Export<'a>> {
		self.name_linear_(name.as_ref())
	}
	fn name_linear_(&self, name: &[u8]) -> Result<Export<'a>> {
		for hint in 0..self.names.len() {
			match self.name_of_hint(hint) {
				Ok(name_it) if name_it == name => return self.hint(hint),
				_ => (),
			}
		}
		Err(Error::Null)
	}
	/// Looks up an export by its name.
	///
	/// If the name table isn't sorted, certain exported functions may fail to be found.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The name is absent or its export slot is empty.
	/// * [`Bounds`][crate::Error::Bounds]: A matching name has an invalid table index.
	/// * [`Encoding`][crate::Error::Encoding], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: An export name or forwarder cannot be read.
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
			use core::cmp::Ordering::*;
			match name.cmp(name_it) {
				Less => upper_bound = i,
				Greater => lower_bound = i + 1,
				Equal => {
					let &index = self.name_indices.get(i).ok_or(Error::Bounds)?;
					return self.index(index as usize);
				},
			};
		}
		// Name not found, return null
		Err(Error::Null)
	}
	/// Looks up an export by its import.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The requested import is absent or its export slot is empty.
	/// * [`Bounds`][crate::Error::Bounds]: The ordinal or a table index is invalid.
	/// * [`Encoding`][crate::Error::Encoding], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: A name or forwarded export cannot be read.
	pub fn import(&self, import: Import) -> Result<Export<'a>> {
		match import {
			Import::ByName { hint, name } => self.hint_name_(hint, name),
			Import::ByOrdinal { ord } => self.ordinal(ord),
		}
	}
	/// Looks up an export by its index.
	///
	/// # Errors
	///
	/// * [`Bounds`][crate::Error::Bounds]: `index` is outside the export address table.
	/// * [`Null`][crate::Error::Null]: The export slot is empty.
	/// * [`Encoding`][crate::Error::Encoding], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: A forwarded export cannot be read.
	pub fn index(&self, index: usize) -> Result<Export<'a>> {
		let rva = self.functions.get(index).ok_or(Error::Bounds)?;
		self.exp.symbol_from_rva(rva)
	}
	/// Looks up an export by its hint.
	///
	/// # Errors
	///
	/// * [`Bounds`][crate::Error::Bounds]: `hint` or its function index is outside its table.
	/// * [`Null`][crate::Error::Null]: The export slot is empty.
	/// * [`Encoding`][crate::Error::Encoding], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: A forwarded export cannot be read.
	pub fn hint(&self, hint: usize) -> Result<Export<'a>> {
		let &index = self.name_indices.get(hint).ok_or(Error::Bounds)?;
		self.index(index as usize)
	}
	/// Looks up an export by its hint and falls back to the name if the hint is incorrect.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The name is absent or its export slot is empty.
	/// * [`Bounds`][crate::Error::Bounds]: A matching name has an invalid table index.
	/// * [`Encoding`][crate::Error::Encoding], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: An export name or forwarder cannot be read.
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
	///
	/// # Errors
	///
	/// * [`Bounds`][crate::Error::Bounds]: `hint` is outside the name table or its RVA is out of bounds.
	/// * [`Null`][crate::Error::Null]: The name RVA is zero.
	/// * [`Encoding`][crate::Error::Encoding], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The name cannot be read.
	pub fn name_of_hint(&self, hint: usize) -> Result<&'a CStr> {
		let &name_rva = self.names.get(hint).ok_or(Error::Bounds)?;
		self.exp.pe.derva_c_str(name_rva)
	}
	/// Returns the import name or ordinal for an export index.
	///
	/// Note that this does a linear scan to find its name,
	/// if this is called in a loop over all the exported functions you are accidentally quadratic.
	///
	/// See [`iter_names`][Self::iter_names] to iterate over the exported names in linear time.
	///
	/// # Errors
	///
	/// * [`Bounds`][crate::Error::Bounds]: `index` is outside the export address table.
	/// * [`Overflow`][crate::Error::Overflow]: The computed ordinal does not fit.
	/// * [`Null`][crate::Error::Null], [`Encoding`][crate::Error::Encoding], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The export name cannot be read.
	pub fn name_lookup(&self, index: usize) -> Result<Import<'a>> {
		if index >= self.functions.len() {
			return Err(Error::Bounds);
		}
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
				let ordinal = self.exp.image.Base.checked_add(index as u32).ok_or(Error::Overflow)?;
				let ord = Ordinal::try_from(ordinal).map_err(|_| Error::Overflow)?;
				Ok(Import::ByOrdinal { ord })
			},
		}
	}
	/// Returns an iterator over exported functions.
	///
	/// Not every exported function has a name, some are exported by ordinal.
	/// Looking up the exported function's name with [`name_lookup`][Self::name_lookup] results in quadratic performance.
	/// If the exported function's name is important consider building a cache or using [`iter_names`][Self::iter_names] instead.
	pub fn iter(&self) -> impl Clone + Iterator<Item = Result<Export<'a>>> {
		self.functions.iter().map(move |rva| self.symbol_from_rva(rva))
	}
	/// Returns an iterator over functions exported by name.
	pub fn iter_names(&self) -> impl Clone + Iterator<Item = (Result<&'a CStr>, Result<Export<'a>>)> {
		(0..self.names().len()).map(move |hint| (self.name_of_hint(hint), self.hint(hint)))
	}
	/// Returns named exports and their function table indices.
	pub fn iter_name_indices(&self) -> impl Clone + Iterator<Item = (Result<&'a CStr>, usize)> {
		(0..self.names().len()).map(move |hint| (self.name_of_hint(hint), self.name_indices[hint] as usize))
	}
}
impl<'a, P: Copy + Pe<'a>> fmt::Debug for ExportBy<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("ExportBy")
			.field("dll_name", &format_args!("{:?}", self.dll_name()))
			.field("time_date_stamp", &self.image.TimeDateStamp)
			.field("version", &self.image.Version)
			.field("ordinal_base", &self.ordinal_base())
			.field("functions.len", &self.functions().len())
			.field("names.len", &self.names.len())
			.finish()
	}
}

//----------------------------------------------------------------

/// Looks up exported symbols.
pub trait GetProcAddress<'a, T>: Pe<'a> {
	/// Returns an export by name, ordinal, or import.
	///
	/// Cache an [`ExportBy`] instance for repeated lookups.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The requested export is absent or its slot is empty.
	/// * [`Bounds`][crate::Error::Bounds]: An ordinal or export table index is invalid.
	/// * Other errors indicate malformed export data that cannot be read.
	fn get_export(self, name: T) -> Result<Export<'a>>;
}
impl<'a, P: Copy + Pe<'a>> GetProcAddress<'a, Ordinal> for P {
	/// Returns an export by name, ordinal, or import.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The requested export is absent or its slot is empty.
	/// * [`Bounds`][crate::Error::Bounds]: An ordinal or export table index is invalid.
	/// * Other errors indicate malformed export data that cannot be read.
	fn get_export(self, name: Ordinal) -> Result<Export<'a>> {
		self.exports()?.by()?.ordinal(name)
	}
}
impl<'b, 'a, P: Copy + Pe<'a>> GetProcAddress<'a, Import<'b>> for P {
	/// Returns an export by name, ordinal, or import.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The requested export is absent or its slot is empty.
	/// * [`Bounds`][crate::Error::Bounds]: An ordinal or export table index is invalid.
	/// * Other errors indicate malformed export data that cannot be read.
	fn get_export(self, name: Import<'b>) -> Result<Export<'a>> {
		self.exports()?.by()?.import(name)
	}
}
impl<'b, 'a, P: Copy + Pe<'a>, S: AsRef<[u8]> + ?Sized> GetProcAddress<'a, &'b S> for P {
	/// Returns an export by name, ordinal, or import.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The requested export is absent or its slot is empty.
	/// * [`Bounds`][crate::Error::Bounds]: An ordinal or export table index is invalid.
	/// * Other errors indicate malformed export data that cannot be read.
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

serde_impl! {
	impl<'a, P: Copy + Pe<'a>> Serialize for ExportDirectory<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			self.by().ok().serialize(serializer)
		}
	}
	impl<'a, P: Copy + Pe<'a>> Serialize for ExportBy<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("ExportDirectory", 7)?;
			state.serialize_field("image", self.image())?;
			state.serialize_field("dll_name", &self.dll_name().ok())?;
			state.serialize_field("ordinal_base", &self.ordinal_base())?;
			state.serialize_field("functions", &self.functions())?;
			state.serialize_field("name_rvas", &self.names())?;
			state.serialize_field("name_indices", &self.name_indices())?;
			let names = self
				.iter_name_indices()
				.filter_map(|(name, index)| name.ok().and_then(|name| name.to_str().ok()).map(|name| (name, index)));
			state.serialize_field("names", &SerdeKV(names))?;
			state.end()
		}
	}
}

//----------------------------------------------------------------

#[cfg(test)]
pub(crate) fn test_exports<'a, P: Copy + Pe<'a>>(pe: P) -> Result<()> {
	let by = pe.exports()?.by()?;
	let _ = format!("{:?}", by);

	let _dll_name = by.dll_name();
	let _ordinal_base = by.ordinal_base();

	// If the name table isn't sorted, skip some tests
	let sorted = by.check_sorted()?;

	// Count occurances of each export name
	use std::collections::HashMap;
	let mut occurances = HashMap::<_, i32>::new();
	for (name, _) in by.iter_names() {
		if let Ok(name) = name {
			*occurances.entry(name).or_default() += 1;
		}
	}

	for (hint, (name, export)) in by.iter_names().enumerate() {
		// println!("hint:{:?} name:{:?} export:{:?}", hint, name, export);

		assert_eq!(name, by.name_of_hint(hint));
		assert_eq!(export, by.hint(hint));

		if let Ok(name) = name {
			// Only do some name lookups if the export is actually unique
			let unique = occurances[name] == 1;

			// Lookup the export by its name
			if unique {
				assert_eq!(export, by.name_linear(name));
				if sorted {
					assert_eq!(export, by.name(name));
				}
			}

			// Lookup the export by its name and hint
			assert_eq!(export, by.hint_name(hint, name));
			assert_eq!(export, by.import(Import::ByName { hint, name }));
			if sorted && unique {
				assert_eq!(export, by.hint_name(0, name));
				assert_eq!(export, by.import(Import::ByName { hint: 0, name }));
			}
		}
	}
	Ok(())
}
