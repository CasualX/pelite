//! File fingerprints.
use pelite::PeFile;
use sha2::{Digest, Sha256};

#[derive(serde::Serialize)]
pub struct Hashes {
	pub md5: String,
	pub sha256: String,
	// A damaged import table has no reliable import hash.
	pub imphash: Option<String>,
}

pub fn hashes(bytes: &[u8], pe: PeFile<'_>) -> Hashes {
	let (libraries, complete) = read_imports(pe);
	hashes_from_imports(bytes, &libraries, complete)
}

fn hashes_from_imports(bytes: &[u8], libraries: &[ImportLibrary], complete: bool) -> Hashes {
	let mut sha256 = Sha256::new();
	sha256.update(bytes);
	let sha256 = sha256.finalize();
	let imphash = if complete {
		let normalized = libraries.iter().flat_map(|library| {
			let dll = library.name.to_ascii_lowercase();
			let dll = strip_library_extension(&dll).to_owned();
			library.symbols.iter().map(move |symbol| format!("{dll}.{}", symbol.to_ascii_lowercase()))
		}).collect::<Vec<_>>().join(",");
		Some(format!("{:x}", md5::compute(normalized.as_bytes())))
	}
	else { None };
	Hashes {
		md5: format!("{:x}", md5::compute(bytes)),
		sha256: basenc::LowerHex.encode(sha256.as_ref()),
		imphash,
	}
}

// Import parsing for fingerprints.
// Symbols remain in descriptor/INT order; ordinals use the conventional "ordNN" spelling.
struct ImportLibrary { pub name: String, pub symbols: Vec<String> }

fn read_imports(pe: PeFile<'_>) -> (Vec<ImportLibrary>, bool) {
	let mut libraries = Vec::new();
	let mut complete = true;
	match pe.imports() {
		Ok(directory) => for descriptor in directory {
			let name = match descriptor.dll_name().ok().and_then(|name| name.to_str().ok()) {
				Some(name) => name.to_owned(),
				None => { complete = false; continue; },
			};
			// TODO(imphash): Fall back to FirstThunk when OriginalFirstThunk is zero
			// and the IAT still contains unbound import names/ordinals.
			let imports = match descriptor.int() {
				Ok(imports) => imports,
				Err(_) => { complete = false; continue; },
			};
			let mut symbols = Vec::new();
			for import in imports {
				let symbol = match import {
					Ok(pelite::Import::ByName { name, .. }) => match name.to_str() {
						Ok(name) => name.to_owned(),
						Err(_) => { complete = false; continue; },
					},
					// TODO(imphash): Resolve known ordinals for oleaut32, ws2_32, and wsock32
					// using pefile's frozen imphash tables; retain ordNN for unknown ordinals.
					Ok(pelite::Import::ByOrdinal { ord }) => format!("ord{ord}"),
					Err(_) => { complete = false; continue; },
				};
				symbols.push(symbol);
			}
			libraries.push(ImportLibrary { name, symbols });
		},
		Err(error) if error.is_null() => (),
		Err(_) => complete = false,
	}
	(libraries, complete)
}

fn strip_library_extension(name: &str) -> &str {
	match name.rsplit_once('.') {
		Some((stem, "dll" | "sys" | "ocx")) => stem,
		_ => name,
	}
}

#[test]
fn conventional_import_normalization() {
	let libraries = vec![ImportLibrary { name: "KERNEL32.DLL".to_owned(), symbols: vec!["ExitProcess".to_owned(), "ord12".to_owned()] }];
	let hashes = hashes_from_imports(b"", &libraries, true);
	assert_eq!(hashes.md5, "d41d8cd98f00b204e9800998ecf8427e");
	assert_eq!(hashes.sha256, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
	assert_eq!(hashes.imphash, Some(format!("{:x}", md5::compute(b"kernel32.exitprocess,kernel32.ord12"))));
	assert!(hashes_from_imports(b"", &libraries, false).imphash.is_none());
	assert_eq!(strip_library_extension("driver.sys"), "driver");
	assert_eq!(strip_library_extension("library.other"), "library.other");
}
