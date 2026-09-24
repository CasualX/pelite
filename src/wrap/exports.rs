use super::*;

/// Exported symbol.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Export<'a> {
	/// Standard exported symbol.
	Symbol(&'a u32),
	/// This export is forwarded to another dll.
	///
	/// Format of the string is `"DllName.ExportName"`.
	/// For more information see this [blog post](https://blogs.msdn.microsoft.com/oldnewthing/20060719-24/?p=30473) by Raymond Chen.
	Forward(&'a CStr),
}
impl<'a> Export<'a> {
	/// Returns some if the symbol is exported.
	#[inline]
	pub fn symbol(self) -> Option<u32> {
		match self {
			Export::Symbol(&rva) => Some(rva),
			_ => None,
		}
	}
	/// Returns some if the symbol is forwarded.
	#[inline]
	pub fn forward(self) -> Option<&'a CStr> {
		match self {
			Export::Forward(name) => Some(name),
			_ => None,
		}
	}
}

/// Export directory.
impl<'a, Pe32: Copy + pe32::Pe<'a>, Pe64: Copy + pe64::Pe<'a>> Wrap<pe32::ExportDirectory<'a, Pe32>, pe64::ExportDirectory<'a, Pe64>> {
	/// Returns the PE instance.
	#[inline]
	pub fn pe(&self) -> Wrap<Pe32, Pe64> {
		match self {
			Wrap::T32(exports) => Wrap::T32(exports.pe()),
			Wrap::T64(exports) => Wrap::T64(exports.pe()),
		}
	}
	/// Returns the underlying export directory image.
	#[inline]
	pub fn image(&self) -> &'a image::IMAGE_EXPORT_DIRECTORY {
		match self {
			Wrap::T32(exports) => exports.image(),
			Wrap::T64(exports) => exports.image(),
		}
	}
	/// Returns the export directory's name for this library.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The name RVA is zero.
	/// * [`Encoding`][crate::Error::Encoding]: The name has no null terminator.
	/// * [`Bounds`][crate::Error::Bounds], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The name cannot be read from the image.
	#[inline]
	pub fn dll_name(&self) -> Result<&'a CStr> {
		match self {
			Wrap::T32(exports) => exports.dll_name(),
			Wrap::T64(exports) => exports.dll_name(),
		}
	}
	/// Returns the ordinal base for the exported functions.
	#[inline]
	pub fn ordinal_base(&self) -> u16 {
		match self {
			Wrap::T32(exports) => exports.ordinal_base(),
			Wrap::T64(exports) => exports.ordinal_base(),
		}
	}
	/// Returns the export address table.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The function table RVA is zero.
	/// * [`Overflow`][crate::Error::Overflow]: The declared table length overflows.
	/// * [`Bounds`][crate::Error::Bounds], [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The function table cannot be read from the image.
	#[inline]
	pub fn functions(&self) -> Result<&'a [u32]> {
		match self {
			Wrap::T32(exports) => exports.functions(),
			Wrap::T64(exports) => exports.functions(),
		}
	}
	/// Returns the name address table.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The name table RVA is zero.
	/// * [`Overflow`][crate::Error::Overflow]: The declared table length overflows.
	/// * [`Bounds`][crate::Error::Bounds], [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The name table cannot be read from the image.
	#[inline]
	pub fn names(&self) -> Result<&'a [u32]> {
		match self {
			Wrap::T32(exports) => exports.names(),
			Wrap::T64(exports) => exports.names(),
		}
	}
	/// Returns the name index table.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The name index table RVA is zero.
	/// * [`Overflow`][crate::Error::Overflow]: The declared table length overflows.
	/// * [`Bounds`][crate::Error::Bounds], [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The name index table cannot be read from the image.
	#[inline]
	pub fn name_indices(&self) -> Result<&'a [u16]> {
		match self {
			Wrap::T32(exports) => exports.name_indices(),
			Wrap::T64(exports) => exports.name_indices(),
		}
	}
	/// Returns validated export lookup tables.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: A nonempty table has a zero RVA.
	/// * [`Overflow`][crate::Error::Overflow]: A declared table length overflows.
	/// * [`Bounds`][crate::Error::Bounds], [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: A declared export table cannot be read.
	#[inline]
	pub fn by(&self) -> Result<Wrap<pe32::ExportBy<'a, Pe32>, pe64::ExportBy<'a, Pe64>>> {
		match self {
			Wrap::T32(exports) => Wrap::T32(exports.by()).transpose(),
			Wrap::T64(exports) => Wrap::T64(exports.by()).transpose(),
		}
	}
}

