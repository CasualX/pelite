
use pelite::pattern as pat;
use pelite::pe64::*;

fn main() {

	let filemap = pelite::FileMap::open(r"D:\SteamLibrary\steamapps\common\Mordhau\Mordhau\Binaries\Win64\Mordhau-Win64-Shipping.exe").unwrap();
	let bin = PeFile::from_bytes(&filemap).unwrap();

	let mut save = [0; 4];

	println!("TimeDateStamp: {:#x}", bin.file_header().TimeDateStamp);
	println!("CheckSum: {:#x}", bin.optional_header().CheckSum);

	assert!(bin.scanner().finds_code(pat!("48 8B 1D $ {'} 48 85 DB 74 3B 41 B0 01"), &mut save));
	println!("Mordhau-Win64-Shipping.exe!{:#x} GWorld", save[1]);

	assert!(bin.scanner().finds_code(pat!("48 8B 05 $ {'} 48 85 C0 75 5F"), &mut save));
	println!("Mordhau-Win64-Shipping.exe!{:#x} GNames", save[1]);

}
