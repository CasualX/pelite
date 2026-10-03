use core::{fmt, iter, mem};

use dataview::DataView;

use crate::image::*;
use crate::util::CStr;
use crate::{Error, Pod, Result};

// Borrow fixed headers directly, checking their bounds and native alignment.
fn get<T: Pod>(bytes: &[u8], offset: usize) -> Result<&T> {
	DataView::from(bytes).try_get(offset).ok_or(Error::Bounds)
}

/// Metadata root and stream directory
/// ([ECMA-335 II.24.2](https://ecma-international.org/wp-content/uploads/ECMA-335_6th_edition_june_2012.pdf)).
///
/// Stream headers are checked during iteration. Table rows and IL remain raw.
#[derive(Copy, Clone)]
pub struct MetadataRoot<'a> {
	bytes: &'a [u8],
	image: &'a CLR_METADATA_ROOT,
	storage: &'a CLR_METADATA_STORAGE_HEADER,
	version: &'a CStr,
	streams_offset: usize,
}

impl fmt::Debug for MetadataRoot<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("MetadataRoot").field("image", &self.image).field("version", &self.version).field("storage", &self.storage).finish()
	}
}

impl<'a> MetadataRoot<'a> {
	/// Parses the fixed headers and bounded version string.
	///
	/// Returns `BadMagic` for a wrong signature, `Bounds` for invalid
	/// ranges or misaligned headers, and `Encoding` for an
	/// unterminated version string. Headers are borrowed from the input.
	pub fn parse(bytes: &'a [u8]) -> Result<MetadataRoot<'a>> {
		let image: &CLR_METADATA_ROOT = get(bytes, 0)?;
		if image.Signature != CLR_METADATA_SIGNATURE {
			return Err(Error::BadMagic);
		}
		let version_end = 16usize.checked_add(image.VersionLength as usize).ok_or(Error::Bounds)?;
		let version = CStr::from_bytes(bytes.get(16..version_end).ok_or(Error::Bounds)?).ok_or(Error::Encoding)?;
		let storage = get(bytes, version_end)?;
		let streams_offset = version_end.checked_add(4).ok_or(Error::Bounds)?;
		Ok(MetadataRoot { bytes, image, storage, version, streams_offset })
	}
	/// Returns the fixed metadata root header.
	pub fn image(&self) -> &'a CLR_METADATA_ROOT { self.image }
	/// Returns the storage flags and declared stream count.
	pub fn storage_header(&self) -> &'a CLR_METADATA_STORAGE_HEADER { self.storage }
	/// Returns the runtime version string, excluding its padding.
	pub fn version(&self) -> &'a CStr { self.version }
	/// Returns the complete metadata blob.
	pub fn bytes(&self) -> &'a [u8] { self.bytes }
	/// Iterates streams, yielding one error and stopping on a malformed header.
	pub fn streams(&self) -> MetadataStreams<'a> {
		MetadataStreams { bytes: self.bytes, offset: self.streams_offset, remaining: self.storage.Streams }
	}
	/// Finds a stream by name, preserving errors encountered during the search.
	pub fn stream(&self, name: &str) -> Result<Option<MetadataStream<'a>>> {
		for stream in self.streams() {
			let stream = stream?;
			if stream.name.as_ref() == name.as_bytes() { return Ok(Some(stream)); }
		}
		Ok(None)
	}
	fn heap(&self, name: &str) -> Result<&'a [u8]> {
		self.stream(name)?.map(|stream| stream.bytes).ok_or(Error::Null)
	}
	/// Parses the tables stream header and row counts, preferring `#~` to `#-`.
	pub fn tables(&self) -> Result<MetadataTables<'a>> {
		let stream = match self.stream("#~")? {
			Some(stream) => stream,
			None => self.stream("#-")?.ok_or(Error::Null)?,
		};
		MetadataTables::parse(stream.bytes)
	}
	/// Reads a nul-terminated UTF-8 identifier from the `#Strings` heap.
	///
	/// The returned C string can be decoded with `to_str()`; suffix offsets are
	/// valid because the heap can share string suffixes.
	pub fn string(&self, offset: u32) -> Result<&'a CStr> {
		let bytes = self.heap("#Strings")?.get(offset as usize..).ok_or(Error::Bounds)?;
		CStr::from_bytes(bytes).ok_or(Error::Encoding)
	}
	/// Reads a GUID using its one-based `#GUID` heap index. Zero returns `Null`.
	/// Returns a reference into the heap, or `Bounds` for an unaligned GUID.
	pub fn guid(&self, index: u32) -> Result<&'a GUID> {
		let index = index.checked_sub(1).ok_or(Error::Null)? as usize;
		get(self.heap("#GUID")?, index.checked_mul(16).ok_or(Error::Bounds)?)
	}
	/// Reads a length-prefixed `#Blob` heap value using a compressed unsigned length.
	pub fn blob(&self, offset: u32) -> Result<&'a [u8]> {
		let bytes = self.heap("#Blob")?.get(offset as usize..).ok_or(Error::Bounds)?;
		let (length, prefix) = compressed_length(bytes)?;
		let end = prefix.checked_add(length as usize).ok_or(Error::Bounds)?;
		bytes.get(prefix..end).ok_or(Error::Bounds)
	}
}

