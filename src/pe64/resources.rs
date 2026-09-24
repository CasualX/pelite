use super::*;

impl<'a> PeFile<'a> {
	#[doc = include_str!("../docs/resources.md")]
	pub fn resources(self) -> Result<crate::resources::ResourceDirectory<'a>> where Self: Copy {
		let datadir = self.data_directory().get(IMAGE_DIRECTORY_ENTRY_RESOURCE).ok_or(Error::Bounds)?;
		if datadir.VirtualAddress == 0 {
			return Err(Error::Null);
		}
		let bytes = self.slice_bytes(datadir.VirtualAddress)?;
		let size = cmp::min(datadir.Size as usize, bytes.len());
		Ok(crate::resources::ResourceDirectory::new(&bytes[..size], datadir))
	}
}

impl<'a> PeView<'a> {
	#[doc = include_str!("../docs/resources.md")]
	pub fn resources(self) -> Result<crate::resources::ResourceDirectory<'a>> where Self: Copy {
		let datadir = self.data_directory().get(IMAGE_DIRECTORY_ENTRY_RESOURCE).ok_or(Error::Bounds)?;
		if datadir.VirtualAddress == 0 {
			return Err(Error::Null);
		}
		let bytes = self.slice_bytes(datadir.VirtualAddress)?;
		let size = cmp::min(datadir.Size as usize, bytes.len());
		Ok(crate::resources::ResourceDirectory::new(&bytes[..size], datadir))
	}
}

#[cfg(test)]
pub(crate) fn test_resources<'a, P: Copy + Pe<'a>>(pe: P) -> Result<()> {
	pe.resources().and_then(crate::resources::test)
}
