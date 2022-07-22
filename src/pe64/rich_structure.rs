#[cfg(test)]
pub(crate) fn test<'a, P: super::Pe<'a>>(pe: P) -> crate::Result<()> {
    let rich_structure = pe.rich_structure()?;
    let _checksum = rich_structure.checksum();

    let records: Vec<_> = rich_structure.records().collect();
    let mut encoded = vec![0u32; rich_structure.image().len()];
    let _ = rich_structure.encode(&records, &mut encoded);

    Ok(())
}
