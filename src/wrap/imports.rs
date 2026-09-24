use super::*;

/// Imported symbol.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Import<'a> {
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
impl<'a, Pe32: Copy + pe32::Pe<'a>, Pe64: Copy + pe64::Pe<'a>> Wrap<pe32::ImportDirectory<'a, Pe32>, pe64::ImportDirectory<'a, Pe64>> {
	/// Returns the PE instance.
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
	/// Returns an iterator over the import descriptors.
	#[inline]
	pub fn iter(&self) -> Wrap<pe32::ImportDescriptorIter<'a, Pe32>, pe64::ImportDescriptorIter<'a, Pe64>> {
		match self {
			Wrap::T32(imports) => Wrap::T32(imports.iter()),
			Wrap::T64(imports) => Wrap::T64(imports.iter()),
		}
	}
}
impl<'a, Pe32: Copy + pe32::Pe<'a>, Pe64: Copy + pe64::Pe<'a>> IntoIterator for Wrap<pe32::ImportDirectory<'a, Pe32>, pe64::ImportDirectory<'a, Pe64>> {
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
impl<'a, Pe32: Copy + pe32::Pe<'a>, Pe64: Copy + pe64::Pe<'a>> Wrap<pe32::ImportAddressTable<'a, Pe32>, pe64::ImportAddressTable<'a, Pe64>> {
	/// Returns the PE instance.
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
	/// Returns an iterator over the IAT.
	///
	/// Bound or loader-resolved slots can contain virtual addresses rather than
	/// import-name RVAs and consequently produce decoding errors. Use the import
	/// descriptors' [`Self::int`][Wrap::int] tables for authoritative symbol names.
	#[inline]
	pub fn iter(&self) -> Wrap<impl Clone + Iterator<Item = (&'a u32, Result<Import<'a>>)>, impl Clone + Iterator<Item = (&'a u64, Result<Import<'a>>)>> {
		match self {
			Wrap::T32(iat) => Wrap::T32(iat.iter()),
			Wrap::T64(iat) => Wrap::T64(iat.iter()),
		}
	}
}

/// Import library descriptor.
impl<'a, Pe32: Copy + pe32::Pe<'a>, Pe64: Copy + pe64::Pe<'a>> Wrap<pe32::ImportDescriptor<'a, Pe32>, pe64::ImportDescriptor<'a, Pe64>> {
	/// Returns the PE instance.
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
	/// Returns the name of the DLL imported from.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The name RVA is zero.
	/// * [`Encoding`][crate::Error::Encoding]: The name has no null terminator.
	/// * [`Bounds`][crate::Error::Bounds], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The name cannot be read from the image.
	#[inline]
	pub fn dll_name(&self) -> Result<&'a CStr> {
		match self {
			Wrap::T32(desc) => desc.dll_name(),
			Wrap::T64(desc) => desc.dll_name(),
		}
	}
	/// Returns the import address table.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The import address table RVA is zero.
	/// * [`Bounds`][crate::Error::Bounds]: The table lies outside the image or has no terminating entry.
	/// * [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The table cannot be read.
	#[inline]
	pub fn iat(&self) -> Result<Wrap<slice::Iter<'a, u32>, slice::Iter<'a, u64>>> {
		match self {
			Wrap::T32(desc) => Wrap::T32(desc.iat()).transpose(),
			Wrap::T64(desc) => Wrap::T64(desc.iat()).transpose(),
		}
	}
	/// Returns the import name table.
	///
	/// # Errors
	///
	/// * [`Null`][crate::Error::Null]: The import name table RVA is zero.
	/// * [`Bounds`][crate::Error::Bounds]: The table lies outside the image or has no terminating entry.
	/// * [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The table cannot be read.
	#[inline]
	pub fn int(&self) -> Result<impl Clone + Iterator<Item = Result<Import<'a>>>> {
		match self {
			Wrap::T32(desc) => Ok(Wrap::T32(desc.int()?).map(Wrap::into)),
			Wrap::T64(desc) => Ok(Wrap::T64(desc.int()?).map(Wrap::into)),
		}
	}
}
