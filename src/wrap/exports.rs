use super::imports::Import;
use super::Wrap;
use crate::*;

/// Exported symbol.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
pub enum Export<'a> {
    /// Standard exported symbol.
    Symbol(&'a u32),
    /// This export is forwarded to another dll.
    ///
    /// Format of the string is `"DllName.ExportName"`.
    /// For more information see this [blog post](https://blogs.msdn.microsoft.com/oldnewthing/20060719-24/?p=30473) by Raymond Chen.
    Forward(&'a util::CStr),
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
    pub fn forward(self) -> Option<&'a util::CStr> {
        match self {
            Export::Forward(name) => Some(name),
            _ => None,
        }
    }
}

/// Export directory.
impl<'a, Pe32: pe32::Pe<'a>, Pe64: pe64::Pe<'a>>
    Wrap<pe32::exports::Exports<'a, Pe32>, pe64::exports::Exports<'a, Pe64>>
{
    /// Gets the PE instance.
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
    /// Gets the export directory's name for this library.
    #[inline]
    pub fn dll_name(&self) -> Result<&'a util::CStr> {
        match self {
            Wrap::T32(exports) => exports.dll_name(),
            Wrap::T64(exports) => exports.dll_name(),
        }
    }
    /// Gets the ordinal base for the exported functions.
    #[inline]
    pub fn ordinal_base(&self) -> u16 {
        match self {
            Wrap::T32(exports) => exports.ordinal_base(),
            Wrap::T64(exports) => exports.ordinal_base(),
        }
    }
    /// Gets the export address table.
    #[inline]
    pub fn functions(&self) -> Result<&'a [u32]> {
        match self {
            Wrap::T32(exports) => exports.functions(),
            Wrap::T64(exports) => exports.functions(),
        }
    }
    /// Gets the name address table.
    #[inline]
    pub fn names(&self) -> Result<&'a [u32]> {
        match self {
            Wrap::T32(exports) => exports.names(),
            Wrap::T64(exports) => exports.names(),
        }
    }
    /// Gets the name index table.
    #[inline]
    pub fn name_indices(&self) -> Result<&'a [u16]> {
        match self {
            Wrap::T32(exports) => exports.name_indices(),
            Wrap::T64(exports) => exports.name_indices(),
        }
    }
    /// Query the exports.
    #[inline]
    pub fn by(&self) -> Result<Wrap<pe32::exports::By<'a, Pe32>, pe64::exports::By<'a, Pe64>>> {
        match self {
            Wrap::T32(exports) => Wrap::T32(exports.by()).transpose(),
            Wrap::T64(exports) => Wrap::T64(exports.by()).transpose(),
        }
    }
}

