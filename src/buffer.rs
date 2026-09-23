use alloc::vec::Vec;
use core::mem;
use core::ops::{Deref, DerefMut};
use core::ptr;

#[cfg(feature = "std")]
use std::fs::File;
#[cfg(feature = "std")]
use std::io::{self, Read};
#[cfg(feature = "std")]
use std::path::Path;

#[repr(align(16))]
#[derive(Copy, Clone)]
struct AlignedBytes(#[allow(dead_code)] [u8; 16]);

unsafe impl dataview::Pod for AlignedBytes {}

/// File contents held in an aligned buffer.
#[derive(Clone)]
pub struct PeMemory {
	bytes: Vec<AlignedBytes>,
	len: usize,
}

impl PeMemory {
	/// Creates a zero-filled buffer of the requested length.
	pub fn zeroed(len: usize) -> PeMemory {
		let bytes = alloc::vec![AlignedBytes([0; 16]); len.div_ceil(16)];
		PeMemory { bytes, len }
	}

	/// Returns the length of the buffer.
	#[inline]
	pub fn len(&self) -> usize {
		self.len
	}

	/// Returns the next 16-byte aligned offset at which bytes can be appended.
	///
	/// This may be greater than `len()`; the gap is zero-filled by `append`.
	#[inline]
	pub fn append_offset(&self) -> usize {
		mem::size_of_val(self.bytes.as_slice())
	}

	/// Pads the buffer with zeros to a 16-byte boundary, then appends `bytes`.
	///
	/// Returns the aligned offset where `bytes` begins. Appending an empty slice
	/// still makes any padding part of the buffer.
	pub fn append(&mut self, bytes: &[u8]) -> usize {
		let offset = self.append_offset();
		let blocks = bytes.len().div_ceil(16);
		self.bytes.reserve(blocks);
		// SAFETY: reserve provides space for all new blocks. The source cannot
		// overlap this buffer through the safe append API. Copying the input and
		// zeroing the remaining bytes initializes every block before set_len.
		unsafe {
			let end = self.bytes.len();
			let dst = self.bytes.as_mut_ptr().add(end).cast::<u8>();
			ptr::copy_nonoverlapping(bytes.as_ptr(), dst, bytes.len());
			ptr::write_bytes(dst.add(bytes.len()), 0, blocks * 16 - bytes.len());
			self.bytes.set_len(end + blocks);
		}
		self.len = offset + bytes.len();
		offset
	}

	/// Reads the whole file into an aligned buffer.
	#[cfg(feature = "std")]
	#[inline]
	pub fn open<P: AsRef<Path> + ?Sized>(path: &P) -> io::Result<PeMemory> {
		Self::_open(path.as_ref())
	}
	#[cfg(feature = "std")]
	fn _open(path: &Path) -> io::Result<PeMemory> {
		let mut file = File::open(path)?;
		let len = usize::try_from(file.metadata()?.len())
			.map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "file is too large"))?;
		let mut bytes = alloc::vec![AlignedBytes([0; 16]); len.div_ceil(16)];
		file.read_exact(&mut dataview::bytes_mut(bytes.as_mut_slice())[..len])?;
		Ok(PeMemory { bytes, len })
	}

	#[inline]
	pub fn view(&self) -> &dataview::DataView {
		dataview::DataView::from(self.as_ref())
	}

	#[inline]
	pub fn view_mut(&mut self) -> &mut dataview::DataView {
		dataview::DataView::from_mut(self.as_mut())
	}
}

impl Deref for PeMemory {
	type Target = [u8];
	fn deref(&self) -> &[u8] {
		self.as_ref()
	}
}

impl DerefMut for PeMemory {
	fn deref_mut(&mut self) -> &mut [u8] {
		self.as_mut()
	}
}

impl AsRef<[u8]> for PeMemory {
	#[inline]
	fn as_ref(&self) -> &[u8] {
		&dataview::bytes(self.bytes.as_slice())[..self.len]
	}
}

impl AsMut<[u8]> for PeMemory {
	#[inline]
	fn as_mut(&mut self) -> &mut [u8] {
		&mut dataview::bytes_mut(self.bytes.as_mut_slice())[..self.len]
	}
}

#[cfg(all(test, feature = "std"))]
mod tests {
	use super::*;

	#[test]
	fn append_pads_to_aligned_offsets() {
		let mut buffer = PeMemory::zeroed(3);
		buffer.as_mut().copy_from_slice(&[1, 2, 3]);
		assert_eq!(buffer.append_offset(), 16);
		assert_eq!(buffer.append(&[4, 5]), 16);
		assert_eq!(buffer.len(), 18);
		assert_eq!(&buffer[..3], &[1, 2, 3]);
		assert_eq!(&buffer[3..16], &[0; 13]);
		assert_eq!(&buffer[16..18], &[4, 5]);

		assert_eq!(buffer.append_offset(), 32);
		assert_eq!(buffer.append(&[]), 32);
		assert_eq!(buffer.len(), 32);
		assert_eq!(&buffer[18..32], &[0; 14]);
		assert_eq!(buffer.append(&[6]), 32);
		assert_eq!(buffer.len(), 33);
		assert_eq!(buffer[32], 6);
		assert_eq!(buffer.append_offset(), 48);
	}

	#[test]
	fn append_spans_multiple_blocks() {
		let mut buffer = PeMemory::zeroed(0);
		let data = [0x5a; 33];
		assert_eq!(buffer.append_offset(), 0);
		assert_eq!(buffer.append(&data), 0);
		assert_eq!(buffer.as_ref(), data);
		assert_eq!(buffer.append_offset(), 48);
		assert_eq!(buffer.append(&[0xa5]), 48);
		assert_eq!(&buffer[33..48], &[0; 15]);
		assert_eq!(buffer[48], 0xa5);
	}

	#[test]
	fn reads_exact_length_with_aligned_storage() {
		let path = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");
		let buffer = PeMemory::open(path).unwrap();
		assert_eq!(buffer.as_ref(), std::fs::read(path).unwrap());
		assert_eq!((buffer.as_ref().as_ptr() as usize) % 16, 0);
	}

	#[test]
	fn empty_buffer_has_aligned_empty_slice() {
		let buffer = PeMemory { bytes: Vec::new(), len: 0 };
		assert!(buffer.as_ref().is_empty());
		assert_eq!((buffer.as_ref().as_ptr() as usize) % 16, 0);
	}

	#[test]
	fn works_with_pefile() {
		let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/tiny/tiny.c.1024");
		let buffer = PeMemory::open(path).unwrap();
		crate::PeFile::from_bytes(&buffer).unwrap();
	}
}
