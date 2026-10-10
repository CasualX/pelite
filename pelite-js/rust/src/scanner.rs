use super::*;

#[unsafe(export_name = "pefileScannerExec")]
pub unsafe fn scanner_exec(pefile: *mut PeFile, rva: u32, pattern: *const [u8]) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	let pattern = unsafe { &*pattern };
	let pattern = match str::from_utf8(pattern) {
		Ok(pattern) => pattern,
		Err(err) => return return_error(err),
	};

	let pattern = match pelite::pattern::parse(pattern, pelite::pattern::ParseOptions::DEFAULT) {
		Ok(pattern) => pattern,
		Err(err) => return return_error(err),
	};

	let mut save = vec![0u32; pelite::pattern::save_len(&pattern)];
	if !pefile.scanner().exec(rva, &pattern, &mut save) {
		return return_null();
	}

	let slots = pelite::pattern::captures_len(&pattern);
	return_json(&save[..slots]);
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum ScannerSection {
	ByName(String),
	ByIndex(usize),
}

#[derive(serde::Deserialize)]
struct ScannerArgs {
	pattern: String,
	#[serde(default)]
	section: Option<ScannerSection>,
	#[serde(default)]
	limit: usize,
}

#[derive(serde::Serialize)]
struct ScannerFindResult {
	rva: u32,
	save: Vec<u32>,
}

#[unsafe(export_name = "pefileScannerFind")]
pub unsafe fn scanner_find(pefile: *mut PeFile, args: *const [u8]) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	let args = unsafe { &*args };
	let args = match str::from_utf8(args) {
		Ok(args) => args,
		Err(err) => return return_error(err),
	};

	let args: ScannerArgs = match serde_json::from_str(args) {
		Ok(args) => args,
		Err(err) => return return_error(err),
	};

	let pattern = match pelite::pattern::parse(&args.pattern, pelite::pattern::ParseOptions::DEFAULT) {
		Ok(pattern) => pattern,
		Err(err) => return return_error(err),
	};

	let mut i = 0;
	let sections = pefile.scanner().sections(|section| {
		match &args.section {
			None => section.is_code(),
			Some(ScannerSection::ByIndex(index)) => {
				i += 1;
				index + 1 == i
			}
			Some(ScannerSection::ByName(name)) => section.name().map(|n| n == name).unwrap_or(false),
		}
	});

	let mut save = vec![0u32; pelite::pattern::save_len(&pattern)];
	let Some(rva) = sections.find(&pattern, &mut save) else {
		return return_null();
	};

	let save = save[..pelite::pattern::captures_len(&pattern)].to_vec();
	return_json(ScannerFindResult { rva, save });
}

#[unsafe(export_name = "pefileScannerMatches")]
pub unsafe fn scanner_matches(pefile: *mut PeFile, args: *const [u8]) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	let args = unsafe { &*args };
	let args = match str::from_utf8(args) {
		Ok(args) => args,
		Err(err) => return return_error(err),
	};

	let args: ScannerArgs = match serde_json::from_str(args) {
		Ok(args) => args,
		Err(err) => return return_error(err),
	};

	let pattern = match pelite::pattern::parse(&args.pattern, pelite::pattern::ParseOptions::DEFAULT) {
		Ok(pattern) => pattern,
		Err(err) => return return_error(err),
	};

	let mut i = 0;
	let sections = pefile.scanner().sections(|section| {
		match &args.section {
			None => section.is_code(),
			Some(ScannerSection::ByIndex(index)) => {
				i += 1;
				index + 1 == i
			}
			Some(ScannerSection::ByName(name)) => section.name().map(|n| n == name).unwrap_or(false),
		}
	});

	let captures_len = pelite::pattern::captures_len(&pattern);
	let mut captures = Vec::new();

	let mut count = 0;

	let mut save = vec![0u32; pelite::pattern::save_len(&pattern)];
	let mut matches = sections.matches(&pattern);
	while let Some(rva) = matches.next(&mut save) {
		let save = save[..captures_len].to_vec();
		captures.push(ScannerFindResult { rva, save });

		if args.limit > 0 {
			count += 1;
			if count >= args.limit {
				break;
			}
		}
	}

	return_json(captures);
}
