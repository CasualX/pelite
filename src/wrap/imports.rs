use super::*;

/// Imported symbol.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ImportSymbol<'a> {
	/// Imported by name.
	///
	/// The hint is an index in the export names table that may contain the desired symbol.
	/// For more information see this [blog post](https://blogs.msdn.microsoft.com/oldnewthing/20100317-00/?p=14573) by Raymond Chen.
	ByName {
		/// Suggested index in the export name pointer table.
		hint: usize,
		/// Imported symbol name.
		name: &'a CStr,
	},
	/// Imported by ordinal.
	ByOrdinal {
		/// Export ordinal requested from the target library.
		ord: u16,
	},
}

/// Import directory.
impl<'a, Pe32: pe32::Pe<'a>, Pe64: pe64::Pe<'a>> Wrap<pe32::ImportDirectory<'a, Pe32>, pe64::ImportDirectory<'a, Pe64>> {
	/// Gets the PE instance.
	#[inline]
	pub fn pe(&self) -> Wrap<Pe32, Pe64> {
		match self {
			Wrap::T32(imports) => Wrap::T32(imports.pe()),
			Wrap::T64(imports) => Wrap::T64(imports.pe()),
		}
	}
	/// Returns the underlying import directory image array.
	#[inline]
	pub fn image(&self) -> &'a [image::IMAGE_IMPORT_DESCRIPTOR] {
		match self {
			Wrap::T32(imports) => imports.image(),
			Wrap::T64(imports) => imports.image(),
		}
	}
	/// Iterator over the import descriptors.
	#[inline]
	pub fn iter(&self) -> Wrap<pe32::ImportDescriptorIter<'a, Pe32>, pe64::ImportDescriptorIter<'a, Pe64>> {
		match self {
			Wrap::T32(imports) => Wrap::T32(imports.iter()),
			Wrap::T64(imports) => Wrap::T64(imports.iter()),
		}
	}
}
impl<'a, Pe32: pe32::Pe<'a>, Pe64: pe64::Pe<'a>> IntoIterator for Wrap<pe32::ImportDirectory<'a, Pe32>, pe64::ImportDirectory<'a, Pe64>> {
	type Item = Wrap<pe32::ImportDescriptor<'a, Pe32>, pe64::ImportDescriptor<'a, Pe64>>;
	type IntoIter = Wrap<pe32::ImportDescriptorIter<'a, Pe32>, pe64::ImportDescriptorIter<'a, Pe64>>;
	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		match self {
			Wrap::T32(imports) => Wrap::T32(imports.into_iter()),
			Wrap::T64(imports) => Wrap::T64(imports.into_iter()),
		}
	}
}

/// Import Address Table.
impl<'a, Pe32: pe32::Pe<'a>, Pe64: pe64::Pe<'a>> Wrap<pe32::ImportAddressTable<'a, Pe32>, pe64::ImportAddressTable<'a, Pe64>> {
	/// Gets the PE instance.
	#[inline]
	pub fn pe(&self) -> Wrap<Pe32, Pe64> {
		match self {
			Wrap::T32(iat) => Wrap::T32(iat.pe()),
			Wrap::T64(iat) => Wrap::T64(iat.pe()),
		}
	}
	/// Returns the underlying iat array.
	#[inline]
	pub fn image(&self) -> Wrap<&'a [u32], &'a [u64]> {
		match self {
			Wrap::T32(iat) => Wrap::T32(iat.image()),
			Wrap::T64(iat) => Wrap::T64(iat.image()),
		}
	}
	/// Iterate over the IAT.
	#[inline]
	pub fn iter(&self) -> Wrap<impl Clone + Iterator<Item = (&'a u32, Result<ImportSymbol<'a>>)>, impl Clone + Iterator<Item = (&'a u64, Result<ImportSymbol<'a>>)>> {
		match self {
			Wrap::T32(iat) => Wrap::T32(iat.iter()),
			Wrap::T64(iat) => Wrap::T64(iat.iter()),
		}
	}
}

/// Import library descriptor.
impl<'a, Pe32: pe32::Pe<'a>, Pe64: pe64::Pe<'a>> Wrap<pe32::ImportDescriptor<'a, Pe32>, pe64::ImportDescriptor<'a, Pe64>> {
	/// Gets the PE instance.
	#[inline]
	pub fn pe(&self) -> Wrap<Pe32, Pe64> {
		match self {
			Wrap::T32(desc) => Wrap::T32(desc.pe()),
			Wrap::T64(desc) => Wrap::T64(desc.pe()),
		}
	}
	/// Returns the underlying import descriptor image.
	#[inline]
	pub fn image(&self) -> &'a image::IMAGE_IMPORT_DESCRIPTOR {
		match self {
			Wrap::T32(desc) => desc.image(),
			Wrap::T64(desc) => desc.image(),
		}
	}
	/// Gets the name of the DLL imported from.
	#[inline]
	pub fn dll_name(&self) -> Result<&'a CStr> {
		match self {
			Wrap::T32(desc) => desc.dll_name(),
			Wrap::T64(desc) => desc.dll_name(),
		}
	}
	/// Gets the import address table.
	#[inline]
	pub fn iat(&self) -> Result<Wrap<slice::Iter<'a, u32>, slice::Iter<'a, u64>>> {
		match self {
			Wrap::T32(desc) => Wrap::T32(desc.iat()).transpose(),
			Wrap::T64(desc) => Wrap::T64(desc.iat()).transpose(),
		}
	}
	/// Gets the import name table.
	#[inline]
	pub fn int(&self) -> Result<impl Clone + Iterator<Item = Result<ImportSymbol<'a>>>> {
		match self {
			Wrap::T32(desc) => Ok(Wrap::T32(desc.int()?).map(Wrap::into)),
			Wrap::T64(desc) => Ok(Wrap::T64(desc.int()?).map(Wrap::into)),
		}
	}
}
