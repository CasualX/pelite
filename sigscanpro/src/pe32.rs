use super::*;
use pelite::pe32 as pe;
use pelite::pe32::Pe;

#[repr(C)]
pub struct PeView32 {
	data: [usize; mem::size_of::<pe::PeView>() / mem::size_of::<usize>()],
}
impl PeView32 {
	fn rust(&self) -> pe::PeView {
		unsafe { *(self.data.as_ptr() as *const pe::PeView) }
	}
}

#[no_mangle]
pub unsafe extern "C" fn PeliteView32(image: *const u8, base_address: u32) -> PeView32 {
	let view = pe::PeView::module(image).set_base_address(base_address);
	PeView32 { data: mem::transmute(view) }
}

#[no_mangle]
pub unsafe extern "C" fn PeliteView32Finds(view: *const PeView32, pat: *const c_char, sect: *const c_char, save_ptr: *mut u32, save_len: usize) -> bool {
	// Parse the pattern
	let pat = CStr::from_ptr(pat);
	let pat = match pat.to_str() {
		Ok(pat) => pat,
		Err(err) => {
			#[cfg(debug_assertions)]
			eprintln!("PelitePatternParse: {}", err);
			return false;
		}
	};
	let pat = match pelite::pattern::parse(pat) {
		Ok(pat) => pat,
		Err(err) => {
			#[cfg(debug_assertions)]
			eprintln!("PelitePatternParse: {}", err);
			return false;
		}
	};

	let view = (*view).rust();

	// Find the section by name
	let mut range = view.headers().image_range();
	if !sect.is_null() {
		let sect = CStr::from_ptr(sect).to_bytes();
		if let Some(sect) = view.section_headers().by_name(sect) {
			range = sect.virtual_range();
		}
	}

	// Find the pattern
	let save = slice::from_raw_parts_mut(save_ptr, save_len);
	view.scanner().finds(&pat, range, save)
}

// type RustPeViewMatches<'a, 'pat> = pe::scanner::Matches<'pat, pe::PeView<'a>>;
// type RustPeFileMatches<'a, 'pat> = pe::scanner::Matches<'pat, pe::PeFile<'a>>;

#[repr(C)]
pub struct PeViewMatches32 {
	data: [usize; mem::size_of::<pe::scanner::Matches<pe::PeView>>() / mem::size_of::<usize>()],
}
impl PeViewMatches32 {
	fn rust<'a>(&mut self) -> &mut pe::scanner::Matches<'a, pe::PeView> {
		unsafe { &mut *(self.data.as_mut_ptr() as *mut pe::scanner::Matches<'a, pe::PeView>) }
	}
}

#[no_mangle]
pub unsafe extern "C" fn PeliteView32Matches(view: *const PeView32, pat: *const Pattern, sect: *const c_char) -> PeViewMatches32 {
	let view = (*view).rust();
	let pat = (*pat).rust();
	let mut range = view.headers().image_range();
	if !sect.is_null() {
		if let Ok(sect) = CStr::from_ptr(sect).to_str() {
			if let Some(sect) = view.section_headers().by_name(sect) {
				range = sect.virtual_range();
			}
		}
	}
	let matches = view.scanner().matches(pat, range);
	PeViewMatches32 { data: mem::transmute(matches) }
}

#[no_mangle]
pub unsafe extern "C" fn PeliteView32MatchesNext(matches: *mut PeViewMatches32, save_ptr: *mut u32, save_len: usize) -> bool {
	let matches = (*matches).rust();
	let save = slice::from_raw_parts_mut(save_ptr, save_len);
	matches.next(save)
}

#[repr(C)]
pub struct PeFile32 {
	data: [usize; mem::size_of::<pe::PeFile>() / mem::size_of::<usize>()],
}
impl PeFile32 {
	fn rust(&self) -> pe::PeFile {
		unsafe { *(self.data.as_ptr() as *const pe::PeFile) }
	}
}

#[no_mangle]
pub unsafe extern "C" fn PeliteFile32(ptr: *const u8, len: usize) -> PeFile32 {
	let bytes = slice::from_raw_parts(ptr, len);
	let file = pe::PeFile::from_bytes(bytes).unwrap();
	PeFile32 { data: mem::transmute(file) }
}

#[no_mangle]
pub unsafe extern "C" fn PeliteFile32Finds(file: *const PeFile32, pat: *const c_char, sect: *const c_char, save_ptr: *mut u32, save_len: usize) -> bool {
	// Parse the pattern
	let pat = CStr::from_ptr(pat);
	let pat = match pat.to_str() {
		Ok(pat) => pat,
		Err(err) => {
			#[cfg(debug_assertions)]
			eprintln!("PelitePatternParse: {}", err);
			return false;
		}
	};
	let pat = match pelite::pattern::parse(pat) {
		Ok(pat) => pat,
		Err(err) => {
			#[cfg(debug_assertions)]
			eprintln!("PelitePatternParse: {}", err);
			return false;
		}
	};

	let file = (*file).rust();

	// Find the section by name
	let mut range = file.headers().image_range();
	if !sect.is_null() {
		let sect = CStr::from_ptr(sect).to_bytes();
		if let Some(sect) = file.section_headers().by_name(sect) {
			range = sect.virtual_range();
		}
	}

	// Find the pattern
	let save = slice::from_raw_parts_mut(save_ptr, save_len);
	file.scanner().finds(&pat, range, save)
}

#[repr(C)]
pub struct PeFileMatches32 {
	data: [usize; mem::size_of::<pe::scanner::Matches<pe::PeFile>>() / mem::size_of::<usize>()],
}
impl PeFileMatches32 {
	fn rust(&mut self) -> &mut pe::scanner::Matches<pe::PeFile> {
		unsafe { &mut *(self.data.as_mut_ptr() as *mut pe::scanner::Matches<pe::PeFile>) }
	}
}

#[no_mangle]
pub unsafe extern "C" fn PeliteFile32Matches(file: *const PeFile32, pat: *const Pattern, sect: *const c_char) -> PeFileMatches32 {
	let file = (*file).rust();
	let pat = (*pat).rust();
	let mut range = file.headers().image_range();
	if !sect.is_null() {
		if let Ok(sect) = CStr::from_ptr(sect).to_str() {
			if let Some(sect) = file.section_headers().by_name(sect) {
				range = sect.virtual_range();
			}
		}
	}
	let matches = file.scanner().matches(pat, range);
	PeFileMatches32 { data: mem::transmute(matches) }
}

#[no_mangle]
pub unsafe extern "C" fn PeliteFile32MatchesNext(matches: *mut PeFileMatches32, save_ptr: *mut u32, save_len: usize) -> bool {
	let matches = (*matches).rust();
	let save = slice::from_raw_parts_mut(save_ptr, save_len);
	matches.next(save)
}
