use super::*;

pub struct PeFile {
	image: Box<[u8]>,
}
impl AsRef<[u8]> for PeFile {
	fn as_ref(&self) -> &[u8] {
		self.image.as_ref()
	}
}

#[unsafe(export_name = "pefileNew")]
pub unsafe fn new(data: *mut [u8]) -> *mut PeFile {
	unsafe {
		let mut return_value = ptr::null_mut();
		match take_bytes(data) {
			Some(image) => match pelite::PeFile::from_bytes(&image) {
				Ok(_) => return_value = Box::into_raw(Box::new(PeFile { image })),
				Err(err) => return_error(err),
			},
			None => return_error(pelite::Error::Null),
		}
		return return_value;
	}
}

#[unsafe(export_name = "pefileDrop")]
pub unsafe fn drop(pefile: *mut PeFile) {
	unsafe {
		let _ = Box::from_raw(pefile);
	}
}

#[unsafe(export_name = "pefileDosHeader")]
pub unsafe fn dos_header(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_json(pefile.dos_header());
}

#[unsafe(export_name = "pefileNtHeaders")]
pub unsafe fn nt_headers(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_json(pefile.nt_headers());
}

#[unsafe(export_name = "pefileFileHeader")]
pub unsafe fn file_header(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_json(pefile.file_header());
}

#[unsafe(export_name = "pefileOptionalHeader")]
pub unsafe fn optional_header(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_json(pefile.optional_header());
}

#[unsafe(export_name = "pefileSectionHeaders")]
pub unsafe fn section_headers(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_json(pefile.section_headers());
}

#[unsafe(export_name = "pefileHeaders")]
pub unsafe fn headers(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_json(pefile.headers());
}

#[unsafe(export_name = "pefileRichStructure")]
pub unsafe fn rich_structure(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_pelite_result(pefile.rich_structure());
}

#[unsafe(export_name = "pefileImports")]
pub unsafe fn imports(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_pelite_result(pefile.imports());
}

#[unsafe(export_name = "pefileExports")]
pub unsafe fn exports(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_pelite_result(pefile.exports());
}

#[unsafe(export_name = "pefileBaseRelocations")]
pub unsafe fn base_relocations(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_pelite_result(pefile.base_relocs());
}

#[unsafe(export_name = "pefileLoadConfig")]
pub unsafe fn load_config(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_pelite_result(pefile.load_config());
}

#[unsafe(export_name = "pefileTls")]
pub unsafe fn tls(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_pelite_result(pefile.tls());
}

#[unsafe(export_name = "pefileExceptionsX64")]
pub unsafe fn exceptions_x64(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	match pefile {
		pelite::Wrap::T32(_) => {
			return_null();
		}
		pelite::Wrap::T64(pefile) => {
			return_pelite_result(pefile.exception_x64());
		}
	}
}

#[unsafe(export_name = "pefileDebug")]
pub unsafe fn debug(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_pelite_result(pefile.debug());
}

#[unsafe(export_name = "pefilePdbFileName")]
pub unsafe fn pdb_file_name(pefile: *mut PeFile) {
	let bytes = unsafe { &*pefile }.as_ref();
	let pe = match pelite::PeFile::from_bytes(bytes) {
		Ok(pe) => pe,
		Err(err) => return return_error(err),
	};
	let debug = match pe.debug() {
		Ok(debug) => debug,
		Err(pelite::Error::Null) => return return_null(),
		Err(err) => return return_error(err),
	};
	match debug.pdb_file_name() {
		Ok(Some(name)) => match name.to_str() {
			Ok(name) => return_str(name),
			Err(err) => return_error(err),
		},
		Ok(None) => return_null(),
		Err(err) => return_error(err),
	}
}

#[unsafe(export_name = "pefileSecurity")]
pub unsafe fn security(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_pelite_result(pefile.security());
}
