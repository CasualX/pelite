use pelite::pe64::exports::{Export, GetProcAddress};
use pelite::pe64::imports::Import;
use pelite::pe64::{Pe, PeFile, Rva};
use pelite::util::CStr;
use pelite::{Error, FileMap};

const FILE_NAME: &str = "demo/Demo64.dll";

//----------------------------------------------------------------

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
    let mut encoded = vec![0; rich_structure.image().len()];
    rich_structure.encode(&records, &mut encoded).unwrap();
    assert_eq!(rich_structure.image(), &*encoded);
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
    let bad_hint = Import::ByName {
        hint: 5,
        name: s_export_name,
    };
    let good_hint = Import::ByName {
        hint: 11,
        name: s_export_name,
    };
    assert_eq!(exports_by.import(bad_hint), Ok(Export::Symbol(&0x1230)));
    assert_eq!(exports_by.import(good_hint), Ok(Export::Symbol(&0x1230)));

    assert_eq!(
        exports_by.hint_name(5, s_export_name),
        Ok(Export::Symbol(&0x1230))
    );
    assert_eq!(
        exports_by.hint_name(11, s_export_name),
        Ok(Export::Symbol(&0x1230))
    );

    assert_eq!(exports_by.hint(0), Ok(Export::Symbol(&0x1260)));
    assert_eq!(exports_by.name_of_hint(11), Ok(s_export_name));

    assert_eq!(exports_by.name_lookup(11), Ok(good_hint));

    assert_eq!(exports_by.iter().count(), 20);
    assert_eq!(exports_by.iter_names().count(), 20);

    assert_eq!(
        file.get_proc_address("ThrowException"),
        file.rva_to_va(0x10C0)
    );
    assert_eq!(
        file.get_proc_address(b"ThrowException".as_ref()),
        file.rva_to_va(0x10C0)
    );
    assert_eq!(file.get_proc_address(0x14), file.rva_to_va(0x10C0));

    assert_eq!(file.get_proc_address(bad_hint), file.rva_to_va(0x1230));
    assert_eq!(file.get_proc_address(good_hint), file.rva_to_va(0x1230));
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
    let rhs = kernel32_dll
        .int()
        .unwrap()
        .chain(msvcr120_dll.int().unwrap());
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
    let mut baseline = base_relocs.iter_blocks().flat_map(move |block| {
        block
            .words()
            .iter()
            .filter(move |&word| block.type_of(word) != 0)
            .map(move |word| block.rva_of(word))
    });
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
    let result = pelite::base_relocs::build(&rvas, &types);
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

    let data2 = resources
        .find_dir("/#MANIFEST")
        .unwrap()
        .get_dir("#2".into())
        .unwrap()
        .get_data("#1033".into())
        .unwrap();
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
fn debug() {
    let file_map = FileMap::open(FILE_NAME).unwrap();
    let file = PeFile::from_bytes(&file_map).unwrap();
    let debug = file.debug().unwrap();

    assert_eq!(
        debug.pdb_file_name().unwrap(),
        r"D:\Projects\pelite\proto\Demo\x64\Release\Demo.pdb"
    );
}

//----------------------------------------------------------------

#[test]
fn security() {
    let file_map = FileMap::open(FILE_NAME).unwrap();
    let file = PeFile::from_bytes(&file_map).unwrap();
    let security = file.security();

    assert!(match security {
        Err(Error::Null) => true,
        _ => false,
    });
}

//----------------------------------------------------------------

#[test]
fn exception() {
    let file_map = FileMap::open(FILE_NAME).unwrap();
    let file = PeFile::from_bytes(&file_map).unwrap();
    let exception = file.exception().unwrap();

    assert_eq!(exception.functions().len(), 38);
}

//----------------------------------------------------------------

#[test]
fn scanner() {
    let mut save = [0; 8];
    let file_map = FileMap::open(FILE_NAME).unwrap();
    let file = PeFile::from_bytes(&file_map).unwrap();

    let pat =
        pelite::pattern!("4C8B41'? 4C2BC2 ????????? 0FB60A 420FB60402 2BC8 75% 8B15${'} 85 C9");
    assert!(file.scanner().finds_code(pat, &mut save));
    assert_eq!(save[0], 0x12F0);
    assert_eq!(save[1], 0x12F3);
    assert_eq!(save[2], 0x5140);

    // Test the edge cases of quicksearch
    // See scanner code for an unfortunate edge case...
    let pat = pelite::pattern!("0F1002 488BC1 0F1101 F20F104A10 F20F114910 C3");
    assert!(file.scanner().finds(pat, 0x148F..0x14A3, &mut save));
    assert_eq!(save[0], 0x1490);

    assert!(!file.scanner().finds(pat, 0x1490..0x149F, &mut save));
}

//----------------------------------------------------------------

#[cfg(windows)]
#[test]
fn imagemap() {
    use pelite::pe64::PeView;
    use pelite::ImageMap;

    let image = ImageMap::open(FILE_NAME).unwrap();
    let _view = PeView::from_bytes(&image).unwrap();
}
