//! Base relocation directory parsing and encoding.

use alloc::vec::Vec;
use core::{cmp, fmt, iter, mem, slice};

use crate::image::{IMAGE_BASE_RELOCATION, IMAGE_REL_BASED_ABSOLUTE};
use crate::util::AlignTo;
use crate::{Error, Result};

/// Base Relocations Directory.
///
/// Entries identify pointers to adjust by type and RVA. See this [relocation layout overview](https://stackoverflow.com/a/22513813) for an example.
///
/// # Examples
///
/// ```
/// # #![allow(unused_variables)]
/// use pelite::pe64::{Pe, PeFile};
///
/// # #[allow(dead_code)]
/// fn example(file: PeFile<'_>) -> pelite::Result<()> {
/// 	// Access the base relocations
/// 	let base_relocs = file.base_relocs()?;
///
/// 	// Iterate over the rva which need relocation
/// 	// Padding relocations of type absolute are skipped
/// 	base_relocs.for_each(|rva, ty| {});
///
/// 	// Iterate over the relocation blocks
/// 	for block in base_relocs.iter_blocks() {}
///
/// 	Ok(())
/// }
/// ```
#[derive(Copy, Clone)]
pub struct BaseRelocationDirectory<'a> {
	relocs: &'a [u8],
}
impl<'a> BaseRelocationDirectory<'a> {
	pub(crate) unsafe fn new(relocs: &'a [u8]) -> BaseRelocationDirectory<'a> {
		// $1
		debug_assert!(relocs.as_ptr().aligned_to(4));
		BaseRelocationDirectory { relocs }
	}
	/// Parses a base relocation directory.
	///
	/// Relocation blocks are read as the directory is iterated.
	///
	/// # Errors
	///
	/// * [Misaligned][Error::Misaligned]: The input is not aligned to a four-byte boundary.
	pub fn parse(relocs: &'a [u8]) -> Result<BaseRelocationDirectory<'a>> {
		// $1
		if !relocs.as_ptr().aligned_to(4) {
			return Err(Error::Misaligned);
		}
		Ok(BaseRelocationDirectory { relocs })
	}
	/// Returns the raw base relocation directory bytes.
	pub fn image(&self) -> &'a [u8] {
		self.relocs
	}
	/// Returns an iterator over relocation blocks.
	///
	/// Incomplete trailing data is ignored, and block sizes are clamped to the available bytes.
	pub fn iter_blocks(&self) -> BaseRelocationBlockIter<'a> {
		BaseRelocationBlockIter { data: self.relocs }
	}
	/// Calls the callback for each relocation except absolute padding entries.
	pub fn for_each<F: FnMut(u32, u8)>(&self, mut f: F) {
		self.fold((), |(), rva, ty| f(rva, ty))
	}
	/// Folds over relocations, skipping absolute padding entries.
	pub fn fold<T, F: FnMut(T, u32, u8) -> T>(&self, init: T, mut f: F) -> T {
		let mut accum = init;
		for block in self.iter_blocks() {
			for word in block.words() {
				let ty = block.type_of(word);
				if ty != IMAGE_REL_BASED_ABSOLUTE {
					let rva = block.rva_of(word);
					accum = f(accum, rva, ty);
				}
			}
		}
		accum
	}
}
impl<'a> fmt::Debug for BaseRelocationDirectory<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("BaseRelocationDirectory").finish()
	}
}

//----------------------------------------------------------------

/// Iterator over base relocation blocks.
#[derive(Clone)]
pub struct BaseRelocationBlockIter<'a> {
	data: &'a [u8],
}
impl<'a> BaseRelocationBlockIter<'a> {
	fn peek(&self) -> Option<BaseRelocationBlock<'a>> {
		// $2
		if mem::size_of_val(self.data) >= mem::size_of::<IMAGE_BASE_RELOCATION>() {
			Some(unsafe {
				// The blocks pointer is dword aligned (see $1) and is at least large enough (see $2).
				let image_p = self.data.as_ptr() as *const IMAGE_BASE_RELOCATION;
				let image = &*image_p;
				// Calculate the number of words following the base relocation carefully
				let len = cmp::min(image.SizeOfBlock as usize, self.data.len()).saturating_sub(mem::size_of::<IMAGE_BASE_RELOCATION>()) / 2;
				let words = slice::from_raw_parts(image_p.offset(1) as *const u16, len);
				BaseRelocationBlock { image, words }
			})
		}
		else {
			None
		}
	}
}
impl<'a> Iterator for BaseRelocationBlockIter<'a> {
	type Item = BaseRelocationBlock<'a>;
	fn next(&mut self) -> Option<BaseRelocationBlock<'a>> {
		if let Some(block) = self.peek() {
			let block_size = block.image.SizeOfBlock;
			// Avoid infinite loop by skipping at least the image base relocation header
			let block_size = cmp::max(block_size, mem::size_of::<IMAGE_BASE_RELOCATION>() as u32);
			// Ensure that the data pointer remains dword aligned $1
			let block_size = block_size.align_to(4);
			// Clamp the length to the data size
			let block_size = cmp::min(block_size as usize, self.data.len());
			self.data = &self.data[block_size..];
			Some(block)
		}
		else {
			None
		}
	}
}
impl<'a> iter::FusedIterator for BaseRelocationBlockIter<'a> {}

