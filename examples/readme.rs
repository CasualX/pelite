extern crate pelite;

use pelite::FileMap;
use pelite::pe64::{Pe, PeFile};

fn main() {
	// Load the desired file into memory
	let file_map = FileMap::open("demo/Demo64.dll").unwrap();
	// Process the image file
	example(file_map.as_ref()).expect("invalid pe file");
}

fn example(image: &[u8]) -> Result<(), pelite::Error> {
	// Interpret the bytes as a PE32+ executable
	let file = PeFile::from_bytes(&image)?;

	// Let's read the DLL dependencies
	let imports = file.imports()?;
	for desc in imports {
		// Get the DLL name being imported from
		let dll_name = desc.dll_name()?;
		// Get the number of imports for this dll
		let iat = desc.iat()?;
		println!("imported {} functions from {}", iat.len(), dll_name);
	}

	Ok(())
}
