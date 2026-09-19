use pelite::pattern as pat;

use crate::PeFile;
use crate::wasm::*;

const OPTS: pelite::pattern::ParseOptions = pelite::pattern::ParseOptions::DEFAULT;

#[unsafe(no_mangle)]
pub unsafe fn pefileScannerExec(pefile: *mut PeFile, rva: u32, pat: *mut str) { unsafe {
	let pattern = match pat::parse(&Box::from_raw(pat), OPTS) {
		Ok(pattern) => pattern,
		Err(err) => return set_error(err),
	};
	let save_len = pat::save_len(&pattern);
	let captures_len = pat::captures_len(&pattern);
	let mut save = vec![0; save_len];
	if (*pefile).as_ref().scanner().exec(rva, &pattern, &mut save) {
		set_json(&save[..captures_len]);
	}
	else {
		set_null();
	}
}}
#[unsafe(no_mangle)]
pub unsafe fn pefileScannerFinds(pefile: *mut PeFile, pat: *mut str, start: u32, end: u32) { unsafe {
	let pattern = match pat::parse(&Box::from_raw(pat), OPTS) {
		Ok(pattern) => pattern,
		Err(err) => return set_error(err),
	};
	let save_len = pat::save_len(&pattern);
	let captures_len = pat::captures_len(&pattern);
	let mut save = vec![0; save_len];
	let pefile = (*pefile).as_ref();
	let selection = if start > end {
		pefile.scanner().code()
	}
	else {
		pefile.scanner().within(start..end)
	};
	match selection.find(&pattern, &mut save) {
		Some(_) => set_json(&save[..captures_len]),
		None => set_null(),
	}
}}
#[unsafe(no_mangle)]
pub unsafe fn pefileScannerFindsCode(pefile: *mut PeFile, pat: *mut str) {
	unsafe { pefileScannerFinds(pefile, pat, !0, 0) }
}
#[unsafe(no_mangle)]
pub unsafe fn pefileScannerMatches(pefile: *mut PeFile, pat: *mut str, start: u32, end: u32, mut offset: usize, limit: usize) { unsafe {
	let pattern = match pat::parse(&Box::from_raw(pat), OPTS) {
		Ok(pattern) => pattern,
		Err(err) => return set_error(err),
	};
	let save_len = pat::save_len(&pattern);
	let captures_len = pat::captures_len(&pattern);
	let mut save = vec![0; save_len];
	let pefile = (*pefile).as_ref();
	let selection = if start > end {
		pefile.scanner().code()
	}
	else {
		pefile.scanner().within(start..end)
	};
	let mut matches = selection.matches(&pattern);
	let mut result = Vec::new();
	loop {
		match matches.next(&mut save) {
			Some(_) => {},
			None => break,
		}
		if offset > 0 {
			offset -= 1;
		}
		else {
			save.truncate(captures_len);
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
pub unsafe fn pefileScannerMatchesCode(pefile: *mut PeFile, pat: *mut str, offset: usize, limit: usize) {
	unsafe { pefileScannerMatches(pefile, pat, !0, 0, offset, limit) }
}
