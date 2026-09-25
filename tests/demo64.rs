use pelite::pe64::*;
use pelite::util::CStr;
use pelite::{Error, FileMap};

const FILE_NAME: &str = "demo/Demo64.dll";

//----------------------------------------------------------------

#[test]
fn conversions_use_aligned_buffers() {
	let file_map = FileMap::open(FILE_NAME).unwrap();
	let file = PeFile::from_bytes(&file_map).unwrap();
	let view = file.to_view();
	assert_eq!((view.as_ref() as &[u8]).as_ptr() as usize % 16, 0);
	let mapped = PeView::from_bytes(&view).unwrap();
	let rebuilt = mapped.to_file();
	assert_eq!((rebuilt.as_ref() as &[u8]).as_ptr() as usize % 16, 0);
	PeFile::from_bytes(&rebuilt).unwrap();
}

#[test]
fn slice_edges() {
	let file_map = FileMap::open(FILE_NAME).unwrap();
	let file = PeFile::from_bytes(&file_map).unwrap();

	let assert_edges = |rva: Rva, len: usize| {
		assert_eq!(file.slice_bytes(rva).unwrap().len(), len);
		// not guaranteed as it may spill over in the next section
		// assert_eq!(file.slice_bytes(rva + len as Rva).unwrap().len(), 0);
		assert_eq!(file.slice(rva, len, 1).unwrap().len(), len);
	};

	assert_edges(0x1000, 0x1200);
	assert_edges(0x3000, 0x1200);
	assert_edges(0x5000, 0x0200);
	assert_edges(0x6000, 0x0200);
	assert_edges(0x7000, 0x0200);
	assert_edges(0x8000, 0x4200);
	assert_edges(0xD000, 0x0200);

	assert_eq!(file.slice(0x5000, 0x710, 1), Err(pelite::Error::ZeroFill));
	assert_eq!(file.slice(0x5000, 0x711, 1), Err(pelite::Error::Bounds));
}

//----------------------------------------------------------------

#[test]
fn rich_structure() {
	let file_map = FileMap::open(FILE_NAME).unwrap();
	let file = PeFile::from_bytes(&file_map).unwrap();
	let rich_structure = file.rich_structure().unwrap();

	assert_eq!(rich_structure.xor_key(), rich_structure.checksum());

	// Re-encode the records
	let records: Vec<_> = rich_structure.records().collect();
	let required_len = rich_structure.encode(&records, &mut []).unwrap_err();
	let mut encoded = vec![0; required_len];
	assert_eq!(rich_structure.encode(&records, &mut encoded), Ok(required_len));
	assert_eq!(rich_structure.image(), &encoded[..rich_structure.image().len()]);
	assert!(encoded[rich_structure.image().len()..].iter().all(|&dword| dword == 0));

	let mut short = vec![u32::MAX; required_len - 1];
	assert_eq!(rich_structure.encode(&records, &mut short), Err(required_len));
	assert!(short.iter().all(|&dword| dword == u32::MAX));
}

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

	assert_eq!(exports_by.hint_name(5, s_export_name), Ok(Export::Symbol(&0x1230)));
	assert_eq!(exports_by.hint_name(11, s_export_name), Ok(Export::Symbol(&0x1230)));

	assert_eq!(exports_by.hint(0), Ok(Export::Symbol(&0x1260)));
	assert_eq!(exports_by.name_of_hint(11), Ok(s_export_name));

	assert_eq!(exports_by.name_lookup(11), Ok(good_hint));
	assert_eq!(exports_by.name_lookup(exports_by.functions().len()), Err(Error::Bounds));

	assert_eq!(exports_by.iter().count(), 20);
	assert_eq!(exports_by.iter_names().count(), 20);

	assert_eq!(file.get_proc_address("ThrowException"), file.rva_to_va(0x10C0));
	assert_eq!(file.get_proc_address(b"ThrowException".as_ref()), file.rva_to_va(0x10C0));
	assert_eq!(file.get_proc_address(0x14), file.rva_to_va(0x10C0));

	assert_eq!(file.get_proc_address(bad_hint), file.rva_to_va(0x1230));
	assert_eq!(file.get_proc_address(good_hint), file.rva_to_va(0x1230));
}

#[test]
fn wrap_get_export() {
	let file_map = FileMap::open(FILE_NAME).unwrap();
	let file = pelite::PeFile::from_bytes(&file_map).unwrap();

	assert_eq!(file.get_export("ThrowException"), Ok(pelite::Export::Symbol(&0x10C0)));
	assert_eq!(file.get_export(0x14), Ok(pelite::Export::Symbol(&0x10C0)));

	let name = CStr::from_bytes(b"?fnPasswdsBypass@@YAHXZ\0").unwrap();
	let import = pelite::Import::ByName { hint: 11, name };
	assert_eq!(file.get_export(import), Ok(pelite::Export::Symbol(&0x1230)));
}

