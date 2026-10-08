use super::*;

const MIN_STRING_SCORE: i32 = 40;

/// Recognize UTF-8 and UTF-16LE NUL-terminated strings at read-only data candidates.
pub fn label_strings(input: &AnalysisInput<'_>, output: &mut AnalysisOutput) {
	for section in input.pe.section_headers() {
		if !section.is_rdata() {
			continue;
		}
		let Ok(bytes) = input.pe.get_section_bytes(section) else { continue };
		let virtual_size = if section.VirtualSize == 0 { section.SizeOfRawData } else { section.VirtualSize };
		let len = bytes.len().min(virtual_size as usize).min(input.size.saturating_sub(section.VirtualAddress) as usize);
		let bytes = &bytes[..len];
		for symbol in output.symbols.values_mut() {
			if !matches!(symbol.name, factmap::SymbolName::Data | factmap::SymbolName::RData) {
				continue;
			}
			let Some(offset) = symbol.rva.checked_sub(section.VirtualAddress) else {
				continue
			};
			let Some(bytes) = bytes.get(offset as usize..) else {
				continue
			};
			if looks_like_va(input.pe, symbol.rva, bytes) {
				continue;
			}
			if utf8z_string(bytes).is_some() {
				symbol.ty = ty::Type::CStr;
			}
			else if utf16lez_string(bytes).is_some() {
				symbol.ty = ty::Type::Utf16LEZ;
			}
		}
	}
}

fn looks_like_va(pe: PeFile<'_>, rva: u32, bytes: &[u8]) -> bool {
	let width = ty::PointerWidth::from(pe).bytes() as usize;
	let Some(address) = pe.image_base().checked_add(u64::from(rva)) else {
		return false
	};
	if address % width as u64 != 0 {
		return false;
	}
	let Some(bytes) = bytes.get(..width) else {
		return false
	};
	let va = if width == 4 {
		u64::from(u32::from_le_bytes(bytes.try_into().unwrap()))
	}
	else {
		u64::from_le_bytes(bytes.try_into().unwrap())
	};
	pe.va_to_rva(va).is_ok()
}

/// The terminator must be present in the supplied section bytes.
fn utf8z_string(bytes: &[u8]) -> Option<&str> {
	let end = bytes.iter().position(|&byte| byte == 0)?;
	let value = std::str::from_utf8(&bytes[..end]).ok()?;
	if score_text(value) < MIN_STRING_SCORE {
		return None;
	}
	Some(value)
}

/// Require a terminator, valid UTF-16, and enough textual structure to reject random data.
fn utf16lez_string(bytes: &[u8]) -> Option<String> {
	let units = dataview::DataView::from(bytes).try_slice::<u16>(0, bytes.len() / 2)?;
	let end = units.iter().position(|&unit| unit == 0)?;
	let candidate = &units[..end];
	let value = char::decode_utf16(candidate.iter().copied().map(u16::from_le)).collect::<result::Result<String, _>>().ok()?;
	if score_text(&value) < MIN_STRING_SCORE {
		return None;
	}
	Some(value)
}

/// Score decoded, NUL-terminated text identically for both encodings.
fn score_text(value: &str) -> i32 {
	let length = value.chars().count();

	// Length and NUL termination provide the base score; ordinary text adds evidence.
	let mut points = 40 + (length.saturating_sub(3).min(7) * 4) as i32;
	if length < 3 {
		points -= 80;
	}
	if value.chars().any(char::is_alphanumeric) {
		points += 10;
	}
	if value.chars().filter(|ch| ch.is_alphabetic()).count() > length - length.div_ceil(4) {
		points += 10;
	}
	if let Some(first) = value.chars().next() {
		if value.chars().all(|ch| ch == first) {
			points -= 100;
		}
	}

	let unprintable = value.chars().filter(|&ch| !matches!(ch, '\t' | '\r' | '\n') && (ch.is_control() || ch == '\u{fffd}')).count();
	points -= unprintable.min(10) as i32 * 100;
	if value.chars().filter(|&ch| ch == '\t').count() > 1 {
		points -= 10;
	}

	// Mostly ASCII or a dominant script adds evidence; mixed scripts subtract it.
	let ascii = value.chars().filter(char::is_ascii).count();
	let mut scripts = [0usize; 10];
	for ch in value.chars() {
		let script = match ch as u32 {
			0x00c0..=0x024f => 1, // Latin extensions
			0x0370..=0x03ff => 2, // Greek
			0x0400..=0x052f => 3, // Cyrillic
			0x0590..=0x05ff => 4, // Hebrew
			0x0600..=0x06ff => 5, // Arabic
			0x0900..=0x097f => 6, // Devanagari
			0x3040..=0x30ff | 0x3400..=0x9fff => 7, // Japanese and Han
			0xac00..=0xd7af => 8, // Hangul
			0x0e00..=0x0e7f => 9, // Thai
			_ => 0,
		};
		scripts[script] += 1;
	}

	let dominant = scripts[1..].iter().copied().max().unwrap_or(0);
	if length != 0 && ascii * 4 >= length * 3 {
		points += 35;
	}
	else if length != 0 && dominant * 5 >= length * 4 {
		points += 15;
		if length < 8 {
			points -= 60;
		}
	}
	else {
		points -= 60;
	}

	// ASCII bytes decoded in pairs often look like CJK. This is evidence about
	// the decoded text, shared by both encodings, rather than a script blacklist.
	let packed_ascii = value.chars().filter(|&ch| {
		let unit = ch as u32;
		let text_byte = |byte: u8| byte.is_ascii_graphic() || matches!(byte, b' ' | b'\t' | b'\r' | b'\n');
		(0x100..=0xffff).contains(&unit) && text_byte(unit as u8) && text_byte((unit >> 8) as u8)
	}).count();
	if packed_ascii >= 3 && packed_ascii * 3 >= length * 2 {
		points -= 100;
	}

	// Float/offset tables frequently contain non-ASCII units with a zero low byte.
	let zero_low_byte = value.chars().filter(|&ch| (0x100..=0xffff).contains(&(ch as u32)) && ch as u32 & 0xff == 0).count();
	if length >= 4 && zero_low_byte * 4 >= length {
		points -= 100;
	}

	// Repeated table patterns can contain more than one distinct character.
	if length >= 8 && ascii * 4 < length * 3 {
		let mut frequencies = std::collections::BTreeMap::new();
		for ch in value.chars() { *frequencies.entry(ch).or_insert(0usize) += 1; }
		let mut counts = frequencies.into_values().collect::<Vec<_>>();
		counts.sort_unstable_by(|a, b| b.cmp(a));
		if counts.iter().take(2).sum::<usize>() * 3 >= length * 2 {
			points -= 100;
		}
	}
	if value.chars().all(char::is_whitespace) {
		points -= 80;
	}

	// Alternating halfwords in a float table often decode as rare CJK characters.
	// Look for the exponent-halfword pattern, rather than penalizing that script.
	let mut halves = [(0usize, 0usize); 2];
	for (index, ch) in value.chars().enumerate() {
		let (count, exponents) = &mut halves[index % 2];
		*count += 1;
		*exponents += usize::from((0x3800..=0x47ff).contains(&(ch as u32)));
	}
	if halves.iter().any(|&(count, exponents)| count >= 4 && exponents * 4 >= count * 3) {
		points -= 100;
	}

	let invalid_unicode = value.chars().filter(|&ch| matches!(ch as u32,
		0xe000..=0xf8ff | 0xf0000..=0xffffd | 0x100000..=0x10fffd | 0xfdd0..=0xfdef
	) || ch as u32 & 0xfffe == 0xfffe).count();
	points -= invalid_unicode.min(10) as i32 * 100;

	points
}

