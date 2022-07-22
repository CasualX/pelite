/*!
Import Directory and the IAT.

The import directory lists all the module dependencies and their imported symbols by this module.

The Import Address Table (IAT) lists all the imported symbols for all the modules in one big list.
When the imports are resolved the IAT is overwritten with pointers to the resolved functions.

# Examples

```
# #![allow(unused_variables)]
use pelite::pe64::{Pe, PeFile};

# #[allow(dead_code)]
fn example(file: PeFile<'_>) -> pelite::Result<()> {
    // Access the import directory
    let imports = file.imports()?;

    // Iterate over the import descriptors
    for desc in imports {
        // DLL being imported from
        let dll_name = desc.dll_name()?;

        // Import Address Table and Import Name Table for this imported DLL
        let iat = desc.iat()?;
        let int = desc.int()?;

        // Iterate over the imported functions from this DLL
        for (va, import) in Iterator::zip(iat, int) {}
    }

    // Iterate over the IAT
    for (va, import) in file.iat()?.iter() {
        // The IAT may contains Null entries where the IAT of imported modules join
        if let Ok(import) = import {}
    }

    Ok(())
}
```
*/

use std::{fmt, iter, mem, slice};

use crate::util::CStr;
use crate::{Error, Result};

use super::image::*;
use super::Pe;

//----------------------------------------------------------------

pub use crate::wrap::imports::Import;

// Gets the import from the import name table.
//
// These aren't actually virtual addresses.
// This function will decode them to get the import.
fn import_from_va<'a, P: Pe<'a>>(pe: P, &va: &'a Va) -> Result<Import<'a>> {
    if va & IMAGE_ORDINAL_FLAG == 0 {
        // TODO! Validate that this really is an Rva in PE32+?
        let rva = va as Rva;
        let hint = pe.derva::<u16>(rva)?;
        let name = pe.derva_c_str(rva + 2)?;
        Ok(Import::ByName {
            hint: *hint as usize,
            name,
        })
    } else {
        Ok(Import::ByOrdinal { ord: va as Ordinal })
    }
}

//----------------------------------------------------------------

/// Import directory.
///
/// For more information see the [module-level documentation](index.html).
#[derive(Copy, Clone)]
pub struct Imports<'a, P> {
    pe: P,
    image: &'a [IMAGE_IMPORT_DESCRIPTOR],
}
impl<'a, P: Pe<'a>> Imports<'a, P> {
    pub(crate) fn try_from(pe: P) -> Result<Imports<'a, P>> {
        let datadir = pe
            .data_directory()
            .get(IMAGE_DIRECTORY_ENTRY_IMPORT)
            .ok_or(Error::Bounds)?;
        let image = pe
            .derva_slice_f(datadir.VirtualAddress, |image: &IMAGE_IMPORT_DESCRIPTOR| {
                image.is_null()
            })?;
        Ok(Imports { pe, image })
    }
    /// Gets the PE instance.
    pub fn pe(&self) -> P {
        self.pe
    }
    /// Returns the underlying import directory image array.
    pub fn image(&self) -> &'a [IMAGE_IMPORT_DESCRIPTOR] {
        self.image
    }
    /// Iterator over the import descriptors.
    pub fn iter(&self) -> Iter<'a, P> {
        Iter {
            pe: self.pe,
            iter: self.image.iter(),
        }
    }
}
impl<'a, P: Pe<'a>> IntoIterator for Imports<'a, P> {
    type Item = Desc<'a, P>;
    type IntoIter = Iter<'a, P>;
    fn into_iter(self) -> Iter<'a, P> {
        self.iter()
    }
}
impl<'a, P: Pe<'a>> fmt::Debug for Imports<'a, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.into_iter()).finish()
    }
}

//----------------------------------------------------------------

/// Import Address Table.
///
/// For more information see the [module-level documentation](index.html).
#[derive(Copy, Clone)]
pub struct IAT<'a, P> {
    pe: P,
    image: &'a [Va],
}
impl<'a, P: Pe<'a>> IAT<'a, P> {
    pub(crate) fn try_from(pe: P) -> Result<IAT<'a, P>> {
        let datadir = pe
            .data_directory()
            .get(IMAGE_DIRECTORY_ENTRY_IAT)
            .ok_or(Error::Bounds)?;
        // Ignore datadir.Size not being a multiple of sizeof(Va), not that big of a deal...
        let image = pe.derva_slice(
            datadir.VirtualAddress,
            datadir.Size as usize / mem::size_of::<Va>(),
        )?;
        Ok(IAT { pe, image })
    }
    /// Gets the PE instance.
    pub fn pe(&self) -> P {
        self.pe
    }
    /// Returns the underlying iat array.
    pub fn image(&self) -> &'a [Va] {
        self.image
    }
    /// Iterate over the IAT.
    ///
    /// When the imports aren't resolved yet the IAT is an alias for the import name table.
    pub fn iter(
        &self,
    ) -> iter::Map<slice::Iter<'a, Va>, impl Clone + FnMut(&'a Va) -> (&'a Va, Result<Import<'a>>)>
    {
        let pe = self.pe;
        self.image
            .iter()
            .map(move |va| (va, import_from_va(pe, va)))
    }
}
impl<'a, P: Pe<'a>> fmt::Debug for IAT<'a, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("IAT")
            .field("iat.len", &self.image.len())
            .finish()
    }
}

