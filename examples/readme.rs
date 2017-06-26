extern crate pelite;

use std::io;

use pelite::FileMap;
use pelite::pe64::{Pe, PeFile};

//----------------------------------------------------------------

#[derive(Debug)]
enum Error {
	IO(io::Error),
	PE(pelite::Error),
}
impl From<io::Error> for Error {
	fn from(err: io::Error) -> Error { Error::IO(err) }
}
impl From<pelite::Error> for Error {
	fn from(err: pelite::Error) -> Error { Error::PE(err) }
}

//----------------------------------------------------------------

fn main() {
	example().expect("something went wrong!");
}

fn example() -> Result<(), Error> {
	// Load the desired file into memory
	let file_map = FileMap::open("demo/Demo64.dll")?;
	// Interpret the bytes as a PE32+ executable
	let file = PeFile::from_bytes(&file_map)?;

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