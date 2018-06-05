/*!
Gets the hash of the import library.

References:

* https://github.com/erocarrera/pefile/blob/4c57c1443bb504281dcc00e1ddec9d62c5e39b35/pefile.py#L3824
* https://www.fireeye.com/blog/threat-research/2014/01/tracking-malware-import-hashing.html
*/

extern crate pelite;

use std::env;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
	let mut args = env::args_os();
	if let (Some(_), Some(path), None) = (args.next(), args.next(), args.next()) {
		match pelite::FileMap::open(&path) {
			Ok(file_map) => {
				let result = imphash32(file_map.as_ref()).or_else(|_| imphash64(file_map.as_ref()));
				match result {
					Ok(hash) => {
						println!("Import hash is {:016X} for {:?}.", hash, path);
					},
					Err(err) => {
						eprintln!("Error reading {:?} with {}.", path, err);
					},
				}
			},
			Err(err) => eprintln!("Error reading {:?} with {}.", path, err),
		};
	}
	else {
		println!("imphash <path.dll>\nGiven a path to a dll calculates a hash of the import table.");
	}
}

fn imphash64(image: &[u8]) -> pelite::Result<u64> {
	use pelite::pe64::{Pe, PeFile};
	use pelite::pe64::imports::Import;

	let file = PeFile::from_bytes(image)?;
	let imports = match file.imports() {
		Ok(imports) => imports,
		Err(err) if err.is_null() => return Ok(0),
		Err(err) => return Err(err),
	};

	let mut h = DefaultHasher::new();

	for desc in imports {
		let dll_name = desc.dll_name()?;
		dll_name.hash(&mut h);
		for imp in desc.int()? {
			match imp? {
				Import::ByName { hint: _, name } => {
					name.hash(&mut h);
				},
				Import::ByOrdinal { ord } => {
					ord.hash(&mut h);
				},
			}
		}
	}

	Ok(h.finish())
}

fn imphash32(image: &[u8]) -> pelite::Result<u64> {
	use pelite::pe32::{Pe, PeFile};
	use pelite::pe32::imports::Import;

	let file = PeFile::from_bytes(image)?;
	let imports = match file.imports() {
		Ok(imports) => imports,
		Err(err) if err.is_null() => return Ok(0),
		Err(err) => return Err(err),
	};

	let mut h = DefaultHasher::new();

	for desc in imports {
		let dll_name = desc.dll_name()?;
		dll_name.hash(&mut h);
		for imp in desc.int()? {
			match imp? {
				Import::ByName { hint: _, name } => {
					name.hash(&mut h);
				},
				Import::ByOrdinal { ord } => {
					ord.hash(&mut h);
				},
			}
		}
	}

	Ok(h.finish())
}