/// Export directory symbol lookup.
impl<'a, Pe32: pe32::Pe<'a>, Pe64: pe64::Pe<'a>>
    Wrap<pe32::exports::By<'a, Pe32>, pe64::exports::By<'a, Pe64>>
{
    /// Gets the PE instance.
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
    /// Gets the export directory's name for this library.
    #[inline]
    pub fn dll_name(&self) -> Result<&'a util::CStr> {
        match self {
            Wrap::T32(by) => by.dll_name(),
            Wrap::T64(by) => by.dll_name(),
        }
    }
    /// Gets the ordinal base for the exported functions.
    #[inline]
    pub fn ordinal_base(&self) -> u16 {
        match self {
            Wrap::T32(by) => by.ordinal_base(),
            Wrap::T64(by) => by.ordinal_base(),
        }
    }
    /// Gets the export address table.
    #[inline]
    pub fn functions(&self) -> &'a [u32] {
        match self {
            Wrap::T32(by) => by.functions(),
            Wrap::T64(by) => by.functions(),
        }
    }
    /// Gets the name address table.
    #[inline]
    pub fn names(&self) -> &'a [u32] {
        match self {
            Wrap::T32(by) => by.names(),
            Wrap::T64(by) => by.names(),
        }
    }
    /// Gets the name index table.
    #[inline]
    pub fn name_indices(&self) -> &'a [u16] {
        match self {
            Wrap::T32(by) => by.name_indices(),
            Wrap::T64(by) => by.name_indices(),
        }
    }
    /// Validates and checks if the name table is sorted.
    #[inline]
    pub fn check_sorted(&self) -> Result<bool> {
        match self {
            Wrap::T32(by) => by.check_sorted(),
            Wrap::T64(by) => by.check_sorted(),
        }
    }
    /// Looks up an `Export` by its ordinal.
    #[inline]
    pub fn ordinal(&self, ordinal: u16) -> Result<Export<'a>> {
        match self {
            Wrap::T32(by) => by.ordinal(ordinal),
            Wrap::T64(by) => by.ordinal(ordinal),
        }
    }
    /// Looks up an `Export` by its name.
    #[inline]
    pub fn name_linear<S: AsRef<[u8]> + ?Sized>(&self, name: &S) -> Result<Export<'a>> {
        match self {
            Wrap::T32(by) => by.name_linear(name),
            Wrap::T64(by) => by.name_linear(name),
        }
    }
    /// Looks up an `Export` by its name.
    #[inline]
    pub fn name<S: AsRef<[u8]> + ?Sized>(&self, name: &S) -> Result<Export<'a>> {
        match self {
            Wrap::T32(by) => by.name(name),
            Wrap::T64(by) => by.name(name),
        }
    }
    /// Looks up an `Export` by its import.
    #[inline]
    pub fn import(&self, import: Import) -> Result<Export<'a>> {
        match self {
            Wrap::T32(by) => by.import(import),
            Wrap::T64(by) => by.import(import),
        }
    }
    /// Looks up an export by its index.
    #[inline]
    pub fn index(&self, index: usize) -> Result<Export<'a>> {
        match self {
            Wrap::T32(by) => by.index(index),
            Wrap::T64(by) => by.index(index),
        }
    }
    /// Looks up an export by its hint.
    #[inline]
    pub fn hint(&self, hint: usize) -> Result<Export<'a>> {
        match self {
            Wrap::T32(by) => by.hint(hint),
            Wrap::T64(by) => by.hint(hint),
        }
    }
    /// Looks up an export by its hint and falls back to the name if the hint is incorrect.
    #[inline]
    pub fn hint_name<S: AsRef<[u8]> + ?Sized>(&self, hint: usize, name: &S) -> Result<Export<'a>> {
        match self {
            Wrap::T32(by) => by.hint_name(hint, name),
            Wrap::T64(by) => by.hint_name(hint, name),
        }
    }
    /// Looks up the name for a hint.
    #[inline]
    pub fn name_of_hint(&self, hint: usize) -> Result<&'a util::CStr> {
        match self {
            Wrap::T32(by) => by.name_of_hint(hint),
            Wrap::T64(by) => by.name_of_hint(hint),
        }
    }
    /// Given an index in the functions array, gets the named export.
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
    /// Iterate over exported functions.
    #[inline]
    pub fn iter<'s>(&'s self) -> impl 's + Clone + Iterator<Item = Result<Export<'a>>> {
        self.functions()
            .iter()
            .map(move |rva| self.symbol_from_rva(rva))
    }
    /// Iterate over functions exported by name.
    #[inline]
    pub fn iter_names<'s>(
        &'s self,
    ) -> impl 's + Clone + Iterator<Item = (Result<&'a util::CStr>, Result<Export<'a>>)> {
        (0..self.names().len() as u32)
            .map(move |hint| (self.name_of_hint(hint as usize), self.hint(hint as usize)))
    }
    /// Iterate over functions exported by name, returning their name and index in the functions table.
    #[inline]
    pub fn iter_name_indices<'s>(
        &'s self,
    ) -> impl 's + Clone + Iterator<Item = (Result<&'a util::CStr>, usize)> {
        (0..self.names().len() as u32).map(move |hint| {
            (
                self.name_of_hint(hint as usize),
                self.name_indices()[hint as usize] as usize,
            )
        })
    }
}

/// Convenient way to get an exported address.
impl<'a, Pe32: pe32::Pe<'a>, Pe64: pe64::Pe<'a>> Wrap<Pe32, Pe64> {
    /// Convenient method to get an exported function by ordinal.
    #[inline]
    pub fn get_export_by_ordinal(&self, ordinal: u16) -> Result<Export<'a>> {
        use pe32::exports::GetProcAddress as _;
        use pe64::exports::GetProcAddress as _;
        match self {
            Wrap::T32(pe32) => pe32.get_export(ordinal),
            Wrap::T64(pe64) => pe64.get_export(ordinal),
        }
    }
    /// Convenient method to get an exported function by import.
    #[inline]
    pub fn get_export_by_import(&self, import: Import<'a>) -> Result<Export<'a>> {
        use pe32::exports::GetProcAddress as _;
        use pe64::exports::GetProcAddress as _;
        match self {
            Wrap::T32(pe32) => pe32.get_export(import),
            Wrap::T64(pe64) => pe64.get_export(import),
        }
    }
    /// Convenient method to get an exported function by name.
    #[inline]
    pub fn get_export_by_name<S: ?Sized + AsRef<[u8]>>(&self, name: &S) -> Result<Export<'a>> {
        use pe32::exports::GetProcAddress as _;
        use pe64::exports::GetProcAddress as _;
        match self {
            Wrap::T32(pe32) => pe32.get_export(name),
            Wrap::T64(pe64) => pe64.get_export(name),
        }
    }
}
