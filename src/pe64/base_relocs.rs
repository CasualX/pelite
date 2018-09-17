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
	// Padding relocations of type absolute are skipped
	base_relocs.for_each(|rva, ty| {});

	// Iterate over the relocation blocks
	for block in base_relocs.iter_blocks() {}

	Ok(())
}
```
*/

use std::{cmp, fmt, iter, mem, slice};

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
	pub(crate) fn try_from(pe: P) -> Result<BaseRelocs<'a, P>> {
		let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_BASERELOC).ok_or(Error::Bounds)?;
		let relocs = pe.slice(datadir.VirtualAddress, datadir.Size as usize, 4)?; // $1
		let relocs = unsafe { relocs.get_unchecked(..datadir.Size as usize) };
		Ok(BaseRelocs { pe, relocs })
	}
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Iterates over the base relocation blocks.
	pub fn iter_blocks(&self) -> IterBlocks<'a> {
		IterBlocks { data: self.relocs }
	}
	/// Iterates over the base relocations with internal iteration.
	///
	/// Relocations padding types are skipped.
	pub fn for_each<F: FnMut(Rva, u8)>(&self, mut f: F) {
		self.fold((), |(), rva, ty| f(rva, ty))
	}
	/// Folds over the base relocations with internal iteration.
	///
	/// Relocations padding types are skipped.
	pub fn fold<T, F>(&self, init: T, mut f: F) -> T where F: FnMut(T, Rva, u8) -> T {
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
impl<'a, P: Pe<'a> + Copy> fmt::Debug for BaseRelocs<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("BaseRelocs").finish()
	}
}

//----------------------------------------------------------------

/// Iterator over the base relocation blocks.
#[derive(Clone)]
pub struct IterBlocks<'a> {
	data: &'a [u8],
}
impl<'a> IterBlocks<'a> {
	fn peek(&self) -> Option<Block<'a>> {
		if mem::size_of_val(self.data) >= mem::size_of::<IMAGE_BASE_RELOCATION>() { // $2
			debug_assert_eq!(self.data.as_ptr() as usize % 4, 0); // MUST BE DWORD ALIGNED!
			let block = unsafe {
				// The blocks pointer is dword aligned (see $1) and is at least large enough (see $2).
				let image_p = self.data.as_ptr() as *const IMAGE_BASE_RELOCATION;
				let image = &*image_p;
				// Calculate the number of words following the base relocation carefully
				let len = cmp::min(image.SizeOfBlock as usize, self.data.len()).saturating_sub(mem::size_of::<IMAGE_BASE_RELOCATION>()) / 2;
				let words = slice::from_raw_parts(image_p.offset(1) as *const u16, len);
				Block { image, words }
			};
			Some(block)
		}
		else {
			None
		}
	}
}
impl<'a> Iterator for IterBlocks<'a> {
	type Item = Block<'a>;
	fn next(&mut self) -> Option<Block<'a>> {
		if let Some(block) = self.peek() {
			let block_size = block.image.SizeOfBlock;
			// Avoid infinite loop by skipping at least the image base relocation header
			let block_size = cmp::max(block_size, mem::size_of::<IMAGE_BASE_RELOCATION>() as u32);
			// Ensure that the data pointer remains dword aligned
			let block_size = ((block_size - 1) & !3) + 4; // $1
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
impl<'a> iter::FusedIterator for IterBlocks<'a> {}

//----------------------------------------------------------------

/// Base Relocation Block.
#[derive(Copy, Clone)]
pub struct Block<'a> {
	image: &'a IMAGE_BASE_RELOCATION,
	words: &'a [u16],
}
impl<'a> Block<'a> {
	/// Returns the underlying base relocation block image.
	pub fn image(&self) -> &'a IMAGE_BASE_RELOCATION {
		self.image
	}
	/// Gets the types and offsets.
	pub fn words(&self) -> &'a [u16] {
		self.words
	}
	/// Gets the final Rva of a type-offset word.
	pub fn rva_of(&self, word: &u16) -> Rva {
		let offset = (word & 0x0FFF) as Rva;
		self.image.VirtualAddress.wrapping_add(offset)
	}
	/// Gets the type of a type-offset word.
	pub fn type_of(&self, word: &u16) -> u8 {
		(word >> 12) as u8
	}
}
impl<'a> fmt::Debug for Block<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Block")
			.field("virtual_address", &self.image.VirtualAddress)
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

#[cfg(feature = "serde")]
mod serde {
	use util::serde_helper::*;
	use super::{Pe, BaseRelocs};

	impl<'a, P: Pe<'a> + Copy> Serialize for BaseRelocs<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let mut state = serializer.serialize_struct("BaseRelocs", 2)?;
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

#[cfg(test)]
pub(crate) fn test<'a, P: Pe<'a> + Copy>(pe: P) -> Result<()> {
	let base_relocs = pe.base_relocs()?;
	let _ = format!("{:?}", base_relocs);

	let mut baseline = base_relocs
		.iter_blocks()
		.flat_map(move |block| {
			let _ = format!("{:?}", block);
			block.words()
				.iter()
				.filter(move |&word| block.type_of(word) != IMAGE_REL_BASED_ABSOLUTE)
				.map(move |word| block.rva_of(word))
		});

	base_relocs.for_each(|rva, _| {
		assert_eq!(baseline.next(), Some(rva));
	});
	assert_eq!(baseline.next(), None);

	Ok(())
}
