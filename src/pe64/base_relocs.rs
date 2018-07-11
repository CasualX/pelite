/*!
Base Relocations Directory.

The base relocations directory describes a list of addresses to pointer values within its module which need to be patched when the module is located at a different address than its preferred load address.
When the module contains pointers to itself these pointers need to be fixed when the module is loaded at a different address than its preferred load address.

For a quick and easy overview of how the base relocations are laid out, see this helpful [stackoverflow answer](https://stackoverflow.com/a/22513813).

# Examples

```
# #![allow(unused_variables)]
use pelite::pe64::{Pe, PeFile};

# #[allow(dead_code)]
fn example(file: PeFile<'_>) -> pelite::Result<()> {
	// Access the base relocations
	let base_relocs = file.base_relocs()?;

	// Iterate over the rva which need relocation
	for rva in base_relocs {}

	// Iterate over the relocation blocks
	for block in base_relocs.iter_blocks() {}

	Ok(())
}
```
*/

use std::{fmt, iter, mem, slice};

use error::{Error, Result};

use super::image::*;
use super::Pe;

//----------------------------------------------------------------

/// Base Relocations Directory.
///
/// For more information see the [module-level documentation](index.html).
#[derive(Copy, Clone)]
pub struct BaseRelocs<'a, P> {
	pe: P,
	relocs: &'a [u8],
}
impl<'a, P: Pe<'a> + Copy> BaseRelocs<'a, P> {
	pub(crate) fn new(pe: P) -> Result<BaseRelocs<'a, P>> {
		let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_BASERELOC).ok_or(Error::OOB)?;
		let relocs = pe.derva_slice(datadir.VirtualAddress, datadir.Size as usize)?;
		// Validate the relocations...
		// All unsafe blocks in this module rely on this being correct, no pressure there.
		// FIXME! I know this broken in subtle ways, please fix it.
		let mut it = relocs;
		while it.len() != 0 {
			if it.len() < mem::size_of::<IMAGE_BASE_RELOCATION>() {
				return Err(Error::Corrupt);
			}
			let image = unsafe { &*(it.as_ptr() as *const IMAGE_BASE_RELOCATION) };
			if image.SizeOfBlock % 4 != 0 || image.SizeOfBlock as usize > it.len() {
				return Err(Error::Corrupt);
			}
			it = &it[image.SizeOfBlock as usize..];
		}
		Ok(BaseRelocs { pe, relocs })
	}
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Iterates over the base relocation blocks.
	pub fn iter_blocks(&self) -> IterBlocks<'a, P> {
		IterBlocks {
			pe: self.pe,
			iter: self.relocs.iter()
		}
	}
}
impl<'a, P: Pe<'a> + Copy> IntoIterator for BaseRelocs<'a, P> {
	type Item = Rva;
	type IntoIter = Iter<'a>;
	fn into_iter(self) -> Iter<'a> {
		Iter {
			iter: self.relocs.iter(),
			offset: mem::size_of::<IMAGE_BASE_RELOCATION>(),
		}
	}
}
impl<'a, P: Pe<'a> + Copy> fmt::Debug for BaseRelocs<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("BaseRelocs").finish()
	}
}

//----------------------------------------------------------------

