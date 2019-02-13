
use std::env;
use std::path::PathBuf;

fn parse_arg() -> Option<PathBuf> {
	let mut args_os = env::args_os();
	args_os.next();
	args_os.next().map(|path| path.into())
}

fn main() {
	match parse_arg() {
		None => {
			eprintln!("Give the path to a dump apex binary.");
			return;
		},
		Some(path) => {
			let filemap = pelite::FileMap::open(&path).unwrap();
			parse(filemap.as_ref());
		},
	}
}

mod classes;
mod interfaces;
mod cvars;
mod globals;

fn parse(image: &[u8]) {
	use pelite::pe64::*;
	let bin = PeFile::from_bytes(image).unwrap();
	classes::print(bin);
	interfaces::print(bin);
	cvars::print(bin);
	globals::print(bin);
}
