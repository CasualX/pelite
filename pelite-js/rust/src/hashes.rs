use super::*;
use sha2::Digest;

#[derive(serde::Serialize)]
pub struct Hashes {
	pub md5: String,
	pub sha256: String,
	// A damaged import table has no reliable import hash.
	pub imphash: Option<String>,
}

#[unsafe(export_name = "pefileHashes")]
pub unsafe fn hashes(pefile: *mut PeFile) {
	let bytes = unsafe { &*pefile }.as_ref();
	let pe = match pelite::PeFile::from_bytes(bytes) {
		Ok(pe) => pe,
		Err(err) => return return_error(err),
	};
	return_json(hashes_impl(pe));
}

fn hash_sha256(bytes: &[u8]) -> String {
	let mut sha256 = sha2::Sha256::new();
	sha256.update(bytes);
	let sha256 = sha256.finalize();
	basenc::LowerHex.encode(sha256.as_ref())
}

fn hash_md5(bytes: &[u8]) -> String {
	format!("{:x}", md5::compute(bytes))
}

fn hashes_impl(pe: pelite::PeFile<'_>) -> Hashes {
	let bytes = pe.image();
	let sha256 = hash_sha256(bytes);
	let md5 = hash_md5(bytes);
	let imphash = hash_imports(pe);

	Hashes { md5, sha256, imphash }
}

fn hash_imports(pe: pelite::PeFile<'_>) -> Option<String> {
	let Some(libs) = read_imports(pe) else {
		return None;
	};
	let normalized = libs.iter().flat_map(|library| {
		let dll = library.name.to_ascii_lowercase();
		let dll = strip_library_extension(&dll).to_owned();
		library.symbols.iter().map(move |symbol| format!("{dll}.{}", symbol.to_ascii_lowercase()))
	}).collect::<Vec<_>>().join(",");
	Some(format!("{:x}", md5::compute(normalized.as_bytes())))
}

// Import parsing for fingerprints.
// Symbols remain in descriptor/INT order; ordinals use the conventional "ordNN" spelling.
struct ImportLibrary { pub name: String, pub symbols: Vec<String> }

fn read_imports(pe: pelite::PeFile<'_>) -> Option<Vec<ImportLibrary>> {
	let mut libs = Vec::new();
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
			libs.push(ImportLibrary { name, symbols });
		},
		Err(error) if error.is_null() => (),
		Err(_) => complete = false,
	}
	if complete { Some(libs) } else { None }
}

fn strip_library_extension(name: &str) -> &str {
	match name.rsplit_once('.') {
		Some((stem, "dll" | "sys" | "ocx")) => stem,
		_ => name,
	}
}

#[test]
fn conventional_import_normalization() {
	fn write_u32(bytes: &mut [u8], offset: usize, value: u32) {
		bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
	}

	// Minimal PE32 with its import table stored in the headers.
	let mut bytes = vec![0; 0x400];
	bytes[..2].copy_from_slice(b"MZ");
	write_u32(&mut bytes, 0x3c, 0x80); // e_lfanew
	bytes[0x80..0x84].copy_from_slice(b"PE\0\0");
	bytes[0x84..0x86].copy_from_slice(&0x14cu16.to_le_bytes()); // Machine
	bytes[0x94..0x96].copy_from_slice(&0xe0u16.to_le_bytes()); // SizeOfOptionalHeader
	bytes[0x98..0x9a].copy_from_slice(&0x10bu16.to_le_bytes()); // PE32 magic
	write_u32(&mut bytes, 0xd0, 0x400); // SizeOfImage
	write_u32(&mut bytes, 0xd4, 0x400); // SizeOfHeaders
	write_u32(&mut bytes, 0xf4, 16); // NumberOfRvaAndSizes
	write_u32(&mut bytes, 0x100, 0x200); // Import directory RVA
	write_u32(&mut bytes, 0x104, 40); // Import directory size
	write_u32(&mut bytes, 0x200, 0x260); // OriginalFirstThunk
	write_u32(&mut bytes, 0x20c, 0x240); // DLL name RVA
	write_u32(&mut bytes, 0x210, 0x260); // FirstThunk
	bytes[0x240..0x24d].copy_from_slice(b"KERNEL32.DLL\0");
	write_u32(&mut bytes, 0x260, 0x280); // Import by name
	write_u32(&mut bytes, 0x264, 0x8000_000c); // Import by ordinal 12
	bytes[0x282..0x28e].copy_from_slice(b"ExitProcess\0");
	let pe = pelite::PeFile::from_bytes(&bytes).unwrap();
	assert_eq!(hash_imports(pe), Some(format!("{:x}", md5::compute(b"kernel32.exitprocess,kernel32.ord12"))));
	write_u32(&mut bytes, 0x260, 0x1000); // Import name outside the image
	let pe = pelite::PeFile::from_bytes(&bytes).unwrap();
	assert!(hash_imports(pe).is_none());
	assert_eq!(strip_library_extension("driver.sys"), "driver");
	assert_eq!(strip_library_extension("library.other"), "library.other");
}
