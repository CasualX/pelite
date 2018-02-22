/*!
Attempt at RE PUBG.
*/

extern crate pelite;

use std::{env, io, mem};
use std::path::Path;

use pelite::pe64::{Pe, PeFile, PeView};

fn open(base_path: &Path, dll_file: &str) -> io::Result<pelite::ImageMap> {
	pelite::ImageMap::open(&base_path.join(dll_file))
}

fn main() {
	// Get the pubg folder
	let pubg_path_buffer;
	let pubg_path = {
		let mut args = env::args_os();
		args.next();
		pubg_path_buffer = args.next();
		pubg_path_buffer.as_ref()
			.map(Path::new)
			.unwrap_or(Path::new(r"C:\Program Files (x86)\Steam\steamapps\common\PUBG"))
	};

	let tslgame_dll = open(pubg_path, r"TslGame\Binaries\Win64\TslGame.exe").unwrap();
	let tslgamebe_dll = open(pubg_path, r"TslGame\Binaries\Win64\TslGame_BE.exe").unwrap();
	let beservice_dll = open(pubg_path, r"TslGame\Binaries\Win64\BattlEye\BEService_x64.exe").unwrap();

	let tslgame_file = PeFile::from_bytes(&tslgame_dll).unwrap();
	let pat = pelite::pattern::parse("48 8B 41 10 48 85 C0 74 09 48 8B 40 20").unwrap();
	let mut save = [0; 4];
	let result = tslgame_file.scanner().finds(&pat, 0..tslgame_file.optional_header().SizeOfImage, &mut save);
	println!("result: {} addr: {}", result, save[0]);

	// let beservice_view = PeView::from_bytes(&beservice_dll).unwrap();

	// let ep = beservice_view.optional_header().AddressOfEntryPoint;
	// let ep_addr = beservice_view.slice_bytes(ep).unwrap().as_ptr();

	// unsafe {
	// 	let ep_fn: unsafe fn(usize, u32, usize) = mem::transmute(ep_addr);
	// 	ep_fn(0, 1, 0);
	// }
}
