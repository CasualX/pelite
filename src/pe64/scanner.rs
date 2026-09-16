use super::*;

/// Size of the prefix buffer for search optimization.
const QS_BUF_LEN: usize = 16;

//----------------------------------------------------------------

/**
Pattern scanner.

See the [`pattern`][mod@crate::pattern] module for more information about patterns.

# Examples

```
# #![allow(unused_variables)]
use pelite::pattern as pat;
use pelite::pe64::{Pe, PeFile};

# #[allow(dead_code)]
fn example(file: PeFile<'_>, pattern: &[pat::Atom]) {
	// Get the pattern scanner interface
	let scanner = file.scanner();

	// Captured references from the pattern are written into this array
	let mut save = [0; 8];

	// Find a unique match in the code range
	if scanner.finds_code(pattern, &mut save) {
		println!("{:x?}", save);
	}

	// Find all matches in the code range
	let mut matches = scanner.matches_code(pattern);
	while matches.next(&mut save) {
		println!("{:x?}", save);
	}
}
```
*/
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
	/// Out-of-bounds save-slot reads return zero and stores are ignored.
	/// Supply at least [`pat::save_len(pat)`](pat::save_len) slots for the given pattern.
	///
	/// If this returns `false`, the contents of the save array are unspecified.
	///
	/// Returns `false` if no match is found or multiple matches are found to prevent subtle bugs where a pattern goes stale by not being unique any more.
	///
	/// After checking uniqueness, the pattern is reexecutes at the known match RVA to restore the save array.
	///
	/// Use `matches(pat, range).next(save)` if just the first match is desired.
	pub fn finds(&self, pat: &[pat::Atom], range: ops::Range<Rva>, save: &mut [Rva]) -> bool {
		let mut matches = self.matches(pat, range);
		if !matches.next(save) {
			return false;
		}
		let cursor = matches.matched;
		// The uniqueness check needs the same scratch space as the first match.
		if matches.next(save) {
			return false;
		}
		// Failed candidates may overwrite captures. Recover them at the known match RVA.
		self.exec(cursor, pat, save)
	}
	/// Finds the unique code match for the pattern.
	///
	/// Restricts the range to the code section. See [`finds`](#finds) for more information.
	pub fn finds_code(&self, pat: &[pat::Atom], save: &mut [Rva]) -> bool {
		self.finds(pat, self.pe.headers().code_range(), save)
	}
	/// Returns an iterator over the matches of a pattern within the given range.
	pub fn matches<'pat>(&self, pat: &'pat [pat::Atom], range: ops::Range<Rva>) -> ScannerMatches<'pat, P> {
		ScannerMatches { scanner: *self, pat, range, hits: 0, matched: !0 }
	}
	/// Returns an iterator over the code matches of a pattern.
	///
	/// Restricts the range to the code section. See [`matches`](#matches) for more information.
	pub fn matches_code<'pat>(&self, pat: &'pat [pat::Atom]) -> ScannerMatches<'pat, P> {
		self.matches(pat, self.pe.headers().code_range())
	}
	/// Pattern interpreter, returns if the pattern matches the binary image at the given rva.
	///
	/// The pattern may contain instructions to capture interesting addresses, these are stored in the save array.
	/// Out-of-bounds save-slot reads return zero and stores are ignored.
	/// Supply at least [`pat::save_len(pat)`](pat::save_len) slots for the given pattern.
	///
	/// If this returns `false`, the contents of the save array are unspecified.
	pub fn exec(&self, cursor: Rva, pat: &[pat::Atom], save: &mut [Rva]) -> bool {
		assert!(
			save.len() > 255 || save.len() >= pat::save_len(pat),
			"save array too small: pattern requires {} slots, got {}",
			pat::save_len(pat),
			save.len(),
		);
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
		Some(unsafe { raw_ptr::read_unaligned(ptr) })
	}
	fn pointer(self, va: Va) -> Option<Rva> {
		Some(va as Rva)
	}
	fn slice(self, rva: Rva) -> Option<&'a [u8]> {
		self.get(rva as usize..)
	}
}

