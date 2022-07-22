use super::image::*;
use pelite::util::AlignTo;
use pelite::{Error, Pod, Result};
use std::{fmt, mem, slice};

//----------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct Schema<'a> {
    image: &'a [u8], // Dword aligned
}
impl<'a> Schema<'a> {
    pub fn parse(image: &'a [u8]) -> Result<Schema<'a>> {
        if !image.as_ptr().aligned_to(4) {
            return Err(Error::Misaligned);
        }
        if image.len() < mem::size_of::<API_SET_NAMESPACE_V6>() {
            return Err(Error::Bounds);
        }
        let header = unsafe { &*(image.as_ptr() as *const API_SET_NAMESPACE_V6) };
        if header.Version != 6 {
            return Err(Error::BadMagic);
        }
        Ok(Schema { image })
    }

    fn slice_len<T: Pod>(&self, offset: u32, len: usize) -> Result<&'a [T]> {
        let slice = self
            .image
            .get(offset as usize..offset as usize + len as usize)
            .ok_or(Error::Bounds)?;
        if !slice.as_ptr().aligned_to(mem::align_of::<T>()) {
            return Err(Error::Misaligned);
        }
        Ok(unsafe { slice::from_raw_parts(slice.as_ptr() as *const T, len) })
    }

    pub fn header(self) -> &'a API_SET_NAMESPACE_V6 {
        unsafe { &*(self.image.as_ptr() as *const API_SET_NAMESPACE_V6) }
    }
    pub fn entries(&self) -> Result<Entries<'a>> {
        let header = self.header();
        let entries = self.slice_len(header.EntryOffset, header.Count as usize)?;
        let hashes = self.slice_len(header.HashOffset, header.Count as usize)?;
        Ok(Entries {
            schema: *self,
            entries,
            hashes,
        })
    }
}
impl<'a> fmt::Debug for Schema<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Schema")
            .field("version", &self.header().Version)
            .field("entries", &self.entries())
            .finish()
    }
}

//----------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct Entries<'a> {
    schema: Schema<'a>,
    entries: &'a [API_SET_NAMESPACE_ENTRY],
    hashes: &'a [API_SET_HASH_ENTRY],
}
impl<'a> Entries<'a> {
    pub fn entries(&self) -> &'a [API_SET_NAMESPACE_ENTRY] {
        self.entries
    }
    pub fn hashes(&self) -> &'a [API_SET_HASH_ENTRY] {
        self.hashes
    }
    pub fn iter(&self) -> impl Iterator<Item = Entry<'a>> + Clone + 'a {
        let schema = self.schema;
        self.entries
            .iter()
            .map(move |image| Entry { schema, image })
    }
    pub fn len(&self) -> usize {
        self.schema.header().Count as usize
    }
    pub fn at(&self, i: usize) -> Option<Entry<'a>> {
        self.entries.get(i).map(|image| Entry {
            schema: self.schema,
            image,
        })
    }
}
impl<'a> fmt::Debug for Entries<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

//----------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct Entry<'a> {
    schema: Schema<'a>,
    image: &'a API_SET_NAMESPACE_ENTRY,
}
impl<'a> Entry<'a> {
    pub fn is_sealed(&self) -> bool {
        self.image.Flags & 1 != 0
    }
    pub fn name(&self) -> Result<&'a [u16]> {
        self.schema
            .slice_len(self.image.NameOffset, (self.image.NameLength / 2) as usize)
    }
    pub fn values(&self) -> Result<Values<'a>> {
        let image = self
            .schema
            .slice_len(self.image.ValueOffset, self.image.ValueCount as usize)?;
        Ok(Values {
            schema: self.schema,
            image,
        })
    }
}
impl<'a> fmt::Debug for Entry<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Entry")
            .field("is_sealed", &self.is_sealed())
            .field(
                "name",
                &self
                    .name()
                    .map(String::from_utf16_lossy)
                    .unwrap_or_else(|err| err.to_string()),
            )
            .field("values", &self.values())
            .finish()
    }
}

//----------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct Values<'a> {
    schema: Schema<'a>,
    image: &'a [API_SET_VALUE_ENTRY],
}
impl<'a> Values<'a> {
    pub fn image(&self) -> &'a [API_SET_VALUE_ENTRY] {
        self.image
    }
    pub fn iter(&self) -> impl Iterator<Item = Value<'a>> + Clone + 'a {
        let schema = self.schema;
        self.image.iter().map(move |image| Value { schema, image })
    }
    pub fn len(&self) -> usize {
        self.image.len()
    }
    pub fn at(&self, i: usize) -> Option<Value<'a>> {
        self.image.get(i).map(|image| Value {
            schema: self.schema,
            image,
        })
    }
}
impl<'a> fmt::Debug for Values<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

//----------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct Value<'a> {
    schema: Schema<'a>,
    image: &'a API_SET_VALUE_ENTRY,
}
impl<'a> Value<'a> {
    pub fn is_sealed(&self) -> bool {
        self.image.Flags & 1 != 0
    }
    pub fn api_set_name(&self) -> Result<&'a [u16]> {
        self.schema
            .slice_len(self.image.NameOffset, (self.image.NameLength / 2) as usize)
    }
    pub fn host_name(&self) -> Result<&'a [u16]> {
        self.schema.slice_len(
            self.image.ValueOffset,
            (self.image.ValueLength / 2) as usize,
        )
    }
}
impl<'a> fmt::Debug for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Value")
            .field("is_sealed", &self.is_sealed())
            .field(
                "api_set_name",
                &self
                    .api_set_name()
                    .map(String::from_utf16_lossy)
                    .unwrap_or_else(|err| err.to_string()),
            )
            .field(
                "host_name",
                &self
                    .host_name()
                    .map(String::from_utf16_lossy)
                    .unwrap_or_else(|err| err.to_string()),
            )
            .finish()
    }
}
