/*!
Pattern Scanner.

See the [`pattern`](../../pattern/index.html) module for more information about patterns.

# Examples

```
# #![allow(unused_variables)]
use pelite::pe64::{Pe, PeFile};
use pelite::pattern as pat;

# #[allow(dead_code)]
fn example(file: PeFile<'_>, pat: &[pat::Atom]) {
	// Gets the pattern scanner interface
	let scanner = file.scanner();

	// Capture references in the pattern in a save array
	let mut save = [0; 8];

	// Finds a singular code match
	if scanner.finds_code(pat, &mut save) {
		println!("{:x?}", save);
	}

	// Finds all the code matches for the pattern
	let mut matches = scanner.matches_code(pat);
	while matches.next(&mut save) {
		println!("{:x?}", save);
	}
}
```
*/

use std::{cmp, mem, ptr};
use std::ops::Range;

use crate::{Pod, pattern as pat};

use super::{Align, Rva, Pe, image::*};

/// Size of the prefix buffer for search optimization.
const QS_BUF_LEN: usize = 16;

//----------------------------------------------------------------

/// Pattern scanner.
///
/// For more information see the [module-level documentation](index.html).
#[derive(Copy, Clone)]
pub struct Scanner<P> {
	pe: P,
}
impl<'a, P: Pe<'a>> Scanner<P> {
	pub(crate) fn new(pe: P) -> Scanner<P> {
		Scanner { pe }
	}
	/// Finds the unique match for the pattern in the given range.
	///
	/// The pattern may contain instructions to capture interesting addresses, these are stored in the save array.
	/// Out of bounds stores are simply ignored, ensure the save array is large enough for the given pattern.
	///
	/// In case of mismatch, ie. returns false, the save array is still overwritten with temporary data and should be considered trashed.
	/// Keep a copy, invoke with a fresh save array or reexecute the pattern at the saved cursor to get around this.
	///
	/// Returns `false` if no match is found or multiple matches are found to prevent subtle bugs where a pattern goes stale by not being unique any more.
	///
	/// Use `matches(pat, range).next(save)` if just the first match is desired.
	pub fn finds(self, pat: &[pat::Atom], range: Range<Rva>, save: &mut [Rva]) -> bool {
		let mut matches = self.matches(pat, range);
		if matches.next(save) {
			// Disallow more than one match as it indicates the signature isn't unique enough
			let cursor = matches.cursor;
			!matches.next(save) && matches.scanner.exec(cursor, pat, save)
		}
		else {
			false
		}
	}
	/// Finds the unique code match for the pattern.
	///
	/// Restricts the range to the code section. See [`finds`](#finds) for more information.
	pub fn finds_code(self, pat: &[pat::Atom], save: &mut [Rva]) -> bool {
		let optional_header = self.pe.optional_header();
		let range = optional_header.BaseOfCode..u32::wrapping_add(optional_header.BaseOfCode, optional_header.SizeOfCode);
		self.finds(pat, range, save)
	}
	/// Finds the unique match for the pattern in the specified section.
	///
	/// Restricts the range to the specified section. See [`finds`](#finds) for more information.
	pub fn finds_section(self, pat: &[pat::Atom], section: &IMAGE_SECTION_HEADER, save: &mut [Rva]) -> bool {
		let range = section.VirtualAddress..section.VirtualAddress + section.VirtualSize;
		self.finds(pat, range, save)
	}
	/// Returns an iterator over the matches of a pattern within the given range.
	pub fn matches<'u>(self, pat: &'u [pat::Atom], range: Range<Rva>) -> Matches<'u, P> {
		let cursor = range.start;
		Matches { scanner: self, pat, range, cursor, hits: 0 }
	}
	/// Returns an iterator over the code matches of a pattern.
	///
	/// Restricts the range to the code section. See [`matches`](#matches) for more information.
	pub fn matches_code<'u>(self, pat: &'u [pat::Atom]) -> Matches<'u, P> {
		let optional_header = self.pe.optional_header();
		let range = optional_header.BaseOfCode..u32::wrapping_add(optional_header.BaseOfCode, optional_header.SizeOfCode);
		self.matches(pat, range)
	}
	/// Returns an iterator over the matches of a pattern in the specified section.
	///
	/// Restricts the range to the specified section. See [`matches`](#matches) for more information.
	pub fn matches_section<'u>(self, pat: &'u [pat::Atom], section: &IMAGE_SECTION_HEADER) -> Matches<'u, P> {
		let range = section.VirtualAddress..section.VirtualAddress + section.VirtualSize;
		self.matches(pat, range)
	}
	/// Pattern interpreter, returns if the pattern matches the binary image at the given rva.
	///
	/// The pattern may contain instructions to capture interesting addresses, these are stored in the save array.
	/// Out of bounds stores are simply ignored, ensure the save array is large enough for the given pattern.
	///
	/// In case of mismatch, ie. returns false, the save array is still overwritten with temporary data and should be considered trashed.
	/// Keep a copy, invoke with a fresh save array or reexecute the pattern at the saved cursor to get around this.
	pub fn exec(self, cursor: Rva, pat: &[pat::Atom], save: &mut [Rva]) -> bool {
		Exec { pe: self.pe, pat, cursor, pc: 0 }.exec(save)
	}
}

