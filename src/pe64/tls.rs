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

use ::std::fmt;

use super::image::*;
use super::{Pe};
use ::{Error, Result};

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
		let rva = self.pe.va_to_rva(self.image.StartAddressOfRawData)?;
		// FIXME! truncation warning on 32bit...
		let len = (self.image.EndAddressOfRawData - self.image.StartAddressOfRawData) as usize;
		self.pe.derva_slice(rva, len)
	}
	pub fn slot(&self) -> Result<&'a u32> {
		let rva = self.pe.va_to_rva(self.image.AddressOfIndex)?;
		self.pe.derva(rva)
	}
	pub fn callbacks(&self) -> Result<&'a [Va]> {
		let rva = self.pe.va_to_rva(self.image.AddressOfCallBacks)?;
		self.pe.derva_slice_f(rva, |&callback| callback == BADVA)
	}
}

//----------------------------------------------------------------
// Formatting

use ::strings::Fmt;

impl<'a, P: Pe<'a> + Copy> fmt::Debug for Tls<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
			#"{:?}", self.image(),
			#"TLS Callbacks",
			#"{}\n", Fmt(|f| {
				match self.callbacks() {
					Ok(callbacks) => {
						for va in callbacks {
							write!(f, "\n  {:·>16X}", va)?;
						}
						Ok(())
					},
					e @ Err(_) => {
						write!(f, ": {:?}", e)
					}
				}
			}),
		)
	}
}
