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

#[cfg(test)]
pub(crate) fn test_rich_structure<'a, P: Copy + Pe<'a>>(pe: P) -> Result<()> {
	let rich_structure = pe.rich_structure()?;
	let _checksum = rich_structure.checksum();

	let records: Vec<_> = rich_structure.records().collect();
	let mut encoded = vec![0u32; rich_structure.image().len()];
	let _ = rich_structure.encode(&records, &mut encoded);

	Ok(())
}
