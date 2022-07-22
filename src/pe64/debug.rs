/*!
Debug Directory.

# Examples

```
# #![allow(unused_variables)]
use pelite::pe64::{Pe, PeFile, debug};

# #[allow(dead_code)]
fn example(file: PeFile<'_>) -> pelite::Result<()> {
    // Access the debug directory
    let debug = file.debug()?;

    // Get the CodeView PDB file name
    if let Some(pdb_file_name) = debug.pdb_file_name() {
        println!("PDB: {}", pdb_file_name);
    }

    Ok(())
}
```
*/

use std::{fmt, iter, mem, slice};

use crate::util::AlignTo;
use crate::util::CStr;
use crate::{Error, Result};

use super::{image::*, Align, Pe};

//----------------------------------------------------------------

/// Debug directory.
///
/// For more information see the [module-level documentation](index.html).
#[derive(Copy, Clone)]
pub struct Debug<'a, P> {
    pe: P,
    image: &'a [IMAGE_DEBUG_DIRECTORY],
}
impl<'a, P: Pe<'a>> Debug<'a, P> {
    pub(crate) fn try_from(pe: P) -> Result<Debug<'a, P>> {
        let datadir = pe
            .data_directory()
            .get(IMAGE_DIRECTORY_ENTRY_DEBUG)
            .ok_or(Error::Bounds)?;
        let (len, rem) = (
            datadir.Size as usize / mem::size_of::<IMAGE_DEBUG_DIRECTORY>(),
            datadir.Size as usize % mem::size_of::<IMAGE_DEBUG_DIRECTORY>(),
        );
        if rem != 0 {
            return Err(Error::Invalid);
        }
        let image = pe.derva_slice(datadir.VirtualAddress, len)?;
        Ok(Debug { pe, image })
    }
    /// Gets the PE instance.
    pub fn pe(&self) -> P {
        self.pe
    }
    /// Returns the underlying debug directories image.
    pub fn image(&self) -> &'a [IMAGE_DEBUG_DIRECTORY] {
        self.image
    }
    /// Gets the CodeView PDB file name.
    pub fn pdb_file_name(&self) -> Option<&'a CStr> {
        self.into_iter()
            .filter_map(|dir| {
                dir.entry()
                    .ok()
                    .and_then(Entry::as_code_view)
                    .map(|cv| cv.pdb_file_name())
            })
            .next()
    }
    /// Iterator over the debug directories.
    pub fn iter(&self) -> Iter<'a, P> {
        Iter {
            pe: self.pe,
            iter: self.image.iter(),
        }
    }
}
impl<'a, P: Pe<'a>> IntoIterator for Debug<'a, P> {
    type Item = Dir<'a, P>;
    type IntoIter = Iter<'a, P>;
    fn into_iter(self) -> Iter<'a, P> {
        self.iter()
    }
}
impl<'a, P: Pe<'a>> fmt::Debug for Debug<'a, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(*self).finish()
    }
}

//----------------------------------------------------------------

/// Iterator over Dir entries.
#[derive(Clone)]
pub struct Iter<'a, P> {
    pe: P,
    iter: slice::Iter<'a, IMAGE_DEBUG_DIRECTORY>,
}
impl<'a, P: Pe<'a>> Iter<'a, P> {
    pub fn image(&self) -> &'a [IMAGE_DEBUG_DIRECTORY] {
        self.iter.as_slice()
    }
}
impl<'a, P: Pe<'a>> Iterator for Iter<'a, P> {
    type Item = Dir<'a, P>;
    fn next(&mut self) -> Option<Dir<'a, P>> {
        self.iter.next().map(|image| Dir { pe: self.pe, image })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn nth(&mut self, n: usize) -> Option<Dir<'a, P>> {
        self.iter.nth(n).map(|image| Dir { pe: self.pe, image })
    }
}
impl<'a, P: Pe<'a>> DoubleEndedIterator for Iter<'a, P> {
    fn next_back(&mut self) -> Option<Dir<'a, P>> {
        self.iter
            .next_back()
            .map(|image| Dir { pe: self.pe, image })
    }
}
impl<'a, P: Pe<'a>> ExactSizeIterator for Iter<'a, P> {}
impl<'a, P: Pe<'a>> iter::FusedIterator for Iter<'a, P> {}

//----------------------------------------------------------------

/// Debug directory entry.
#[derive(Copy, Clone)]
pub struct Dir<'a, P> {
    pe: P,
    image: &'a IMAGE_DEBUG_DIRECTORY,
}
impl<'a, P: Pe<'a>> Dir<'a, P> {
    /// Gets the PE instance.
    pub fn pe(&self) -> P {
        self.pe
    }
    /// Gets the underlying debug directory image.
    pub fn image(&self) -> &'a IMAGE_DEBUG_DIRECTORY {
        self.image
    }
    /// Gets the raw data of this debug directory entry.
    pub fn data(&self) -> Option<&'a [u8]> {
        let image = self.pe.image();
        let size = self.image.SizeOfData as usize;
        let offset = match self.pe.align() {
            Align::File => self.image.PointerToRawData,
            Align::Section => self.image.AddressOfRawData,
        } as usize;
        image.get(offset..offset.wrapping_add(size))
    }
    /// Interprets the directory entry.
    pub fn entry(&self) -> Result<Entry<'a>> {
        match self.image.Type {
            IMAGE_DEBUG_TYPE_CODEVIEW => Ok(Entry::CodeView(code_view(&self)?)),
            IMAGE_DEBUG_TYPE_MISC => Ok(Entry::Dbg(dbg(&self)?)),
            IMAGE_DEBUG_TYPE_POGO => Ok(Entry::Pgo(pgo(&self)?)),
            _ => Ok(Entry::Unknown(self.data())),
        }
    }
}
impl<'a, P: Pe<'a>> fmt::Debug for Dir<'a, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Dir")
            .field(
                "type",
                &crate::stringify::DebugType(self.image.Type)
                    .to_str()
                    .ok_or(self.image.Type),
            )
            .field("time_date_stamp", &self.image.TimeDateStamp)
            .field("version", &self.image.Version)
            .field("entry", &self.entry())
            .finish()
    }
}

