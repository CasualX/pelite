#[cfg(test)]
use super::*;

#[doc(inline)]
pub use crate::rich_structure::*;

#[cfg(test)]
pub(crate) fn test_rich_structure<'a, P: Pe<'a>>(pe: P) -> Result<()> {
	let rich_structure = pe.rich_structure()?;
	let _checksum = rich_structure.checksum();

	let records: Vec<_> = rich_structure.records().collect();
	let mut encoded = vec![0u32; rich_structure.image().len()];
	let _ = rich_structure.encode(&records, &mut encoded);

	Ok(())
}
