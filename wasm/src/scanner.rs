use pelite::pattern as pat;

use crate::{wasm::*, PeFile};

#[unsafe(no_mangle)]
pub unsafe fn pefileScannerExec(pefile: *mut PeFile, rva: u32, pat: *mut str) { unsafe {
	let pattern = match pat::parse(&Box::from_raw(pat)) {
		Ok(pattern) => pattern,
		Err(err) => return set_error(err),
	};
	let save_len = pat::save_len(&pattern);
	let mut save = vec![0; save_len];
	if (*pefile).as_ref().scanner().exec(rva, &pattern, &mut save) {
		set_json(&save);
	}
	else {
		set_null();
	}
}}
#[unsafe(no_mangle)]
pub unsafe fn pefileScannerFinds(pefile: *mut PeFile, pat: *mut str, start: u32, end: u32) { unsafe {
	let pattern = match pat::parse(&Box::from_raw(pat)) {
		Ok(pattern) => pattern,
		Err(err) => return set_error(err),
	};
	let save_len = pat::save_len(&pattern);
	let mut save = vec![0; save_len];
	let pefile = (*pefile).as_ref();
	let range = if start > end { pefile.headers().code_range() } else { start..end };
	if pefile.scanner().finds(&pattern, range, &mut save) {
		set_json(&save);
	}
	else {
		set_null();
	}
}}
#[unsafe(no_mangle)]
pub unsafe fn pefileScannerFindsCode(pefile: *mut PeFile, pat: *mut str) { unsafe {
	pefileScannerFinds(pefile, pat, !0, 0)
}}
#[unsafe(no_mangle)]
pub unsafe fn pefileScannerMatches(pefile: *mut PeFile, pat: *mut str, start: u32, end: u32, mut offset: usize, limit: usize) { unsafe {
	let pattern = match pat::parse(&Box::from_raw(pat)) {
		Ok(pattern) => pattern,
		Err(err) => return set_error(err),
	};
	let save_len = pat::save_len(&pattern);
	let mut save = vec![0; save_len];
	let pefile = (*pefile).as_ref();
	let range = if start > end { pefile.headers().code_range() } else { start..end };
	let mut matches = pefile.scanner().matches(&pattern, range);
	let mut result = Vec::new();
	while matches.next(&mut save) {
		if offset > 0 {
			offset -= 1;
		}
		else {
			result.push(save);
			save = vec![0; save_len];
			if result.len() >= limit {
				break;
			}
		}
	}
	set_json(result);
}}
#[unsafe(no_mangle)]
pub unsafe fn pefileScannerMatchesCode(pefile: *mut PeFile, pat: *mut str, offset: usize, limit: usize) { unsafe {
	pefileScannerMatches(pefile, pat, !0, 0, offset, limit)
}}
