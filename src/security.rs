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

use crate::image::WIN_CERTIFICATE;
use crate::util::AlignTo;
use std::{fmt, mem};

/// Security Directory.
///
/// For more information see the [module-level documentation](index.html).
#[derive(Copy, Clone)]
pub struct Security<'a> {
    image: &'a [u8], // unsafe: MUST BE DWORD ALIGNED!
}
impl<'a> Security<'a> {
    pub(crate) unsafe fn new(image: &'a [u8]) -> Security<'a> {
        debug_assert!(image
            .as_ptr()
            .aligned_to(mem::align_of::<WIN_CERTIFICATE>()));
        debug_assert!(image.len() >= 8);
        Security { image }
    }
    /// Returns the underlying security directory image.
    pub fn image(&self) -> &'a WIN_CERTIFICATE {
        // Safety checked by new
        unsafe { &*(self.image.as_ptr() as *const _) }
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
        unsafe { self.image.get_unchecked(8..) }
    }
}
impl<'a> fmt::Debug for Security<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Security")
            .field("certificate_type", &self.certificate_type())
            .field("certificate_data.len", &self.certificate_data().len())
            .finish()
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::Security;
    use crate::util::serde_helper::*;

    impl<'a> Serialize for Security<'a> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let is_human_readable = serializer.is_human_readable();
            let mut state = serializer.serialize_struct("Security", 2)?;
            state.serialize_field("certificate_type", &self.certificate_type())?;
            if cfg!(feature = "data-encoding") && is_human_readable {
                #[cfg(feature = "data-encoding")]
                state.serialize_field(
                    "certificate_data",
                    &data_encoding::BASE64.encode(self.certificate_data()),
                )?;
            } else {
                state.serialize_field("certificate_data", &self.certificate_data())?;
            }
            state.end()
        }
    }
}
