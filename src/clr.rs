//! Common Language Runtime directory headers and raw data.

use core::fmt;

use crate::image::IMAGE_COR20_HEADER;
use crate::Result;

mod metadata;
pub use self::metadata::*;

/// Common Language Runtime directory, shared by PE32 and PE32+.
///
/// Exposes CLR headers, metadata streams and heap values. Metadata table rows,
/// IL, managed resources, and signatures remain uninterpreted.
///
/// ```
/// # fn example(file: pelite::PeFile<'_>) -> pelite::Result<()> {
/// let clr: pelite::clr::ClrDirectory<'_> = file.clr()?;
/// let header = clr.image();
/// let metadata = clr.metadata_root()?;
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
	data: [Result<&'a [u8]>; 7],
	image: &'a IMAGE_COR20_HEADER,
}

impl<'a> ClrDirectory<'a> {
	pub(crate) fn new(image: &'a IMAGE_COR20_HEADER, data: [Result<&'a [u8]>; 7]) -> Self {
		ClrDirectory { image, data }
	}
	/// Returns the underlying CLR header.
	pub fn image(&self) -> &'a IMAGE_COR20_HEADER {
		self.image
	}
	/// Returns the metadata blob, bounded by its directory size.
	///
	/// Each blob accessor returns [`Null`][crate::Error::Null] for an absent
	/// subdirectory, or the PE read error for an unreadable range. An unreadable
	/// blob does not prevent loading the header or accessing other blobs.
	pub fn metadata(&self) -> Result<&'a [u8]> {
		self.data[0]
	}
	/// Parses the metadata root and exposes its stream directory and heaps.
	pub fn metadata_root(&self) -> Result<MetadataRoot<'a>> {
		MetadataRoot::parse(self.metadata()?)
	}
	/// Returns the managed resource blob, bounded by its directory size.
	///
	/// Resource names and offsets reside in the metadata tables.
	pub fn resources(&self) -> Result<&'a [u8]> {
		self.data[1]
	}
	/// Returns the strong-name signature bytes without verifying the signature.
	pub fn strong_name_signature(&self) -> Result<&'a [u8]> {
		self.data[2]
	}
	/// Returns the raw code manager table, a deprecated subdirectory.
	pub fn code_manager_table(&self) -> Result<&'a [u8]> {
		self.data[3]
	}
	/// Returns the raw vtable fixup descriptors without interpreting slot widths.
	pub fn vtable_fixups(&self) -> Result<&'a [u8]> {
		self.data[4]
	}
	/// Returns the raw export address table jumps.
	pub fn export_address_table_jumps(&self) -> Result<&'a [u8]> {
		self.data[5]
	}
	/// Returns the raw managed native header without interpreting native formats.
	pub fn managed_native_header(&self) -> Result<&'a [u8]> {
		self.data[6]
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