#[test]
fn utf8_strings_require_valid_encoding_and_termination() {
	for bytes in [b"\0".as_slice(), b"A\0", b"ab\0", b"abcdefg", b"hello\xffworld\0", b"hello\xc3\0"] {
		assert!(utf8z_string(bytes).is_none(), "{bytes:?}");
	}
	assert_eq!(utf8z_string(b"hello world\0ignored"), Some("hello world"));
}

#[test]
fn unicode_string_heuristics() {
	fn wide(value: &str) -> Vec<u8> {
		value.encode_utf16().flat_map(u16::to_le_bytes).collect()
	}
	for value in ["abc", "hello world!", "Привет мир", "日本語の文字列です", "hello 😀 world", "hello\nworld", "line one\r\nline two"] {
		let bytes = wide(&format!("{value}\0ignored"));
		assert_eq!(utf16lez_string(&bytes).as_deref(), Some(value));
		assert_eq!(utf8z_string(format!("{value}\0ignored").as_bytes()), Some(value));
	}
	for value in ["", "ab", "aaaaaaaa", "hello\u{fffd}world", "αЖ中한αЖ中한"] {
		assert!(utf16lez_string(&wide(&format!("{value}\0"))).is_none(), "{value:?}");
		assert!(utf8z_string(format!("{value}\0").as_bytes()).is_none(), "{value:?}");
	}
	assert!(utf16lez_string(&wide("unterminated")).is_none());
	assert!(utf16lez_string(&[b'a', 0, b'b', 0, b'c', 0, 0]).is_none());
	for invalid in [0xd800u16, 0xdc00] {
		let mut bytes = wide("hello");
		bytes.extend_from_slice(&invalid.to_le_bytes());
		bytes.extend_from_slice(&[0, 0]);
		assert!(utf16lez_string(&bytes).is_none());
	}
}

#[test]
fn text_score_weighs_evidence_and_penalties() {
	assert!(score_text("猫犬鳥") < MIN_STRING_SCORE);
	assert!(score_text("αЖ中한αЖ中한") < MIN_STRING_SCORE);
	assert!(score_text("hello\nworld") >= MIN_STRING_SCORE);
	assert!(score_text("abcdefgh") > score_text("aaaaaaaa"));
	assert!(score_text("abc") > score_text("ab"));
	assert!(score_text("") < MIN_STRING_SCORE);
}

#[test]
fn corpus_regressions_for_logs_and_false_wide_text() {
	for text in ["LoadNavMesh: bad buffer alignment.\n", "GC Warning: Finalization cycle involving %p\n", "stack backtrace:\n"] {
		assert!(score_text(text) >= MIN_STRING_SCORE, "{text:?}");
	}
	for text in ["晦晦晦㿮", "耀䑵耀䑸", "䀀䔉倀䔋耀䔌က䔗", "괏긏꼏놰", "馚㽙䞮㾡馚香香㾹⛩䀑率䀞", "肀肀肀肀肀肀肀肀肀肀時￷", "\n       "] {
		assert!(score_text(text) < MIN_STRING_SCORE, "{text:?}");
	}
	let packed = String::from_utf16(&b"GC Warning: test\n".chunks_exact(2).map(|b| u16::from_le_bytes([b[0], b[1]])).collect::<Vec<_>>()).unwrap();
	assert!(score_text(&packed) < MIN_STRING_SCORE);
	for text in ["日本語の文字列です", "Привет мир", "中文字符串测试内容", "Αυτό είναι κείμενο"] {
		assert!(score_text(text) >= MIN_STRING_SCORE, "{text:?}");
	}
}