/// Resolve a signed save operand. Negative underflow wraps out of bounds, so the
/// caller's slice lookup retains the usual behavior for unavailable slots.
fn save_index(slot: i8, len: usize) -> usize {
	if slot < 0 {
		len.wrapping_add_signed(slot as isize)
	}
	else {
		slot as usize
	}
}

/// Read a signed save slot, returning zero when it is unavailable.
fn save_read(save: &[Rva], slot: i8) -> Rva {
	save.get(save_index(slot, save.len())).copied().unwrap_or(0)
}

/// Write a signed save slot, ignoring unavailable slots.
fn save_write(save: &mut [Rva], slot: i8, value: Rva) {
	if let Some(slot) = save.get_mut(save_index(slot, save.len())) {
		*slot = value;
	}
}

#[derive(Clone)]
struct Exec<'pat, P> {
	pe: P,
	pat: &'pat [pat::Atom],
	cursor: Rva,
	pc: usize,
}
impl<'a, 'pat, P: Scan<'a>> Exec<'pat, P> {
	// Each recursive call attempts the entire remaining pattern, so Goto and Seek
	// cannot commit an alternative. Scratch writes intentionally survive failure.
	fn exec(&mut self, save: &mut [Rva]) -> bool {
		const SKIP_VA: u32 = mem::size_of::<Va>() as u32;
		let mut mask = 0xff;
		let mut extend = 0u32;
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
					save_write(save, slot, self.cursor);
				},
				pat::Atom::Seek(slot) => {
					self.cursor = save_read(save, slot);
				},
				pat::Atom::Fuzzy(pat_mask) => {
					mask = pat_mask;
				},
				pat::Atom::Skip(skip) => {
					let skip = (extend << 8) + skip as u32;
					let skip = if skip == 0 { SKIP_VA } else { skip };
					let cursor = self.cursor.wrapping_add(skip);
					extend = 0;
					self.cursor = cursor;
				},
				pat::Atom::Rewind(back) => {
					let rewind = (extend << 8) + back as u32;
					let rewind = if rewind == 0 { SKIP_VA } else { rewind };
					let cursor = self.cursor.wrapping_sub(rewind);
					extend = 0;
					self.cursor = cursor;
				},
				pat::Atom::Extend(ext) => {
					extend = (extend << 8) + ext as u32;
				},
				pat::Atom::Scan(limit) => {
					let limit = (extend << 8) + limit as u32;
					return self.exec_scan(save, limit);
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
						let base = save_read(save, slot);
						self.cursor = base.wrapping_add(sdword as Rva);
					}
					else {
						return false;
					}
				},
				pat::Atom::Check(slot) => {
					if save_read(save, slot) != self.cursor {
						return false;
					}
				},
				pat::Atom::IsAlign(align) => {
					if !self.cursor.aligned_to(1 << align as u32) {
						return false;
					}
				},
				pat::Atom::ReadU8(slot) | pat::Atom::TestU8(slot) => {
					if let Some(byte) = self.pe.read::<u8>(self.cursor) {
						if matches!(atom, pat::Atom::TestU8(_)) {
							if save_read(save, slot) != byte as Rva {
								return false;
							}
						}
						else {
							save_write(save, slot, byte as Rva);
						}
						self.cursor = self.cursor.wrapping_add(1);
					}
					else {
						return false;
					}
				},
				pat::Atom::ReadI8(slot) | pat::Atom::TestI8(slot) => {
					if let Some(sbyte) = self.pe.read::<i8>(self.cursor) {
						if matches!(atom, pat::Atom::TestI8(_)) {
							if save_read(save, slot) != sbyte as Rva {
								return false;
							}
						}
						else {
							save_write(save, slot, sbyte as Rva);
						}
						self.cursor = self.cursor.wrapping_add(1);
					}
					else {
						return false;
					}
				},
				pat::Atom::ReadU16(slot) | pat::Atom::TestU16(slot) => {
					if let Some(word) = self.pe.read::<u16>(self.cursor) {
						if matches!(atom, pat::Atom::TestU16(_)) {
							if save_read(save, slot) != word as Rva {
								return false;
							}
						}
						else {
							save_write(save, slot, word as Rva);
						}
						self.cursor = self.cursor.wrapping_add(2);
					}
					else {
						return false;
					}
				},
				pat::Atom::ReadI16(slot) | pat::Atom::TestI16(slot) => {
					if let Some(sword) = self.pe.read::<i16>(self.cursor) {
						if matches!(atom, pat::Atom::TestI16(_)) {
							if save_read(save, slot) != sword as Rva {
								return false;
							}
						}
						else {
							save_write(save, slot, sword as Rva);
						}
						self.cursor = self.cursor.wrapping_add(2);
					}
					else {
						return false;
					}
				},
				pat::Atom::ReadU32(slot) | pat::Atom::TestU32(slot) => {
					if let Some(dword) = self.pe.read::<Rva>(self.cursor) {
						if matches!(atom, pat::Atom::TestU32(_)) {
							if save_read(save, slot) != dword {
								return false;
							}
						}
						else {
							save_write(save, slot, dword);
						}
						self.cursor = self.cursor.wrapping_add(4);
					}
					else {
						return false;
					}
				},
				pat::Atom::Zero(slot) => {
					save_write(save, slot, 0);
				},
				pat::Atom::Fork(next) => {
					let pc = self.pc + ((extend << 8) + next as u32) as usize;
					let cursor = self.cursor;
					extend = 0;
					if self.exec(save) {
						return true;
					}
					self.pc = pc;
					self.cursor = cursor;
				},
				pat::Atom::Goto(next) => {
					self.pc += ((extend << 8) + next as u32) as usize;
					extend = 0;
				},
				pat::Atom::Nop => {},
			}
		}
		return true;
	}
	fn exec_scan(&mut self, save: &mut [Rva], limit: u32) -> bool {
		// Capture the current cursor and PC to restore while trying
		let cursor = self.cursor;
		let pc = self.pc;
		// Slice a section of bytes to limit the scan to
		let bytes = match self.pe.slice(cursor) {
			Some(bytes) if limit == 0 => bytes,
			Some(bytes) => &bytes[..cmp::min(limit.saturating_add(1) as usize, bytes.len())],
			None => return false,
		};
		// Only prefilter an immediate byte: skipping earlier instructions could omit
		// scratch writes that remain observable when an enclosing fork retries.
		if let Some(&pat::Atom::Byte(byte)) = self.pat.get(pc) {
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
		// Execute every candidate when the continuation does not start with a byte.
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
/// Created with the method [`Scanner::matches`].
#[derive(Clone)]
pub struct ScannerMatches<'pat, P> {
	scanner: Scanner<P>,
	pat: &'pat [pat::Atom],
	range: ops::Range<Rva>,
	hits: u32,
	matched: Rva,
}

impl<'a, 'pat, P: Pe<'a>> ScannerMatches<'pat, P> {
	/// Gets the scanner instance.
	pub fn scanner(&self) -> Scanner<P> {
		self.scanner
	}
	/// Gets the pattern.
	pub fn pattern(&self) -> &'pat [pat::Atom] {
		self.pat
	}
	/// Gets the remaining range to scan.
	pub fn range(&self) -> ops::Range<Rva> {
		self.range.clone()
	}
	/// Performance counter.
	///
	/// Number of times the slow [`Scanner::exec`] was invoked.
	pub fn hits(&self) -> u32 {
		self.hits
	}
	// Extract the prefix of bytes for optimizing the search
	fn setup<'b>(&self, qsbuf: &'b mut [u8; QS_BUF_LEN]) -> &'b [u8] {
		let mut qslen = 0usize;
		for &atom in self.pat {
			match atom {
				pat::Atom::Byte(byte) => {
					if qslen >= QS_BUF_LEN {
						break;
					}
					qsbuf[qslen] = byte;
					qslen += 1;
				},
				// These atoms do not interfere with optimizing search
				pat::Atom::Save(_) => {},
				pat::Atom::IsAlign(_) => {},
				pat::Atom::Nop => {},
				// All other atoms interfere with optimizing search
				_ => break,
			}
		}
		&qsbuf[..qslen]
	}
	// Select the search strategy and execute the query.
	fn strategy(&mut self, qsbuf: &[u8], slice: &'a [u8], save: &mut [Rva]) -> bool {
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
	//  Cannot optimize the search, just (slowly) brute-force it.
	fn strategy0(&mut self, _qsbuf: &[u8], slice: &'a [u8], save: &mut [Rva]) -> bool {
		let end = self.range.start + slice.len() as u32;
		while self.range.start < end {
			let cursor = self.range.start;
			self.hits += 1;
			self.range.start += 1;
			if self.scanner.exec(cursor, self.pat, save) {
				self.matched = cursor;
				return true;
			}
		}
		return false;
	}
	// Strategy:
	//  Prefix is too small for full blown quicksearch.
	//  Memchr for the first byte and only eval pattern on potential matches.
	fn strategy1(&mut self, qsbuf: &[u8], slice: &'a [u8], save: &mut [Rva]) -> bool {
		let byte = qsbuf[0];
		// Find all places with matching byte
		// TODO! Replace with actual memchr
		for i in slice.iter().enumerate().filter_map(|(i, &a)| {
			if a == byte {
				Some(i as u32)
			}
			else {
				None
			}
		}) {
			self.hits += 1;
			let cursor = self.range.start + i;
			if self.scanner.exec(cursor, self.pat, save) {
				self.matched = cursor;
				self.range.start = cursor + 1;
				return true;
			}
		}
		self.range.start += slice.len() as u32;
		return false;
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
		// Quicksearch baby!
		let mut i = 0;
		while i + qslen <= slice.len() {
			let tbuf = &slice[i..i + qslen];
			let last = tbuf[qslen - 1];
			let jump = jumps[last as usize] as u32;
			if qsbuf[qslen - 1] == last && tbuf == qsbuf {
				self.hits += 1;
				let cursor = self.range.start + i as u32;
				if self.scanner.exec(cursor, self.pat, save) {
					self.matched = cursor;
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
		self.range.start += slice.len() as u32;
		return false;
	}
	/// Finds the next match with the given save array.
	///
	/// If this returns `false`, the contents of the save array are unspecified.
	pub fn next(&mut self, save: &mut [Rva]) -> bool {
		assert!(
			save.len() > 255 || save.len() >= pat::save_len(self.pat),
			"save array too small: pattern requires {} slots, got {}",
			pat::save_len(self.pat),
			save.len(),
		);
		// Build the quicksearch buffer
		let mut qsbuf = [0u8; QS_BUF_LEN];
		let qsbuf = self.setup(&mut qsbuf);

		let image = self.scanner.pe.image();
		match self.scanner.pe.align() {
			Align::File => {
				for section in self.scanner.pe.section_headers() {
					// If section overlaps with the scanning range
					if section.VirtualAddress < self.range.end && u32::wrapping_add(section.VirtualAddress, section.VirtualSize) > self.range.start {
						// Get the image slice for this section for further processing, skipping corrupt section headers
						if let Some(slice) = image.get(section.PointerToRawData as usize..u32::wrapping_add(section.PointerToRawData, section.SizeOfRawData) as usize) {
							if self.next_section(qsbuf, section.VirtualAddress, slice, save) {
								return true;
							}
						}
					}
				}
				false
			},
			Align::Section => self.next_section(qsbuf, 0, image, save),
		}
	}
	fn next_section(&mut self, qsbuf: &[u8], base: Rva, slice: &'a [u8], save: &mut [Rva]) -> bool {
		// Let's talk about this code for a sec, for it has a problem.
		// This method gets called for every section and is supposed to find matches in that section.
		// A match is found, true is returned, all is well?
		// Unfortunately, no, the caller is going to want to find the next match and calls us again.
		// We have to continue from where we left off but that is currently not really possible.
		// It is implicitly assumed that all sections are sorted by their virtual addresses.
		// If this is not the case the code below will incorrectly clamp self.range.start and the section with lower virtual address will be skipped.
		// Plz fix.

		// Clamp the slice to the expected input scan range
		self.range.start = cmp::max(base, self.range.start);
		let start = self.range.start - base;
		let end = cmp::min(base + slice.len() as u32, self.range.end) - base;

		self.strategy(qsbuf, &slice[start as usize..end as usize], save)
	}
}

//----------------------------------------------------------------

#[cfg(test)]
pub(crate) fn test_scanner<'a, P: Pe<'a>>(pe: P) -> crate::Result<()> {
	use crate::pattern::Atom::*;
	let scanner = pe.scanner();
	let mut save = [0; 4];

	let mut matches = scanner.matches_code(&[Save(0), Byte(0xE8), Save(-1), Jump4, Save(1), Seek(-1), Skip(4), Save(2)]);
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
	use crate::pattern::{save_len, Atom::*};

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
	use crate::pattern::{parse, Atom};

	fn exec(bytes: &[u8], pat: &[Atom], save: &mut [Rva]) -> bool {
		Exec { pe: bytes, pat, cursor: 0, pc: 0 }.exec(save)
	}

	{
		let bytes = [0x55, 0x89, 0xe5, 0x83, 0xff, 0xec];
		let pat = parse("55 89 e5 83 ? ec").unwrap();
		assert!(exec(&bytes, &pat, &mut []));
	}
	{
		let bytes = [0xb9, 0x37, 0x13, 0x00, 0x00];
		let pat = parse("b9 '37 13 00 00").unwrap();
		let mut save = [0; 2];
		assert!(exec(&bytes, &pat, &mut save));
		assert_eq!(save[1], 1);
	}
	{
		let mut bytes = [0; 64];
		bytes[0] = 0xb8;
		bytes[17] = 0x50;
		bytes[41] = 0xff;
		let pat = parse("b8 skip(16) 50 skip(13) scan(28) 'ff").unwrap();
		let mut save = [0; 2];
		assert!(exec(&bytes, &pat, &mut save));
		assert_eq!(save[1], 41);
	}
	{
		let bytes = [0x31, 0xc0, 0x74, (-3i8) as u8];
		let pat = parse("31 c0 74 % 'c0").unwrap();
		let mut save = [0; 2];
		assert!(exec(&bytes, &pat, &mut save));
		assert_eq!(save[1], 1);
	}
	{
		let bytes = [0xe8, 10, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0x31, 0xc0, 0xc3];
		let pat = parse("e8 $ '31 c0 c3").unwrap();
		let mut save = [0; 2];
		assert!(exec(&bytes, &pat, &mut save));
		assert_eq!(save[1], 15);
	}
	{
		let bytes = [0x68, 10, 0, 0, 0, 1, 2, 3, 4, 5, 0x31, 0xc0, 0xc3];
		let pat = parse("68 * '31 c0 c3").unwrap();
		let mut save = [0; 2];
		assert!(exec(&bytes, &pat, &mut save));
		assert_eq!(save[1], 10);
	}
	{
		let bytes = b"\xb8\x0a\x00\x00\x00\x01\x02\x03\x04\x05STRING\x00";
		let pat = parse(r#"b8 * "STRING" 00"#).unwrap();
		assert!(exec(bytes, &pat, &mut []));
	}
	{
		let bytes = [0xe8, 10, 0, 0, 0, 0x83, 0xf0, 0x5c, 0xc3, 5, 6, 7, 8, 9, 10];
		let pat = parse("e8 ${'} 83 f0 5c c3").unwrap();
		let mut save = vec![0; crate::pattern::save_len(&pat)];
		assert!(exec(&bytes, &pat, &mut save));
		assert_eq!(save[1], 15);
	}
	{
		let bytes = [0xe8, 0xff, 0xa0, 0x78, 0x56, 0x34, 0x12];
		let pat = parse("e8 i1 a0 u4").unwrap();
		let mut save = [0; 3];
		assert!(exec(&bytes, &pat, &mut save));
		assert_eq!(save[1], (-1i8) as u32);
		assert_eq!(save[2], 0x12345678);
	}
	{
		let bytes1 = [0x83, 0xc0, 0x2a, 0x6a, 0x00, 0xe8];
		let bytes2 = [0x83, 0xc0, 0x2a, 0x68, 0x00, 0x00, 0x00, 0x10, 0xe8];
		let pat = parse("83 c0 2a ( 6a ? | 68 ? ? ? ? ) e8").unwrap();
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
	use crate::pattern::{save_len, Atom::*};

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
	use crate::pattern::{parse, save_len, Atom::*};

	let mut bytes = [0; 21];
	bytes[0] = 9; // First reference goes to the search at 10.
	bytes[1] = 18; // Second reference goes to 20.
	bytes[2] = 9; // The final comparison rejects the first search candidate.
	bytes[10] = 7;
	bytes[11] = 9;
	bytes[20] = 0xaa;
	let mut pat = parse("%{scan(1)u1}%{aa}00").unwrap();
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
		assert_eq!(parse(source).unwrap(), pat, "{source}");
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
	use crate::pattern::{parse, save_len, Atom::*};

	let mut bytes = [0; 11];
	bytes[0] = 4; // First scope visits the alternatives at 5.
	bytes[1] = 8; // Second scope visits 10.
	bytes[2] = b'c'; // Rejects the first alternative after both scopes return.
	bytes[5..8].copy_from_slice(b"abc");
	bytes[10] = 0xaa;
	let mut pat = parse("%{(61|6162)u1}%{aa}00").unwrap();
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
	macro_rules! cases {
		($(($source:expr, $bytes:expr, $matched:expr)),* $(,)?) => {
			&[$(($source, &const { parse_const::<{ parse_len($source) }>($source) } as &[pat::Atom], &$bytes[..], $matched)),*]
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
		("scan()", b"", false),
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
		assert_eq!(parse(source).unwrap(), pat, "{source}");
		let mut save = vec![0; save_len(pat)];
		assert_eq!(Exec { pe: bytes, pat, cursor: 0, pc: 0 }.exec(&mut save), matched, "{source}");
	}

	// Both the extended fork and the extended goto must land on the continuation.
	let source = alloc::format!("(61{}|62{})63", "?".repeat(300), "?".repeat(300));
	let pat = parse(&source).unwrap();
	let mut bytes = [0; 302];
	bytes[301] = b'c';
	for byte in [b'a', b'b'] {
		bytes[0] = byte;
		assert!(Exec { pe: &bytes[..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut [0]));
	}

	// Retry a choice inside a reference body after two returns and a failed test.
	let source = "%{(61|6162)u1[1]}%{AA}=u1[1]";
	let pat = parse(source).unwrap();
	let mut bytes = [4, 8, b'c', 0, 0, b'a', b'b', b'c', 0, 0, 0xaa];
	let mut save = vec![0; save_len(&pat)];
	assert!(Exec { pe: &bytes[..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut save));
	assert_eq!(save[1], b'c' as u32);
	bytes[2] = b'd';
	assert!(!Exec { pe: &bytes[..], pat: &pat, cursor: 0, pc: 0 }.exec(&mut save));
}
