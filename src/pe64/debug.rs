use super::*;

impl<'a> PeFile<'a> {
	#[doc = include_str!("../docs/debug.md")]
	#[inline]
	pub fn debug(self) -> Result<crate::debug::DebugDirectory<'a>> {
		try_from(self)
	}
}

impl<'a> PeView<'a> {
	#[doc = include_str!("../docs/debug.md")]
	#[inline]
	pub fn debug(self) -> Result<crate::debug::DebugDirectory<'a>> {
		try_from(self)
	}
}

#[doc(inline)]
pub use crate::debug::*;

pub(crate) fn try_from<'a, P: Copy + Pe<'a>>(pe: P) -> Result<DebugDirectory<'a>> {
	let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_DEBUG).ok_or(Error::Bounds)?;
	if datadir.VirtualAddress == 0 {
		return Err(Error::Null);
	}
	let size = datadir.Size as usize;
	if size % mem::size_of::<IMAGE_DEBUG_DIRECTORY>() != 0 {
		return Err(Error::Invalid);
	}
	let image = pe.derva_slice(datadir.VirtualAddress, size / mem::size_of::<IMAGE_DEBUG_DIRECTORY>())?;
	Ok(DebugDirectory::new(pe.image(), pe.layout(), image))
}
