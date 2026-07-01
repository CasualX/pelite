#[cfg(test)]
pub(crate) fn test<'a, P: super::Pe<'a>>(pe: P) -> crate::Result<()> {
    use heapless::Vec;

	let rich_structure = pe.rich_structure()?;
	let _checksum = rich_structure.checksum();

	let records: Vec<_, 64> = rich_structure.records().collect();
	let mut encoded: Vec<u32, 128> = Vec::new();
	let _ = rich_structure.encode(&records, &mut encoded);

	Ok(())
}
