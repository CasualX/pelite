/*!
Gets the hash of the import library.

References:

* https://github.com/erocarrera/pefile/blob/4c57c1443bb504281dcc00e1ddec9d62c5e39b35/pefile.py#L3824
* https://www.fireeye.com/blog/threat-research/2014/01/tracking-malware-import-hashing.html

 */

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::*;

pub fn main(pefile: PeFile<'_>) -> Result<u64> {
	match pefile {
		PeFile::Pe32(file) => imphash32(file),
		PeFile::Pe64(file) => imphash64(file),
	}
}

pub fn imphash32<'a, P: pe32::Pe<'a> + Copy>(pe: P) -> Result<u64> {
	use crate::pe32::imports::Import;
	let imports = match pe.imports() {
		Ok(imports) => imports,
		Err(Error::Null) => return Ok(0),
		Err(err) => return Err(err),
	};
	let mut h = DefaultHasher::new();
	for desc in imports {
		if let Ok(dll_name) = desc.dll_name() {
			dll_name.hash(&mut h);
		}
		if let Ok(int) = desc.int() {
			for imp in int {
				match imp {
					Ok(Import::ByName { name, .. }) => {
						name.hash(&mut h);
					},
					Ok(Import::ByOrdinal { ord }) => {
						ord.hash(&mut h);
					},
					Err(_) => (),
				}
			}
		}
	}
	Ok(h.finish())
}

pub fn imphash64<'a, P: pe64::Pe<'a> + Copy>(pe: P) -> Result<u64> {
	use crate::pe64::imports::Import;
	let imports = match pe.imports() {
		Ok(imports) => imports,
		Err(Error::Null) => return Ok(0),
		Err(err) => return Err(err),
	};
	let mut h = DefaultHasher::new();
	for desc in imports {
		if let Ok(dll_name) = desc.dll_name() {
			dll_name.hash(&mut h);
		}
		if let Ok(int) = desc.int() {
			for imp in int {
				match imp {
					Ok(Import::ByName { name, .. }) => {
						name.hash(&mut h);
					},
					Ok(Import::ByOrdinal { ord }) => {
						ord.hash(&mut h);
					},
					Err(_) => (),
				}
			}
		}
	}
	Ok(h.finish())
}
