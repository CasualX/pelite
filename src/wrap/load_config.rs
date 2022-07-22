use super::Wrap;
use crate::*;

/// Load Config Directory.
impl<'a, Pe32: pe32::Pe<'a>, Pe64: pe64::Pe<'a>>
    Wrap<pe32::load_config::LoadConfig<'a, Pe32>, pe64::load_config::LoadConfig<'a, Pe64>>
{
    /// Gets the PE instance.
    #[inline]
    pub fn pe(&self) -> Wrap<Pe32, Pe64> {
        match self {
            Wrap::T32(load_config) => Wrap::T32(load_config.pe()),
            Wrap::T64(load_config) => Wrap::T64(load_config.pe()),
        }
    }
    /// Returns the underlying load config directory image.
    #[inline]
    pub fn image(
        &self,
    ) -> Wrap<&'a image::IMAGE_LOAD_CONFIG_DIRECTORY32, &'a image::IMAGE_LOAD_CONFIG_DIRECTORY64>
    {
        match self {
            Wrap::T32(load_config) => Wrap::T32(load_config.image()),
            Wrap::T64(load_config) => Wrap::T64(load_config.image()),
        }
    }
    /// Gets the default security cookie for the image.
    #[inline]
    pub fn security_cookie(&self) -> Result<&'a u32> {
        match self {
            Wrap::T32(load_config) => load_config.security_cookie(),
            Wrap::T64(load_config) => load_config.security_cookie(),
        }
    }
    /// Gets the structured exception handler table.
    #[inline]
    pub fn se_handler_table(&self) -> Result<Wrap<&'a [u32], &'a [u64]>> {
        match self {
            Wrap::T32(load_config) => Wrap::T32(load_config.se_handler_table()).transpose(),
            Wrap::T64(load_config) => Wrap::T64(load_config.se_handler_table()).transpose(),
        }
    }
}