/// Iterator over the addresses of pointer values which need relocation in the image.
#[derive(Clone)]
pub struct Iter<'a> {
	iter: slice::Iter<'a, u8>,
	offset: usize,
}
impl<'a> Iterator for Iter<'a> {
	type Item = Rva;
	fn next(&mut self) -> Option<Rva> {
		while self.iter.len() != 0 {
			// Safety checked by new
			unsafe {
				let bytes = self.iter.as_slice();
				let image = &*(bytes.as_ptr() as *const IMAGE_BASE_RELOCATION);
				let word = *(bytes.as_ptr().offset(self.offset as isize) as *const u16);

				let next_offset = self.offset + 2;
				self.offset = if next_offset >= image.SizeOfBlock as usize {
					self.iter = bytes.get_unchecked(image.SizeOfBlock as usize..).iter();
					mem::size_of::<IMAGE_BASE_RELOCATION>()
				}
				else {
					next_offset
				};

				let ty = (word >> 12) as u8;
				if ty != IMAGE_REL_BASED_ABSOLUTE {
					let rva = image.VirtualAddress + (word & 0x0fff) as Rva;
					return Some(rva);
				}
			}
		}
		None
	}
	fn size_hint(&self) -> (usize, Option<usize>) {
		// Rough upper bound, ignoring base relocation blocks and padding
		let upper_bound = self.iter.as_slice().len() / 2;
		(0, Some(upper_bound))
	}
	// Should implement `try_fold` instead but it uses unstable features.
	fn fold<B, F>(self, init: B, mut f: F) -> B
		where F: FnMut(B, Self::Item) -> B,
	{
		let mut accum = init;
		// The outer loop iterates over base relocation blocks
		let mut bytes = self.iter.as_slice();
		let mut offset = self.offset;
		while bytes.len() != 0 {
			// Safety checked by new
			unsafe {
				// The inner loop iterates over the relocation words
				let image = &*(bytes.as_ptr() as *const IMAGE_BASE_RELOCATION);
				while offset < image.SizeOfBlock as usize {
					let word = *(bytes.as_ptr().offset(offset as isize) as *const u16);
					// Filter out padding relocations
					let ty = (word >> 12) as u8;
					if ty != IMAGE_REL_BASED_ABSOLUTE {
						let rva = image.VirtualAddress + (word & 0x0fff) as Rva;
						accum = f(accum, rva);
					}
					offset += 2;
				}
				// Advance to the next base relocation block and reset the offset to the first word
				bytes = bytes.get_unchecked(image.SizeOfBlock as usize..);
				offset = mem::size_of::<IMAGE_BASE_RELOCATION>();
			}
		}
		accum
	}
}
impl<'a> iter::FusedIterator for Iter<'a> {}

//----------------------------------------------------------------

/// Iterator over the base relocation blocks.
#[derive(Clone)]
pub struct IterBlocks<'a, P> {
	pe: P,
	iter: slice::Iter<'a, u8>,
}
impl<'a, P: Pe<'a> + Copy> Iterator for IterBlocks<'a, P> {
	type Item = Block<'a, P>;
	fn next(&mut self) -> Option<Block<'a, P>> {
		if self.iter.len() != 0 {
			// Safety checked by new
			unsafe {
				let bytes = self.iter.as_slice();
				let image = &*(bytes.as_ptr() as *const IMAGE_BASE_RELOCATION);
				self.iter = bytes.get_unchecked(image.SizeOfBlock as usize..).iter();
				Some(Block { pe: self.pe, image })
			}
		}
		else {
			None
		}
	}
}
impl<'a, P: Pe<'a> + Copy> iter::FusedIterator for IterBlocks<'a, P> {}

//----------------------------------------------------------------

/// Base Relocation Block.
#[derive(Copy, Clone)]
pub struct Block<'a, P> {
	pe: P,
	image: &'a IMAGE_BASE_RELOCATION,
}
impl<'a, P: Pe<'a> + Copy> Block<'a, P> {
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Returns the underlying base relocation block image.
	pub fn image(&self) -> &'a IMAGE_BASE_RELOCATION {
		self.image
	}
	/// Base rva of rva_of calculations.
	pub fn rva(&self) -> Rva {
		self.image.VirtualAddress
	}
	/// Gets the types and offsets.
	pub fn words(&self) -> &'a [u16] {
		// Safety checked by new
		unsafe {
			let p = (self.image as *const IMAGE_BASE_RELOCATION).offset(1) as *const u16;
			let len = (self.image.SizeOfBlock as usize - mem::size_of::<IMAGE_BASE_RELOCATION>()) / 2;
			slice::from_raw_parts(p, len)
		}
	}
	/// Gets the final Rva of a type-offset word.
	pub fn rva_of(&self, word: &u16) -> Rva {
		let offset = (word & 0x0FFF) as Rva;
		self.image.VirtualAddress + offset
	}
	/// Gets the type of a type-offset word.
	pub fn type_of(&self, word: &u16) -> u8 {
		(word >> 12) as u8
	}
}
impl<'a, P: Pe<'a> + Copy> fmt::Debug for Block<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Block")
			.field("rva", &self.rva())
			.field("words.len", &self.words().len())
			.finish()
	}
}

//----------------------------------------------------------------

/*
	"base_relocs": [
		1000, 1002, 1018, 2048, 2498,
	],
*/

#[cfg(feature = "serde")]
mod serde {
	use util::serde_helper::*;
	use super::{Pe, BaseRelocs};

	impl<'a, P: Pe<'a> + Copy> Serialize for BaseRelocs<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			serializer.collect_seq(self.into_iter())
		}
	}
}