//----------------------------------------------------------------

/// Block of relocations for one RVA range.
#[derive(Copy, Clone)]
pub struct BaseRelocationBlock<'a> {
	image: &'a IMAGE_BASE_RELOCATION,
	words: &'a [u16],
}
impl<'a> BaseRelocationBlock<'a> {
	/// Returns the raw block header.
	pub fn image(&self) -> &'a IMAGE_BASE_RELOCATION {
		self.image
	}
	/// Returns the encoded type and offset words in this block.
	pub fn words(&self) -> &'a [u16] {
		self.words
	}
	/// Returns the RVA represented by a type and offset word.
	pub fn rva_of(&self, word: &u16) -> u32 {
		let offset = (word & 0x0fff) as u32;
		self.image.VirtualAddress.wrapping_add(offset)
	}
	/// Returns the relocation type encoded in a word.
	pub fn type_of(&self, word: &u16) -> u8 {
		(word >> 12) as u8
	}
}
#[rustfmt::skip]
impl<'a> fmt::Debug for BaseRelocationBlock<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("BaseRelocationBlock")
			.field("virtual_address", &format_args!("{:#x?}", self.image.VirtualAddress))
			.field("words.len", &self.words().len())
			.finish()
	}
}

//----------------------------------------------------------------

/*
	"base_relocs": {
		"rvas": [1000, 1002, 1018, 2048, 2498],
		"types": [3, 3, 3, 3, 3, 3]
	}
*/

serde_impl! {
	impl<'a> Serialize for BaseRelocationDirectory<'a> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("BaseRelocationDirectory", 2)?;
			let mut rvas = Vec::new();
			let mut types = Vec::new();
			self.for_each(|rva, ty| {
				rvas.push(rva);
				types.push(ty);
			});
			state.serialize_field("rvas", &*rvas)?;
			state.serialize_field("types", &*types)?;
			state.end()
		}
	}
}

//----------------------------------------------------------------

fn encode_type_offset(base: u32, rva: u32, ty: u8) -> u16 {
	(((rva - base) | (ty as u32) << 12) & 0xffff) as u16
}

impl BaseRelocationDirectory<'_> {
	/// Builds a base relocation directory from RVAs and types.
	///
	/// The arrays must have equal lengths. For predictable block order, provide RVAs in ascending order.
	///
	/// # Panics
	///
	/// Panics if the RVA and type arrays have different lengths.
	pub fn build(mut rvas: &[u32], mut types: &[u8]) -> Vec<u8> {
		assert_eq!(rvas.len(), types.len());

		let mut result = Vec::<u8>::new();
		while rvas.len() > 0 {
			// Given RVA range for the relocation block
			let start = rvas[0] & !0x0fff;
			let end = start + 0x0fff;

			// Figure the number of rvas to fit in this block
			let mut n = 0;
			while n < rvas.len() && rvas[n] >= start && rvas[n] < end {
				n += 1;
			}

			// Size of block should be multiple of 4 to ensure alignment
			let size = (8 + 2 * n).align_to(4);

			// Encode as bytes because Vec<u8> does not guarantee the alignment required
			// to write IMAGE_BASE_RELOCATION or u16 values through typed pointers.
			result.reserve(size);
			result.extend_from_slice(&start.to_ne_bytes());
			result.extend_from_slice(&(size as u32).to_ne_bytes());
			for i in 0..n {
				let word = encode_type_offset(start, rvas[i], types[i]);
				result.extend_from_slice(&word.to_ne_bytes());
			}
			// Add alignment padding.
			if n % 2 != 0 {
				result.extend_from_slice(&0u16.to_ne_bytes());
			}

			rvas = &rvas[n..];
			types = &types[n..];
		}
		result
	}
}

#[test]
fn test_build_unaligned_storage() {
	let result = BaseRelocationDirectory::build(&[1000, 1002], &[3, 3]);
	let mut expected = Vec::new();
	expected.extend_from_slice(&0u32.to_ne_bytes());
	expected.extend_from_slice(&12u32.to_ne_bytes());
	expected.extend_from_slice(&0x33e8u16.to_ne_bytes());
	expected.extend_from_slice(&0x33eau16.to_ne_bytes());
	assert_eq!(result, expected);
}

#[cfg(windows)]
#[test]
fn test_build_self() {
	use crate::pe::*;
	let view = unsafe { PeView::new() };
	if let Ok(base_relocs) = view.base_relocs() {
		let mut rvas = Vec::new();
		let mut types = Vec::new();
		base_relocs.for_each(|rva, ty| {
			rvas.push(rva);
			types.push(ty);
		});
		let rebuild = BaseRelocationDirectory::build(&rvas, &types);
		assert_eq!(rebuild, base_relocs.image());
	}
}
