/*!
Rich Structure.
 */

// References:
//
// * https://github.com/dishather/richprint
// * https://ntcore.com/?p=27
// * https://securelist.com/the-devils-in-the-rich-header/84348/
// * http://bytepointer.com/articles/the_microsoft_rich_header.htm
// * http://bytepointer.com/articles/rich_header_lifewire_vxmags_29A-8.009.htm
// * https://pdfs.semanticscholar.org/44ad/fa896e6598b1723507060126125a0cad39a1.pdf

use std::{fmt, iter, mem, result, slice};

use {Error, Result};
use super::{Pe};

//----------------------------------------------------------------

// The Rich structure:
// 'DanS' ^ x, x, x, x,
// compid ^ x, revision ^ x, ...
// 'Rich', x
// padding, ...

const DANS_MARKER: u32 = 0x536e6144; // "DanS"
const RICH_MARKER: u32 = 0x68636952; // "Rich"

/// Rich structure.
#[derive(Copy, Clone)]
pub struct RichHeader<'a> {
	dos_stub: &'a [u32],
	image: &'a [u32],
}
impl<'a> RichHeader<'a> {
	pub(crate) fn try_from<P: Pe<'a> + Copy>(pe: P) -> Result<RichHeader<'a>> {
		let dos_header = pe.dos_header();

		// Read as a slice of dwords up until the PE headers
		let len = (dos_header.e_lfanew / 4) as usize;
		let image = unsafe { slice::from_raw_parts(dos_header as *const _ as *const u32, len) };

		// Skip the padding zeroes
		let mut end = image.len();
		loop {
			if end < 16 {
				return Err(Error::Invalid);
			}
			if image[end - 1] != 0 {
				break;
			}
			end -= 1;
		}
		let end = end;

		// Find the Rich marker and the xor key
		if image[end - 2] != RICH_MARKER {
			return Err(Error::BadMagic);
		}
		let x = image[end - 1];
		let dx = DANS_MARKER ^ x;

		// Scan to find the header block
		let mut start = end - 6;
		loop {
			if start < 16 {
				return Err(Error::Invalid);
			}
			if image[start] == dx && image[start + 1] == x && image[start + 2] == x && image[start + 3] == x {
				break;
			}
			start -= 2;
		}
		let start = start;

		// Everything before is the dos stub
		let dos_stub = &image[..start];
		let image = &image[start..end];

		Ok(RichHeader { dos_stub, image })
	}
	/// Returns the Rich image without the padding.
	pub fn image(&self) -> &'a [u32] {
		self.image
	}
	/// Calculate the checksum.
	///
	/// The checksum should be equal to the xor key.
	pub fn checksum(&self) -> u32 {
		Self::_checksum(self.dos_stub, self.records())
	}
	fn _checksum<I>(dos_stub: &[u32], records: I) -> u32 where I: Iterator<Item = RichRecord> {
		let mut csum = mem::size_of_val(dos_stub) as u32;

		let mut i = 0;
		for dword in dos_stub {
			// Zero the e_lfanew field
			let bytes = if i == 0x3c { [0; 4] }
			else { unsafe { *(dword as *const _ as *const [u8; 4]) } };
			// Accumulate
			csum = u32::wrapping_add(csum, (bytes[0] as u32).rotate_left(i + 0));
			csum = u32::wrapping_add(csum, (bytes[1] as u32).rotate_left(i + 1));
			csum = u32::wrapping_add(csum, (bytes[2] as u32).rotate_left(i + 2));
			csum = u32::wrapping_add(csum, (bytes[3] as u32).rotate_left(i + 3));
			i += 4;
		}

		for record in records {
			let value = (record.product as u32) << 16 | (record.build as u32);
			csum = u32::wrapping_add(csum, value.rotate_left(record.count));
		}

		csum
	}
	/// Gets the xor key.
	pub fn xor_key(&self) -> u32 {
		self.image[1]
	}
	/// Gets the records.
	pub fn records(&self) -> RichIter<'a> {
		let iter = &self.image[4..self.image.len() - 2];
		let key = self.xor_key();
		RichIter { iter, key }
	}
	/// Encodes a new set of records.
	///
	/// If the destination does not have the right len, returns Err with the right len.
	/// Call encode again with destination of the returned len, destination is not modified.
	///
	/// Returns Ok with the len of the destination when encoding was successful.
	pub fn encode(&self, records: &[RichRecord], dest: &mut [u32]) -> result::Result<usize, usize> {
		let xor_key = Self::_checksum(self.dos_stub, records.iter().cloned());
		let n = records.len();
		let total_size = ((xor_key / 32) % 3 + n as u32) * 8 + 0x20;
		let total_len = (total_size / 4) as usize;
		if dest.len() < n * 2 + 6 {
			Err(total_len)
		}
		else {
			// Write the header
			dest[0] = DANS_MARKER ^ xor_key;
			dest[1] = xor_key;
			dest[2] = xor_key;
			dest[3] = xor_key;
			// Write the records
			for (i, record) in records.iter().enumerate() {
				let values = record.encode(xor_key);
				dest[i * 2 + 4] = values[0];
				dest[i * 2 + 5] = values[1];
			}
			// Write the footer
			dest[n * 2 + 4] = RICH_MARKER;
			dest[n * 2 + 5] = xor_key;
			// Write the padding
			for i in n * 2 + 6..dest.len() {
				dest[i] = 0;
			}
			Ok(total_len)
		}
	}
}
impl<'a> fmt::Debug for RichHeader<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("RichHeader")
			.field("xor_key", &self.xor_key())
			.field("checksum", &self.checksum())
			.field("records", &self.records())
			.finish()
	}
}

