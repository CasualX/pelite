use crate::security::Security;
use crate::util::AlignTo;
use crate::{Error, Result};

use super::image::*;
use super::{Align, Pe};

pub(crate) fn try_from<'a, P: Pe<'a>>(pe: P) -> Result<Security<'a>> {
    // The security info is part of the mapped image
    if pe.align() != Align::File {
        return Err(Error::Unmapped);
    }
    // Manual alignment and size check
    let datadir = pe
        .data_directory()
        .get(IMAGE_DIRECTORY_ENTRY_SECURITY)
        .ok_or(Error::Bounds)?;
    if datadir.VirtualAddress == 0 {
        return Err(Error::Null);
    }
    if !datadir.VirtualAddress.aligned_to(8) || !datadir.Size.aligned_to(8) {
        return Err(Error::Misaligned);
    }
    if datadir.Size == 0 {
        return Err(Error::Bounds);
    }
    // Interpret the bytes
    let start = datadir.VirtualAddress as usize;
    let end = (datadir.VirtualAddress + datadir.Size) as usize;
    let image = pe.image().get(start..end).ok_or(Error::Bounds)?;
    Ok(unsafe { Security::new(image) })
}

#[cfg(test)]
pub(crate) fn test<'a, P: Pe<'a>>(pe: P) -> Result<()> {
    let security = pe.security()?;
    let _ = format!("{:?}", security);
    let _certificate_type = security.certificate_type();
    let _certificate_data = security.certificate_data();
    Ok(())
}
