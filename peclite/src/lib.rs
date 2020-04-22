use std::{mem, slice, str};

#[no_mangle]
pub unsafe extern "C" fn pelite_scanner_exec(image_ptr: *const u8, image_len: usize, rva: u32, patstring_ptr: *const u8, patstring_len: usize, save_ptr: *mut u32, save_len: usize) -> u8 {
	let patstring = slice::from_raw_parts(patstring_ptr, patstring_len);
	let patstring = match str::from_utf8(patstring) {
		Ok(patstring) => patstring,
		Err(_) => return 3,
	};

	let image = slice::from_raw_parts(image_ptr, image_len);
	let file = match pelite::PeFile::from_bytes(image) {
		Ok(file) => file,
		Err(_) => return 2,
	};

	let pattern = match pelite::pattern::parse(patstring) {
		Ok(pattern) => pattern,
		Err(_) => return 3,
	};

	let save = slice::from_raw_parts_mut(save_ptr, save_len);
	if !file.scanner().exec(rva, &pattern, save) {
		return 1;
	}

	return 0;
}

#[no_mangle]
pub unsafe extern "C" fn pelite_scanner_find(image_ptr: *const u8, image_len: usize, patstring_ptr: *const u8, patstring_len: usize, section_index: usize, save_ptr: *mut u32, save_len: usize) -> u8 {
	let patstring = slice::from_raw_parts(patstring_ptr, patstring_len);
	let patstring = match str::from_utf8(patstring) {
		Ok(patstring) => patstring,
		Err(_) => return 3,
	};

	let image = slice::from_raw_parts(image_ptr, image_len);
	let file = match pelite::PeFile::from_bytes(image) {
		Ok(file) => file,
		Err(_) => return 2,
	};

	let pattern = match pelite::pattern::parse(patstring) {
		Ok(pattern) => pattern,
		Err(_) => return 3,
	};

	let range = match file.section_headers().get(section_index) {
		Some(section) => section.virtual_range(),
		None => file.headers().code_range(),
	};

	let save = slice::from_raw_parts_mut(save_ptr, save_len);
	if !file.scanner().finds(&pattern, range, save) {
		return 1;
	}

	return 0;
}

type Matches<'a> = pelite::Wrap<pelite::pe32::scanner::Matches<'a, pelite::pe32::PeFile<'a>>, pelite::pe64::scanner::Matches<'a, pelite::pe64::PeFile<'a>>>;

#[no_mangle]
pub unsafe extern "C" fn pelite_scanner_matches_new<'a>(image_ptr: *const u8, image_len: usize, patstring_ptr: *const u8, patstring_len: usize, section_index: usize, result: *mut *mut Matches<'a>) -> u8 {
	let patstring = slice::from_raw_parts(patstring_ptr, patstring_len);
	let patstring = match str::from_utf8(patstring) {
		Ok(patstring) => patstring,
		Err(_) => return 3,
	};

	let image = slice::from_raw_parts(image_ptr, image_len);
	let file = match pelite::PeFile::from_bytes(image) {
		Ok(file) => file,
		Err(_) => return 2,
	};

	let pattern = match pelite::pattern::parse(patstring) {
		Ok(pattern) => pattern,
		Err(_) => return 3,
	};

	let range = match file.section_headers().get(section_index) {
		Some(section) => section.virtual_range(),
		None => file.headers().code_range(),
	};

	let matches = Box::new(file.scanner().matches(&pattern, range));

	// Uh-oh...
	*result = mem::transmute(Box::into_raw(matches));
	mem::forget(pattern);

	return 0;
}

#[no_mangle]
pub unsafe extern "C" fn pelite_scanner_matches_delete<'a>(matches: *mut Matches<'a>) {
	let matches = Box::from_raw(matches);
	drop(Box::from_raw(matches.pattern() as *const [pelite::pattern::Atom] as *mut [pelite::pattern::Atom]));
	drop(matches);
}

#[no_mangle]
pub unsafe extern "C" fn pelite_scanner_matches_next<'a>(matches: *mut Matches<'a>, save_ptr: *mut u32, save_len: usize) -> bool {
	let save = slice::from_raw_parts_mut(save_ptr, save_len);
	(*matches).next(save)
}
