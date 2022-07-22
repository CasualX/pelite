use dataview::PodMethods;
use pelite::pe32::{image, Pe, PeFile};
use pelite::{FileMap, Pod};

// For fun let's try loading tiny PE files.
// The examples are sourced from:
//
// https://webserver2.tecgraf.puc-rio.br/~ismael/Cursos/YC++/apostilas/win32_xcoff_pe/tyne-example/Tiny%20PE.htm
//
// I used Internet Archive's Wayback Machine to download the samples since the original links 404'd

fn assert_memcmp<T: std::fmt::Debug + Pod>(lhs: &T, rhs: &T) {
    let lhs_bytes = lhs.as_bytes();
    let rhs_bytes = rhs.as_bytes();
    assert_eq!(lhs_bytes, rhs_bytes, "lhs: {:?} rhs: {:?}", lhs, rhs);
}

/*
Barebones C program without linking to the CRT
*/

#[test]
fn tiny_c_1024() {
    let file_map = FileMap::open("tests/tiny/tiny.c.1024").unwrap();
    let file = PeFile::from_bytes(&file_map).unwrap();

    assert_eq!(file.dos_header().e_lfanew, 0xB0);

    assert_eq!(file.file_header().TimeDateStamp, 0x4545BD6F);
    assert_eq!(file.file_header().SizeOfOptionalHeader, 0xE0);

    let optional_header = file.optional_header();
    assert_eq!(optional_header.SizeOfImage, 0x2000);
    assert_eq!(optional_header.SizeOfHeaders, 0x200);
    assert_eq!(optional_header.SizeOfCode, 0x200);
    assert_eq!(optional_header.AddressOfEntryPoint, 0x1000);

    assert_eq!(file.data_directory().len(), 0x10);
    for data_dir in file.data_directory() {
        assert_eq!(data_dir.VirtualAddress, 0);
        assert_eq!(data_dir.Size, 0);
    }

    let sections = file.section_headers().as_slice();
    assert_eq!(sections[0].VirtualSize, 0x4);
    assert_eq!(sections[0].VirtualAddress, 0x1000);
    assert_eq!(sections[0].SizeOfRawData, 0x200);
    assert_eq!(sections[0].PointerToRawData, 0x200);
    assert_eq!(sections[0].Characteristics, 0x60000020);
}

/*
Barebones C program with no CRT and decrease alignment
*/

#[test]
fn tiny_c_468() {
    let file_map = FileMap::open("tests/tiny/tiny.c.468").unwrap();
    let file = PeFile::from_bytes(&file_map).unwrap();

    assert_eq!(file.dos_header().e_lfanew, 0xB0);

    assert_eq!(file.file_header().TimeDateStamp, 0x4545BE5D);
    assert_eq!(file.file_header().SizeOfOptionalHeader, 0xE0);

    let optional_header = file.optional_header();
    assert_eq!(optional_header.SizeOfImage, 0x1D4);
    assert_eq!(optional_header.SizeOfHeaders, 0x1D0);
    assert_eq!(optional_header.SectionAlignment, 1);
    assert_eq!(optional_header.FileAlignment, 1);

    assert_eq!(file.data_directory().len(), 0x10);
    for data_dir in file.data_directory() {
        assert_eq!(data_dir.VirtualAddress, 0);
        assert_eq!(data_dir.Size, 0);
    }

    let sections = file.section_headers().as_slice();
    assert_eq!(sections[0].VirtualSize, 0x4);
    assert_eq!(sections[0].VirtualAddress, 0x1D0);
    assert_eq!(sections[0].SizeOfRawData, 0x4);
    assert_eq!(sections[0].PointerToRawData, 0x1D0);
    assert_eq!(sections[0].Characteristics, 0x60000020);
}

/*
Removing the DOS stub
*/

#[test]
fn tiny_356() {
    let file_map = FileMap::open("tests/tiny/tiny.356").unwrap();
    let file = PeFile::from_bytes(&file_map).unwrap();

    assert_eq!(file.dos_header().e_lfanew, 0x40);

    assert_eq!(file.file_header().TimeDateStamp, 0x4545BE5D);
    assert_eq!(file.file_header().SizeOfOptionalHeader, 0xE0);

    let optional_header = file.optional_header();
    assert_eq!(optional_header.SizeOfImage, 0x164);
    assert_eq!(optional_header.SizeOfHeaders, 0x160);
    assert_eq!(optional_header.SectionAlignment, 1);
    assert_eq!(optional_header.FileAlignment, 1);

    assert_eq!(
        file.data_directory().len(),
        image::IMAGE_NUMBEROF_DIRECTORY_ENTRIES
    );

    let sections = file.section_headers().as_slice();
    assert_eq!(sections[0].VirtualSize, 0x4);
    assert_eq!(sections[0].VirtualAddress, 0x160);
    assert_eq!(sections[0].SizeOfRawData, 0x4);
    assert_eq!(sections[0].PointerToRawData, 0x160);
    assert_eq!(sections[0].Characteristics, 0x60000020);
}

