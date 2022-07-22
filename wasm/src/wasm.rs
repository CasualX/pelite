use std::error;

#[allow(dead_code)]
extern "C" {
    fn setJSON(ptr: *const u8, len: usize);
    fn setError(ptr: *const u8, len: usize);
    fn setData(ptr: *const u8, len: usize);
    fn setString(ptr: *const u8, len: usize);
}

pub fn set_json<T: serde::Serialize>(value: T) {
    let json = serde_json::to_string(&value).unwrap();
    unsafe {
        setJSON(json.as_ptr(), json.len());
    }
}
pub fn set_null() {
    // Null is the default return value
}
pub fn set_bytes(bytes: &[u8]) {
    unsafe {
        setData(bytes.as_ptr(), bytes.len());
    }
}
pub fn set_raw_data(ptr: *const u8, len: usize) {
    unsafe { setData(ptr, len) }
}
pub fn set_string(string: &str) {
    unsafe {
        setString(string.as_ptr(), string.len());
    }
}
pub fn set_error<E: error::Error>(error: E) {
    let msg = error.to_string();
    unsafe {
        setError(msg.as_ptr(), msg.len());
    }
}

#[no_mangle]
pub unsafe fn bytesAllocate(len: usize) -> *mut u8 {
    let boxed = vec![0u8; len].into_boxed_slice();
    let raw = Box::into_raw(boxed);
    (*raw).as_mut_ptr()
}
#[no_mangle]
pub unsafe fn bytesFree(data: *mut [u8]) {
    let boxed = Box::from_raw(data);
    drop(boxed);
}
pub unsafe fn take_bytes(data: *mut [u8]) -> Option<Box<[u8]>> {
    if data.is_null() {
        return None;
    }
    let boxed = Box::from_raw(data);
    Some(boxed)
}
