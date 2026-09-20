
#[allow(dead_code)]
#[link(wasm_import_module = "env")]
unsafe extern "C" {
	fn returnJSON(ptr: *const u8, len: usize);
	fn returnString(ptr: *const u8, len: usize);
	fn returnError(ptr: *const u8, len: usize);
	fn returnUint8Array(ptr: *const u8, len: usize);
	fn returnUint8Slice(ptr: *const u8, len: usize);
	fn returnNull();
	fn consoleLog(ptr: *const u8, len: usize);
}

pub fn return_json<T: serde::Serialize>(value: T) {
	let json = serde_json::to_string(&value).unwrap();
	unsafe {
		returnJSON(json.as_ptr(), json.len());
	}
}

pub fn return_pelite_result<T: serde::Serialize>(value: pelite::Result<T>) {
	match value {
		Ok(value) => return_json(value),
		Err(pelite::Error::Null) => return_null(),
		Err(err) => return_error(err),
	}
}

pub fn return_resource_find_result<T: serde::Serialize>(value: Result<T, pelite::resources::ResourceFindError>) {
	use pelite::resources::ResourceFindError::{NotFound, Pe};
	match value {
		Ok(value) => return_json(value),
		Err(NotFound | Pe(pelite::Error::Null)) => return_null(),
		Err(err) => return_error(err),
	}
}

pub fn return_str(string: &str) {
	unsafe {
		returnString(string.as_ptr(), string.len());
	}
}

pub fn return_bytes(bytes: &[u8]) {
	unsafe {
		returnUint8Array(bytes.as_ptr(), bytes.len());
	}
}

pub fn return_slice(bytes: &[u8]) {
	unsafe {
		returnUint8Slice(bytes.as_ptr(), bytes.len());
	}
}

pub fn return_error<E: ToString>(error: E) {
	let message = error.to_string();
	unsafe {
		returnError(message.as_ptr(), message.len());
	}
}

pub fn return_null() {
	unsafe {
		returnNull();
	}
}

#[allow(dead_code)]
pub fn console_log(s: &str) {
	unsafe {
		consoleLog(s.as_ptr(), s.len());
	}
}

#[unsafe(export_name = "bytesAllocate")]
pub unsafe fn allocate_bytes(len: usize) -> *mut u8 {
	unsafe {
		let boxed = vec![0u8; len].into_boxed_slice();
		let raw = Box::into_raw(boxed);
		(*raw).as_mut_ptr()
	}
}
#[unsafe(export_name = "bytesFree")]
pub unsafe fn free_bytes(data: *mut [u8]) {
	if data.is_null() {
		return;
	}
	unsafe {
		let boxed = Box::from_raw(data);
		drop(boxed);
	}
}

pub unsafe fn take_bytes(data: *mut [u8]) -> Option<Box<[u8]>> {
	if data.is_null() {
		return None;
	}
	unsafe {
		let boxed = Box::from_raw(data);
		Some(boxed)
	}
}