//----------------------------------------------------------------

trait Scan<'a>: Copy {
	fn read<T: Copy + Pod>(self, rva: Rva) -> Option<T>;
	fn pointer(self, va: Va) -> Option<Rva>;
	fn slice(self, rva: Rva) -> Option<&'a [u8]>;
}

impl<'a, P: Pe<'a>> Scan<'a> for P {
	fn read<T: Copy + Pod>(self, rva: Rva) -> Option<T> {
		self.derva_copy(rva).ok()
	}
	fn pointer(self, va: Va) -> Option<Rva> {
		self.va_to_rva(va).ok()
	}
	fn slice(self, rva: Rva) -> Option<&'a [u8]> {
		self.slice_bytes(rva).ok()
	}
}

impl<'a> Scan<'a> for &'a [u8] {
	fn read<T: Copy + Pod>(self, rva: Rva) -> Option<T> {
		let bytes = self.get(rva as usize..(rva as usize + mem::size_of::<T>()))?;
		let ptr = bytes.as_ptr() as *const T;
		Some(unsafe { ptr::read_unaligned(ptr) })
	}
	fn pointer(self, va: Va) -> Option<Rva> {
		Some(va as Rva)
	}
	fn slice(self, rva: Rva) -> Option<&'a [u8]> {
		self.get(rva as usize..)
	}
}

