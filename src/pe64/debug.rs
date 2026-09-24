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

#[cfg(test)]
pub(crate) fn test_debug<'a, P: Copy + Pe<'a>>(pe: P) -> Result<()> {
	let debug = pe.debug()?;
	for dir in debug {
		let _data = dir.data();
		match dir.entry() {
			Ok(Some(DebugData::CodeView(cv))) => {
				let _format = cv.format();
				let _pdb_file_name = cv.pdb_file_name();
			},
			Ok(Some(DebugData::Misc(_misc))) => (),
			Ok(Some(DebugData::Pgo(pgo))) => for _section in pgo {},
			Ok(None) => (),
			Err(_) => (),
		}
	}
	Ok(())
}
