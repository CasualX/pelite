#![allow(non_snake_case)]

use core::ffi::*;
use core::{mem, slice, ptr};

#[repr(C)]
pub struct Pattern {
	ptr: *const u16,
	len: usize,
}
impl Pattern {
	fn rust(&self) -> &[pelite::pattern::Atom] {
		unsafe { slice::from_raw_parts(self.ptr as *const pelite::pattern::Atom, self.len) }
	}
}

#[no_mangle]
pub unsafe extern "C" fn PelitePatternParse(string: *const c_char) -> Pattern {
	let string = CStr::from_ptr(string);
	let string = match string.to_str() {
		Ok(string) => string,
		Err(err) => {
			eprintln!("PelitePatternParse: {}", err);
			return Pattern { ptr: ptr::null(), len: 0 };
		}
	};
	let (ptr, len) = match pelite::pattern::parse(string) {
		Ok(pattern) => {
			let slice = &*Box::into_raw(pattern.into_boxed_slice());
			(slice.as_ptr(), slice.len())
		}
		Err(err) => {
			eprintln!("PelitePatternParse: {}", err);
			return Pattern { ptr: ptr::null(), len: 0 };
		}
	};
	Pattern { ptr: ptr as *const u16, len }
}

#[no_mangle]
pub unsafe extern "C" fn PelitePatternFree(pat: Pattern) {
	if !pat.ptr.is_null() {
		let slice = slice::from_raw_parts_mut(pat.ptr as *mut u16, pat.len);
		let _ = Box::from_raw(slice);
	}
}

pub mod pe64;
pub mod pe32;