#[derive(Clone)]
struct Exec<'u, P> {
	pe: P,
	pat: &'u [pat::Atom],
	cursor: Rva,
	pc: usize,
}
impl<'a, 'u, P: Scan<'a>> Exec<'u, P> {
	fn exec(&mut self, save: &mut [Rva]) -> bool {
		const SKIP_VA: u32 = mem::size_of::<Va>() as u32;
		let mut mask = 0xff;
		let mut ext_range = 0u32;
		while let Some(atom) = self.pat.get(self.pc).cloned() {
			self.pc += 1;
			match atom {
				pat::Atom::Byte(pat_byte) => {
					match self.pe.read::<u8>(self.cursor) {
						Some(byte) if byte & mask == pat_byte & mask => (),
						_ => return false,
					}
					mask = 0xff;
					self.cursor += 1;
				},
				pat::Atom::Save(slot) => {
					if let Some(slot) = save.get_mut(slot as usize) {
						*slot = self.cursor;
					}
				},
				pat::Atom::Push(skip) => {
					let skip = ext_range + skip as u32;
					let skip = if skip == 0 { SKIP_VA } else { skip };
					let cursor = self.cursor.wrapping_add(skip);
					if !self.exec(save) {
						return false;
					}
					mask = 0xff;
					ext_range = 0;
					self.cursor = cursor;
				},
				pat::Atom::Pop => {
					return true;
				},
				pat::Atom::Fuzzy(pat_mask) => {
					mask = pat_mask;
				},
				pat::Atom::Skip(skip) => {
					let skip = ext_range + skip as u32;
					let skip = if skip == 0 { SKIP_VA } else { skip };
					let cursor = self.cursor.wrapping_add(skip);
					ext_range = 0;
					self.cursor = cursor;
				},
				pat::Atom::Rangext(ext) => {
					ext_range = ext as u32 * 256;
				},
				pat::Atom::Many(limit) => {
					let limit = ext_range + limit as u32;
					return self.exec_many(save, limit);
				},
				pat::Atom::Jump1 => {
					if let Some(sbyte) = self.pe.read::<i8>(self.cursor) {
						self.cursor = self.cursor.wrapping_add(sbyte as Rva).wrapping_add(1);
					}
					else {
						return false;
					}
				},
				pat::Atom::Jump4 => {
					if let Some(sdword) = self.pe.read::<i32>(self.cursor) {
						self.cursor = self.cursor.wrapping_add(sdword as Rva).wrapping_add(4);
					}
					else {
						return false;
					}
				},
				pat::Atom::Ptr => {
					if let Some(rva) = self.pe.read::<Va>(self.cursor).and_then(|va| self.pe.pointer(va)) {
						self.cursor = rva;
					}
					else {
						return false;
					}
				},
				pat::Atom::Pir(slot) => {
					if let Some(sdword) = self.pe.read::<i32>(self.cursor) {
						let base = save.get(slot as usize).cloned().unwrap_or(self.cursor);
						self.cursor = base.wrapping_add(sdword as Rva);
					}
					else {
						return false;
					}
				},
				pat::Atom::ReadU8(slot) => {
					if let Some(byte) = self.pe.read::<u8>(self.cursor) {
						if let Some(slot) = save.get_mut(slot as usize) {
							*slot = byte as Rva;
						}
						self.cursor = self.cursor.wrapping_add(1);
					}
					else {
						return false;
					}
				},
				pat::Atom::ReadI8(slot) => {
					if let Some(sbyte) = self.pe.read::<i8>(self.cursor) {
						if let Some(slot) = save.get_mut(slot as usize) {
							*slot = sbyte as Rva;
						}
						self.cursor = self.cursor.wrapping_add(1);
					}
					else {
						return false;
					}
				},
				pat::Atom::ReadU16(slot) => {
					if let Some(word) = self.pe.read::<u16>(self.cursor) {
						if let Some(slot) = save.get_mut(slot as usize) {
							*slot = word as Rva;
						}
						self.cursor = self.cursor.wrapping_add(2);
					}
					else {
						return false;
					}
				},
				pat::Atom::ReadI16(slot) => {
					if let Some(sword) = self.pe.read::<i16>(self.cursor) {
						if let Some(slot) = save.get_mut(slot as usize) {
							*slot = sword as Rva;
						}
						self.cursor = self.cursor.wrapping_add(2);
					}
					else {
						return false;
					}
				},
				pat::Atom::ReadU32(slot) | pat::Atom::ReadI32(slot) => {
					if let Some(dword) = self.pe.read::<Rva>(self.cursor) {
						if let Some(slot) = save.get_mut(slot as usize) {
							*slot = dword;
						}
						self.cursor = self.cursor.wrapping_add(4);
					}
					else {
						return false;
					}
				},
				pat::Atom::Case(next) => {
					let pc = self.pc;
					let cursor = self.cursor;
					if !self.exec(save) {
						self.pc = pc + next as usize;
						self.cursor = cursor;
					}
				},
				pat::Atom::Break(next) => {
					self.pc = self.pc + next as usize;
					return true;
				},
				pat::Atom::Nop => {
				},
			}
		}
		return true;
	}
	fn exec_many(&mut self, save: &mut [Rva], limit: u32) -> bool {
		// Capture the current cursor and PC to restore while trying
		let cursor = self.cursor;
		let pc = self.pc;
		// Slice a section of bytes to limit the scan to
		let bytes = match self.pe.slice(cursor) {
			Some(bytes) if limit == 0 => bytes,
			Some(bytes) => &bytes[..cmp::min(limit as usize, bytes.len())],
			None => return false,
		};
		// Peek at a byte to match on
		let mut peek = None;
		for &atom in &self.pat[pc..] {
			match atom {
				pat::Atom::Byte(byte) => {
					peek = Some(byte);
					break;
				},
				pat::Atom::Save(_) => (),
				_ => break,
			}
		}
		// Optimize the next scan with memchr, happy path
		if let Some(byte) = peek {
			for i in 0..bytes.len() as u32 {
				if bytes[i as usize] == byte {
					self.cursor = cursor.wrapping_add(i);
					self.pc = pc;
					if self.exec(save) {
						return true;
					}
				}
			}
		}
		// Not optimizable, perf cliff!
		else {
			for i in 0..bytes.len() as u32 {
				self.cursor = cursor.wrapping_add(i);
				self.pc = pc;
				if self.exec(save) {
					return true;
				}
			}
		}
		// No match found, exec fails
		return false;
	}
}

