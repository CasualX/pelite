/*!
*/

extern crate pelite;
extern crate winapi;

mod util;

use std::{env, io, mem};
use std::path::Path;

use pelite::pe::{Pe, PeView};
use pelite::ImageMap;
use pelite::util::CStr;

fn open(base_path: &Path, dll_file: &str) -> io::Result<ImageMap> {
	ImageMap::open(&base_path.join(dll_file))
}

fn main() {
	// Get the tf2 folder
	let tf2_path_buffer;
	let tf2_path = {
		let mut args = env::args_os();
		args.next();
		tf2_path_buffer = args.next();
		tf2_path_buffer.as_ref()
			.map(Path::new)
			.unwrap_or(Path::new(r"C:\Program Files (x86)\Steam\steamapps\common\Team Fortress 2"))
	};

	// Start by opening relevant tf2 binaries
	let engine_image = open(tf2_path, "bin/engine.dll").unwrap();
	let client_image = open(tf2_path, "tf/bin/client.dll").unwrap();

	// Make the fully accessible
	// engine_image.protect_rwx();
	// client_image.protect_rwx();

	// Interpret them as 32-bit PeFiles
	let engine_view = PeView::from_bytes(&engine_image).unwrap();
	let client_view = PeView::from_bytes(&client_image).unwrap();

	unsafe {
		// Print the load address of client to debug crashes
		let client_base = client_view.image().as_ptr();
		eprintln!("client_base: {:?}", client_base);

		// Run its DllMain to let it initialize static resources
		let entry_point = client_view.optional_header().ImageBase as usize + client_view.optional_header().AddressOfEntryPoint as usize;
		let dll_entry_point: unsafe extern "stdcall" fn(usize, u32, usize) = mem::transmute(entry_point);

		// Skip ___security_init_cookie
		//*((entry_point + 0x9) as *mut [u8; 5]) = [0xCC; 5];

		// Initialize static resources
		dll_entry_point(client_base as usize, 1, 0);
	}
}
