use super::*;
use core::mem::size_of;

const RVA: u32 = 0x1000;
const BASE: Va = 0x400000;
const HEADER_SIZE: usize = size_of::<IMAGE_DOS_HEADER>()
	+ size_of::<IMAGE_NT_HEADERS>()
	+ size_of::<IMAGE_SECTION_HEADER>();

#[repr(C)]
struct TestImage {
	dos: IMAGE_DOS_HEADER,
	nt: IMAGE_NT_HEADERS,
	section: IMAGE_SECTION_HEADER,
	padding: [u8; 0x200 - HEADER_SIZE],
	raw: [u8; 4],
}

unsafe impl Pod for TestImage {}

fn image() -> TestImage {
	let mut image: TestImage = dataview::zeroed();
	image.dos.e_magic = IMAGE_DOS_SIGNATURE;
	image.dos.e_lfanew = size_of::<IMAGE_DOS_HEADER>() as u32;
	image.nt.Signature = IMAGE_NT_HEADERS_SIGNATURE;
	image.nt.FileHeader.NumberOfSections = 1;
	image.nt.FileHeader.SizeOfOptionalHeader = size_of::<IMAGE_OPTIONAL_HEADER>() as u16;
	image.nt.OptionalHeader.Magic = IMAGE_NT_OPTIONAL_HDR_MAGIC;
	image.nt.OptionalHeader.ImageBase = BASE.into();
	image.nt.OptionalHeader.SectionAlignment = 0x1000;
	image.nt.OptionalHeader.FileAlignment = 0x200;
	image.nt.OptionalHeader.SizeOfHeaders = 0x200;
	image.nt.OptionalHeader.SizeOfImage = 0x2000;
	image.section.VirtualAddress = RVA;
	image.section.VirtualSize = 8;
	image.section.SizeOfRawData = 4;
	image.section.PointerToRawData = 0x200;
	image.raw = [1, 2, 3, 4];
	image
}

#[test]
fn copy_and_into_zero_fill_section_tail() {
	let image = image();
	let file = PeFile::from_bytes(dataview::bytes(&image)).unwrap();
	let ptr = Ptr::<[u8; 8]>::from(BASE + RVA as Va);

	assert_eq!(file.derva_copy::<[u8; 8]>(RVA), Ok([1, 2, 3, 4, 0, 0, 0, 0]));
	assert_eq!(file.deref_copy(ptr), Ok([1, 2, 3, 4, 0, 0, 0, 0]));
	assert_eq!(file.derva_copy::<u32>(RVA + 4), Ok(0));

	let mut dest = [0xff; 8];
	assert_eq!(file.derva_into(RVA, &mut dest), Ok(()));
	assert_eq!(dest, [1, 2, 3, 4, 0, 0, 0, 0]);
	dest.fill(0xff);
	assert_eq!(file.deref_into(ptr, &mut dest), Ok(()));
	assert_eq!(dest, [1, 2, 3, 4, 0, 0, 0, 0]);

	let mapped = file.to_view();
	let view = PeView::from_bytes(&mapped).unwrap();
	assert_eq!(view.derva_copy::<[u8; 8]>(RVA), Ok(dest));
	assert_eq!(view.deref_copy(ptr), Ok(dest));
}

#[test]
fn copy_stays_within_one_section() {
	let image = image();
	let file = PeFile::from_bytes(dataview::bytes(&image)).unwrap();
	let mut dest = [0xff; 5];
	assert_eq!(file.derva_into(RVA + 4, &mut dest), Err(Error::Bounds));
	assert_eq!(dest, [0xff; 5]);
	assert_eq!(file.derva_copy::<[u8; 5]>(RVA + 4), Err(Error::Bounds));
	assert_eq!(file.deref_copy::<u8>(Ptr::from(0)), Err(Error::Null));
}