//----------------------------------------------------------------

/// An iterator over the matches of a pattern.
///
/// Created with the method [`matches`](struct.Scanner.html#method.matches).
#[derive(Clone)]
pub struct Matches<'u, P> {
	scanner: Scanner<P>,
	pat: &'u [pat::Atom],
	range: Range<Rva>,
	cursor: Rva,
	hits: u32,
}

impl<'a, 'u, P: Pe<'a>> Matches<'u, P> {
	/// Gets the scanner instance.
	pub fn scanner(&self) -> Scanner<P> {
		self.scanner
	}
	/// Gets the pattern.
	pub fn pattern(&self) -> &'u [pat::Atom] {
		self.pat
	}
	/// Gets the remaining RVA range to scan.
	pub fn range(&self) -> Range<Rva> {
		self.range.clone()
	}
	/// The RVA where the last match was found.
	pub fn cursor(&self) -> Rva {
		self.cursor
	}
	/// Performance counter.
	///
	/// Number of times the slow [`exec`](struct.Scanner.html#method.exec) was invoked.
	pub fn hits(&self) -> u32 {
		self.hits
	}
	// Extract the prefix of bytes for optimizing the search
	fn setup<'b>(&self, qsbuf: &'b mut [u8; QS_BUF_LEN]) -> &'b [u8] {
		let mut qslen = 0usize;
		for unit in self.pat {
			match *unit {
				pat::Atom::Byte(byte) => {
					if qslen >= QS_BUF_LEN {
						break;
					}
					qsbuf[qslen] = byte;
					qslen += 1;
				},
				pat::Atom::Save(_) => {},
				_ => break,
			}
		}
		&qsbuf[..qslen]
	}
	// Select the search strategy and execute the query.
	fn strategy(&mut self, cursor: u32, qsbuf: &[u8], slice: &'a [u8], save: &mut [Rva]) -> bool {
		self.cursor = cursor;
		// FIXME! Profile the performance!
		if qsbuf.len() == 0 {
			self.strategy0(qsbuf, slice, save)
		}
		else if qsbuf.len() < 4 {
			self.strategy1(qsbuf, slice, save)
		}
		else {
			self.strategy2(qsbuf, slice, save)
		}
	}
	// Strategy:
	//  Cannot optimize the search, just brute-force it.
	//  Note that this is (relatively) slow...
	fn strategy0(&mut self, _qsbuf: &[u8], slice: &'a [u8], save: &mut [Rva]) -> bool {
		let mut it = self.cursor;
		let end = it + slice.len() as Rva;
		while it < end {
			self.hits += 1;
			if self.scanner.exec(it, self.pat, save) {
				self.cursor = it;
				self.range.start = it + 1;
				return true;
			}
			it += 1;
		}
		self.cursor = it;
		self.range.start = it;
		false
	}
	// Strategy:
	//  Prefix is too small for full blown quicksearch.
	//  Memchr for the first byte and only eval pattern on potential matches.
	fn strategy1(&mut self, qsbuf: &[u8], slice: &'a [u8], save: &mut [Rva]) -> bool {
		let byte = qsbuf[0];
		let it = self.cursor;
		// Find all places with matching byte
		// TODO! Replace with actual memchr
		for cursor in slice.iter().enumerate().filter_map(|(i, &a)| if a == byte { Some(it + i as Rva) } else { None }) {
			self.hits += 1;
			if self.scanner.exec(cursor, self.pat, save) {
				self.cursor = cursor;
				self.range.start = cursor + 1;
				return true;
			}
		}
		let end = it + slice.len() as Rva;
		self.cursor = end;
		self.range.start = end;
		false
	}
	// Strategy:
	//  Full blown quicksearch for the prefix.
	//  Most likely completely unnecessary but oh well... it was fun to write!
	fn strategy2(&mut self, qsbuf: &[u8], slice: &'a [u8], save: &mut [Rva]) -> bool {
		// Initialize jump table for quicksearch
		let qslen = qsbuf.len();
		let mut jumps = [qslen as u8; 256];
		for i in 0..qslen - 1 {
			jumps[qsbuf[i] as usize] = qslen as u8 - i as u8 - 1;
		}
		let jumps = jumps;
		// Quicksearch baby!
		let mut i = 0;
		while i + qslen <= slice.len() {
			let tbuf = &slice[i..i + qslen];
			let last = tbuf[qslen - 1];
			let jump = jumps[last as usize] as Rva;
			if qsbuf[qslen - 1] == last && tbuf == qsbuf {
				self.hits += 1;
				let cursor = self.cursor + i as Rva;
				if self.scanner.exec(cursor, self.pat, save) {
					self.cursor = cursor;
					self.range.start = cursor + jump;
					return true;
				}
			}
			i += jump as usize;
		}
		// FIXME! Quicksearch stops too soon!
		// It assumes there can't be another match in the last `qsbuf.len()` bytes
		// Even though there clearly can since the scan range can be artificially limited
		// For now let's ignore this edge case...
		let end = self.cursor + slice.len() as Rva;
		self.cursor = end;
		self.range.start = end;
		false
	}
	/// Finds the next match with the given save array.
	pub fn next(&mut self, save: &mut [Rva]) -> bool {
		// Build the quicksearch buffer
		let mut qsbuf = [0u8; QS_BUF_LEN];
		let qsbuf = self.setup(&mut qsbuf);

		// Take care of unmapped PE files.
		// Their sections aren't continous and need to be scanned separately.
		finder_section(self.scanner.pe, self.range.clone(), |it, slice| self.strategy(it, qsbuf, slice, save))
	}
}

