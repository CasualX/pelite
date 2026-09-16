use super::*;

//----------------------------------------------------------------

#[doc(inline)]
pub use crate::Import;

//----------------------------------------------------------------

// Gets the import from the import name table.
//
// These aren't actually virtual addresses.
// This function will decode them to get the import.
fn import_from_va<'a, P: Pe<'a>>(pe: P, &va: &'a Va) -> Result<Import<'a>> {
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

//----------------------------------------------------------------

/// Import directory.
///
/// The import directory lists an image's module dependencies and the symbols
/// imported from each one. Each [`ImportDescriptor`] represents one dependency.
///
/// The separate [`ImportAddressTable`] provides one combined view of the imported
/// symbols. The loader overwrites that table with resolved function pointers.
///
/// # Examples
///
/// ```
/// # #![allow(unused_variables)]
/// use pelite::pe64::{Pe, PeFile};
///
/// # #[allow(dead_code)]
/// fn example(file: PeFile<'_>) -> pelite::Result<()> {
/// 	// Access the import directory and iterate over its DLL descriptors
/// 	for descriptor in file.imports()? {
/// 		// Name of the DLL being imported from
/// 		let dll_name = descriptor.dll_name()?;
///
/// 		// Import Address Table and Import Name Table for this DLL
/// 		let addresses = descriptor.iat()?;
/// 		let names = descriptor.int()?;
///
/// 		// Pair each address-table slot with its imported symbol
/// 		for (address, import) in Iterator::zip(addresses, names) {}
/// 	}
///
/// 	// Alternatively, iterate over the combined Import Address Table
/// 	for (address, import) in file.iat()?.iter() {
/// 		// Null entries can occur where the tables of imported DLLs join
/// 		if let Ok(import) = import {}
/// 	}
///
/// 	Ok(())
/// }
/// ```
#[derive(Copy, Clone)]
pub struct ImportDirectory<'a, P> {
	pe: P,
	image: &'a [IMAGE_IMPORT_DESCRIPTOR],
}
impl<'a, P: Pe<'a>> ImportDirectory<'a, P> {
	pub(crate) fn try_from(pe: P) -> Result<ImportDirectory<'a, P>> {
		let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_IMPORT).ok_or(Error::Bounds)?;
		let image = pe.derva_slice_f(datadir.VirtualAddress, |image: &IMAGE_IMPORT_DESCRIPTOR| image.is_null())?;
		Ok(ImportDirectory { pe, image })
	}
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Returns the underlying import directory image array.
	pub fn image(&self) -> &'a [IMAGE_IMPORT_DESCRIPTOR] {
		self.image
	}
	/// Iterator over the import descriptors.
	pub fn iter(&self) -> ImportDescriptorIter<'a, P> {
		ImportDescriptorIter { pe: self.pe, iter: self.image.iter() }
	}
}
impl<'a, P: Pe<'a>> IntoIterator for ImportDirectory<'a, P> {
	type Item = ImportDescriptor<'a, P>;
	type IntoIter = ImportDescriptorIter<'a, P>;
	fn into_iter(self) -> ImportDescriptorIter<'a, P> {
		self.iter()
	}
}
#[rustfmt::skip]
impl<'a, P: Pe<'a>> fmt::Debug for ImportDirectory<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("ImportDirectory")
			.field("descriptors", &crate::util::DebugList(self.iter()))
			.finish()
	}
}

//----------------------------------------------------------------

/// Import Address Table.
#[derive(Copy, Clone)]
pub struct ImportAddressTable<'a, P> {
	pe: P,
	image: &'a [Va],
}
impl<'a, P: Pe<'a>> ImportAddressTable<'a, P> {
	pub(crate) fn try_from(pe: P) -> Result<ImportAddressTable<'a, P>> {
		let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_IAT).ok_or(Error::Bounds)?;
		// Ignore datadir.Size not being a multiple of sizeof(Va), not that big of a deal...
		let image = pe.derva_slice(datadir.VirtualAddress, datadir.Size as usize / mem::size_of::<Va>())?;
		Ok(ImportAddressTable { pe, image })
	}
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Returns the underlying iat array.
	pub fn image(&self) -> &'a [Va] {
		self.image
	}
	/// Iterate over the IAT.
	///
	/// When the imports aren't resolved yet the IAT is an alias for the import name table.
	pub fn iter(&self) -> iter::Map<slice::Iter<'a, Va>, impl Clone + FnMut(&'a Va) -> (&'a Va, Result<Import<'a>>)> {
		let pe = self.pe;
		self.image.iter().map(move |va| (va, import_from_va(pe, va)))
	}
}
#[rustfmt::skip]
impl<'a, P: Pe<'a>> fmt::Debug for ImportAddressTable<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("ImportAddressTable")
			.field("iat.len", &self.image.len())
			.finish()
	}
}

//----------------------------------------------------------------

