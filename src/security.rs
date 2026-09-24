//! Authenticode security directory parsing.

use core::{fmt, mem};

use crate::image::WIN_CERTIFICATE;
use crate::util::AlignTo;

/// Security directory.
///
/// Certificates are stored in the file and are not mapped into the loaded image.
///
/// # Examples
///
/// ```
/// # #![allow(unused_variables)]
/// use pelite::pe64::{Pe, PeFile};
///
/// # #[allow(dead_code)]
/// fn example(file: PeFile<'_>) -> pelite::Result<()> {
/// 	// Access the security directory
/// 	let security = file.security()?;
///
/// 	// The raw certificate data bytes
/// 	let certificate_data = security.certificate_data();
///
/// 	Ok(())
/// }
/// ```
#[derive(Copy, Clone)]
pub struct SecurityDirectory<'a> {
	image: &'a [u8], // unsafe: MUST BE DWORD ALIGNED!
}
impl<'a> SecurityDirectory<'a> {
	pub(crate) unsafe fn new(image: &'a [u8]) -> SecurityDirectory<'a> {
		debug_assert!(image.as_ptr().aligned_to(mem::align_of::<WIN_CERTIFICATE>()));
		debug_assert!(image.len() >= 8);
		SecurityDirectory { image }
	}
	/// Returns the raw certificate header.
	pub fn image(&self) -> &'a WIN_CERTIFICATE {
		// Safety checked by new
		unsafe { &*(self.image.as_ptr() as *const _) }
	}
	/// Returns the certificate type.
	///
	/// Known types include [X.509][crate::image::WIN_CERT_TYPE_X509], [PKCS SignedData][crate::image::WIN_CERT_TYPE_PKCS_SIGNED_DATA], and [PKCS1 module sign][crate::image::WIN_CERT_TYPE_PKCS1_SIGN].
	pub fn certificate_type(&self) -> u16 {
		self.image().wCertificateType
	}
	/// Returns the raw certificate data bytes.
	///
	/// The format depends on the [certificate type][Self::certificate_type]. For PKCS SignedData, external tools can decode the DER data.
	pub fn certificate_data(&self) -> &'a [u8] {
		// Safety checked by new
		unsafe { self.image.get_unchecked(8..) }
	}
}
impl<'a> fmt::Debug for SecurityDirectory<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("SecurityDirectory")
			.field("length", &self.image().dwLength)
			.field("revision", &self.image().wRevision)
			.field("certificate_type", &self.certificate_type())
			.field("certificate_data.len", &self.certificate_data().len())
			.finish()
	}
}

serde_impl! {
	impl<'a> Serialize for SecurityDirectory<'a> {
		fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
			let is_human_readable = serializer.is_human_readable();
			let mut state = serializer.serialize_struct("SecurityDirectory", 3)?;
			state.serialize_field("image", self.image())?;
			state.serialize_field("certificate_type", &self.certificate_type())?;
			if cfg!(feature = "basenc") && is_human_readable {
				#[cfg(feature = "basenc")]
				state.serialize_field("certificate_data", &basenc::Base64Std.encode(self.certificate_data()))?;
			}
			else {
				state.serialize_field("certificate_data", &self.certificate_data())?;
			}
			state.end()
		}
	}
}
