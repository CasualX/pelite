extern crate pelite;

use pelite::FileMap;
use pelite::pe64::{Pe, PeFile};
use pelite::pe64::exports::Export;
use pelite::pe64::imports::Import;
use pelite::pe64::debug::Info;
use pelite::util::CStr;

const FILE_NAME: &str = "demo/Demo64.dll";

//----------------------------------------------------------------

#[test]
fn exports() {
	let file_map = FileMap::open(FILE_NAME).unwrap();
	let file = PeFile::from_bytes(&file_map).unwrap();
	let exports_by = file.exports().unwrap().by().unwrap();

	assert_eq!(exports_by.dll_name().unwrap(), "Demo.dll");
	assert_eq!(exports_by.ordinal_base(), 1);

	assert_eq!(exports_by.functions().len(), 20);
	assert_eq!(exports_by.names().len(), 20);
	assert_eq!(exports_by.name_indices().len(), 20);

	assert_eq!(exports_by.ordinal(9), Ok(Export::Symbol(&0x12D0)));

	let s_export_name = CStr::from_bytes(b"?fnPasswdsBypass@@YAHXZ\0").unwrap();

	// Double check it can look up broken hints
	let bad_hint = Import::ByName { hint: 5, name: s_export_name };
	let good_hint = Import::ByName { hint: 11, name: s_export_name };
	assert_eq!(exports_by.import(bad_hint), Ok(Export::Symbol(&0x1230)));
	assert_eq!(exports_by.import(good_hint), Ok(Export::Symbol(&0x1230)));

	assert_eq!(exports_by.hint(0), Ok(Export::Symbol(&0x1260)));
	assert_eq!(exports_by.hint_name(11), Ok(s_export_name));

	assert_eq!(exports_by.name_lookup(11), Ok(good_hint));
}

//----------------------------------------------------------------

#[test]
fn imports() {
	let file_map = FileMap::open(FILE_NAME).unwrap();
	let file = PeFile::from_bytes(&file_map).unwrap();
	let imports = file.imports().unwrap();

	assert_eq!(imports.image().len(), 2);

	let mut descriptors = imports.into_iter();
	let kernel32_dll = descriptors.next().unwrap();
	let msvcr120_dll = descriptors.next().unwrap();

	assert_eq!(kernel32_dll.dll_name().unwrap(), "KERNEL32.dll");
	assert_eq!(kernel32_dll.iat().unwrap().len(), 8);
	assert_eq!(kernel32_dll.int().unwrap().len(), 8);

	assert_eq!(msvcr120_dll.dll_name().unwrap(), "MSVCR120.dll");
	assert_eq!(msvcr120_dll.iat().unwrap().len(), 31);
	assert_eq!(msvcr120_dll.int().unwrap().len(), 31);
}

//----------------------------------------------------------------

#[test]
fn base_relocs() {
	let file_map = FileMap::open(FILE_NAME).unwrap();
	let file = PeFile::from_bytes(&file_map).unwrap();
	let base_relocs = file.base_relocs().unwrap();

	let mut blocks = base_relocs.into_iter();

	let block1 = blocks.next().unwrap();
	assert_eq!(block1.va(), 0x3000);
	assert_eq!(block1.words().len(), 28);

	let block2 = blocks.next().unwrap();
	assert_eq!(block2.va(), 0x5000);
	assert_eq!(block2.words().len(), 12);

	assert_eq!(blocks.count(), 0);
}

//----------------------------------------------------------------

#[test]
fn find_data() {
	let file_map = FileMap::open(FILE_NAME).unwrap();
	let file = PeFile::from_bytes(&file_map).unwrap();
	let resources = file.resources().unwrap();
	let entry = resources.find_data("/Manifest/2/1033").unwrap();
	let data = entry.data().unwrap();
	let manifest = ::std::str::from_utf8(data).unwrap();
	println!("\n{}", manifest);
}

//----------------------------------------------------------------

#[test]
fn tls() {
	let file_map = FileMap::open(FILE_NAME).unwrap();
	let file = PeFile::from_bytes(&file_map).unwrap();
	let tls = file.tls().unwrap();

	assert_eq!(tls.callbacks().unwrap(), &[0x1800014C0]);
	assert_eq!(tls.raw_data().unwrap(), &[0, 0]);
	assert_eq!(tls.slot().unwrap(), &0);
}

//----------------------------------------------------------------

#[test]
fn debug() {
	let file_map = FileMap::open(FILE_NAME).unwrap();
	let file = PeFile::from_bytes(&file_map).unwrap();
	let debug = file.debug().unwrap();

	for debug_dir in debug {
		match debug_dir.info().unwrap() {
			Info::CvRSDS { pdb_file_name, .. } => {
				assert_eq!(pdb_file_name, r"D:\Projects\pelite\proto\Demo\x64\Release\Demo.pdb");
			},
			_ => (),
		};
	}
}
