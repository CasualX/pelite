use crate::{wasm::*, PeFile};
use pelite::resources::FindError::NotFound;
use pelite::Error::Null;
use std::mem;

enum ResourceName {
    String(Box<str>),
    Id(u32),
}
impl ResourceName {
    fn from(ptr: *mut str) -> ResourceName {
        unsafe {
            if ptr.is_null() {
                let [_, len]: [usize; 2] = mem::transmute(ptr);
                return ResourceName::Id(len as u32);
            } else {
                ResourceName::String(Box::from_raw(ptr))
            }
        }
    }
    fn as_name(&self) -> pelite::resources::Name<'_> {
        match self {
            ResourceName::String(s) => pelite::resources::Name::Str(&s),
            ResourceName::Id(id) => pelite::resources::Name::Id(*id),
        }
    }
}

#[no_mangle]
pub unsafe fn pefileResourcesTree(pefile: *mut PeFile) {
    match (*pefile).as_ref().resources() {
        Ok(resources) => set_json(resources),
        Err(Null) => set_null(),
        Err(err) => set_error(err),
    }
}
#[no_mangle]
pub unsafe fn pefileResourcesFindData(pefile: *mut PeFile, path: *mut str) {
    let path = Box::from_raw(path);
    let path = &*path;
    match (*pefile).as_ref().resources() {
        Ok(resources) => match resources.find_data(path) {
            Ok(data_entry) => set_json(data_entry),
            Err(NotFound) => set_null(),
            Err(err) => set_error(err),
        },
        Err(Null) => set_null(),
        Err(err) => set_error(err),
    }
}
#[no_mangle]
pub unsafe fn pefileResourcesFindResource(pefile: *mut PeFile, ty: *mut str, name: *mut str) {
    let ty = ResourceName::from(ty);
    let name = ResourceName::from(name);
    let path = &[ty.as_name(), name.as_name()];
    match (*pefile).as_ref().resources() {
        Ok(resources) => match resources.find_resource(path) {
            Ok(bytes) => set_bytes(bytes),
            Err(NotFound) => set_null(),
            Err(err) => set_error(err),
        },
        Err(Null) => set_null(),
        Err(err) => set_error(err),
    }
}
#[no_mangle]
pub unsafe fn pefileResourcesFindResourceEx(
    pefile: *mut PeFile,
    ty: *mut str,
    name: *mut str,
    lang: *mut str,
) {
    let ty = ResourceName::from(ty);
    let name = ResourceName::from(name);
    let lang = ResourceName::from(lang);
    let path = &[ty.as_name(), name.as_name(), lang.as_name()];
    match (*pefile).as_ref().resources() {
        Ok(resources) => match resources.find_resource_ex(path) {
            Ok(bytes) => set_bytes(bytes),
            Err(NotFound) => set_null(),
            Err(err) => set_error(err),
        },
        Err(Null) => set_null(),
        Err(err) => set_error(err),
    }
}
#[no_mangle]
pub unsafe fn pefileResourcesVersionInfo(pefile: *mut PeFile) {
    match (*pefile).as_ref().resources() {
        Ok(resources) => match resources.version_info() {
            Ok(version_info) => set_json(version_info),
            Err(NotFound) => set_null(),
            Err(err) => set_error(err),
        },
        Err(Null) => set_null(),
        Err(err) => set_error(err),
    }
}
#[no_mangle]
pub unsafe fn pefileResourcesManifest(pefile: *mut PeFile) {
    match (*pefile).as_ref().resources() {
        Ok(resources) => match resources.manifest() {
            Ok(manifest) => set_string(manifest),
            Err(NotFound) => set_null(),
            Err(err) => set_error(err),
        },
        Err(Null) => set_null(),
        Err(err) => set_error(err),
    }
}