fn compressed_length(bytes: &[u8]) -> Result<(u32, usize)> {
	let first = *bytes.first().ok_or(Error::Bounds)?;
	if first & 0x80 == 0 { return Ok((u32::from(first), 1)); }
	if first & 0xc0 == 0x80 {
		let second = *bytes.get(1).ok_or(Error::Bounds)?;
		return Ok(((u32::from(first & 0x3f) << 8) | u32::from(second), 2));
	}
	if first & 0xe0 == 0xc0 {
		let tail = bytes.get(1..4).ok_or(Error::Bounds)?;
		return Ok(((u32::from(first & 0x1f) << 24) | (u32::from(tail[0]) << 16) | (u32::from(tail[1]) << 8) | u32::from(tail[2]), 4));
	}
	Err(Error::Invalid)
}

/// A metadata stream header and its bounded payload.
#[derive(Copy, Clone)]
pub struct MetadataStream<'a> {
	image: &'a CLR_METADATA_STREAM_HEADER,
	name: &'a CStr,
	bytes: &'a [u8],
}

impl fmt::Debug for MetadataStream<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("MetadataStream").field("image", &self.image).field("name", &self.name).finish()
	}
}

impl<'a> MetadataStream<'a> {
	/// Returns the stream's offset and declared size.
	pub fn image(&self) -> &'a CLR_METADATA_STREAM_HEADER { self.image }
	/// Returns the stream name, excluding its nul terminator and padding.
	pub fn name(&self) -> &'a CStr { self.name }
	/// Returns the payload bounded by the stream's declared size.
	pub fn bytes(&self) -> &'a [u8] { self.bytes }
}

/// Iterator over bounded stream headers. It stops after the first error.
#[derive(Clone)]
pub struct MetadataStreams<'a> {
	bytes: &'a [u8],
	offset: usize,
	remaining: u16,
}

impl<'a> MetadataStreams<'a> {
	fn read_stream(&mut self) -> Result<MetadataStream<'a>> {
		let image: &CLR_METADATA_STREAM_HEADER = get(self.bytes, self.offset)?;
		let name_start = self.offset.checked_add(8).ok_or(Error::Bounds)?;
		let names = self.bytes.get(name_start..).ok_or(Error::Bounds)?;
		let name = CStr::from_bytes(&names[..names.len().min(32)]).ok_or(Error::Encoding)?;
		if !name.is_ascii() { return Err(Error::Encoding); }
		let padded = name.c_str().len().checked_add(3).ok_or(Error::Bounds)? & !3;
		self.offset = name_start.checked_add(padded).ok_or(Error::Bounds)?;
		if self.offset > self.bytes.len() { return Err(Error::Bounds); }
		let start = image.Offset as usize;
		let end = start.checked_add(image.Size as usize).ok_or(Error::Bounds)?;
		let bytes = self.bytes.get(start..end).ok_or(Error::Bounds)?;
		Ok(MetadataStream { image, name, bytes })
	}
}

impl<'a> Iterator for MetadataStreams<'a> {
	type Item = Result<MetadataStream<'a>>;
	fn next(&mut self) -> Option<Self::Item> {
		if self.remaining == 0 { return None; }
		self.remaining -= 1;
		let result = self.read_stream();
		if result.is_err() { self.remaining = 0; }
		Some(result)
	}
	fn size_hint(&self) -> (usize, Option<usize>) { (0, Some(self.remaining as usize)) }
}
impl iter::FusedIterator for MetadataStreams<'_> {}

