use super::*;

impl<'a> PeFile<'a> {
	#[doc = include_str!("../docs/security.md")]
	#[inline]
	pub fn security(self) -> Result<crate::security::SecurityDirectory<'a>> {
		try_from(self)
	}
}

impl<'a> PeView<'a> {
	#[doc = include_str!("../docs/security.md")]
	#[inline]
	pub fn security(self) -> Result<crate::security::SecurityDirectory<'a>> {
		try_from(self)
	}
}

#[doc(inline)]
pub use crate::security::*;

pub(crate) fn try_from<'a, P: Copy + Pe<'a>>(pe: P) -> Result<SecurityDirectory<'a>> {
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