//----------------------------------------------------------------

pub use crate::wrap::debug::{CodeView, Dbg, Entry, Pgo, PgoItem, PgoIter};

fn code_view<'a, P: Pe<'a>>(dir: &Dir<'a, P>) -> Result<CodeView<'a>> {
    let bytes = dir.data().ok_or(Error::Bounds)?;
    if bytes.len() < 16 {
        return Err(Error::Bounds);
    }
    if !(cfg!(feature = "unsafe_alignment") || bytes.as_ptr().aligned_to(4)) {
        return Err(Error::Misaligned);
    }
    let cv_signature = unsafe { &*(bytes.as_ptr() as *const [u8; 4]) };
    match cv_signature {
        b"NB10" => {
            if bytes.len() < 16 {
                return Err(Error::Bounds);
            }
            let image = unsafe { &*(bytes.as_ptr() as *const IMAGE_DEBUG_CV_INFO_PDB20) };
            let pdb_file_name = CStr::from_bytes(&bytes[16..]).ok_or(Error::Encoding)?;
            Ok(CodeView::Cv20 {
                image,
                pdb_file_name,
            })
        }
        b"RSDS" => {
            if bytes.len() < 24 {
                return Err(Error::Bounds);
            }
            let image = unsafe { &*(bytes.as_ptr() as *const IMAGE_DEBUG_CV_INFO_PDB70) };
            let pdb_file_name = CStr::from_bytes(&bytes[24..]).ok_or(Error::Encoding)?;
            Ok(CodeView::Cv70 {
                image,
                pdb_file_name,
            })
        }
        _ => Err(Error::BadMagic),
    }
}

fn dbg<'a, P: Pe<'a>>(dir: &Dir<'a, P>) -> Result<Dbg<'a>> {
    let data = dir.data().ok_or(Error::Bounds)?;
    if data.len() < mem::size_of::<IMAGE_DEBUG_MISC>() {
        return Err(Error::Bounds);
    }
    if !(cfg!(feature = "unsafe_alignment") || data.as_ptr().aligned_to(4)) {
        return Err(Error::Misaligned);
    }
    let image = unsafe { &*(data.as_ptr() as *const IMAGE_DEBUG_MISC) };
    Ok(Dbg { image })
}

fn pgo<'a, P: Pe<'a>>(dir: &Dir<'a, P>) -> Result<Pgo<'a>> {
    let data = dir.data().ok_or(Error::Bounds)?;
    if data.len() < 4 {
        return Err(Error::Bounds);
    }
    if !(cfg!(feature = "unsafe_alignment") || data.as_ptr().aligned_to(4)) {
        return Err(Error::Misaligned);
    }
    let len = data.len() / 4;
    let image = unsafe { slice::from_raw_parts(data.as_ptr() as *const u32, len) };
    Ok(Pgo { image })
}

//----------------------------------------------------------------

/*
    "debug": [
        {
            "type": "CodeView",
            "time_date_stamp": 0,
            "version": "1.0",
            "entry": {
                format: "RSDS",
                pdb_file_name: "",
                ...
            },
        },
    ],
*/

#[cfg(feature = "serde")]
mod serde {
    use super::{Debug, Dir, Pe};
    use crate::util::serde_helper::*;

    impl<'a, P: Pe<'a>> Serialize for Debug<'a, P> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.collect_seq(self.into_iter())
        }
    }
    impl<'a, P: Pe<'a>> Serialize for Dir<'a, P> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let is_human_readable = serializer.is_human_readable();
            let mut state = serializer.serialize_struct("Dir", 4)?;
            if is_human_readable {
                state.serialize_field(
                    "type",
                    &crate::stringify::DebugType(self.image.Type).to_str(),
                )?;
            } else {
                state.serialize_field("type", &self.image.Type)?;
            }
            state.serialize_field("time_date_stamp", &self.image.TimeDateStamp)?;
            state.serialize_field("version", &self.image.Version)?;
            state.serialize_field("entry", &self.entry().ok())?;
            state.end()
        }
    }
}

//----------------------------------------------------------------

#[cfg(test)]
pub(crate) fn test<'a, P: Pe<'a>>(pe: P) -> Result<()> {
    let debug = pe.debug()?;
    for dir in debug {
        let _data = dir.data();
        match dir.entry() {
            Ok(Entry::CodeView(cv)) => {
                let _format = cv.format();
                let _pdb_file_name = cv.pdb_file_name();
            }
            Ok(Entry::Dbg(_dbg)) => (),
            Ok(Entry::Pgo(pgo)) => for _sec in pgo {},
            Ok(Entry::Unknown(_data)) => (),
            Err(_) => (),
        }
    }
    Ok(())
}