#[test]
fn wrapped_headers_convert_file_offsets_for_both_formats() {
	for path in ["demo/Demo.dll", "demo/Demo64.dll"] {
		let file_map = FileMap::open(path).unwrap();
		let file = pelite::PeFile::from_bytes(&file_map).unwrap();
		let headers = file.headers();
		assert_eq!(headers.rva_to_file_offset(0x1000), Ok(0x400), "{path}");
		assert_eq!(headers.file_offset_to_rva(0x400), Ok(0x1000), "{path}");
	}
}

#[test]
fn wrapped_images_convert_u64_virtual_addresses_for_both_formats() {
	for path in ["demo/Demo.dll", "demo/Demo64.dll"] {
		let file_map = FileMap::open(path).unwrap();
		let file = pelite::PeFile::from_bytes(&file_map).unwrap();
		let va = file.image_base() + 0x1000;
		assert_eq!(file.rva_to_va(0x1000), Ok(va), "{path}");
		assert_eq!(file.va_to_rva(va), Ok(0x1000), "{path}");
		assert_eq!(file.va_to_rva(0), Err(Error::Null), "{path}");
		assert_eq!(file.rva_to_va(0), Err(Error::Null), "{path}");
	}
	let file_map = FileMap::open("demo/Demo.dll").unwrap();
	let file = pelite::PeFile::from_bytes(&file_map).unwrap();
	assert_eq!(file.va_to_rva(u32::MAX as u64 + 1), Err(Error::Bounds));
}

#[test]
fn exports_reject_null_nonempty_name_index_table() {
	let mut image = std::fs::read(FILE_NAME).unwrap();
	let export_offset = {
		let file = PeFile::from_bytes(&image).unwrap();
		let export_rva = file.data_directory()[pelite::image::IMAGE_DIRECTORY_ENTRY_EXPORT].VirtualAddress;
		file.headers().rva_to_file_offset(export_rva).unwrap()
	};

	// AddressOfNameOrdinals is the final field of IMAGE_EXPORT_DIRECTORY.
	image[export_offset + 36..export_offset + 40].copy_from_slice(&0u32.to_le_bytes());
	let file = PeFile::from_bytes(&image).unwrap();
	assert!(matches!(file.exports().unwrap().by(), Err(Error::Null)));
}

#[test]
fn rva_to_va_reports_address_overflow() {
	let image = std::fs::read(FILE_NAME).unwrap();
	let file = PeFile::from_bytes(&image).unwrap();
	let view_image = file.to_view();
	let view = PeView::from_bytes(&view_image).unwrap();
	let end = view.image_base() + view.optional_header().SizeOfImage as Va;
	assert_eq!(view.va_to_rva(end), Ok(view.optional_header().SizeOfImage));
	assert_eq!(view.rva_to_va(view.optional_header().SizeOfImage), Ok(end));
	assert_eq!(view.read_bytes(end), Ok(&[][..]));
	assert_eq!(view.read(end, 1, 1), Err(Error::Bounds));
	let overflow_view = view.set_base_address(u64::MAX);
	assert_eq!(overflow_view.rva_to_va(1), Err(Error::Overflow));
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

	let iat = file.iat().unwrap();
	assert_eq!(iat.image().len(), 8 + 31 + 2);

	let lhs = iat.iter().filter_map(|(_, import)| import.ok()).map(Ok);
	let rhs = kernel32_dll.int().unwrap().chain(msvcr120_dll.int().unwrap());
	assert!(Iterator::eq(lhs, rhs));
}

//----------------------------------------------------------------

#[test]
fn base_relocs() {
	let file_map = FileMap::open(FILE_NAME).unwrap();
	let file = PeFile::from_bytes(&file_map).unwrap();
	let base_relocs = file.base_relocs().unwrap();

	let mut blocks = base_relocs.iter_blocks();

	let block1 = blocks.next().unwrap();
	assert_eq!(block1.image().VirtualAddress, 0x3000);
	assert_eq!(block1.words().len(), 28);

	let block2 = blocks.next().unwrap();
	assert_eq!(block2.image().VirtualAddress, 0x5000);
	assert_eq!(block2.words().len(), 12);

	assert_eq!(blocks.count(), 0);

	// Test all the iterator impls against this baseline
	let mut baseline = base_relocs
		.iter_blocks()
		.flat_map(move |block| block.words().iter().filter(move |&word| block.type_of(word) != 0).map(move |word| block.rva_of(word)));
	base_relocs.for_each(|rva, _| {
		assert_eq!(baseline.next(), Some(rva));
	});
	assert_eq!(baseline.next(), None);

	// Ensure that rebuilding the base relocation round trips
	let mut rvas = Vec::new();
	let mut types = Vec::new();
	base_relocs.for_each(|rva, ty| {
		rvas.push(rva);
		types.push(ty);
	});
	let result = pelite::base_relocs::BaseRelocationDirectory::build(&rvas, &types);
	assert_eq!(result, base_relocs.image());
}

//----------------------------------------------------------------

