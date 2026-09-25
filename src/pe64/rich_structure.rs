use super::*;

impl<'a> PeFile<'a> {
	#[doc = include_str!("../docs/rich_structure.md")]
	pub fn rich_structure(self) -> Result<crate::rich_structure::RichStructure<'a>> {
		let image = self.image();
		let image = unsafe { slice::from_raw_parts(image.as_ptr() as *const u32, image.len() / 4) };
		crate::rich_structure::RichStructure::try_from(image)
	}
}

impl<'a> PeView<'a> {
	#[doc = include_str!("../docs/rich_structure.md")]
	pub fn rich_structure(self) -> Result<crate::rich_structure::RichStructure<'a>> {
		let image = self.image();
		let image = unsafe { slice::from_raw_parts(image.as_ptr() as *const u32, image.len() / 4) };
		crate::rich_structure::RichStructure::try_from(image)
	}
}

#[doc(inline)]
pub use crate::rich_structure::*;