/// Iterator over the descriptors in an import directory.
#[derive(Clone)]
pub struct ImportDescriptorIter<'a, P> {
	pe: P,
	iter: slice::Iter<'a, IMAGE_IMPORT_DESCRIPTOR>,
}
impl<'a, P: Pe<'a>> ImportDescriptorIter<'a, P> {
	/// Returns the unconsumed import descriptor records.
	pub fn image(&self) -> &'a [IMAGE_IMPORT_DESCRIPTOR] {
		self.iter.as_slice()
	}
}
impl<'a, P: Pe<'a>> Iterator for ImportDescriptorIter<'a, P> {
	type Item = ImportDescriptor<'a, P>;
	fn next(&mut self) -> Option<ImportDescriptor<'a, P>> {
		self.iter.next().map(|image| ImportDescriptor { pe: self.pe, image })
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
	fn count(self) -> usize {
		self.iter.count()
	}
	fn nth(&mut self, n: usize) -> Option<ImportDescriptor<'a, P>> {
		self.iter.nth(n).map(|image| ImportDescriptor { pe: self.pe, image })
	}
}
impl<'a, P: Pe<'a>> DoubleEndedIterator for ImportDescriptorIter<'a, P> {
	fn next_back(&mut self) -> Option<ImportDescriptor<'a, P>> {
		self.iter.next_back().map(|image| ImportDescriptor { pe: self.pe, image })
	}
}
impl<'a, P: Pe<'a>> ExactSizeIterator for ImportDescriptorIter<'a, P> {}
impl<'a, P: Pe<'a>> iter::FusedIterator for ImportDescriptorIter<'a, P> {}

//----------------------------------------------------------------

/// Import library descriptor.
#[derive(Copy, Clone)]
pub struct ImportDescriptor<'a, P> {
	pe: P,
	image: &'a IMAGE_IMPORT_DESCRIPTOR,
}
impl<'a, P: Pe<'a>> ImportDescriptor<'a, P> {
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
		self.pe.derva_c_str(self.image.Name.get())
	}
	/// Gets the import address table.
	///
	/// After being loaded as a library their values are resolved to the addresses of the imported functions.
	///
	/// Otherwise these contain references to the imported functions.
	/// See [`ImportDescriptor::int`] to get their names.
	pub fn iat(&self) -> Result<slice::Iter<'a, Va>> {
		let slice = self.pe.derva_slice_s(self.image.FirstThunk.get(), 0)?;
		Ok(slice.iter())
	}
	/// Gets the import name table.
	pub fn int(&self) -> Result<iter::Map<slice::Iter<'a, Va>, impl Clone + FnMut(&'a Va) -> Result<Import<'a>>>> {
		let slice = self.pe.derva_slice_s(self.image.OriginalFirstThunk.get(), 0)?;
		let pe = self.pe;
		Ok(slice.iter().map(move |va| import_from_va(pe, va)))
	}
}
#[rustfmt::skip]
impl<'a, P: Pe<'a>> fmt::Debug for ImportDescriptor<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("ImportDescriptor")
			.field("dll_name", &format_args!("{:?}", self.dll_name()))
			.field("iat.len", &format_args!("{:?}", &self.iat().map(|iter| iter.len())))
			.field("int.len", &format_args!("{:?}", &self.int().map(|iter| iter.len())))
			.finish()
	}
}

//----------------------------------------------------------------

/*
	imports: [
		{
			"dll_name": "KERNEL32.dll",
			"int": [
				{
					"ByName": { .. }
				},
				{
					"ByOrdinal": { .. }
				}
			]
		}
	]
*/

serde_impl! {
	impl<'a, P: Pe<'a>> Serialize for ImportDirectory<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			serializer.collect_seq(self.into_iter())
		}
	}
	impl<'a, P: Pe<'a>> Serialize for ImportAddressTable<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let iat = self.iter().map(|(address, import)| (address, import.ok()));
			serializer.collect_seq(iat)
		}
	}
	impl<'a, P: Pe<'a>> Serialize for ImportDescriptor<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("ImportDescriptor", 4)?;
			state.serialize_field("image", self.image())?;
			state.serialize_field("dll_name", &self.dll_name().ok())?;
			let iat = self.iat().map(SerdeIter);
			state.serialize_field("iat", &iat.ok())?;
			let int = self.int().map(|int| SerdeIter(int.map(|import| import.ok())));
			state.serialize_field("int", &int.ok())?;
			state.end()
		}
	}
}

//----------------------------------------------------------------

#[cfg(test)]
pub(crate) fn test_imports<'a, P: Pe<'a>>(pe: P) -> Result<()> {
	let imports = pe.imports()?;
	let _ = format!("{:?}", imports);

	for desc in imports {
		let _ = format!("{:?}", desc);
		let _dll_name = desc.dll_name();
		if let Ok(iat) = desc.iat() {
			for _ in iat {}
		}
		if let Ok(int) = desc.int() {
			for _ in int {}
		}
	}

	let iat = pe.iat()?;
	for (va, import) in iat.iter() {
		let _ = format!("{:?}", import);
		if import.is_ok() {
			assert_eq!(import_from_va(pe, va), import);
		}
	}

	Ok(())
}
