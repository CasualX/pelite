//! Common Language Runtime directory headers and raw data.

use core::fmt;

use crate::image;
use crate::wrap::sections::PeSectionHeaders;
use crate::{Error, PeLayout, Result};

mod metadata;
pub use self::metadata::*;

/// Common Language Runtime directory, shared by PE32 and PE32+.
///
/// Exposes CLR headers, metadata streams and heap values. Metadata table rows,
/// IL, managed resources, and signatures remain uninterpreted.
///
/// ```
/// # fn example(file: pelite::PeFile<'_>) -> pelite::Result<()> {
/// let clr = file.clr()?;
/// let header = clr.image();
/// let metadata = clr.metadata()?;
/// let tables = metadata.tables()?;
/// let methods = tables.row_count(pelite::clr::Table::MethodDef);
/// for stream in metadata.streams() {
///     let stream = stream?;
///     println!("{}: {} bytes", stream.name(), stream.bytes().len());
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Copy, Clone)]
pub struct ClrDirectory<'a> {
	bytes: &'a [u8],
	image: &'a image::IMAGE_COR20_HEADER,
	layout: PeLayout,
}

impl<'a> ClrDirectory<'a> {
	/// # Safety
	/// The bytes must contain validated PE headers.
	pub(crate) unsafe fn new(image: &'a image::IMAGE_COR20_HEADER, bytes: &'a [u8], layout: PeLayout) -> ClrDirectory<'a> {
		ClrDirectory { image, bytes, layout }
	}
	fn data(&self, directory: &image::IMAGE_DATA_DIRECTORY) -> Result<&'a [u8]> {
		if directory.VirtualAddress == 0 {
			return Err(Error::Null);
		}
		let size = directory.Size as usize;
		let bytes = match self.layout {
			PeLayout::File => {
				// The constructor receives a validated PE image.
				let sections = unsafe { PeSectionHeaders::from_image(self.bytes) };
				sections.range_file(self.bytes, directory.VirtualAddress, size)?
			},
			PeLayout::Section => self.bytes.get(directory.VirtualAddress as usize..).ok_or(Error::Bounds)?,
		};
		bytes.get(..size).ok_or(Error::Bounds)
	}

	/// Returns the underlying CLR header.
	pub fn image(&self) -> &'a image::IMAGE_COR20_HEADER {
		self.image
	}
	/// Parses the metadata root and exposes its stream directory and heaps.
	///
	/// Metadata is resolved and parsed on demand. Returns
	/// [`Null`][crate::Error::Null] for an absent subdirectory, or a read or parse
	/// error for unreadable or invalid metadata.
	pub fn metadata(&self) -> Result<MetadataRoot<'a>> {
		MetadataRoot::parse(self.data(&self.image.MetaData)?)
	}
	/// Returns the managed resource blob, bounded by its directory size.
	///
	/// Resource names and offsets reside in the metadata tables.
	pub fn resources(&self) -> Result<&'a [u8]> {
		self.data(&self.image.Resources)
	}
	/// Returns the strong-name signature bytes without verifying the signature.
	pub fn strong_name_signature(&self) -> Result<&'a [u8]> {
		self.data(&self.image.StrongNameSignature)
	}
	/// Returns the raw code manager table, a deprecated subdirectory.
	pub fn code_manager_table(&self) -> Result<&'a [u8]> {
		self.data(&self.image.CodeManagerTable)
	}
	/// Returns the raw vtable fixup descriptors without interpreting slot widths.
	pub fn vtable_fixups(&self) -> Result<&'a [u8]> {
		self.data(&self.image.VTableFixups)
	}
	/// Returns the raw export address table jumps.
	pub fn export_address_table_jumps(&self) -> Result<&'a [u8]> {
		self.data(&self.image.ExportAddressTableJumps)
	}
	/// Returns the raw managed native header without interpreting native formats.
	pub fn managed_native_header(&self) -> Result<&'a [u8]> {
		self.data(&self.image.ManagedNativeHeader)
	}
}

impl fmt::Debug for ClrDirectory<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("ClrDirectory").field("image", self.image).finish()
	}
}

serde_impl! {
	impl<'a> Serialize for ClrDirectory<'a> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			self.image.serialize(serializer)
		}
	}
}
