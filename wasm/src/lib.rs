#![allow(non_snake_case)]

use crate::wasm::*;
use pelite::Error::Null;
use std::ptr;

mod base_relocs;
mod headers;
mod resources;
mod scanner;
mod wasm;

pub struct PeFile {
    image: Box<[u8]>,
}
impl PeFile {
    pub fn as_ref(&'_ self) -> pelite::PeFile<'_> {
        // Checked by the constructor
        pelite::PeFile::from_bytes(&self.image).unwrap()
    }
}

#[no_mangle]
pub unsafe fn pefileNew(data: *mut [u8]) -> *mut PeFile {
    let mut return_value = ptr::null_mut();
    match take_bytes(data) {
        Some(image) => match pelite::PeFile::from_bytes(&image) {
            Ok(_) => return_value = Box::into_raw(Box::new(PeFile { image })),
            Err(err) => set_error(err),
        },
        None => set_null(),
    }
    return return_value;
}
#[no_mangle]
pub unsafe fn pefileDrop(pefile: *mut PeFile) {
    let _ = Box::from_raw(pefile);
}

#[no_mangle]
pub unsafe fn pefileSlice(pefile: *mut PeFile, rva: u32, min_size: usize, align: usize) {
    match (*pefile).as_ref().slice(rva, min_size, align) {
        Ok(data) => set_bytes(data),
        Err(Null) => set_null(),
        Err(err) => set_error(err),
    }
}
#[no_mangle]
pub unsafe fn pefileSliceBytes(pefile: *mut PeFile, rva: u32) {
    match (*pefile).as_ref().slice_bytes(rva) {
        Ok(data) => set_bytes(data),
        Err(Null) => set_null(),
        Err(err) => set_error(err),
    }
}
#[no_mangle]
pub unsafe fn pefileSliceArray(pefile: *mut PeFile, rva: u32, len: usize, align: usize) {
    let size = match len.checked_mul(align) {
        Some(size) => size,
        None => return set_error(pelite::Error::Overflow),
    };
    match (*pefile).as_ref().slice(rva, size, align) {
        Ok(data) => set_raw_data(data.as_ptr(), len),
        Err(Null) => set_null(),
        Err(err) => set_error(err),
    }
}
#[no_mangle]
pub unsafe fn pefileSliceCString(pefile: *mut PeFile, rva: u32) {
    match (*pefile).as_ref().derva_c_str(rva) {
        Ok(c_str) => set_bytes(c_str),
        Err(Null) => set_null(),
        Err(err) => set_error(err),
    }
}
