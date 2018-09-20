/*!
Security Directory.

The security directory contains the digital signature if the module is signed.
The security directory is only available on disk images and isn't mapped to memory.

# Examples

```
# #![allow(unused_variables)]
use pelite::pe64::{Pe, PeFile};

# #[allow(dead_code)]
fn example(file: PeFile<'_>) -> pelite::Result<()> {
	// Access the security directory
	let security = file.security()?;

	// The raw certificate data bytes
	let certificate_data = security.certificate_data();

	Ok(())
}
```
*/

use std::{fmt};

use error::{Error, Result};

use super::image::*;
use super::{Align, Pe};

/// Security Directory.
///
/// For more information see the [module-level documentation](index.html).
pub struct Security<'a, P> {
	pe: P,
	security: &'a [u8],
}
impl<'a, P: Pe<'a> + Copy> Security<'a, P> {
	pub(crate) fn try_from(pe: P) -> Result<Security<'a, P>> {
		// The security info is part of the mapped image
		if pe.align() != Align::File {
			return Err(Error::Unmapped);
		}
		// Manual alignment and size check
		let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_SECURITY).ok_or(Error::Bounds)?;
		if datadir.VirtualAddress == 0 {
			return Err(Error::Null);
		}
		if datadir.VirtualAddress % 8 != 0 || datadir.Size % 8 != 0 {
			return Err(Error::Misaligned);
		}
		if datadir.Size == 0 {
			return Err(Error::Bounds);
		}
		// Interpret the bytes
		let start = datadir.VirtualAddress as usize;
		let end = (datadir.VirtualAddress + datadir.Size) as usize;
		let security = pe.image().get(start..end).ok_or(Error::Bounds)?;
		Ok(Security { pe, security })
	}
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Returns the underlying security directory image.
	pub fn image(&self) -> &'a WIN_CERTIFICATE {
		// Safety checked by new
		unsafe {
			&*(self.security.as_ptr() as *const _)
		}
	}
	/// Gets the type of the certificate.
	///
	/// List of known certificate types:
	///
	/// * [X.509: `WIN_CERT_TYPE_X509`](../../image/constant.WIN_CERT_TYPE_X509.html)
	/// * [PKCS SignedData: `WIN_CERT_TYPE_PKCS_SIGNED_DATA`](../../image/constant.WIN_CERT_TYPE_PKCS_SIGNED_DATA.html)
	/// * [PKCS1_MODULE_SIGN: `WIN_CERT_TYPE_PKCS1_SIGN`](../../image/constant.WIN_CERT_TYPE_PKCS1_SIGN.html)
	pub fn certificate_type(&self) -> u16 {
		self.image().wCertificateType
	}
	/// Gets the raw certificate data bytes.
	///
	/// The interpretation of this data depends the type of the certificate.
	/// No further introspection is provided.
	///
	/// External tools such as _OpenSSL_ can be used to further disect and analyze this data.
	/// Eg. for `WIN_CERT_TYPE_PKCS_SIGNED_DATA` the following can be used to decode the bytes:
	///
	/// ```sh
	/// openssl pkcs7 -inform DER -print_certs -text -in pe_certificate
	/// ```
	pub fn certificate_data(&self) -> &'a [u8] {
		// Safety checked by new
		unsafe {
			self.security.get_unchecked(8..)
		}
	}
}
impl<'a, P: Pe<'a> + Copy> fmt::Debug for Security<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("Security")
			.field("certificate_type", &self.certificate_type())
			.field("certificate_data.len", &self.certificate_data().len())
			.finish()
	}
}

#[cfg(feature = "serde")]
mod serde {
	use util::serde_helper::*;
	use super::{Pe, Security};

	impl<'a, P: Pe<'a> + Copy> Serialize for Security<'a, P> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let is_human_readable = serializer.is_human_readable();
			let mut state = serializer.serialize_struct("Security", 2)?;
			state.serialize_field("certificate_type", &self.certificate_type())?;
			if cfg!(feature = "data-encoding") && is_human_readable {
				#[cfg(feature = "data-encoding")]
				state.serialize_field("certificate_data",
					&::data_encoding::BASE64.encode(self.certificate_data()))?;
			}
			else {
				state.serialize_field("certificate_data", &self.certificate_data())?;
			}
			state.end()
		}
	}
}

#[cfg(test)]
pub(crate) fn test<'a, P: Pe<'a> + Copy>(pe: P) -> Result<()> {
	let security = pe.security()?;
	let _ = format!("{:?}", security);
	let _certificate_type = security.certificate_type();
	let _certificate_data = security.certificate_data();
	Ok(())
}
