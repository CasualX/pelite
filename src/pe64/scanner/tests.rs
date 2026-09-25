use super::*;

const OPTS: crate::pattern::ParseOptions = crate::pattern::ParseOptions::DEFAULT;

// Test the core scanner engine
#[test]
fn exec_goto() {
	use crate::pattern::Atom::*;

	// Skip a failing atom without consuming bytes.
	let bytes = [0xafu8, 0, 0xbb];
	let pat = [Goto(1), Byte(0), Fuzzy(0xf0), Byte(0xa0), Skip(1), Byte(0xbb)];
	assert!(Exec { pe: &bytes[..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut []));

	// Extend counts atoms and must be consumed before the subsequent byte skip.
	let mut pat = vec![Extend(1), Goto(0)];
	pat.extend_from_slice(&[Byte(0); 256]);
	pat.extend_from_slice(&[Goto(0), Byte(0xaf), Skip(1), Byte(0xbb)]);
	assert!(Exec { pe: &bytes[..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut []));

	let pat = [Extend(1), Extend(2), Skip(3), Save(0)];
	let mut save = [0];
	assert!(Exec { pe: &[][..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut save));
	assert_eq!(save[0], 0x010203);
}

#[test]
fn exec_test_values() {
	use crate::pattern::{Atom::*, save_len};

	let cases: &[(_, &[u8], u32)] = &[
		(TestI8(0), &[0x80], 0xffff_ff80),
		(TestU8(0), &[0x80], 0x80),
		(TestI16(0), &[0x01, 0x80], 0xffff_8001),
		(TestU16(0), &[0x01, 0x80], 0x8001),
		(TestU32(0), &[0x78, 0x56, 0x34, 0x92], 0x9234_5678),
	];
	for &(test, bytes, expected) in cases {
		assert_eq!(save_len(&[test]), 1);
		let pat = [test, Save(1)];
		let mut save = [expected, 0];
		assert!(Exec { pe: bytes, pat: &pat, cursor: 0, pc: 0 }.exec(&mut save));
		assert_eq!(save, [expected, bytes.len() as u32]);

		// Compare all 32 bits, including the extension, and leave the slot untouched.
		let mut save = [expected ^ 0x8000_0000, 123];
		assert!(!Exec { pe: bytes, pat: &pat, cursor: 0, pc: 0 }.exec(&mut save));
		assert_eq!(save, [expected ^ 0x8000_0000, 123]);

		let short = &bytes[..bytes.len() - 1];
		assert!(!Exec { pe: short, pat: &pat, cursor: 0, pc: 0 }.exec(&mut [expected, 0]));
		assert!(!Exec { pe: short, pat: &pat, cursor: 0, pc: 0 }.exec(&mut []));
		assert!(!Exec { pe: bytes, pat: &pat, cursor: 0, pc: 0 }.exec(&mut []));
	}
}

#[test]
fn exec_seek() {
	use crate::pattern::Atom::*;

	// Read an RVA, visit it, then resume immediately after the RVA field.
	let bytes = [6u8, 0, 0, 0, 0xaa, 0, 0xbb];
	let pat = [ReadU32(0), Save(-1), Seek(0), Byte(0xbb), Seek(-1), Byte(0xaa)];
	let mut save = [0; 2];
	assert!(Exec { pe: &bytes[..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut save));
	assert_eq!(save, [6, 4]);

	// Seeking does not read or validate the destination; the subsequent read fails.
	let pat = [Seek(0), Save(1)];
	let mut save = [100, 0];
	assert!(Exec { pe: &bytes[..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut save));
	assert_eq!(save[1], 100);
	let pat = [Seek(0), Byte(0)];
	assert!(!Exec { pe: &bytes[..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut save));

	let pat = [Seek(0), Byte(6)];
	assert!(Exec { pe: &bytes[..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut []));
}

#[test]
fn exec_tests_parse_docs() {
	use crate::pattern::{Atom, parse};

	fn exec(bytes: &[u8], pat: &[Atom], save: &mut [Rva]) -> bool {
		Exec { pe: bytes, pat, cursor: 0, pc: 0 }.exec(save)
	}

	{
		let bytes = [0x55, 0x89, 0xe5, 0x83, 0xff, 0xec];
		let pat = parse("55 89 e5 83 ? ec", OPTS).unwrap();
		assert!(exec(&bytes, &pat, &mut []));
	}
	{
		let bytes = [0xb9, 0x37, 0x13, 0x00, 0x00];
		let pat = parse("b9 '37 13 00 00", OPTS).unwrap();
		let mut save = [0; 2];
		assert!(exec(&bytes, &pat, &mut save));
		assert_eq!(save[1], 1);
	}
	{
		let mut bytes = [0; 64];
		bytes[0] = 0xb8;
		bytes[17] = 0x50;
		bytes[41] = 0xff;
		let pat = parse("b8 skip(16) 50 skip(13) scan(28) 'ff", OPTS).unwrap();
		let mut save = [0; 2];
		assert!(exec(&bytes, &pat, &mut save));
		assert_eq!(save[1], 41);
	}
	{
		let bytes = [0x31, 0xc0, 0x74, (-3i8) as u8];
		let pat = parse("31 c0 74 % 'c0", OPTS).unwrap();
		let mut save = [0; 2];
		assert!(exec(&bytes, &pat, &mut save));
		assert_eq!(save[1], 1);
	}
	{
		let bytes = [0xe8, 10, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0x31, 0xc0, 0xc3];
		let pat = parse("e8 $ '31 c0 c3", OPTS).unwrap();
		let mut save = [0; 2];
		assert!(exec(&bytes, &pat, &mut save));
		assert_eq!(save[1], 15);
	}
	{
		let bytes = [0x68, 10, 0, 0, 0, 1, 2, 3, 4, 5, 0x31, 0xc0, 0xc3];
		let pat = parse("68 * '31 c0 c3", OPTS).unwrap();
		let mut save = [0; 2];
		assert!(exec(&bytes, &pat, &mut save));
		assert_eq!(save[1], 10);
	}
	{
		let bytes = b"\xb8\x0a\x00\x00\x00\x01\x02\x03\x04\x05STRING\x00";
		let pat = parse(r#"b8 * "STRING" 00"#, OPTS).unwrap();
		assert!(exec(bytes, &pat, &mut []));
	}
	{
		let bytes = [0xe8, 10, 0, 0, 0, 0x83, 0xf0, 0x5c, 0xc3, 5, 6, 7, 8, 9, 10];
		let pat = parse("e8 ${'} 83 f0 5c c3", OPTS).unwrap();
		let mut save = vec![0; crate::pattern::save_len(&pat)];
		assert!(exec(&bytes, &pat, &mut save));
		assert_eq!(save[1], 15);
	}
	{
		let bytes = [0xe8, 0xff, 0xa0, 0x78, 0x56, 0x34, 0x12];
		let pat = parse("e8 i1 a0 u4", OPTS).unwrap();
		let mut save = [0; 3];
		assert!(exec(&bytes, &pat, &mut save));
		assert_eq!(save[1], (-1i8) as u32);
		assert_eq!(save[2], 0x12345678);
	}
	{
		let bytes1 = [0x83, 0xc0, 0x2a, 0x6a, 0x00, 0xe8];
		let bytes2 = [0x83, 0xc0, 0x2a, 0x68, 0x00, 0x00, 0x00, 0x10, 0xe8];
		let pat = parse("83 c0 2a ( 6a ? | 68 ? ? ? ? ) e8", OPTS).unwrap();
		assert!(exec(&bytes1, &pat, &mut []));
		assert!(exec(&bytes2, &pat, &mut []));
	}
}

#[test]
fn exec_signed_slots() {
	use crate::pattern::Atom::*;

	// Front and back addressing remain separate even with a large caller-owned buffer.
	for len in [129, 512] {
		let mut save = vec![99; len];
		let pat = [Save(127), Save(-128), Skip(1), Save(-1), Seek(-128), Check(-128), Byte(0xaa), Check(-1), Zero(-1)];
		assert!(Exec { pe: &b"\xaa"[..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut save));
		assert_eq!(save[127], 0);
		assert_eq!(save[len - 128], 0);
		assert_eq!(save[len - 1], 0);
		assert_eq!(save[0], 99);
	}

	// Negative Check operands must actually compare, rather than be ignored.
	assert!(!Exec { pe: &b""[..], pat: &[Check(-1)], cursor: 0, pc: 0 }.exec(&mut [1]));

	// PIR uses the addressed slot as its base, including a signed displacement.
	let mut save = [99, 5];
	let pat = [Pir(-1), Byte(0xaa)];
	assert!(Exec { pe: &b"\xff\xff\xff\xff\xaa"[..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut save));

	for len in [0, 127] {
		let mut save = vec![99; len];
		// Missing writes are ignored; Seek and Check read zero, including i8::MIN.
		let pat = [Save(-128), Seek(-128), Check(-128), Zero(-128), Byte(0xaa)];
		assert!(Exec { pe: &b"\xaa"[..], pat: &pat, cursor: 1, pc: 0 }.exec(&mut save));
		assert!(save.iter().all(|&value| value == 99));
		assert!(!Exec { pe: &b""[..], pat: &[Check(-128)], cursor: 1, pc: 0 }.exec(&mut save));
		// Missing PIR slots use base zero even when the current cursor is nonzero.
		let pat = [Pir(-128), Byte(0xaa)];
		assert!(Exec { pe: &b"\xff\x05\0\0\0\xaa"[..], pat: &pat, cursor: 1, pc: 0 }.exec(&mut save));
	}
}

#[test]
fn exec_signed_read_slots() {
	use crate::pattern::Atom::{self, *};

	let cases: &[(fn(i8) -> Atom, fn(i8) -> Atom, &[u8], u32)] = &[
		(ReadI8, TestI8, b"\x80", 0xffff_ff80),
		(ReadU8, TestU8, b"\x80", 0x80),
		(ReadI16, TestI16, b"\x01\x80", 0xffff_8001),
		(ReadU16, TestU16, b"\x01\x80", 0x8001),
		(ReadU32, TestU32, b"\x78\x56\x34\x92", 0x9234_5678),
	];
	for &(read, test, bytes, expected) in cases {
		for (slot, len) in [(127, 128), (-1, 1), (-2, 8), (-128, 128), (-128, 512)] {
			let mut save = vec![0; len];
			let index = if slot < 0 { (len as i32 + slot as i32) as usize } else { slot as usize };
			let pat = [read(slot), Rewind(bytes.len() as u8), test(slot)];
			assert!(Exec { pe: bytes, pat: &pat, cursor: 0, pc: 0 }.exec(&mut save));
			assert_eq!(save[index], expected);
			assert!(save.iter().enumerate().all(|(i, &value)| i == index || value == 0));
			save[index] ^= 1;
			assert!(!Exec { pe: bytes, pat: &[test(slot)], cursor: 0, pc: 0 }.exec(&mut save));
		}
		for (slot, len) in [(-1, 0), (-128, 127), (127, 127)] {
			let mut save = vec![0; len];
			let pat = [read(slot), Rewind(bytes.len() as u8), test(slot)];
			// Ignored writes leave missing slots reading zero, so nonzero comparisons fail.
			assert!(!Exec { pe: bytes, pat: &pat, cursor: 0, pc: 0 }.exec(&mut save));
			let zeros = [0u8; 4];
			let mut exec = Exec { pe: &zeros[..bytes.len()], pat: &pat, cursor: 0, pc: 0 };
			assert!(exec.exec(&mut save));
			assert_eq!(exec.cursor, bytes.len() as u32);
			assert!(save.iter().all(|&value| value == 0));
			// An unavailable slot does not excuse an unreadable source.
			let short = &bytes[..bytes.len() - 1];
			assert!(!Exec { pe: short, pat: &pat, cursor: 0, pc: 0 }.exec(&mut save));
			assert!(!Exec { pe: short, pat: &[test(slot)], cursor: 0, pc: 0 }.exec(&mut save));
		}
	}
}

#[test]
fn exec_explicit_returns() {
	use crate::pattern::{Atom::*, save_len};

	let mut bytes = [0; 32];
	bytes[0] = 4; // First reference: 0 + 1 + 4 = 5.
	bytes[5] = 4; // Nested reference: 5 + 1 + 4 = 10.
	bytes[10] = 0xaa;
	bytes[6] = 0xbb;
	bytes[1] = 0xcc;
	let pat = crate::pattern!("%{%{aa}bb}cc");
	let mut save = vec![0; save_len(pat)];
	assert!(Exec { pe: &bytes[..], pat, cursor: 0, pc: 0 }.exec(&mut save));
	bytes[6] = 0;
	assert!(!Exec { pe: &bytes[..], pat, cursor: 0, pc: 0 }.exec(&mut save));

	// The pointer-width skip follows the scanned architecture, including on PE32.
	bytes[..mem::size_of::<Va>()].copy_from_slice(&(24 as Va).to_le_bytes());
	bytes[mem::size_of::<Va>()] = 0xcc;
	bytes[24] = 0xaa;
	let pat = crate::pattern!("*{aa}cc");
	let mut save = vec![0; save_len(pat)];
	assert!(Exec { pe: &bytes[..], pat, cursor: 0, pc: 0 }.exec(&mut save));

	// A primitive return retains the extension in its current frame.
	let pat = [Save(-1), Extend(1), Seek(-1), Skip(0), Fuzzy(0xf0), Byte(0xa0)];
	let mut bytes = [0; 257];
	bytes[256] = 0xaf;
	assert!(Exec { pe: &bytes[..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut [0]));
}

#[test]
fn exec_scan_across_returns() {
	use crate::pattern::{Atom::*, parse, save_len};

	let mut bytes = [0; 21];
	bytes[0] = 9; // First reference goes to the search at 10.
	bytes[1] = 18; // Second reference goes to 20.
	bytes[2] = 9; // The final comparison rejects the first search candidate.
	bytes[10] = 7;
	bytes[11] = 9;
	bytes[20] = 0xaa;
	let mut pat = parse("%{scan(1)u1}%{aa}00", OPTS).unwrap();
	*pat.last_mut().unwrap() = TestU8(1);
	let mut save = vec![0; save_len(&pat)];
	assert!(Exec { pe: &bytes[..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut save));
	assert_eq!(save[1], 9);
	// The later scope must not overwrite the first scope's return slot during retries.
	assert_eq!(save_read(&save, -1), 0);
	assert_eq!(save_read(&save, -2), 1);
	bytes[2] = 8;
	assert!(!Exec { pe: &bytes[..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut save));
}

#[test]
fn exec_fork_state() {
	use crate::pattern::Atom::*;

	// The primary path can match a masked byte and jump past a failing fallback.
	let pat = [Fork(3), Fuzzy(0xf0), Byte(0xa0), Goto(1), Byte(0)];
	assert!(Exec { pe: &b"\xaf"[..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut []));

	// Retry restores the cursor; each alternative supplies its own byte mask.
	let pat = [Fork(3), Fuzzy(0xf0), Byte(0xa0), Byte(0), Fuzzy(0xf0), Byte(0xa0)];
	assert!(Exec { pe: &b"\xaf"[..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut []));

	// Writes are deliberately retained when retrying from the fork's original cursor.
	let pat = [Fork(2), ReadU8(-1), Byte(0), TestU8(-1)];
	let mut save = [0];
	assert!(Exec { pe: &b"\xaf"[..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut save));
	assert_eq!(save, [0xaf]);

	// Extended fork offsets count atoms and are consumed before the retry path.
	let mut pat = vec![Extend(1), Fork(2), Byte(0)];
	pat.resize(260, Nop);
	pat.extend_from_slice(&[Skip(1), Byte(0xab)]);
	assert!(Exec { pe: &b"\xcd\xab"[..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut []));

	// The extension is also consumed before the primary path. Its Goto skips the fallback.
	let mut pat = vec![Extend(1), Fork(0), Skip(1), Byte(0xab), Goto(254)];
	pat.resize(258, Nop);
	pat.push(Byte(0xff));
	assert!(Exec { pe: &b"\xcd\xab"[..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut []));
}

#[test]
fn exec_alternatives() {
	use crate::pattern::{parse, save_len};

	// Each constant expansion is checked against runtime compilation as well as execution.
	macro_rules! cases {
		($(($source:expr, $bytes:expr, $matched:expr)),* $(,)?) => {
			&[$(($source, crate::pattern!($source), &$bytes[..], $matched)),*]
		};
	}
	let cases: &[(&str, &[pat::Atom], &[u8], bool)] = cases![
		("(61|6162)63", b"abc", true),
		("(61|6162)63", b"ac", true),
		("(61|6162)63", b"abd", false),
		("((61|6162)|64)63", b"abc", true),
		("((61|6162)|64)63", b"dc", true),
		("(61|6162)(63|6364)65", b"abcde", true),
		("(|61)62", b"ab", true),
		("(61|)62", b"b", true),
		("()61", b"a", true),
		("((|)|61)62", b"ab", true),
		("(61|61)62", b"ac", false),
		("scan(1)(61|6162)63", b"xabc", true),
		("scan(1)(61|6162)63", b"xxabc", false),
		("(scan(1)61|6263)64", b"bcd", true),
	];
	for &(source, pat, bytes, matched) in cases {
		assert_eq!(parse(source, OPTS).unwrap(), pat, "{source}");
		let mut save = vec![0; save_len(pat)];
		assert_eq!(Exec { pe: bytes, pat, cursor: 0, pc: 0 }.exec(&mut save), matched, "{source}");
	}

	// Exhaust alternatives at the nearest scan candidate before trying a later candidate.
	let pat = crate::pattern!("scan(4)'(61|6162)63");
	let mut save = [0; 2];
	let mut exec = Exec { pe: &b"abcac"[..], pat, cursor: 0, pc: 0 };
	assert!(exec.exec(&mut save));
	assert_eq!(save[1], 0);
	assert_eq!(exec.cursor, 3);
}

#[test]
fn exec_fork_across_returns() {
	use crate::pattern::{Atom::*, parse, save_len};

	let mut bytes = [0; 11];
	bytes[0] = 4; // First scope visits the alternatives at 5.
	bytes[1] = 8; // Second scope visits 10.
	bytes[2] = b'c'; // Rejects the first alternative after both scopes return.
	bytes[5..8].copy_from_slice(b"abc");
	bytes[10] = 0xaa;
	let mut pat = parse("%{(61|6162)u1}%{aa}00", OPTS).unwrap();
	*pat.last_mut().unwrap() = TestU8(1);
	let mut save = vec![0; save_len(&pat)];
	assert!(Exec { pe: &bytes[..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut save));
	assert_eq!(save[1], b'c' as u32);
	assert_eq!(save_read(&save, -1), 0);
	assert_eq!(save_read(&save, -2), 1);
	bytes[2] = b'd';
	assert!(!Exec { pe: &bytes[..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut save));
}

#[test]
fn exec_scan_preserves_failed_writes() {
	use crate::pattern::Atom::*;

	// Both scan candidates fail, but the final Save(0) must remain visible to the
	// fork's fallback. Adding a Nop must not change the result or scratch contents.
	let patterns: &[&[pat::Atom]] = &[
		&[Zero(0), Fork(4), Scan(2), Save(0), Byte(0xff), Goto(1), TestU8(0)],
		&[Zero(0), Fork(5), Scan(2), Save(0), Nop, Byte(0xff), Goto(1), TestU8(0)],
	];
	for &pat in patterns {
		let mut save = [99];
		assert!(Exec { pe: &b"\x01\x02"[..], pat, cursor: 0, pc: 0 }.exec(&mut save));
		assert_eq!(save, [1]);
	}
}

#[test]
fn exec_pattern_language() {
	use crate::pattern::{parse, parse_const, parse_len, save_len};
	const OPTIONS: pat::ParseOptions = pat::ParseOptions { legacy_gap: true };
	macro_rules! cases {
		($(($source:expr, $bytes:expr, $matched:expr)),* $(,)?) => {
			&[$(($source, &const { parse_const::<{ parse_len($source, OPTIONS) }>($source, OPTIONS) } as &[pat::Atom], &$bytes[..], $matched)),*]
		};
	}
	let cases: &[(&str, &[pat::Atom], &[u8], bool)] = cases![
		("(61|6162)63", b"abc", true),
		("(61|6162)63", b"ac", true),
		("(61|6162)63", b"abd", false),
		("((61|6162)|64)63", b"abc", true),
		("(61|6162)(63|6364)65", b"abcde", true),
		("(00|11|61)62", b"ab", true),
		("(00|61|11)62", b"ab", true),
		("(|61)62", b"ab", true),
		("(61|)62", b"b", true),
		("()61", b"a", true),
		("scan(1)(61|6162)63", b"xabc", true),
		("scan(1)(61|6162)63", b"xxabc", false),
		("skip(1)scan(2)61", b"xxxa", true),
		("[2]61", b"xxa", true),
		("[2]61", b"xa", false),
		("[1-3]61", b"xxa", true),
		("[1-3]61", b"xxxa", false),
		("scan()", b"", true),
		("scan()", b"a", true),
		("skip(0)", b"", true),
		("?", b"", true),
		("A0/F8", b"\xa7", true),
		("A0/F8", b"\xa8", false),
		("u1[1] =u1[1]", b"aa", true),
		("u1[1] =u1[1]", b"ab", false),
		("i1[1] =i2[1]", b"\xff\xff\xff", true),
		("u1[1] =i2[1]", b"\xff\xff\xff", false),
		("61 save[1] 62 seek[1] check[1] 62 skip(-1)62", b"ab", true),
	];
	for &(source, pat, bytes, matched) in cases {
		assert_eq!(parse(source, OPTIONS).unwrap(), pat, "{source}");
		let mut save = vec![0; save_len(pat)];
		assert_eq!(Exec { pe: bytes, pat, cursor: 0, pc: 0 }.exec(&mut save), matched, "{source}");
	}

	// Both the extended fork and the extended goto must land on the continuation.
	let source = alloc::format!("(61{}|62{})63", "?".repeat(300), "?".repeat(300));
	let pat = parse(&source, OPTS).unwrap();
	let mut bytes = [0; 302];
	bytes[301] = b'c';
	for byte in [b'a', b'b'] {
		bytes[0] = byte;
		assert!(Exec { pe: &bytes[..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut [0]));
	}

	// Retry a choice inside a reference body after two returns and a failed test.
	let source = "%{(61|6162)u1[1]}%{AA}=u1[1]";
	let pat = parse(source, OPTS).unwrap();
	let mut bytes = [4, 8, b'c', 0, 0, b'a', b'b', b'c', 0, 0, 0xaa];
	let mut save = vec![0; save_len(&pat)];
	assert!(Exec { pe: &bytes[..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut save));
	assert_eq!(save[1], b'c' as u32);
	bytes[2] = b'd';
	assert!(!Exec { pe: &bytes[..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut save));
}

use crate::pattern::Atom::*;

const EXECUTE: u32 = IMAGE_SCN_MEM_EXECUTE;
const CODE: u32 = IMAGE_SCN_CNT_CODE;
const HEADER_SIZE: usize = 0x400;

fn put16(bytes: &mut [u8], offset: usize, value: u16) {
	bytes[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}
fn put32(bytes: &mut [u8], offset: usize, value: u32) {
	bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}
fn section_offset(index: usize) -> usize {
	0x98 + mem::size_of::<IMAGE_OPTIONAL_HEADER>() + index * 40
}

// Each entry is (RVA, virtual size, raw size, characteristics).
fn fixture(sections: &[(u32, u32, u32, u32)]) -> Vec<u8> {
	let mut bytes = vec![0; HEADER_SIZE + sections.len() * 0x200];
	put16(&mut bytes, 0, 0x5a4d);
	put32(&mut bytes, 0x3c, 0x80);
	put32(&mut bytes, 0x80, 0x4550);
	put16(&mut bytes, 0x86, sections.len() as u16);
	put16(&mut bytes, 0x94, mem::size_of::<IMAGE_OPTIONAL_HEADER>() as u16);
	put16(&mut bytes, 0x98, IMAGE_NT_OPTIONAL_HDR_MAGIC);
	// Deliberately leave BaseOfCode and SizeOfCode zero.
	put32(&mut bytes, 0xb8, 0x1000);
	put32(&mut bytes, 0xbc, 0x200);
	put32(&mut bytes, 0xd0, 0x8000);
	put32(&mut bytes, 0xd4, HEADER_SIZE as u32);
	for (i, &(rva, virtual_size, raw_size, flags)) in sections.iter().enumerate() {
		let offset = section_offset(i);
		put32(&mut bytes, offset + 8, virtual_size);
		put32(&mut bytes, offset + 12, rva);
		put32(&mut bytes, offset + 16, raw_size);
		put32(&mut bytes, offset + 20, (HEADER_SIZE + i * 0x200) as u32);
		put32(&mut bytes, offset + 36, flags);
	}
	bytes
}

fn mapped(bytes: &[u8]) -> Vec<u8> {
	let pe = PeFile::from_bytes(bytes).unwrap();
	let mut result = vec![0; 0x8000];
	result[..HEADER_SIZE].copy_from_slice(&bytes[..HEADER_SIZE]);
	for section in pe.section_headers() {
		let size = cmp::min(section.VirtualSize, section.SizeOfRawData) as usize;
		let raw = section.PointerToRawData as usize;
		let rva = section.VirtualAddress as usize;
		result[rva..rva + size].copy_from_slice(&bytes[raw..raw + size]);
	}
	result
}

fn collect<'a, P: Copy + Pe<'a>, F: FnMut(&SectionHeader) -> bool>(selection: ScanSections<'a, P, F>, pat: &[pat::Atom]) -> Result<Vec<Rva>> {
	let mut matches = selection.matches(pat);
	let mut save = vec![0; pat::save_len(pat)];
	let mut rvas = Vec::new();
	while let Some(rva) = matches.next(&mut save) {
		rvas.push(rva);
	}
	assert_eq!(matches.next(&mut save), None);
	Ok(rvas)
}

#[test]
fn sections_are_independent_and_uniqueness_is_global() {
	let mut bytes = fixture(&[(0x3000, 0x200, 0x200, EXECUTE), (0x1000, 0x200, 0x200, EXECUTE), (0x5000, 0x200, 0x200, CODE)]);
	bytes[0x400] = 0xaa;
	bytes[0x600] = 0xaa;
	bytes[0x800] = 0xaa;
	fn check<'a>(pe: impl Copy + Pe<'a>) {
		let scanner = pe.scanner();
		assert_eq!(collect(scanner.code(), &[Byte(0xaa)]).unwrap(), [0x3000, 0x1000]);
		assert_eq!(scanner.code().find(&[Byte(0xaa)], &mut []), None);
		assert_eq!(scanner.section(&pe.section_headers().as_slice()[1]).find(&[Byte(0xaa)], &mut []), Some(0x1000));
		assert_eq!(scanner.within(0x1000..0x1200).find(&[Byte(0xaa)], &mut []), Some(0x1000));
		assert_eq!(collect(scanner.sections(|s| s.Characteristics & CODE != 0), &[Byte(0xaa)]).unwrap(), [0x5000]);
		assert_eq!(collect(scanner.sections(|_| true), &[Byte(0xaa)]).unwrap(), [0x3000, 0x1000, 0x5000]);
		assert_eq!(scanner.sections(|_| true).find(&[Byte(0xbb)], &mut []), None);
		let mut calls = 0;
		assert_eq!(collect(scanner.sections(|_| { calls += 1; true }), &[Byte(0xaa)]).unwrap().len(), 3);
		assert_eq!(calls, 3);
	}
	check(PeFile::from_bytes(&bytes).unwrap());
	check(PeView::from_bytes(&mapped(&bytes)).unwrap());
	// Nonzero aggregate header values still do not define the search.
	put32(&mut bytes, 0x9c, 0x400);
	put32(&mut bytes, 0xac, 0x1000);
	check(PeFile::from_bytes(&bytes).unwrap());
}

#[test]
fn prefix_optimization_respects_candidate_bounds_and_overlapping_matches() {
	let mut bytes = fixture(&[(0x1000, 0x200, 0x200, EXECUTE)]);
	bytes[0x400..0x40a].copy_from_slice(b"abcdababab");
	fn check<'a>(pe: impl Copy + Pe<'a>) {
		let scanner = pe.scanner();
		let fast = [Byte(b'a'), Byte(b'b'), Byte(b'c'), Byte(b'd')];
		let slow = [Byte(b'a'), Fuzzy(0xff), Byte(b'b'), Byte(b'c'), Byte(b'd')];
		for pat in [&fast[..], &slow[..]] {
			assert_eq!(collect(scanner.within(0x1000..0x1001), pat).unwrap(), [0x1000]);
			for range in [0x1000..0x1000, 0x1001..0x1000, 0..0x1000, 0x1200..0x1300, 0x8000..0x9000] {
				assert!(collect(scanner.within(range), pat).unwrap().is_empty());
			}
		}
		assert_eq!(collect(scanner.sections(|_| true), &[Byte(b'a'), Byte(b'b'), Byte(b'a'), Byte(b'b')]).unwrap(), [0x1004, 0x1006]);
		assert!(collect(scanner.within(0x1000..0x1001).within(0x1001..0x1002), &[]).unwrap().is_empty());
		assert_eq!(collect(scanner.within(0x1000..0x1002), &[]).unwrap(), [0x1000, 0x1001]);
	}
	check(PeFile::from_bytes(&bytes).unwrap());
	check(PeView::from_bytes(&mapped(&bytes)).unwrap());
}

#[test]
fn prefix_can_cross_a_backing_section_boundary() {
	let mut bytes = fixture(&[(0x1000, 2, 2, EXECUTE), (0x1002, 2, 2, 0)]);
	bytes[0x400..0x402].copy_from_slice(b"ab");
	bytes[0x600..0x602].copy_from_slice(b"cd");
	let pat = [Byte(b'a'), Byte(b'b'), Byte(b'c'), Byte(b'd')];
	assert_eq!(collect(PeFile::from_bytes(&bytes).unwrap().scanner().code(), &pat).unwrap(), [0x1000]);
	assert_eq!(collect(PeView::from_bytes(&mapped(&bytes)).unwrap().scanner().code(), &pat).unwrap(), [0x1000]);
}

#[test]
fn raw_padding_and_virtual_zero_fill_have_explicit_extents() {
	let mut bytes = fixture(&[(0x1000, 0x400, 0x200, EXECUTE), (0x3000, 0x100, 0x200, EXECUTE), (0x5000, 0, 0x200, EXECUTE)]);
	bytes[0x780] = 0xaa; // Raw padding in the second section.
	bytes[0x800] = 0xaa; // Raw bytes with zero VirtualSize.
	let pe = PeFile::from_bytes(&bytes).unwrap();
	assert!(collect(pe.scanner().within(0x1300..0x1400), &[Byte(0)]).unwrap().is_empty());
	assert_eq!(collect(pe.scanner().sections(|_| true), &[Byte(0xaa)]).unwrap(), [0x3180, 0x5000]);
	let view_bytes = mapped(&bytes);
	let view = PeView::from_bytes(&view_bytes).unwrap();
	assert_eq!(collect(view.scanner().within(0x1300..0x1301), &[Byte(0)]).unwrap(), [0x1300]);
	assert!(collect(view.scanner().sections(|_| true), &[Byte(0xaa)]).unwrap().is_empty());
}

#[test]
fn references_escape_selection_but_scan_stays_in_current_section() {
	let mut bytes = fixture(&[(0x1000, 0x200, 0x200, EXECUTE), (0x3000, 0x200, 0x200, 0)]);
	// A relative reference followed by an absolute reference in the code section.
	put32(&mut bytes, 0x400, 0x3000 - 0x1004);
	bytes[0x404..0x404 + mem::size_of::<Va>()].copy_from_slice(&(0x3000 as Va).to_le_bytes());
	bytes[0x600] = 0xaa;
	fn check<'a>(pe: impl Copy + Pe<'a>) {
		let scanner = pe.scanner();
		let mut save = [0];
		assert_eq!(scanner.section(&pe.section_headers().as_slice()[0]).within(0x1000..0x1001).find(&[Jump4, Save(0), Byte(0xaa)], &mut save), Some(0x1000));
		assert_eq!(save[0], 0x3000);
		assert_eq!(scanner.section(&pe.section_headers().as_slice()[0]).within(0x1004..0x1005).find(&[Ptr, Byte(0xaa)], &mut []), Some(0x1004));
		assert!(!scanner.exec(0x1000, &[Scan(0), Byte(0xaa)], &mut []));
		assert!(scanner.exec(0x1000, &[Jump4, Scan(0), Byte(0xaa)], &mut []));
	}
	check(PeFile::from_bytes(&bytes).unwrap());
	check(PeView::from_bytes(&mapped(&bytes)).unwrap());
}

#[test]
fn malformed_sections_are_skipped() {
	let mut bytes = fixture(&[(0x1000, 0x200, 0x200, EXECUTE), (0x3000, 0x200, 0x200, EXECUTE)]);
	bytes[0x400] = 0xaa;
	bytes.truncate(0x600);
	let pe = PeFile::from_bytes(&bytes).unwrap();
	let scanner = pe.scanner();
	let mut matches = scanner.sections(|_| true).matches(&[Byte(0xaa)]);
	assert_eq!(matches.next(&mut []), Some(0x1000));
	assert_eq!(matches.next(&mut []), None);
	assert_eq!(matches.next(&mut []), None);
	assert_eq!(scanner.sections(|_| true).find(&[Byte(0xaa)], &mut []), Some(0x1000));
	assert_eq!(scanner.section(&pe.section_headers().as_slice()[0]).find(&[Byte(0xaa)], &mut []), Some(0x1000));
	// The mapped source also skips a section extending beyond its supplied bytes.
	let mut view = mapped(&fixture(&[(0x1000, 0x200, 0x200, EXECUTE), (0x3000, 0x200, 0x200, EXECUTE)]));
	view.truncate(0x3000);
	assert_eq!(collect(PeView::from_bytes(&view).unwrap().scanner().sections(|_| true), &[Byte(0xaa)]), Ok(Vec::new()));
	put32(&mut bytes, section_offset(1) + 20, u32::MAX - 1);
	assert_eq!(collect(PeFile::from_bytes(&bytes).unwrap().scanner().sections(|_| true), &[Byte(0xaa)]), Ok(vec![0x1000]));
	put32(&mut bytes, section_offset(1) + 12, u32::MAX - 1);
	assert_eq!(collect(PeFile::from_bytes(&bytes).unwrap().scanner().sections(|_| true), &[Byte(0xaa)]), Ok(vec![0x1000]));
}

#[test]
fn overlapping_rva_mappings_follow_normal_section_iteration() {
	let bytes = fixture(&[(0x1000, 0x200, 0x200, EXECUTE), (0x1100, 0x200, 0x200, 0)]);
	assert_eq!(PeFile::from_bytes(&bytes).unwrap().scanner().code().within(0x1000..0x1001).find(&[], &mut []), Some(0x1000));
	assert_eq!(PeView::from_bytes(&mapped(&bytes)).unwrap().scanner().code().within(0x1000..0x1001).find(&[], &mut []), Some(0x1000));
}

#[test]
fn uniqueness_reexecutes_the_match_to_restore_captures() {
	let mut bytes = fixture(&[(0x1000, 0x200, 0x200, EXECUTE), (0x3000, 0x200, 0x200, EXECUTE)]);
	bytes[0x400..0x402].copy_from_slice(&[0xaa, 0xbb]);
	bytes[0x600..0x602].copy_from_slice(&[0xaa, 0xcc]);
	let pe = PeFile::from_bytes(&bytes).unwrap();
	let scanner = pe.scanner();
	for len in [2, 128, 129, 255, 256, 257, 512] {
		let mut save = vec![99; len];
		let pat = [Byte(0xaa), Save(0), ReadU8(-1), Byte(0), Rewind(2), Byte(0xbb)];
		assert_eq!(scanner.sections(|_| true).find(&pat, &mut save), Some(0x1000));
		assert_eq!(save[0], 0x1001);
		assert_eq!(save[len - 1], 0xbb);
		assert!(save[1..len - 1].iter().all(|&value| value == 99));
	}
	// Re-execution fails if the pattern overwrites caller state needed to match.
	let mut save = [0xaa];
	assert_eq!(scanner.within(0x1000..0x1001).find(&[TestU8(0), ReadU8(0)], &mut save), None);
	assert_eq!(save, [0xbb]);
	// Uniqueness must retain the relative addressing/aliasing of signed save slots.
	for len in [128, 256, 512] {
		let mut save = vec![0xaa; len];
		assert_eq!(scanner.sections(|_| true).find(&[TestU8(-128)], &mut save), None);
	}
}

#[test]
fn architecture_wrapper_exposes_the_same_selection() {
	let mut bytes = fixture(&[(0x1000, 0x200, 0x200, EXECUTE)]);
	bytes[0x400] = 0xaa;
	let pe = crate::PeFile::from_bytes(&bytes).unwrap();
	let scanner = pe.scanner();
	assert_eq!(scanner.code().find(&[Byte(0xaa)], &mut []), Some(0x1000));
	assert_eq!(scanner.sections(|s| s.Characteristics & EXECUTE != 0).within(0x1000..0x1001)
		.find(&[Byte(0xaa)], &mut []), Some(0x1000));
	let mut matches = scanner.section(&pe.section_headers().as_slice()[0]).matches(&[Byte(0xaa)]);
	assert_eq!(matches.next(&mut []), Some(0x1000));
	assert_eq!(matches.next(&mut []), None);
}

#[test]
fn optimized_search_agrees_with_direct_execution() {
	let mut bytes = fixture(&[(0x1010, 16, 16, EXECUTE), (0x1000, 16, 16, EXECUTE)]);
	for i in 0..16 {
		bytes[0x400 + i] = ((i + 16) % 3) as u8;
		bytes[0x600 + i] = (i % 3) as u8;
	}
	fn check<'a>(pe: impl Copy + Pe<'a>) {
		for len in [0, 1, 2, 3, 4, 5, 15, 16, 17, 18] {
			let pat: Vec<_> = (0..len).map(|i| Byte((i % 3) as u8)).collect();
			for start in 0x1000..0x1020 {
				for end in [start + 1, 0x1020] {
					let mut expected = Vec::new();
					for section in pe.section_headers() {
						for rva in section.virtual_range() {
							if (start..end).contains(&rva) && pe.scanner().exec(rva, &pat, &mut []) {
								expected.push(rva);
							}
						}
					}
					assert_eq!(
						collect(pe.scanner().within(start..end), &pat).unwrap(),
						expected,
						"prefix length {len}, range {start:#x}..{end:#x}"
					);
				}
			}
		}
	}
	check(PeFile::from_bytes(&bytes).unwrap());
	check(PeView::from_bytes(&mapped(&bytes)).unwrap());
}
