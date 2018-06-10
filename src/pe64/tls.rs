/*!
TLS Directory.

# Examples

```
# #![allow(unused_variables)]
use pelite::pe64::{Pe, PeFile};

# #[allow(dead_code)]
fn example(file: PeFile) -> pelite::Result<()> {
	// Access the TLS directory
	let tls = file.tls()?;

	// Access the initialized thread local data
	let raw_data = tls.raw_data()?;

	// Access the TLS slot
	let slot = tls.slot()?;

	// Access the TLS callbacks
	let callbacks = tls.callbacks()?;

	Ok(())
}
```
*/

use std::fmt;

use error::{Error, Result};

use super::image::*;
use super::Pe;

//----------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct Tls<'a, P> {
	pe: P,
	image: &'a IMAGE_TLS_DIRECTORY,
}
impl<'a, P: Pe<'a> + Copy> Tls<'a, P> {
	pub(crate) fn new(pe: P) -> Result<Tls<'a, P>> {
		let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_TLS).ok_or(Error::OOB)?;
		let image = pe.derva(datadir.VirtualAddress)?;
		Ok(Tls { pe, image })
	}
	pub fn pe(&self) -> P {
		self.pe
	}
	pub fn image(&self) -> &'a IMAGE_TLS_DIRECTORY {
		self.image
	}
	pub fn raw_data(&self) -> Result<&'a [u8]> {
		if self.image.StartAddressOfRawData > self.image.EndAddressOfRawData {
			return Err(Error::Corrupt);
		}
		// FIXME! truncation warning on 32bit...
		let len = (self.image.EndAddressOfRawData - self.image.StartAddressOfRawData) as usize;
		self.pe.deref_slice(self.image.StartAddressOfRawData, len)
	}
	pub fn slot(&self) -> Result<&'a u32> {
		self.pe.deref(self.image.AddressOfIndex)
	}
	pub fn callbacks(&self) -> Result<&'a [Va]> {
		self.pe.deref_slice_s(self.image.AddressOfCallBacks, BADVA)
	}
}
impl<'a, P: Pe<'a> + Copy> fmt::Debug for Tls<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Tls")
			.field("raw_data.len", &format_args!("{:?}", self.raw_data().map(|raw_data| raw_data.len())))
			.field("callbacks.len", &format_args!("{:?}", &self.callbacks().map(|cbs| cbs.len())))
			.finish()
	}
}
