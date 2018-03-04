/*!
*/

use std::ptr;

use winapi::um::libloaderapi::GetModuleHandleA;

use pelite;
use pelite::pe::{Pe, PeView};
use pelite::pe::imports::Import;
use pelite::util::CStr;

unsafe fn get_module_handle(dll_name: &CStr) -> *mut u8 {
	GetModuleHandleA(dll_name.as_ptr() as *const i8) as *mut u8
}
unsafe fn get_proc_address(hmodule: *mut u8, imp: Import) -> *mut u8 {
	let view = PeView::module(hmodule);
	view.exports()
		.and_then(|exports| exports.by())
		.and_then(|by| by.import(imp))
		.and_then(|export| export.symbol().ok_or(pelite::Error::Null))
		.map(|rva| hmodule.offset(rva as isize))
		.unwrap_or(ptr::null_mut())
}

unsafe fn load_imports(view: PeView, dll_name: &str) -> pelite::Result<()> {
	let imports = view.imports()?;
	if let Some(desc) = imports.into_iter().find(|&desc| desc.dll_name().unwrap() == dll_name.as_bytes()) {
		let hmodule = get_module_handle(desc.dll_name().unwrap());
		for (int, iat) in desc.int().unwrap().into_iter().zip(desc.iat().unwrap()) {
		}
	}
	Ok(())
}