/*
Collapsing the DOS header to overlap the NT headers
*/

#[test]
fn tiny_296() {
    let file_map = FileMap::open("tests/tiny/tiny.296").unwrap();
    let file = PeFile::from_bytes(&file_map).unwrap();

    let dos_header = image::IMAGE_DOS_HEADER {
        e_magic: image::IMAGE_DOS_SIGNATURE,
        e_cblp: 0,
        // PE signature
        e_cp: image::IMAGE_NT_HEADERS_SIGNATURE as u16,
        e_crlc: 0,
        // File header
        e_cparhdr: 0x014C,
        e_minalloc: 1,
        e_maxalloc: 0xBE5D,
        e_ss: 0x4545,
        e_sp: 0,
        e_csum: 0,
        e_ip: 0,
        e_cs: 0,
        e_lfarlc: 0x00E0,
        e_ovno: 0x0103,
        // Optional header
        e_res: [0x010B, 8, 4, 0],
        e_oemid: 0,
        e_oeminfo: 0,
        e_res2: [0, 0, 0x0124, 0, 0x0124, 0, 0x0128, 0, 0, 0x40],
        e_lfanew: 4,
    };

    assert_memcmp(file.dos_header(), &dos_header);

    let file_header = image::IMAGE_FILE_HEADER {
        Machine: 0x014C,
        NumberOfSections: 1,
        TimeDateStamp: 0x4545BE5D,
        PointerToSymbolTable: 0,
        NumberOfSymbols: 0,
        SizeOfOptionalHeader: 0x00E0,
        Characteristics: 0x0103,
    };

    assert_memcmp(file.file_header(), &file_header);

    let optional_header = image::IMAGE_OPTIONAL_HEADER {
        Magic: image::IMAGE_NT_OPTIONAL_HDR_MAGIC,
        LinkerVersion: image::IMAGE_VERSION { Major: 8, Minor: 0 },
        SizeOfCode: 4,
        SizeOfInitializedData: 0,
        SizeOfUninitializedData: 0,
        AddressOfEntryPoint: 0x124,
        BaseOfCode: 0x124,
        BaseOfData: 0x128,
        ImageBase: 0x00400000,
        SectionAlignment: 4,
        FileAlignment: 4,
        OperatingSystemVersion: image::IMAGE_VERSION { Major: 4, Minor: 0 },
        ImageVersion: image::IMAGE_VERSION { Major: 0, Minor: 0 },
        SubsystemVersion: image::IMAGE_VERSION { Major: 4, Minor: 0 },
        Win32VersionValue: 0,
        SizeOfImage: 0x128,
        SizeOfHeaders: 0x124,
        CheckSum: 0,
        Subsystem: 0x002,
        DllCharacteristics: 0x0400,
        SizeOfStackReserve: 0x00100000,
        SizeOfStackCommit: 0x00001000,
        SizeOfHeapReserve: 0x00100000,
        SizeOfHeapCommit: 0x00001000,
        LoaderFlags: 0,
        NumberOfRvaAndSizes: image::IMAGE_NUMBEROF_DIRECTORY_ENTRIES as u32,
        DataDirectory: [],
    };

    assert_memcmp(file.optional_header(), &optional_header);
}

/*
Removing the data directories
*/

#[test]
fn tiny_168() {
    let file_map = FileMap::open("tests/tiny/tiny.168").unwrap();
    let file = PeFile::from_bytes(&file_map).unwrap();

    assert_eq!(file.optional_header().NumberOfRvaAndSizes, 0);
}

/*
Merge the PE header with the section headers
*/

#[test]
fn tiny_128() {
    let file_map = FileMap::open("tests/tiny/tiny.128").unwrap();
    let file = PeFile::from_bytes(&file_map).unwrap();

    let optional_header = image::IMAGE_OPTIONAL_HEADER {
        Magic: image::IMAGE_NT_OPTIONAL_HDR_MAGIC,
        LinkerVersion: image::IMAGE_VERSION { Major: 8, Minor: 0 },
        SizeOfCode: 4,
        SizeOfInitializedData: 0,
        SizeOfUninitializedData: 4,
        AddressOfEntryPoint: 0x7C,
        BaseOfCode: 4,
        BaseOfData: 0x7C,
        ImageBase: 0x00400000,
        SectionAlignment: 4,
        FileAlignment: 4,
        OperatingSystemVersion: image::IMAGE_VERSION { Major: 4, Minor: 0 },
        ImageVersion: image::IMAGE_VERSION { Major: 0, Minor: 0 },
        SubsystemVersion: image::IMAGE_VERSION { Major: 4, Minor: 0 },
        Win32VersionValue: 0,
        SizeOfImage: 0x80,
        SizeOfHeaders: 0x7C,
        CheckSum: 0,
        Subsystem: 0x002,
        DllCharacteristics: 0x0400,
        SizeOfStackReserve: 0x00100000,
        SizeOfStackCommit: 0x00001000,
        SizeOfHeapReserve: 0x00100000,
        SizeOfHeapCommit: 0x00001000,
        LoaderFlags: 0,
        NumberOfRvaAndSizes: 0,
        DataDirectory: [],
    };

    assert_memcmp(file.optional_header(), &optional_header);
}