//----------------------------------------------------------------

/// Rich record.
///
/// Rich records contain a product identifier and its build number.
/// The count value indicates how many .obj files were linked generated by the product.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[repr(C)]
pub struct RichRecord {
	pub build: u16,
	pub product: u16,
	pub count: u32,
}
impl RichRecord {
	/// Decodes the record with the given key.
	pub fn decode(key: u32, values: &[u32; 2]) -> RichRecord {
		let field = values[0] ^ key;
		let build = (field & 0xffff) as u16;
		let product = ((field >> 16) & 0xffff) as u16;
		let count = values[1] ^ key;
		RichRecord { build, product, count }
	}
	/// Encodes the record with the given key.
	pub fn encode(&self, key: u32) -> [u32; 2] {
		let value = (self.product as u32) << 16 | (self.build as u32);
		[value ^ key, self.count ^ key]
	}
}

//----------------------------------------------------------------

/// Iterator over the Rich records.
#[derive(Clone)]
pub struct RichIter<'a> {
	iter: &'a [u32],
	key: u32,
}
impl<'a> Iterator for RichIter<'a> {
	type Item = RichRecord;
	fn next(&mut self) -> Option<RichRecord> {
		if self.iter.len() >= 2 {
			let record = RichRecord::decode(self.key, &[self.iter[0], self.iter[1]]);
			self.iter = &self.iter[2..];
			Some(record)
		}
		else {
			None
		}
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.iter.len() / 2;
		(len, Some(len))
	}
	fn count(self) -> usize {
		self.size_hint().0
	}
	fn nth(&mut self, n: usize) -> Option<RichRecord> {
		if self.iter.len() >= n * 2 + 2 {
			let record = RichRecord::decode(self.key, &[self.iter[n * 2], self.iter[n * 2 + 1]]);
			self.iter = &self.iter[n * 2 + 2..];
			Some(record)
		}
		else {
			self.iter = &self.iter[..0];
			None
		}
	}
}
impl<'a> DoubleEndedIterator for RichIter<'a> {
	fn next_back(&mut self) -> Option<RichRecord> {
		let len = self.iter.len();
		if len >= 2 {
			let record = RichRecord::decode(self.key, &[self.iter[len - 2], self.iter[len - 1]]);
			self.iter = &self.iter[..len - 2];
			Some(record)
		}
		else {
			None
		}
	}
}
impl<'a> ExactSizeIterator for RichIter<'a> {}
impl<'a> iter::FusedIterator for RichIter<'a> {}
impl<'a> fmt::Debug for RichIter<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_list().entries(self.clone()).finish()
	}
}

//----------------------------------------------------------------

/*
	"rich_header": {
		"xor_key": 129284757318,
		"checksum": 129284757318,
		"records": [
			{
				"build": 6030,
				"product": 95,
				"count": 68,
			},
		]
	},
*/

#[cfg(feature = "serde")]
mod serde {
	use util::serde_helper::*;
	use super::RichHeader;

	impl<'a> Serialize for RichHeader<'a> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("RichHeader", 3)?;
			state.serialize_field("xor_key", &self.xor_key())?;
			state.serialize_field("checksum", &self.checksum())?;
			state.serialize_field("records", &SerdeIter(self.records()))?;
			state.end()
		}
	}
}

//----------------------------------------------------------------

#[cfg(test)]
pub(crate) fn test<'a, P: 'a + Pe<'a> + Copy>(pe: P) -> Result<()> {
	let rich_header = pe.rich_header()?;
	let _checksum = rich_header.checksum();

	let records: Vec<_> = rich_header.records().collect();
	let mut encoded = vec![0u32; rich_header.image().len()];
	let _ = rich_header.encode(&records, &mut encoded);

	Ok(())
}