/// Standard metadata table identifiers, shared across architectures.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[repr(u8)]
pub enum Table {
	Module = 0x00, TypeRef = 0x01, TypeDef = 0x02, FieldPtr = 0x03,
	Field = 0x04, MethodPtr = 0x05, MethodDef = 0x06, ParamPtr = 0x07,
	Param = 0x08, InterfaceImpl = 0x09, MemberRef = 0x0a, Constant = 0x0b,
	CustomAttribute = 0x0c, FieldMarshal = 0x0d, DeclSecurity = 0x0e,
	ClassLayout = 0x0f, FieldLayout = 0x10, StandAloneSig = 0x11,
	EventMap = 0x12, EventPtr = 0x13, Event = 0x14, PropertyMap = 0x15,
	PropertyPtr = 0x16, Property = 0x17, MethodSemantics = 0x18,
	MethodImpl = 0x19, ModuleRef = 0x1a, TypeSpec = 0x1b, ImplMap = 0x1c,
	FieldRVA = 0x1d, ENCLog = 0x1e, ENCMap = 0x1f, Assembly = 0x20,
	AssemblyProcessor = 0x21, AssemblyOS = 0x22, AssemblyRef = 0x23,
	AssemblyRefProcessor = 0x24, AssemblyRefOS = 0x25, File = 0x26,
	ExportedType = 0x27, ManifestResource = 0x28, NestedClass = 0x29,
	GenericParam = 0x2a, MethodSpec = 0x2b, GenericParamConstraint = 0x2c,
}

impl Table {
	/// Returns a standard table identifier, or `None` for an unknown ID.
	pub fn from_id(id: u8) -> Option<Table> {
		const ALL: [Table; 45] = [
			Table::Module, Table::TypeRef, Table::TypeDef, Table::FieldPtr,
			Table::Field, Table::MethodPtr, Table::MethodDef, Table::ParamPtr,
			Table::Param, Table::InterfaceImpl, Table::MemberRef, Table::Constant,
			Table::CustomAttribute, Table::FieldMarshal, Table::DeclSecurity,
			Table::ClassLayout, Table::FieldLayout, Table::StandAloneSig,
			Table::EventMap, Table::EventPtr, Table::Event, Table::PropertyMap,
			Table::PropertyPtr, Table::Property, Table::MethodSemantics,
			Table::MethodImpl, Table::ModuleRef, Table::TypeSpec, Table::ImplMap,
			Table::FieldRVA, Table::ENCLog, Table::ENCMap, Table::Assembly,
			Table::AssemblyProcessor, Table::AssemblyOS, Table::AssemblyRef,
			Table::AssemblyRefProcessor, Table::AssemblyRefOS, Table::File,
			Table::ExportedType, Table::ManifestResource, Table::NestedClass,
			Table::GenericParam, Table::MethodSpec, Table::GenericParamConstraint,
		];
		ALL.get(id as usize).copied()
	}
}

/// Tables header and row counts. Table rows are exposed as raw bytes.
#[derive(Copy, Clone)]
pub struct MetadataTables<'a> {
	image: &'a CLR_METADATA_TABLES_HEADER,
	rows: &'a [u32],
	data: &'a [u8],
}