//----------------------------------------------------------------

#[derive(Clone)]
pub struct Iter<'a, P> {
    pe: P,
    iter: slice::Iter<'a, IMAGE_IMPORT_DESCRIPTOR>,
}
impl<'a, P: Pe<'a>> Iter<'a, P> {
    pub fn image(&self) -> &'a [IMAGE_IMPORT_DESCRIPTOR] {
        self.iter.as_slice()
    }
}
impl<'a, P: Pe<'a>> Iterator for Iter<'a, P> {
    type Item = Desc<'a, P>;
    fn next(&mut self) -> Option<Desc<'a, P>> {
        self.iter.next().map(|image| Desc { pe: self.pe, image })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
    fn count(self) -> usize {
        self.iter.count()
    }
    fn nth(&mut self, n: usize) -> Option<Desc<'a, P>> {
        self.iter.nth(n).map(|image| Desc { pe: self.pe, image })
    }
}
impl<'a, P: Pe<'a>> DoubleEndedIterator for Iter<'a, P> {
    fn next_back(&mut self) -> Option<Desc<'a, P>> {
        self.iter
            .next_back()
            .map(|image| Desc { pe: self.pe, image })
    }
}
impl<'a, P: Pe<'a>> ExactSizeIterator for Iter<'a, P> {}
impl<'a, P: Pe<'a>> iter::FusedIterator for Iter<'a, P> {}

//----------------------------------------------------------------

/// Import library descriptor.
#[derive(Copy, Clone)]
pub struct Desc<'a, P> {
    pe: P,
    image: &'a IMAGE_IMPORT_DESCRIPTOR,
}
impl<'a, P: Pe<'a>> Desc<'a, P> {
    /// Gets the PE instance.
    pub fn pe(&self) -> P {
        self.pe
    }
    /// Returns the underlying import descriptor image.
    pub fn image(&self) -> &'a IMAGE_IMPORT_DESCRIPTOR {
        self.image
    }
    /// Gets the name of the DLL imported from.
    pub fn dll_name(&self) -> Result<&'a CStr> {
        self.pe.derva_c_str(self.image.Name)
    }
    /// Gets the import address table.
    ///
    /// After being loaded as a library their values are resolved to the addresses of the imported functions.
    ///
    /// Otherwise these contain references to the imported functions.
    /// See [`import_from_va`](struct.Desc.html#import_from_va) to get their names.
    pub fn iat(&self) -> Result<slice::Iter<'a, Va>> {
        let slice = self.pe.derva_slice_s(self.image.FirstThunk, 0)?;
        Ok(slice.iter())
    }
    /// Gets the import name table.
    pub fn int(
        &self,
    ) -> Result<iter::Map<slice::Iter<'a, Va>, impl Clone + FnMut(&'a Va) -> Result<Import<'a>>>>
    {
        let slice = self.pe.derva_slice_s(self.image.OriginalFirstThunk, 0)?;
        let pe = self.pe;
        Ok(slice.iter().map(move |va| import_from_va(pe, va)))
    }
}
impl<'a, P: Pe<'a>> fmt::Debug for Desc<'a, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Imports")
            .field("dll_name", &format_args!("{:?}", self.dll_name()))
            .field(
                "iat.len",
                &format_args!("{:?}", &self.iat().map(|iter| iter.len())),
            )
            .field(
                "int.len",
                &format_args!("{:?}", &self.int().map(|iter| iter.len())),
            )
            .finish()
    }
}

//----------------------------------------------------------------

/*
    imports: [
        {
            "dll_name": "KERNEL32.dll",
            "int": [
                {
                    "ByName": { .. }
                },
                {
                    "ByOrdinal": { .. }
                }
            ]
        }
    ]
*/

#[cfg(feature = "serde")]
mod serde {
    use super::{Desc, Imports, Pe, IAT};
    use crate::util::serde_helper::*;

    impl<'a, P: Pe<'a>> Serialize for Imports<'a, P> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.collect_seq(self.into_iter())
        }
    }
    impl<'a, P: Pe<'a>> Serialize for IAT<'a, P> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let iat = self.iter().filter_map(|(_va, import)| import.ok());
            serializer.collect_seq(iat)
        }
    }
    impl<'a, P: Pe<'a>> Serialize for Desc<'a, P> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut state = serializer.serialize_struct("Desc", 2)?;
            state.serialize_field("dll_name", &self.dll_name().ok())?;
            let int = self
                .int()
                .map(|int| SerdeIter(int.filter_map(|import| import.ok())));
            state.serialize_field("int", &int.ok())?;
            state.end()
        }
    }
}

//----------------------------------------------------------------

#[cfg(test)]
pub(crate) fn test<'a, P: Pe<'a>>(pe: P) -> Result<()> {
    let imports = pe.imports()?;
    let _ = format!("{:?}", imports);

    for desc in imports {
        let _ = format!("{:?}", desc);
        let _dll_name = desc.dll_name();
        for _ in desc.iat() {}
        for _ in desc.int() {}
    }

    let iat = pe.iat()?;
    for (va, import) in iat.iter() {
        let _ = format!("{:?}", import);
        if import.is_ok() {
            assert_eq!(import_from_va(pe, va), import);
        }
    }

    Ok(())
}