/// Export directory symbol lookup.
impl<'a, Pe32: Copy + pe32::Pe<'a>, Pe64: Copy + pe64::Pe<'a>> Wrap<pe32::ExportBy<'a, Pe32>, pe64::ExportBy<'a, Pe64>> {
	/// Returns the PE instance.
	#[inline]
	pub fn pe(&self) -> Wrap<Pe32, Pe64> {
		match self {
			Wrap::T32(by) => Wrap::T32(by.pe()),
			Wrap::T64(by) => Wrap::T64(by.pe()),
		}
	}
	/// Returns the underlying export directory image.
	#[inline]
	pub fn image(&self) -> &'a image::IMAGE_EXPORT_DIRECTORY {
		match self {
			Wrap::T32(by) => by.image(),
			Wrap::T64(by) => by.image(),
		}
	}
	/// Returns the export directory's name for this library.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The name RVA is zero.
	/// * [`Encoding`][crate::Error::Encoding]: The name has no null terminator.
	/// * [`Bounds`][crate::Error::Bounds], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The name cannot be read from the image.
	#[inline]
	pub fn dll_name(&self) -> Result<&'a CStr> {
		match self {
			Wrap::T32(by) => by.dll_name(),
			Wrap::T64(by) => by.dll_name(),
		}
	}
	/// Returns the ordinal base for the exported functions.
	#[inline]
	pub fn ordinal_base(&self) -> u16 {
		match self {
			Wrap::T32(by) => by.ordinal_base(),
			Wrap::T64(by) => by.ordinal_base(),
		}
	}
	/// Returns the export address table.
	#[inline]
	pub fn functions(&self) -> &'a [u32] {
		match self {
			Wrap::T32(by) => by.functions(),
			Wrap::T64(by) => by.functions(),
		}
	}
	/// Returns the name address table.
	#[inline]
	pub fn names(&self) -> &'a [u32] {
		match self {
			Wrap::T32(by) => by.names(),
			Wrap::T64(by) => by.names(),
		}
	}
	/// Returns the name index table.
	#[inline]
	pub fn name_indices(&self) -> &'a [u16] {
		match self {
			Wrap::T32(by) => by.name_indices(),
			Wrap::T64(by) => by.name_indices(),
		}
	}
	/// Checks whether export names are sorted.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null], [`Bounds`][crate::Error::Bounds], [`Encoding`][crate::Error::Encoding], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: An export name cannot be read.
	#[inline]
	pub fn check_sorted(&self) -> Result<bool> {
		match self {
			Wrap::T32(by) => by.check_sorted(),
			Wrap::T64(by) => by.check_sorted(),
		}
	}
	/// Looks up an export by its ordinal.
	///
	/// # Errors
	///
	/// * [`Bounds`][crate::Error::Bounds]: The ordinal is outside the export address table.
	/// * [`Null`][crate::Error::Null]: The selected export slot is empty.
	/// * [`Encoding`][crate::Error::Encoding], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: A forwarded export cannot be read.
	#[inline]
	pub fn ordinal(&self, ordinal: u16) -> Result<Export<'a>> {
		match self {
			Wrap::T32(by) => by.ordinal(ordinal),
			Wrap::T64(by) => by.ordinal(ordinal),
		}
	}
	/// Looks up an export by its name.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The name is absent or its export slot is empty.
	/// * [`Bounds`][crate::Error::Bounds]: A matching name has an invalid table index.
	/// * [`Encoding`][crate::Error::Encoding], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: An export name or forwarder cannot be read.
	#[inline]
	pub fn name_linear<S: AsRef<[u8]> + ?Sized>(&self, name: &S) -> Result<Export<'a>> {
		match self {
			Wrap::T32(by) => by.name_linear(name),
			Wrap::T64(by) => by.name_linear(name),
		}
	}
	/// Looks up an export by its name.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The name is absent or its export slot is empty.
	/// * [`Bounds`][crate::Error::Bounds]: A matching name has an invalid table index.
	/// * [`Encoding`][crate::Error::Encoding], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: An export name or forwarder cannot be read.
	#[inline]
	pub fn name<S: AsRef<[u8]> + ?Sized>(&self, name: &S) -> Result<Export<'a>> {
		match self {
			Wrap::T32(by) => by.name(name),
			Wrap::T64(by) => by.name(name),
		}
	}
	/// Looks up an export by its import.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The requested import is absent or its export slot is empty.
	/// * [`Bounds`][crate::Error::Bounds]: The ordinal or a table index is invalid.
	/// * [`Encoding`][crate::Error::Encoding], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: A name or forwarded export cannot be read.
	#[inline]
	pub fn import(&self, import: Import) -> Result<Export<'a>> {
		match self {
			Wrap::T32(by) => by.import(import),
			Wrap::T64(by) => by.import(import),
		}
	}
	/// Looks up an export by its index.
	///
	/// # Errors
	///
	/// * [`Bounds`][crate::Error::Bounds]: `index` is outside the export address table.
	/// * [`Null`][crate::Error::Null]: The export slot is empty.
	/// * [`Encoding`][crate::Error::Encoding], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: A forwarded export cannot be read.
	#[inline]
	pub fn index(&self, index: usize) -> Result<Export<'a>> {
		match self {
			Wrap::T32(by) => by.index(index),
			Wrap::T64(by) => by.index(index),
		}
	}
	/// Looks up an export by its hint.
	///
	/// # Errors
	///
	/// * [`Bounds`][crate::Error::Bounds]: `hint` or its function index is outside its table.
	/// * [`Null`][crate::Error::Null]: The export slot is empty.
	/// * [`Encoding`][crate::Error::Encoding], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: A forwarded export cannot be read.
	#[inline]
	pub fn hint(&self, hint: usize) -> Result<Export<'a>> {
		match self {
			Wrap::T32(by) => by.hint(hint),
			Wrap::T64(by) => by.hint(hint),
		}
	}
	/// Looks up an export by its hint and falls back to the name if the hint is incorrect.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The name is absent or its export slot is empty.
	/// * [`Bounds`][crate::Error::Bounds]: A matching name has an invalid table index.
	/// * [`Encoding`][crate::Error::Encoding], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: An export name or forwarder cannot be read.
	#[inline]
	pub fn hint_name<S: AsRef<[u8]> + ?Sized>(&self, hint: usize, name: &S) -> Result<Export<'a>> {
		match self {
			Wrap::T32(by) => by.hint_name(hint, name),
			Wrap::T64(by) => by.hint_name(hint, name),
		}
	}
	/// Looks up the name for a hint.
	///
	/// # Errors
	///
	/// * [`Bounds`][crate::Error::Bounds]: `hint` is outside the name table or its RVA is out of bounds.
	/// * [`Null`][crate::Error::Null]: The name RVA is zero.
	/// * [`Encoding`][crate::Error::Encoding], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The name cannot be read.
	#[inline]
	pub fn name_of_hint(&self, hint: usize) -> Result<&'a CStr> {
		match self {
			Wrap::T32(by) => by.name_of_hint(hint),
			Wrap::T64(by) => by.name_of_hint(hint),
		}
	}
	/// Returns the import name or ordinal for an export index.
	///
	/// # Errors
	///
	/// * [`Bounds`][crate::Error::Bounds]: `index` is outside the export address table.
	/// * [`Overflow`][crate::Error::Overflow]: The computed ordinal does not fit.
	/// * [`Null`][crate::Error::Null], [`Encoding`][crate::Error::Encoding], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The export name cannot be read.
	#[inline]
	pub fn name_lookup(&self, index: usize) -> Result<Import<'a>> {
		match self {
			Wrap::T32(by) => by.name_lookup(index),
			Wrap::T64(by) => by.name_lookup(index),
		}
	}
	#[inline]
	fn symbol_from_rva(&self, rva: &'a u32) -> Result<Export<'a>> {
		match self {
			Wrap::T32(by) => by.symbol_from_rva(rva),
			Wrap::T64(by) => by.symbol_from_rva(rva),
		}
	}
	/// Returns an iterator over exported functions.
	#[inline]
	pub fn iter(&self) -> impl Clone + Iterator<Item = Result<Export<'a>>> {
		self.functions().iter().map(move |rva| self.symbol_from_rva(rva))
	}
	/// Returns an iterator over functions exported by name.
	#[inline]
	pub fn iter_names(&self) -> impl Clone + Iterator<Item = (Result<&'a CStr>, Result<Export<'a>>)> {
		(0..self.names().len()).map(move |hint| (self.name_of_hint(hint), self.hint(hint)))
	}
	/// Returns an iterator over functions exported by name, returning their name and index in the functions table.
	#[inline]
	pub fn iter_name_indices(&self) -> impl Clone + Iterator<Item = (Result<&'a CStr>, usize)> {
		(0..self.names().len()).map(move |hint| (self.name_of_hint(hint), self.name_indices()[hint] as usize))
	}
}

/// Convenient way to get an exported symbol.
impl<'a, Pe32: Copy + pe32::Pe<'a>, Pe64: Copy + pe64::Pe<'a>> Wrap<Pe32, Pe64> {
	/// Returns an export by name, ordinal, or import.
	///
	/// The returned export contains an RVA or the name of a forwarded export.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The requested export is absent or its slot is empty.
	/// * [`Bounds`][crate::Error::Bounds]: An ordinal or export table index is invalid.
	/// * Other errors indicate malformed export data that cannot be read.
	#[inline]
	pub fn get_export<T>(&self, name: T) -> Result<Export<'a>>
	where
		Pe32: pe32::GetProcAddress<'a, T>,
		Pe64: pe64::GetProcAddress<'a, T>,
	{
		match self {
			Wrap::T32(pe32) => pe32.get_export(name),
			Wrap::T64(pe64) => pe64.get_export(name),
		}
	}
}
