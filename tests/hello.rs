extern crate pelite;

use std::mem;

use pelite::pe64::{Pe, PeFile};

#[cfg(all(windows, target_pointer_width = "64"))]
#[test]
fn main() {
	let fmap = pelite::FileMap::open("tests/bin/hello.dll").unwrap();
	let pf = PeFile::from_bytes(&fmap).unwrap();
	unsafe {
		// ManualMap the module
		let module = pf.mmap().load().unwrap();
		println!("mapped hello.dll at {:?}", module);

		// Avert thy eyes from the unwraps
		let rva = pf.exports().unwrap().by().unwrap().name("hello").unwrap().symbol().unwrap();

		// Call an exported function as a very basic test
		let f: unsafe fn() = mem::transmute(module.offset(rva as isize));
		f();
	};
}
