/*!
Load Config Directory.

# Examples

```
# #![allow(unused_variables)]
use pelite::pe64::{Pe, PeFile};

# #[allow(dead_code)]
fn example(file: PeFile<'_>) -> pelite::Result<()> {
    // Access the load config directory
    let load_config = file.load_config()?;

    // The only bits of interest here
    let security_cookie = load_config.security_cookie()?;
    let se_handler_table = load_config.se_handler_table()?;

    Ok(())
}
```
*/

use std::fmt;

use crate::{Error, Result};

use super::image::*;
use super::Pe;

/// Load Config Directory.
///
/// For more information see the [module-level documentation](index.html).
#[derive(Copy, Clone)]
pub struct LoadConfig<'a, P> {
    pe: P,
    image: &'a IMAGE_LOAD_CONFIG_DIRECTORY,
}
impl<'a, P: Pe<'a>> LoadConfig<'a, P> {
    pub(crate) fn try_from(pe: P) -> Result<LoadConfig<'a, P>> {
        let datadir = pe
            .data_directory()
            .get(IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG)
            .ok_or(Error::Bounds)?;
        let image = pe.derva(datadir.VirtualAddress)?;
        Ok(LoadConfig { pe, image })
    }
    /// Gets the PE instance.
    pub fn pe(&self) -> P {
        self.pe
    }
    /// Returns the underlying load config directory image.
    pub fn image(&self) -> &'a IMAGE_LOAD_CONFIG_DIRECTORY {
        self.image
    }
    /// Gets the default security cookie for the image.
    pub fn security_cookie(&self) -> Result<&'a u32> {
        self.pe.deref(self.image.SecurityCookie.into())
    }
    /// Gets the structured exception handler table.
    pub fn se_handler_table(&self) -> Result<&'a [Va]> {
        self.pe.deref_slice(
            self.image.SEHandlerTable.into(),
            self.image.SEHandlerCount as usize,
        )
    }
}
impl<'a, P: Pe<'a>> fmt::Debug for LoadConfig<'a, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("LoadConfig")
            .field(
                "security_cookie",
                &format_args!("{:x?}", self.security_cookie()),
            )
            .field(
                "se_handler_table.len",
                &format_args!("{:?}", self.se_handler_table().map(|seh| seh.len())),
            )
            .finish()
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::{LoadConfig, Pe};
    use crate::util::serde_helper::*;

    impl<'a, P: Pe<'a>> Serialize for LoadConfig<'a, P> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut state = serializer.serialize_struct("LoadConfig", 2)?;
            state.serialize_field("security_cookie", &self.security_cookie().ok())?;
            state.serialize_field("se_handler_table", &self.se_handler_table().ok())?;
            state.end()
        }
    }
}

#[cfg(test)]
pub(crate) fn test<'a, P: Pe<'a>>(pe: P) -> Result<()> {
    let load_config = pe.load_config()?;
    let _ = format!("{:?}", load_config);
    let _security_cookie = load_config.security_cookie();
    let _se_handler_table = load_config.se_handler_table();
    Ok(())
}