/*
Abuse file mapping padding with zeroes to fill the virtual memory page
*/

#[test]
fn tiny_97() {
    let file_map = FileMap::open("tests/tiny/tiny.97").unwrap();
    let file = PeFile::from_bytes(&file_map).unwrap();

    let optional_header = image::IMAGE_OPTIONAL_HEADER {
        Magic: image::IMAGE_NT_OPTIONAL_HDR_MAGIC,
        LinkerVersion: image::IMAGE_VERSION { Major: 8, Minor: 0 },
        SizeOfCode: 4,
        SizeOfInitializedData: 0,
        SizeOfUninitializedData: 4,
        AddressOfEntryPoint: 0xC,
        BaseOfCode: 4,
        BaseOfData: 0xC,
        ImageBase: 0x400000,
        SectionAlignment: 4,
        FileAlignment: 4,
        OperatingSystemVersion: image::IMAGE_VERSION { Major: 4, Minor: 0 },
        ImageVersion: image::IMAGE_VERSION { Major: 0, Minor: 0 },
        SubsystemVersion: image::IMAGE_VERSION { Major: 4, Minor: 0 },
        Win32VersionValue: 0,
        SizeOfImage: 0x68,
        SizeOfHeaders: 0x64,
        CheckSum: 0,
        Subsystem: 2,
        DllCharacteristics: 0,
        SizeOfStackReserve: 0,
        SizeOfStackCommit: 0,
        SizeOfHeapReserve: 0,
        SizeOfHeapCommit: 0,
        LoaderFlags: 0,
        NumberOfRvaAndSizes: 0,
        DataDirectory: [],
    };

    assert_memcmp(file.optional_header(), &optional_header);
}

/*
Smallest PE file with imports
*/

#[test]
fn tiny_import_209() {
    let file_map = FileMap::open("tests/tiny/tiny.import.209").unwrap();
    let file = PeFile::from_bytes(&file_map).unwrap();

    let imports = file.imports().unwrap();
    assert_eq!(imports.image().len(), 1);

    let kernel32 = imports.into_iter().next().unwrap();

    assert_eq!(kernel32.dll_name().unwrap(), "KERNEL32.dll");
    assert_eq!(kernel32.iat().unwrap().len(), 1);
    assert_eq!(kernel32.int().unwrap().len(), 1);
}

/*
Merge the import directory inside the PE headers
*/

#[test]
fn tiny_import_161() {
    let file_map = FileMap::open("tests/tiny/tiny.import.161").unwrap();
    let file = PeFile::from_bytes(&file_map).unwrap();

    let imports = file.imports().unwrap();
    assert_eq!(imports.image().len(), 1);

    let kernel32 = imports.into_iter().next().unwrap();

    assert_eq!(kernel32.dll_name().unwrap(), "KERNEL32.dll");
    assert_eq!(kernel32.iat().unwrap().len(), 1);
    assert!(kernel32.int().is_err());
}

/*
Merge the imports even further with the PE headers
*/

#[test]
fn tiny_import_133() {
    let file_map = FileMap::open("tests/tiny/tiny.import.133").unwrap();
    let file = PeFile::from_bytes(&file_map).unwrap();

    let imports = file.imports().unwrap();
    let kernel32 = imports.into_iter().next().unwrap();
    assert_eq!(kernel32.dll_name().unwrap(), "KERNEL32.dll");
    assert_eq!(kernel32.iat().unwrap().len(), 1);
}

/*
Smallest PE file that downloads a file from the Internet and executes it
*/

#[test]
fn tiny_webdav_133() {
    let file_map = FileMap::open("tests/tiny/tiny.webdav.133").unwrap();
    let file = PeFile::from_bytes(&file_map).unwrap();

    let imports = file.imports().unwrap();
    let unc = imports.into_iter().next().unwrap();
    assert_eq!(unc.dll_name().unwrap(), r"\\66.93.68.6\z");
    assert_eq!(unc.iat().unwrap().len(), 1);
}