#[test]
fn image_base_is_readable() {
	let image = image();
	let file = PeFile::from_bytes(dataview::bytes(&image)).unwrap();
	let ptr = Ptr::<u16>::from(BASE);

	assert_eq!(file.rva_to_va(0), Ok(BASE));
	assert_eq!(file.va_to_rva(BASE), Ok(0));
	assert_eq!(&file.slice(0, 2, 1).unwrap()[..2], &IMAGE_DOS_SIGNATURE.to_le_bytes());
	assert_eq!(file.slice_bytes(0).unwrap().len(), 0x200);
	assert_eq!(&file.read(BASE, 2, 1).unwrap()[..2], &IMAGE_DOS_SIGNATURE.to_le_bytes());
	assert_eq!(file.derva::<u16>(0), Ok(&IMAGE_DOS_SIGNATURE));
	assert_eq!(file.derva_slice::<u8>(0, 2), Ok(&IMAGE_DOS_SIGNATURE.to_le_bytes()[..]));
	assert_eq!(file.derva_slice_s::<u8>(0, 0), Ok(&IMAGE_DOS_SIGNATURE.to_le_bytes()[..]));
	assert_eq!(file.deref(ptr), Ok(&IMAGE_DOS_SIGNATURE));
	assert_eq!(file.derva_copy::<u16>(0), Ok(IMAGE_DOS_SIGNATURE));
	assert_eq!(file.deref_copy(ptr), Ok(IMAGE_DOS_SIGNATURE));
	let mut dest = [0xff; 2];
	assert_eq!(file.derva_into(0, &mut dest), Ok(()));
	assert_eq!(dest, IMAGE_DOS_SIGNATURE.to_le_bytes());
	dest.fill(0xff);
	assert_eq!(file.deref_into(Ptr::<[u8; 2]>::from(BASE), &mut dest), Ok(()));
	assert_eq!(dest, IMAGE_DOS_SIGNATURE.to_le_bytes());
	dest.fill(0xff);
	assert_eq!(file.derva_into(0x1ff, &mut dest), Err(Error::Bounds));
	assert_eq!(dest, [0xff; 2]);

	let mapped = file.to_view();
	let view = PeView::from_bytes(&mapped).unwrap();
	assert_eq!(view.rva_to_va(0), Ok(BASE));
	assert_eq!(view.va_to_rva(BASE), Ok(0));
	assert_eq!(&view.slice(0, 2, 1).unwrap()[..2], &IMAGE_DOS_SIGNATURE.to_le_bytes());
	assert_eq!(view.derva::<u16>(0), Ok(&IMAGE_DOS_SIGNATURE));
	assert_eq!(view.derva_copy::<u16>(0), Ok(IMAGE_DOS_SIGNATURE));
	assert_eq!(view.deref_copy(ptr), Ok(IMAGE_DOS_SIGNATURE));
	dest.fill(0xff);
	assert_eq!(view.derva_into(0, &mut dest), Ok(()));
	assert_eq!(dest, IMAGE_DOS_SIGNATURE.to_le_bytes());
	dest.fill(0xff);
	assert_eq!(view.deref_into(Ptr::<[u8; 2]>::from(BASE), &mut dest), Ok(()));
	assert_eq!(dest, IMAGE_DOS_SIGNATURE.to_le_bytes());
}

#[test]
fn copy_from_section_without_raw_data() {
	let mut image = image();
	image.section.SizeOfRawData = 0;
	image.section.PointerToRawData = 0;
	let file = PeFile::from_bytes(dataview::bytes(&image)).unwrap();
	let ptr = Ptr::<[u8; 8]>::from(BASE + RVA as Va);

	assert_eq!(file.derva_copy::<[u8; 8]>(RVA), Ok([0; 8]));
	assert_eq!(file.deref_copy(ptr), Ok([0; 8]));
	let mut dest = [0xff; 8];
	assert_eq!(file.derva_into(RVA, &mut dest), Ok(()));
	assert_eq!(dest, [0; 8]);
	dest.fill(0xff);
	assert_eq!(file.deref_into(ptr, &mut dest), Ok(()));
	assert_eq!(dest, [0; 8]);
}