/// Map over continuous pe memory.
///
/// For PeFiles this means providing each section separately.
/// For PeViews this means just provide the whole image at once.
fn for_each_section<'a, P, F>(pe: P, mut f: F) -> bool where
	P: Pe<'a>,
	F: FnMut(Rva, &'a [u8]) -> bool
{
	let image = pe.image();
	match pe.align() {
		Align::File => {
			for section in pe.section_headers() {
				let start = section.PointerToRawData as usize;
				let end = section.PointerToRawData as usize + section.SizeOfRawData as usize;
				if let Some(slice) = image.get(start..end) {
					if f(section.VirtualAddress, slice) {
						return true;
					}
				}
			}
			return false;
		},
		Align::Section => f(0, image),
	}
}
/// Map over continuous pe memory in the given range.
fn finder_section<'a, P, F>(pe: P, range: Range<Rva>, mut f: F) -> bool where
	P: Pe<'a>,
	F: FnMut(Rva, &'a [u8]) -> bool
{
	for_each_section(pe, |rva, bytes| {
		if range.start < rva + bytes.len() as Rva && range.end >= rva {
			let start = cmp::max(range.start, rva);
			let end = cmp::min(range.end, rva + bytes.len() as Rva);
			if let Some(slice) = bytes.get((start - rva) as usize..(end - rva) as usize) {
				let result = f(start, slice);
				if result {
					return result;
				}
			}
		}
		return false;
	})
}

//----------------------------------------------------------------

#[cfg(test)]
pub(crate) fn test<'a, P: Pe<'a>>(pe: P) -> crate::Result<()> {
	use crate::pattern::Atom::*;
	let scanner = pe.scanner();
	let mut save = [0; 4];

	let mut matches = scanner.matches_code(&[Save(0), Byte(0xE8), Push(4), Jump4, Save(1), Pop, Save(2)]);
	while matches.next(&mut save) {
		assert_eq!(save[0] + 5, save[2]);
	}

	let mut matches = scanner.matches_code(&[Jump1, Save(1), Byte(0x0F), Byte(0x0D)]);
	while matches.next(&mut save) {}

	scanner.finds_code(&[Byte(0x8B), Byte(0x01), Byte(0x8B), Byte(0x10), Byte(0xFF), Byte(0xD2)], &mut save);

	Ok(())
}

// Test the core scanner engine
#[test]
fn exec_tests_parse_docs() {
	use crate::pattern::{Atom, parse};

	fn exec(bytes: &[u8], pat: &[Atom], save: &mut [Rva]) -> bool {
		Exec { pe: bytes, pat, cursor: 0, pc: 0 }.exec(save)
	}

	{
		let bytes = [0x55, 0x89, 0xe5, 0x83, 0xff, 0xec];
		let pat = parse("55 89 e5 83 ? ec").unwrap();
		assert!(exec(&bytes, &pat, &mut []));
	}{
		let bytes = [0xb9, 0x37, 0x13, 0x00, 0x00];
		let pat = parse("b9 '37 13 00 00").unwrap();
		let mut save = [0; 2];
		assert!(exec(&bytes, &pat, &mut save));
		assert_eq!(save[1], 1);
	}{
		let mut bytes = [0; 64];
		bytes[0] = 0xb8;
		bytes[17] = 0x50;
		bytes[41] = 0xff;
		let pat = parse("b8 [16] 50 [13-42] 'ff").unwrap();
		let mut save = [0; 2];
		assert!(exec(&bytes, &pat, &mut save));
		assert_eq!(save[1], 41);
	}{
		let bytes = [0x31, 0xc0, 0x74, (-3i8) as u8];
		let pat = parse("31 c0 74 % 'c0").unwrap();
		let mut save = [0; 2];
		assert!(exec(&bytes, &pat, &mut save));
		assert_eq!(save[1], 1);
	}{
		let bytes = [0xe8, 10, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0x31, 0xc0, 0xc3];
		let pat = parse("e8 $ '31 c0 c3").unwrap();
		let mut save = [0; 2];
		assert!(exec(&bytes, &pat, &mut save));
		assert_eq!(save[1], 15);
	}{
		let bytes = [0x68, 10, 0, 0, 0, 1, 2, 3, 4, 5, 0x31, 0xc0, 0xc3];
		let pat = parse("68 * '31 c0 c3").unwrap();
		let mut save = [0; 2];
		assert!(exec(&bytes, &pat, &mut save));
		assert_eq!(save[1], 10);
	}{
		let bytes = b"\xb8\x0a\x00\x00\x00\x01\x02\x03\x04\x05STRING\x00";
		let pat = parse(r#"b8 * "STRING" 00"#).unwrap();
		assert!(exec(bytes, &pat, &mut []));
	}{
		let bytes = [0xe8, 10, 0, 0, 0, 0x83, 0xf0, 0x5c, 0xc3, 5, 6, 7, 8, 9, 10];
		let pat = parse("e8 $ { ' } 83 f0 5c c3").unwrap();
		let mut save = [0; 2];
		assert!(exec(&bytes, &pat, &mut save));
		assert_eq!(save[1], 15);
	}{
		let bytes = [0xe8, 0xff, 0xa0, 0x78, 0x56, 0x34, 0x12];
		let pat = parse("e8 i1 a0 u4").unwrap();
		let mut save = [0; 3];
		assert!(exec(&bytes, &pat, &mut save));
		assert_eq!(save[1], (-1i8) as u32);
		assert_eq!(save[2], 0x12345678);
	}{
		let bytes1 = [0x83, 0xc0, 0x2a, 0x6a, 0x00, 0xe8];
		let bytes2 = [0x83, 0xc0, 0x2a, 0x68, 0x00, 0x00, 0x00, 0x10, 0xe8];
		let pat = parse("83 c0 2a ( 6a ? | 68 ? ? ? ? ) e8").unwrap();
		assert!(exec(&bytes1, &pat, &mut []));
		assert!(exec(&bytes2, &pat, &mut []));
	}
}
