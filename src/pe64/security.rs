use super::*;

#[doc(inline)]
pub use crate::security::*;

pub(crate) fn try_from<'a, P: Pe<'a>>(pe: P) -> Result<SecurityDirectory<'a>> {
	let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_SECURITY).ok_or(Error::Bounds)?;
	if datadir.VirtualAddress == 0 {
		return Err(Error::Null);
	}
	// The security info is not part of the mapped image
	if pe.layout() != PeLayout::File {
		return Err(Error::Unmapped);
	}
	// Manual alignment and size check
	if !datadir.VirtualAddress.aligned_to(8) || !datadir.Size.aligned_to(8) {
		return Err(Error::Misaligned);
	}
	if datadir.Size == 0 {
		return Err(Error::Bounds);
	}
	// Interpret the bytes
	let start = datadir.VirtualAddress as usize;
	let end = datadir.VirtualAddress.checked_add(datadir.Size).ok_or(Error::Overflow)? as usize;
	let image = pe.image().get(start..end).ok_or(Error::Bounds)?;
	Ok(unsafe { SecurityDirectory::new(image) })
}

#[cfg(test)]
pub(crate) fn test_security<'a, P: Pe<'a>>(pe: P) -> Result<()> {
	let security = pe.security()?;
	let _ = format!("{:?}", security);
	let _certificate_type = security.certificate_type();
	let _certificate_data = security.certificate_data();
	Ok(())
}
