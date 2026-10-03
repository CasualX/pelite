use super::*;

impl<'a> PeFile<'a> {
	#[doc = include_str!("../docs/clr.md")]
	#[inline]
	pub fn clr(self) -> Result<crate::clr::ClrDirectory<'a>> {
		try_from(self)
	}
}

impl<'a> PeView<'a> {
	#[doc = include_str!("../docs/clr.md")]
	#[inline]
	pub fn clr(self) -> Result<crate::clr::ClrDirectory<'a>> {
		try_from(self)
	}
}

#[doc(inline)]
pub use crate::clr::*;

pub(crate) fn try_from<'a, P: Copy + Pe<'a>>(pe: P) -> Result<ClrDirectory<'a>> {
	let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR).ok_or(Error::Bounds)?;
	if datadir.VirtualAddress == 0 {
		return Err(Error::Null);
	}
	if datadir.Size < mem::size_of::<IMAGE_COR20_HEADER>() as u32 {
		return Err(Error::Bounds);
	}
	let image: &IMAGE_COR20_HEADER = pe.derva(datadir.VirtualAddress)?;
	if image.cb < mem::size_of::<IMAGE_COR20_HEADER>() as u32 || image.cb > datadir.Size {
		return Err(Error::Invalid);
	}
	// Validate the declared header extent, including any future extension.
	pe.slice(datadir.VirtualAddress, image.cb as usize, mem::align_of::<IMAGE_COR20_HEADER>())?;
	let directories = [
		image.MetaData, image.Resources, image.StrongNameSignature,
		image.CodeManagerTable, image.VTableFixups,
		image.ExportAddressTableJumps, image.ManagedNativeHeader,
	];
	// Retain each read result independently, so bad blobs do not reject the
	// header. The shared directory only needs borrowed data, not a typed PE.
	let data = directories.map(|directory| {
		if directory.VirtualAddress == 0 {
			Err(Error::Null)
		}
		else {
			pe.derva_slice(directory.VirtualAddress, directory.Size as usize)
		}
	});
	Ok(ClrDirectory::new(image, data))
}
