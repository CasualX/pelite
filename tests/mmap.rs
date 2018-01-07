extern crate pelite;

use std::mem;

#[cfg(all(windows, target_pointer_width = "64"))]
#[test]
fn mmap_x64() {
	use pelite::pe;
	use pelite::ManualMap;

	let fmap = pelite::FileMap::open("demo/Demo64.dll").unwrap();
	let pf = pe::PeFile::from_bytes(&fmap).unwrap();
	unsafe {
		// ManualMap the module
		let module = pf.mmap().unwrap();
		println!("mapped module at {:?}", module);

		// Call an exported function as a very basic test
		let f: unsafe fn() = mem::transmute(module.offset(0x1230));
		f();
	};
}

#[cfg(all(windows, target_pointer_width = "32"))]
#[test]
fn mmap_x86() {
	use pelite::pe;
	use pelite::ManualMap;

	let fmap = pelite::FileMap::open("demo/Demo.dll").unwrap();
	let pf = pe::PeFile::from_bytes(&fmap).unwrap();
	unsafe {
		// ManualMap the module
		let module = pf.mmap().unwrap();
		println!("mapped module at {:?}", module);

		// Call an exported function as a very basic test
		let f: unsafe fn() = mem::transmute(module.offset(0x1220));
		f();
	}
}
