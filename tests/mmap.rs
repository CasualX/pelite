extern crate pelite;

use std::mem;

#[cfg(all(windows, target_pointer_width = "64"))]
#[test]
fn mmap_x64() {
	use pelite::pe64::{Pe, PeFile};

	let fmap = pelite::FileMap::open("demo/Demo64.dll").unwrap();
	let pf = PeFile::from_bytes(&fmap).unwrap();
	unsafe {
		// ManualMap the module
		let module = pf.mmap().load().unwrap();
		println!("mapped module at {:?}", module);

		// Call an exported function as a very basic test
		let f: unsafe fn() = mem::transmute(module.offset(0x1230));
		f();
	};
}

#[cfg(all(windows, target_pointer_width = "32"))]
#[test]
fn mmap_x86() {
	use pelite::pe32::{Pe, PeFile};

	let fmap = pelite::FileMap::open("demo/Demo.dll").unwrap();
	let pf = PeFile::from_bytes(&fmap).unwrap();
	unsafe {
		// ManualMap the module
		let module = pf.mmap().load().unwrap();
		println!("mapped module at {:?}", module);

		// Call an exported function as a very basic test
		let f: unsafe fn() = mem::transmute(module.offset(0x1220));
		f();
	}
}
