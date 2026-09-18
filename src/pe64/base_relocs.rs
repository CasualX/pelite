use super::*;

#[doc(inline)]
pub use crate::base_relocs::*;

pub(crate) fn try_from<'a, P: Pe<'a>>(pe: P) -> Result<BaseRelocationDirectory<'a>> {
	let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_BASERELOC).ok_or(Error::Bounds)?;
	if datadir.VirtualAddress == 0 {
		return Err(Error::Null);
	}
	let relocs = pe.slice(datadir.VirtualAddress, datadir.Size as usize, 4)?; // $1
	let relocs = unsafe { relocs.get_unchecked(..datadir.Size as usize) };
	Ok(unsafe { BaseRelocationDirectory::new(relocs) })
}

#[cfg(test)]
pub(crate) fn test_base_relocs<'a, P: Pe<'a>>(pe: P) -> Result<()> {
	let base_relocs = pe.base_relocs()?;
	let _ = format!("{:?}", base_relocs);

	let mut baseline = base_relocs.iter_blocks().flat_map(move |block| {
		let _ = format!("{:?}", block);
		block
			.words()
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