#[test]
fn find_data() {
	let file_map = FileMap::open(FILE_NAME).unwrap();
	let file = PeFile::from_bytes(&file_map).unwrap();
	let resources = file.resources().unwrap();
	let data = resources.find_data("/#MANIFEST/#2/#1033").unwrap();
	let bytes = data.bytes().unwrap();
	let manifest = std::str::from_utf8(bytes).unwrap();
	println!("\n{}", manifest);

	let data2 = resources.find_dir("/#MANIFEST").unwrap().get_dir("#2".into()).unwrap().get_data("#1033".into()).unwrap();
	assert!(std::ptr::eq(data.image(), data2.image()));
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
fn load_config() {
	let file_map = FileMap::open(FILE_NAME).unwrap();
	let file = PeFile::from_bytes(&file_map).unwrap();
	let load_config = file.load_config().unwrap();
	let image = load_config.image_copy();

	// This VS2013 image has a 112-byte load config. Fields appended in newer
	// revisions must be zero rather than bytes copied from the rest of .rdata.
	assert_eq!(image.Size, 0x70);
	assert_eq!(image.SecurityCookie.get(), 0x180005000);
	assert_eq!(image.GuardCFCheckFunctionPointer.get(), 0);
	assert_eq!(image.GuardFlags, 0);
	assert_eq!(image.UmaFunctionPointers.get(), 0);

	use pelite::pe64::image::IMAGE_LOAD_CONFIG_DIRECTORY;
	assert_eq!(load_config.get(dataview::Field!(IMAGE_LOAD_CONFIG_DIRECTORY.Size)), Some(0x70));
	assert_eq!(load_config.get(dataview::Field!(IMAGE_LOAD_CONFIG_DIRECTORY.SecurityCookie)).map(|value| value.get()), Some(0x180005000));
	assert_eq!(load_config.get(dataview::Field!(IMAGE_LOAD_CONFIG_DIRECTORY.GuardFlags)), None);
	assert_eq!(load_config.get(dataview::Field!(IMAGE_LOAD_CONFIG_DIRECTORY.HotPatchTableOffset)), None);
}

//----------------------------------------------------------------

#[test]
fn debug() {
	let file_map = FileMap::open(FILE_NAME).unwrap();
	let file = PeFile::from_bytes(&file_map).unwrap();
	let debug = file.debug().unwrap();

	assert_eq!(debug.pdb_file_name().unwrap().unwrap(), r"D:\Projects\pelite\proto\Demo\x64\Release\Demo.pdb");
}

//----------------------------------------------------------------

#[test]
fn security() {
	let file_map = FileMap::open(FILE_NAME).unwrap();
	let file = PeFile::from_bytes(&file_map).unwrap();
	let security = file.security();

	assert!(matches!(security, Err(Error::Null)));
}

//----------------------------------------------------------------

#[test]
fn exception_x64() {
	let file_map = FileMap::open(FILE_NAME).unwrap();
	let file = PeFile::from_bytes(&file_map).unwrap();
	let exception = file.exception_x64().unwrap();

	assert_eq!(exception.functions().len(), 38);
	for (index, function) in exception.functions().enumerate() {
		let image = function.image();
		assert_eq!(exception.index_of(image.BeginAddress), Ok(index));
		assert_ne!(exception.index_of(image.EndAddress), Ok(index));
	}
	assert!(matches!(file.exception_arm64(), Err(Error::Invalid)));
}

//----------------------------------------------------------------

#[test]
fn scanner() {
	let mut save = [0; 8];
	let file_map = FileMap::open(FILE_NAME).unwrap();
	let file = PeFile::from_bytes(&file_map).unwrap();

	let pat = pelite::pattern!("4C8B41'? 4C2BC2 ????????? 0FB60A 420FB60402 2BC8 75% 8B15${'} 85 C9");
	assert!(file.scanner().code().find(pat, &mut save).is_some());
	assert_eq!(save[0], 0x12F0);
	assert_eq!(save[1], 0x12F3);
	assert_eq!(save[2], 0x5140);

	// Test the edge cases of quicksearch
	// The range restricts starts; the rest of the pattern can extend beyond it.
	let pat = pelite::pattern!("0F1002 488BC1 0F1101 F20F104A10 F20F114910 C3");
	assert!(file.scanner().within(0x148F..0x14A3).find(pat, &mut save).is_some());
	assert_eq!(save[0], 0x1490);

	assert_eq!(file.scanner().within(0x1490..0x1491).find(pat, &mut save), Some(0x1490));
}

//----------------------------------------------------------------

#[cfg(any(windows, unix))]
#[test]
fn imagemap() {
	use pelite::pe64::PeView;

	let image = pelite::ImageMap::open(FILE_NAME).unwrap();
	let _view = PeView::from_bytes(&image).unwrap();
	#[cfg(unix)] {
		let file_map = FileMap::open(FILE_NAME).unwrap();
		let expected = PeFile::from_bytes(&file_map).unwrap().to_view();
		assert_eq!(image.as_ref(), expected.as_ref());
	}
}
