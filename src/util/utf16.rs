/// Encodes a string expression as a constant UTF-16 slice.
macro_rules! utf16 {
	($s:expr) => {
		&const { $crate::util::utf16::encode::<{ $crate::util::utf16::len($s) }>($s) }
	};
}

const fn next(bytes: &[u8]) -> Option<(u32, &[u8])> {
	match bytes {
		&[a, ref tail @ ..] if a & 0x80 == 0x00 => Some((a as u32, tail)),
		&[a, b, ref tail @ ..] if a & 0xe0 == 0xc0 => Some(((a as u32 & 0x1f) << 6 | (b as u32 & 0x3f), tail)),
		&[a, b, c, ref tail @ ..] if a & 0xf0 == 0xe0 => Some(((a as u32 & 0x0f) << 12 | (b as u32 & 0x3f) << 6 | (c as u32 & 0x3f), tail)),
		&[a, b, c, d, ref tail @ ..] if a & 0xf8 == 0xf0 => Some(((a as u32 & 0x07) << 18 | (b as u32 & 0x3f) << 12 | (c as u32 & 0x3f) << 6 | (d as u32 & 0x3f), tail)),
		&[..] => None,
	}
}

#[doc(hidden)]
pub(crate) const fn len(s: &str) -> usize {
	let mut bytes = s.as_bytes();
	let mut len = 0;
	while let Some((chr, tail)) = next(bytes) {
		bytes = tail;
		len += if chr >= 0x10000 { 2 } else { 1 };
	}
	len
}

#[doc(hidden)]
pub(crate) const fn encode<const LEN: usize>(s: &str) -> [u16; LEN] {
	let mut bytes = s.as_bytes();
	let mut data = [0u16; LEN];
	let mut i = 0;
	while let Some((chr, tail)) = next(bytes) {
		bytes = tail;
		if chr >= 0x10000 {
			data[i] = (0xd800 + (chr - 0x10000) / 0x400) as u16;
			data[i + 1] = (0xdc00 + (chr - 0x10000) % 0x400) as u16;
			i += 2;
		}
		else {
			data[i] = chr as u16;
			i += 1;
		}
	}
	data
}

#[test]
fn test_utf16() {
	const TEXT: &str = "ASCII, ¶, 🦀";
	static WORDS: &[u16] = utf16!(TEXT);
	assert!(TEXT.encode_utf16().eq(WORDS.iter().copied()));
	assert!(utf16!("").is_empty());
}
