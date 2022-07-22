use crate::wasm::*;
use crate::PeFile;
use pelite::Error::Null;

#[no_mangle]
pub unsafe fn pefileBaseRelocs(pefile: *mut PeFile) {
    match (*pefile).as_ref().base_relocs() {
        Ok(relocs) => set_json(relocs),
        Err(Null) => set_null(),
        Err(err) => set_error(err),
    }
}