impl<'a> MetadataTables<'a> {
	/// Reads a `#~` or `#-` stream's fixed header and row counts.
	///
	/// This checks the header and count array, not the table rows or schemas.
	/// Both are borrowed from the input; misaligned headers return `Bounds`.
	pub fn parse(bytes: &'a [u8]) -> Result<MetadataTables<'a>> {
		let image: &CLR_METADATA_TABLES_HEADER = get(bytes, 0)?;
		let count = image.Valid.get().count_ones() as usize;
		// The header is four-byte aligned and the array immediately follows it.
		let rows = DataView::from(bytes).try_slice::<u32>(24, count).ok_or(Error::Bounds)?;
		let offset = 24 + count * mem::size_of::<u32>();
		Ok(MetadataTables { image, rows, data: &bytes[offset..] })
	}
	/// Returns the fixed tables header, including valid and sorted table masks.
	pub fn image(&self) -> &'a CLR_METADATA_TABLES_HEADER { self.image }
	/// Returns the borrowed packed count array, one value per set bit in `Valid`.
	pub fn row_count_array(&self) -> &'a [u32] { self.rows }
	/// Returns zero for an absent table.
	pub fn row_count(&self, table: Table) -> u32 {
		let bit = 1u64 << table as u8;
		let valid = self.image.Valid.get();
		if valid & bit == 0 { return 0; }
		let index = (valid & (bit - 1)).count_ones() as usize;
		self.rows[index]
	}
	/// Iterates present table IDs and row counts, including unknown table IDs.
	pub fn row_counts(&self) -> impl Iterator<Item = (u8, u32)> + Clone + '_ {
		let valid = self.image.Valid.get();
		(0u8..64).filter(move |&id| valid & (1u64 << id) != 0).zip(self.rows.iter().copied())
	}
	/// Returns the raw bytes following the row count array.
	pub fn data(&self) -> &'a [u8] { self.data }
	/// Returns the string heap index width in bytes.
	pub fn string_index_size(&self) -> usize { if self.image.HeapSizes & CLR_HEAP_LARGE_STRINGS != 0 { 4 } else { 2 } }
	/// Returns the GUID heap index width in bytes.
	pub fn guid_index_size(&self) -> usize { if self.image.HeapSizes & CLR_HEAP_LARGE_GUID != 0 { 4 } else { 2 } }
	/// Returns the blob heap index width in bytes.
	pub fn blob_index_size(&self) -> usize { if self.image.HeapSizes & CLR_HEAP_LARGE_BLOB != 0 { 4 } else { 2 } }
	/// Returns the width of a simple table index; coded indices use different rules.
	pub fn table_index_size(&self, table: Table) -> usize { if self.row_count(table) >= 0x10000 { 4 } else { 2 } }
}

impl fmt::Debug for MetadataTables<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("MetadataTables").field("image", &self.image).field("row_counts", &crate::util::DebugList(self.row_counts())).finish()
	}
}

serde_impl! {
	impl<'a> Serialize for MetadataRoot<'a> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let tables = match self.tables() {
				Ok(tables) => Some(tables),
				Err(Error::Null) => None,
				Err(error) => return Err(serde::ser::Error::custom(error)),
			};
			let mut state = serializer.serialize_struct("MetadataRoot", 5)?;
			state.serialize_field("image", self.image())?;
			state.serialize_field("version", self.version())?;
			state.serialize_field("storage_header", self.storage_header())?;
			state.serialize_field("streams", &self.streams())?;
			state.serialize_field("tables", &tables)?;
			state.end()
		}
	}

	impl<'a> Serialize for MetadataStreams<'a> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut seq = serializer.serialize_seq(None)?;
			for stream in self.clone() {
				let stream = stream.map_err(serde::ser::Error::custom)?;
				seq.serialize_element(&stream)?;
			}
			seq.end()
		}
	}

	impl<'a> Serialize for MetadataStream<'a> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("MetadataStream", 3)?;
			state.serialize_field("name", self.name())?;
			state.serialize_field("offset", &self.image().Offset)?;
			state.serialize_field("size", &self.image().Size)?;
			state.end()
		}
	}

	#[derive(serde::Serialize)]
	struct RowCount {
		id: u8,
		table: Option<Table>,
		rows: u32,
		index_size: usize,
		sorted: bool,
	}

	#[derive(serde::Serialize)]
	struct HeapIndexSizes {
		strings: usize,
		guid: usize,
		blob: usize,
	}

	impl<'a> Serialize for MetadataTables<'a> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let row_counts = self.row_counts().map(|(id, rows)| RowCount {
				id, table: Table::from_id(id), rows,
				index_size: if rows >= 0x10000 { 4 } else { 2 },
				sorted: self.image().Sorted.get() & (1u64 << id) != 0,
			});
			let heap_index_sizes = HeapIndexSizes {
				strings: self.string_index_size(), guid: self.guid_index_size(), blob: self.blob_index_size(),
			};
			let mut state = serializer.serialize_struct("MetadataTables", 3)?;
			state.serialize_field("image", self.image())?;
			state.serialize_field("row_counts", &crate::util::serde_helper::SerdeIter(row_counts))?;
			state.serialize_field("heap_index_sizes", &heap_index_sizes)?;
			state.end()
		}
	}
}
