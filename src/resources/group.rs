/*!
Group Icons and Cursors.

References:

* http://msdn.microsoft.com/en-us/library/ms997538.aspx
* https://devblogs.microsoft.com/oldnewthing/20120720-00/?p=7083
* https://github.com/MathewSachin/NIco/wiki/Ico,-Cur-and-PE-Formats

# Examples

The following example prints all group icon resource names which contain a PNG image.

```
// Aqcuire the resources of a Portable Executable file
let resources: pelite::resources::Resources;

# fn example(resources: pelite::resources::Resources<'_>) {
// Iterate over the group icons in the resources and throw away any invalid results
// If the resources contain no group icons the iterator is empty
for (name, group) in resources.icons().filter_map(Result::ok) {
    // Enumerate the entries in the group
    for entry in group.entries() {
        // Fetch the image data for this entry
        match group.image(entry.nId) {
            Ok(image) => {
                // Check if the image data starts with the PNG magic bytes
                if image.starts_with(b"\x89PNG") {
                    println!("{}: contains PNG", name);
                }
            },
            Err(err) => {
                println!("{}: Error {}!", name, err)
            },
        }
    }
}
# }
```

 */

use dataview::PodMethods;
use std::prelude::v1::*;

#[cfg(feature = "std")]
use std::io;

use crate::util::AlignTo;
use crate::Error;
use std::{fmt, mem, slice};

use super::{FindError, Resources};

use self::image::*;

//----------------------------------------------------------------

/// Icon or Cursor type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ResourceType {
    Icon,
    Cursor,
}
impl ResourceType {
    #[inline]
    pub fn id(self) -> u16 {
        match self {
            ResourceType::Icon => crate::image::RT_ICON,
            ResourceType::Cursor => crate::image::RT_CURSOR,
        }
    }
}
impl<'a> From<ResourceType> for super::Name<'a> {
    fn from(resource_type: ResourceType) -> super::Name<'a> {
        resource_type.id().into()
    }
}

/// Group resources, Icons and Cursors.
#[derive(Copy, Clone)]
pub struct GroupResource<'a> {
    resources: Resources<'a>,
    image: &'a GRPICONDIR,
}
impl<'a> GroupResource<'a> {
    /// Parses the GroupResource from the byte slice.
    ///
    /// The pixel data of the group resource is stored in separate data entries, requiring the resources to access.
    pub fn new(resources: Resources<'a>, bytes: &'a [u8]) -> Result<GroupResource<'a>, Error> {
        if !bytes.as_ptr().aligned_to(2) {
            return Err(Error::Misaligned);
        }
        if bytes.len() < mem::size_of::<GRPICONDIR>() {
            return Err(Error::Bounds);
        }
        let image: &'a GRPICONDIR = unsafe { &*(bytes.as_ptr() as *const GRPICONDIR) };
        if image.idReserved != 0 || !(image.idType == 1 || image.idType == 2) {
            return Err(Error::BadMagic);
        }
        let total_size = mem::size_of::<GRPICONDIR>()
            + image.idCount as usize * mem::size_of::<GRPICONDIRENTRY>();
        if bytes.len() != total_size {
            return Err(Error::Bounds);
        }
        Ok(GroupResource { resources, image })
    }
    /// Gets the Group header.
    pub fn header(&self) -> &'a GRPICONDIR {
        self.image
    }
    /// Gets the Group entries.
    pub fn entries(&self) -> &'a [GRPICONDIRENTRY] {
        let len = self.image.idCount as usize;
        // Checked by try_from constructor
        unsafe {
            let ptr = (self.image as *const GRPICONDIR).offset(1) as *const GRPICONDIRENTRY;
            slice::from_raw_parts(ptr, len)
        }
    }
    /// Gets the Group resource type.
    pub fn ty(&self) -> ResourceType {
        match self.image.idType {
            1 => ResourceType::Icon,
            2 => ResourceType::Cursor,
            _ => unreachable!(), // Checked by constructor
        }
    }
    /// Gets the image data for the given icon id.
    pub fn image(&self, id: u16) -> Result<&'a [u8], FindError> {
        self.resources
            .root()?
            .get_dir(self.ty().into())?
            .get_dir(id.into())?
            .first_data()?
            .bytes()
            .map_err(FindError::Pe)
    }
    /// Reassemble the file.
    #[cfg(feature = "std")]
    pub fn write(&self, dest: &mut dyn io::Write) -> io::Result<()> {
        // Start by appending the header
        dest.write(self.image.as_bytes())?;
        // Write all the icon entries
        let entries = self.entries();
        let mut image_offset = (6 + entries.len() * 16) as u32;
        for entry in entries {
            // Fixup the dwImageOffset field of the icon entry
            // NOTE! It is expected that the actual icon data size matches dwBytesInRes information!
            let mut icon_entry = [0u32; 4];
            icon_entry.as_bytes_mut()[..14].copy_from_slice(entry.as_bytes());
            icon_entry[3] = image_offset;
            image_offset += entry.bytes_in_resource();
            dest.write(icon_entry.as_bytes())?;
        }
        // Append the bytes for every entry
        for entry in entries {
            // Find the Icon data and append it
            // FIXME! What do if dwBytesInRes does not match the icon data size?
            // Ignoring this check may lead to corrupt icon files
            if let Ok(bytes) = self.image(entry.nId) {
                // assert_eq!(entry.bytes_in_resource() as usize, bytes.len());
                dest.write(bytes)?;
            }
        }
        Ok(())
    }
}

impl fmt::Debug for GroupResource<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("GroupResource")
            .field("type", &self.ty())
            .field("entries.len", &self.entries().len())
            .finish()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for GroupResource<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut bytes = Vec::new();
        mem::forget(self.write(&mut bytes));
        #[cfg(feature = "data-encoding")]
        {
            if serializer.is_human_readable() {
                return serializer.serialize_str(&data_encoding::BASE64.encode(&bytes));
            }
        }
        serializer.serialize_bytes(&bytes)
    }
}

/// Group Icon.
pub type GroupIcon<'a> = GroupResource<'a>;
/// Group Cursor.
pub type GroupCursor<'a> = GroupResource<'a>;

//----------------------------------------------------------------

#[allow(non_snake_case)]
pub mod image {
    use crate::Pod;
    #[derive(Copy, Clone, Debug)]
    #[repr(C)]
    pub struct GRPICONDIR {
        pub idReserved: u16,
        pub idType: u16,
        pub idCount: u16,
        pub idEntries: [GRPICONDIRENTRY; 0],
    }
    #[derive(Copy, Clone, Debug)]
    #[repr(C)]
    pub struct GRPICONDIRENTRY {
        pub bWidth: u8,
        pub bHeight: u8,
        pub bColorCount: u8,
        pub bReserved: u8,
        pub wPlanes: u16,
        pub wBitCount: u16,
        pub dwBytesInResLo: u16,
        pub dwBytesInResHi: u16,
        pub nId: u16,
    }
    impl GRPICONDIRENTRY {
        pub fn bytes_in_resource(&self) -> u32 {
            self.dwBytesInResHi as u32 * 0x10000 + self.dwBytesInResLo as u32
        }
    }
    unsafe impl Pod for GRPICONDIR {}
    unsafe impl Pod for GRPICONDIRENTRY {}
}
