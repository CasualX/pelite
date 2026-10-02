use super::*;

#[repr(align(8))]
struct Aligned([u8; 33]);

#[test]
fn rejects_misaligned_section() {
	let storage = Aligned([0; 33]);
	let dir = IMAGE_DATA_DIRECTORY { VirtualAddress: 0, Size: 32 };
	let resources = ResourceDirectory::new(&storage.0[1..], &dir);

	assert!(matches!(resources.root(), Err(Error::Misaligned)));
	assert!(matches!(resources.slice_len::<u16>(0, 1), Err(Error::Misaligned)));
	assert!(matches!(resources.slice_ws(0), Err(Error::Misaligned)));
}

#[test]
fn slices_wide_strings() {
	let mut storage = Aligned([0; 33]);
	storage.0[..4].copy_from_slice(&[1, 0, b'A', 0]);
	let dir = IMAGE_DATA_DIRECTORY { VirtualAddress: 0, Size: 32 };
	let resources = ResourceDirectory::new(&storage.0[..32], &dir);

	assert_eq!(resources.slice_ws(0), Ok(&[b'A' as u16][..]));
	assert!(matches!(resources.slice_len::<u16>(0, usize::MAX), Err(Error::Overflow)));
}

#[test]
fn matches_numeric_resource_names_and_type_aliases() {
	for (id, name) in [(0, "#0"), (1, "#1"), (10, "#10"), (u32::MAX, "#4294967295")] {
		assert_eq!(ResourceName::Id(id), ResourceName::Str(name));
	}
	assert_eq!(ResourceName::Id(10), ResourceName::Str("#RCDATA"));
	assert_eq!(ResourceName::Id(0), ResourceName::Str("#00"));
	for name in ["0", "#", "#-0", "#+0", "#0x0", "#0junk", "#4294967296"] {
		assert_ne!(ResourceName::Id(0), ResourceName::Str(name));
	}
}
