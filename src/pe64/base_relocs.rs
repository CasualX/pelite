/*!
Base Relocations Directory.

# Examples

```
# #![allow(unused_variables)]
use pelite::pe64::{Pe, PeFile};

# #[allow(dead_code)]
fn example(file: PeFile) -> pelite::Result<()> {
	// Access the base relocations
	let base_relocs = file.base_relocs()?;

	// Iterate over the flattened list of relocations
	for rva in base_relocs.into_iter().flat_map(|relocs| relocs) {}

	Ok(())
}
```
*/

use std::{fmt, mem, slice};

use error::{Error, Result};

use super::image::*;
use super::Pe;

//----------------------------------------------------------------

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
		// This better not contain any bugs.
		let mut it = relocs;
		while it.len() > 0 {
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
	pub fn pe(&self) -> P {
		self.pe
	}
}
impl<'a, P: Pe<'a> + Copy> IntoIterator for BaseRelocs<'a, P> {
	type Item = Block<'a, P>;
	type IntoIter = Iter<'a, P>;
	fn into_iter(self) -> Iter<'a, P> {
		Iter { pe: self.pe, iter: self.relocs.iter() }
	}
}

//----------------------------------------------------------------

/// Iterator over the base relocations.
#[derive(Clone)]
pub struct Iter<'a, P> {
	pe: P,
	iter: slice::Iter<'a, u8>,
}
impl<'a, P: Pe<'a> + Copy> Iterator for Iter<'a, P> {
	type Item = Block<'a, P>;
	fn next(&mut self) -> Option<Block<'a, P>> {
		if self.iter.len() != 0 {
			// This is safe as it should have been checked in `new`.
			unsafe {
				let slice = self.iter.as_slice();
				let image = &*(slice.as_ptr() as *const IMAGE_BASE_RELOCATION);
				self.iter = slice.get_unchecked(image.SizeOfBlock as usize..).iter();
				Some(Block { pe: self.pe, image })
			}
		}
		else {
			None
		}
	}
}

//----------------------------------------------------------------

/// Base Relocations Block.
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
	/// Returns the underlying base relocations image.
	pub fn image(&self) -> &'a IMAGE_BASE_RELOCATION {
		self.image
	}
	/// Rva starting from.
	pub fn va(&self) -> Rva {
		self.image.VirtualAddress
	}
	/// Gets the types and offsets.
	pub fn words(&self) -> &'a [IMAGE_BASE_RELOC_TYPEOFFSET] {
		unsafe {
			let p = (self.image as *const _).offset(1) as *const IMAGE_BASE_RELOC_TYPEOFFSET;
			let len = (self.image.SizeOfBlock as usize - mem::size_of::<IMAGE_BASE_RELOCATION>()) / mem::size_of::<IMAGE_BASE_RELOC_TYPEOFFSET>();
			slice::from_raw_parts(p, len)
		}
	}
	/// Gets the final Rva of a typeoffset.
	pub fn rva_of(&self, tyoff: &IMAGE_BASE_RELOC_TYPEOFFSET) -> Rva {
		let offset = (tyoff.0 & 0x0FFF) as Rva;
		self.image.VirtualAddress + offset
	}
	/// Gets the type of a typeoffset.
	pub fn type_of(&self, tyoff: &IMAGE_BASE_RELOC_TYPEOFFSET) -> u8 {
		((tyoff.0 >> 12) & 0xFF) as u8
	}
}
impl<'a, P: Pe<'a> + Copy> IntoIterator for Block<'a, P> {
	type Item = Rva;
	type IntoIter = BlockIter<'a>;
	fn into_iter(self) -> BlockIter<'a> {
		BlockIter {
			rva: self.image.VirtualAddress,
			iter: self.words().iter(),
		}
	}
}

//----------------------------------------------------------------

pub struct BlockIter<'a> {
	rva: Rva,
	iter: slice::Iter<'a, IMAGE_BASE_RELOC_TYPEOFFSET>,
}
impl<'a> Iterator for BlockIter<'a> {
	type Item = Rva;
	fn next(&mut self) -> Option<Rva> {
		self.iter.next().and_then(|tyoff| {
			let ty = ((tyoff.0 >> 12) & 0xFF) as u8;
			if ty != IMAGE_REL_BASED_ABSOLUTE {
				let offset = (tyoff.0 & 0x0FFF) as Rva;
				Some(self.rva + offset)
			}
			else {
				None
			}
		})
	}
}
// def_iter!(struct BlockIter -> IMAGE_BASE_RELOC_TYPEOFFSET, (Rva, u8); this |e| {
// 	let ty = ((e.0 >> 12) & 0xFF) as u8;
// 	let offset = (e.0 & 0x0FFF) as Rva;
// 	let rva = this.rva + offset;
// 	(rva, ty)
// });

//----------------------------------------------------------------
// Formatting

use strings::{Fmt, stringify_reloc_type};

impl<'a, P: Pe<'a> + Copy> fmt::Debug for BaseRelocs<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for it in *self {
			write!(f, "{:?}", it)?;
		}
		Ok(())
	}
}

impl<'a, P: Pe<'a> + Copy> fmt::Debug for Block<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			#"Base Relocations\n",
			#"  VirtualAddress: {:·>8X}\n", self.image.VirtualAddress,
			#"  SizeOfBlock:    {:·>8X}", self.image.SizeOfBlock,
			#"{}\n", Fmt(|f| {
				for it in self.words() {
					let rva = self.rva_of(it);
					let ty = self.type_of(it);
					write!(f, "\n    {:X} {:·>8X}", ty, rva)?;
					if let Some(ty) = stringify_reloc_type(it) {
						write!(f, ": {}", ty)?;
					}
				}
				Ok(())
			}),
		)
	}
}
