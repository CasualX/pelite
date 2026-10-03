use super::*;

impl Analysis<'_> {
	/// Recognize strict ASCII C strings at candidate data addresses in read-only sections.
	pub fn label_strings(&mut self) {
		for section in self.pe.section_headers() {
			if section.Characteristics & (image::IMAGE_SCN_MEM_READ | image::IMAGE_SCN_MEM_WRITE | image::IMAGE_SCN_MEM_EXECUTE)
				!= image::IMAGE_SCN_MEM_READ {
				continue;
			}
			let Ok(bytes) = self.pe.get_section_bytes(section) else { continue };
			let virtual_size = if section.VirtualSize == 0 { section.SizeOfRawData } else { section.VirtualSize };
			let len = bytes.len().min(virtual_size as usize).min(self.size.saturating_sub(section.VirtualAddress) as usize);
			let bytes = &bytes[..len];
			for symbol in self.symbols.values_mut() {
				if symbol.name != factmap::SymbolName::Data {
					continue;
				}
				let Some(offset) = symbol.rva.checked_sub(section.VirtualAddress) else { continue };
				let Some(bytes) = bytes.get(offset as usize..) else { continue };
				if looks_like_va(self.pe, symbol.rva, bytes) {
					continue;
				}
				let Some(string) = ascii_string(bytes) else { continue };
				symbol.ty = ty::Type::CStr;
				symbol.name = factmap::SymbolName::Named(string_label(string));
			}
		}
	}
}

fn looks_like_va(pe: PeFile<'_>, rva: u32, bytes: &[u8]) -> bool {
	let width = ty::PointerWidth::from(pe).bytes() as usize;
	let Some(address) = pe.image_base().checked_add(u64::from(rva)) else { return false };
	if address % width as u64 != 0 {
		return false;
	}
	let Some(bytes) = bytes.get(..width) else { return false };
	let va = if width == 4 {
		u64::from(u32::from_le_bytes(bytes.try_into().unwrap()))
	}
	else {
		u64::from_le_bytes(bytes.try_into().unwrap())
	};
	pe.va_to_rva(va).is_ok()
}

/// The terminator must be present in the supplied section bytes.
fn ascii_string(bytes: &[u8]) -> Option<&str> {
	let mut alphanumeric = 0;
	for (len, &byte) in bytes.iter().enumerate() {
		if byte == 0 {
			if len < 3 || (len <= 6 && alphanumeric != len) || alphanumeric < len.div_ceil(2) {
				return None;
			}
			return std::str::from_utf8(&bytes[..len]).ok();
		}
		if !(byte.is_ascii_graphic() || byte == b' ') {
			return None;
		}
		alphanumeric += usize::from(byte.is_ascii_alphanumeric());
	}
	None
}

fn string_label(string: &str) -> String {
	const MAX_LEN: usize = 64;
	let mut name = String::from("sz");
	for word in string.split(|ch: char| !ch.is_ascii_alphanumeric()).filter(|word| !word.is_empty()) {
		for (index, byte) in word.bytes().enumerate() {
			let byte = if index == 0 { byte.to_ascii_uppercase() } else { byte.to_ascii_lowercase() };
			name.push(char::from(byte));
			if name.len() > MAX_LEN {
				name.truncate(MAX_LEN - 3);
				name.push_str("...");
				return name;
			}
		}
	}
	name
}

#[test]
fn strict_ascii_strings() {
	for bytes in [b"abc\0".as_slice(), b"abc123\0", b"hello world!\0", b"a b c d\0", b"abcd!!!!\0"] {
		assert!(ascii_string(bytes).is_some(), "{bytes:?}");
	}
	for bytes in [b"\0".as_slice(), b"A\0", b"ab\0", b"abc 12\0", b"abcde!\0", b"abc!!!!!\0", b"       \0", b"abcdefg", b"hello\nworld\0", b"hello\xffworld\0"] {
		assert!(ascii_string(bytes).is_none(), "{bytes:?}");
	}
	assert_eq!(ascii_string(b"hello world\0ignored"), Some("hello world"));
}

#[test]
fn pascal_case_string_labels() {
	assert_eq!(string_label("hello WORLD! error_code 42"), "szHelloWorldErrorCode42");
	assert_eq!(string_label(&"a".repeat(62)).len(), 64);
	let long = string_label(&"a".repeat(63));
	assert_eq!(long.len(), 64);
	assert!(long.ends_with("..."));
}
