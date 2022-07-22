use super::Wrap;
use crate::*;

/// TLS Directory.
impl<'a, Pe32: pe32::Pe<'a>, Pe64: pe64::Pe<'a>>
    Wrap<pe32::tls::Tls<'a, Pe32>, pe64::tls::Tls<'a, Pe64>>
{
    /// Gets the PE instance.
    #[inline]
    pub fn pe(&self) -> Wrap<Pe32, Pe64> {
        match self {
            Wrap::T32(tls) => Wrap::T32(tls.pe()),
            Wrap::T64(tls) => Wrap::T64(tls.pe()),
        }
    }
    /// Returns the underlying TLS directory image.
    #[inline]
    pub fn image(
        &self,
    ) -> Wrap<&'a image::IMAGE_TLS_DIRECTORY32, &'a image::IMAGE_TLS_DIRECTORY64> {
        match self {
            Wrap::T32(tls) => Wrap::T32(tls.image()),
            Wrap::T64(tls) => Wrap::T64(tls.image()),
        }
    }
    /// Gets the raw TLS initialization data.
    #[inline]
    pub fn raw_data(&self) -> Result<&'a [u8]> {
        match self {
            Wrap::T32(tls) => tls.raw_data(),
            Wrap::T64(tls) => tls.raw_data(),
        }
    }
    /// Gets the TLS slot location.
    #[inline]
    pub fn slot(&self) -> Result<&'a u32> {
        match self {
            Wrap::T32(tls) => tls.slot(),
            Wrap::T64(tls) => tls.slot(),
        }
    }
    /// Gets the TLS initialization callbacks.
    #[inline]
    pub fn callbacks(&self) -> Result<Wrap<&'a [u32], &'a [u64]>> {
        match self {
            Wrap::T32(tls) => Wrap::T32(tls.callbacks()).transpose(),
            Wrap::T64(tls) => Wrap::T64(tls.callbacks()).transpose(),
        }
    }
}
