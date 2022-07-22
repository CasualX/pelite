use crate::wasm::*;
use crate::PeFile;
use pelite::Error::Null;

#[no_mangle]
pub unsafe fn pefileDosHeader(pefile: *mut PeFile) {
    let dos_header = (*pefile).as_ref().dos_header();
    set_json(dos_header);
}
#[no_mangle]
pub unsafe fn pefileDosImage(pefile: *mut PeFile) {
    let dos_image = (*pefile).as_ref().dos_image();
    set_bytes(dos_image);
}
#[no_mangle]
pub unsafe fn pefileNtHeaders(pefile: *mut PeFile) {
    let nt_headers = (*pefile).as_ref().nt_headers();
    set_json(nt_headers);
}
#[no_mangle]
pub unsafe fn pefileFileHeader(pefile: *mut PeFile) {
    let file_header = (*pefile).as_ref().file_header();
    set_json(file_header);
}
#[no_mangle]
pub unsafe fn pefileOptionalHeader(pefile: *mut PeFile) {
    let optional_header = (*pefile).as_ref().optional_header();
    set_json(optional_header);
}
#[no_mangle]
pub unsafe fn pefileDataDirectory(pefile: *mut PeFile) {
    let data_directory = (*pefile).as_ref().data_directory();
    set_json(data_directory);
}
#[no_mangle]
pub unsafe fn pefileSectionHeaders(pefile: *mut PeFile) {
    let section_headers = (*pefile).as_ref().section_headers();
    set_json(section_headers);
}
#[no_mangle]
pub unsafe fn pefileHeaders(pefile: *mut PeFile) {
    let headers = (*pefile).as_ref().headers();
    set_json(headers);
}

#[no_mangle]
pub unsafe fn pefileRichStructure(pefile: *mut PeFile) {
    match (*pefile).as_ref().rich_structure() {
        Ok(rich_structure) => set_json(rich_structure),
        Err(Null) => set_null(),
        Err(err) => set_error(err),
    }
}
#[no_mangle]
pub unsafe fn pefileExports(pefile: *mut PeFile) {
    match (*pefile).as_ref().exports() {
        Ok(exports) => set_json(exports),
        Err(Null) => set_null(),
        Err(err) => set_error(err),
    }
}
#[no_mangle]
pub unsafe fn pefileImports(pefile: *mut PeFile) {
    match (*pefile).as_ref().imports() {
        Ok(imports) => set_json(imports),
        Err(Null) => set_null(),
        Err(err) => set_error(err),
    }
}
#[no_mangle]
pub unsafe fn pefileLoadConfig(pefile: *mut PeFile) {
    match (*pefile).as_ref().load_config() {
        Ok(load_config) => set_json(load_config),
        Err(Null) => set_null(),
        Err(err) => set_error(err),
    }
}
#[no_mangle]
pub unsafe fn pefileTls(pefile: *mut PeFile) {
    match (*pefile).as_ref().tls() {
        Ok(tls) => set_json(tls),
        Err(Null) => set_null(),
        Err(err) => set_error(err),
    }
}
#[no_mangle]
pub unsafe fn pefileDebug(pefile: *mut PeFile) {
    match (*pefile).as_ref().debug() {
        Ok(debug) => set_json(debug),
        Err(Null) => set_null(),
        Err(err) => set_error(err),
    }
}
