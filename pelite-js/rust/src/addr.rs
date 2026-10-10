use super::*;

fn pe_va_to_rva(pefile: pelite::PeFile<'_>, va: u64) -> pelite::Result<u32> {
	match pefile {
		pelite::Wrap::T32(pefile) => {
			match u32::try_from(va) {
				Ok(va) => pefile.va_to_rva(va),
				Err(_) => Err(pelite::Error::Overflow),
			}
		},
		pelite::Wrap::T64(pefile) => {
			pefile.va_to_rva(va)
		},
	}
}

fn pe_rva_to_va(pefile: pelite::PeFile<'_>, rva: u32) -> pelite::Result<u64> {
	match pefile {
		pelite::Wrap::T32(pefile) => {
			pefile.rva_to_va(rva).map(|va| va as u64)
		},
		pelite::Wrap::T64(pefile) => {
			pefile.rva_to_va(rva)
		},
	}
}

#[unsafe(export_name = "pefileVaToRva")]
pub unsafe fn va_to_rva(pefile: *mut PeFile, va: u64) -> u32 {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => { return_error(err); return 0; }
	};

	let result = pe_va_to_rva(pefile, va);

	match result {
		Ok(rva) => { return_null(); rva }
		Err(err) => { return_error(err); 0 }
	}
}

#[unsafe(export_name = "pefileRvaToVa")]
pub unsafe fn rva_to_va(pefile: *mut PeFile, rva: u32) -> u64 {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => { return_error(err); return 0; }
	};

	let result = pe_rva_to_va(pefile, rva);

	match result {
		Ok(va) => { return_null(); va }
		Err(err) => { return_error(err); 0 }
	}
}

#[unsafe(export_name = "pefileRvaToFileOffset")]
pub unsafe fn rva_to_file_offset(pefile: *mut PeFile, rva: u32) -> u32 {
	let pefile = unsafe { &*pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => { return_error(err); return 0; }
	};
	let result = pefile.headers().rva_to_file_offset(rva).map(|offset| offset as u32);
	match result {
		Ok(value) => { return_null(); value }
		Err(err) => { return_error(err); 0 }
	}
}

#[unsafe(export_name = "pefileVaToFileOffset")]
pub unsafe fn va_to_file_offset(pefile: *mut PeFile, va: u64) -> u32 {
	let pefile = unsafe { &*pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => { return_error(err); return 0; }
	};
	let result = pe_va_to_rva(pefile, va).and_then(|rva| pefile.headers().rva_to_file_offset(rva)).map(|offset| offset as u32);
	match result {
		Ok(value) => { return_null(); value }
		Err(err) => { return_error(err); 0 }
	}
}

#[unsafe(export_name = "pefileFileOffsetToRva")]
pub unsafe fn file_offset_to_rva(pefile: *mut PeFile, offset: u32) -> u32 {
	let pefile = unsafe { &*pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => { return_error(err); return 0; }
	};
	let result = pefile.headers().file_offset_to_rva(offset as usize);
	match result {
		Ok(value) => { return_null(); value }
		Err(err) => { return_error(err); 0 }
	}
}

#[unsafe(export_name = "pefileFileOffsetToVa")]
pub unsafe fn file_offset_to_va(pefile: *mut PeFile, offset: u32) -> u64 {
	let pefile = unsafe { &*pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => { return_error(err); return 0; }
	};
	let result = pefile.headers().file_offset_to_rva(offset as usize).and_then(|rva| pe_rva_to_va(pefile, rva));
	match result {
		Ok(value) => { return_null(); value }
		Err(err) => { return_error(err); 0 }
	}
}

#[unsafe(export_name = "pefileSliceBytes")]
pub unsafe fn slice_bytes(pefile: *mut PeFile, rva: u32, min_size: usize, align_of: usize) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	let bytes = match pefile.slice(rva, min_size, align_of) {
		Ok(bytes) => bytes,
		Err(err) => return return_error(err),
	};

	return_slice(bytes);
}

#[unsafe(export_name = "pefileSliceCString")]
pub unsafe fn slice_cstring(pefile: *mut PeFile, rva: u32, utf8: bool) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	let cstr = match pefile.derva_c_str(rva) {
		Ok(cstr) => cstr,
		Err(err) => return return_error(err),
	};

	if utf8 {
		let s = match cstr.to_str() {
			Ok(s) => s,
			Err(err) => return return_error(err),
		};
		return_str(s);
	}
	else {
		return_str(&cstr.to_string());
	}
}

#[unsafe(export_name = "pefileReadBytes")]
pub unsafe fn read_bytes(pefile: *mut PeFile, va: u64, min_size: usize, align_of: usize) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	let bytes = match pe_va_to_rva(pefile, va).and_then(|rva| pefile.slice(rva, min_size, align_of)) {
		Ok(bytes) => bytes,
		Err(err) => return return_error(err),
	};

	return_slice(bytes);
}

#[unsafe(export_name = "pefileReadCString")]
pub unsafe fn read_cstring(pefile: *mut PeFile, va: u64, utf8: bool) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	let cstr = match pe_va_to_rva(pefile, va).and_then(|rva| pefile.derva_c_str(rva)) {
		Ok(cstr) => cstr,
		Err(err) => return return_error(err),
	};

	if utf8 {
		let s = match cstr.to_str() {
			Ok(s) => s,
			Err(err) => return return_error(err),
		};
		return_str(s);
	}
	else {
		return_str(&cstr.to_string());
	}
}
