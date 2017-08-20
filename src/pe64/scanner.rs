/*!
Pattern Scanner.

See the [`pattern`](../../pattern/index.html) module for more information about patterns.

# Examples

```
# #![allow(unused_variables)]
use pelite::pe64::{Pe, PeFile};
use pelite::pattern::{Atom, Match};

# #[allow(dead_code)]
fn example(file: PeFile, pat: &[Atom]) -> Option<Match> {
	// Get the pattern scanner interface
	let scanner = file.scanner();

	// Find a singular code match
	if let Some(m) = scanner.find_code(pat) {
		return Some(m);
	}

	// Find all the code matches for the pattern
	for m in scanner.matches_code(pat) {
		println!("found: {}", m);
	}

	None
}
```
*/

use std::mem;
use std::ops::Range;

use pattern as pat;

use super::{Rva, Pe};
use super::image::*;

/// Size of the prefix buffer for search optimization.
const QS_BUF_LEN: usize = 16;

//----------------------------------------------------------------

/// Pattern scanner.
#[derive(Copy, Clone)]
pub struct Scanner<P> {
	pe: P,
}
impl<'a, P: Pe<'a> + Copy> Scanner<P> {
	pub(crate) fn new(pe: P) -> Scanner<P> {
		Scanner { pe }
	}
	/// Finds the unique match for the pattern in the given range.
	///
	/// Returns `None` if multiple matches are found to prevent subtle bugs where a pattern goes stale by not being unique any more.
	///
	/// Use `matches(pat, range).next()` if just the first match is desired.
	pub fn find(self, pat: &[self::pat::Atom], range: Range<Rva>) -> Option<pat::Match> {
		let mut matches = self.matches(pat, range);
		if let Some(found) = matches.next() {
			// Disallow more than one match as it indicates the signature isn't unique enough
			match matches.next() {
				None => Some(found),
				Some(_) => None,
			}
		}
		else {
			None
		}
	}
	/// Finds the unique code match for the pattern.
	///
	/// Restricts the range to the code section. See [`find`](#find) for more information.
	pub fn find_code(self, pat: &[self::pat::Atom]) -> Option<pat::Match> {
		let optional_header = self.pe.optional_header();
		let range = optional_header.BaseOfCode..optional_header.BaseOfCode + optional_header.SizeOfCode;
		self.find(pat, range)
	}
	/// Returns an iterator over the matches of a pattern within the given range.
	pub fn matches(self, pat: &[self::pat::Atom], range: Range<Rva>) -> Matches<P> {
		Matches { scanner: self, pat, range, hits: 0 }
	}
	/// Returns an iterator over the code matches of a pattern.
	///
	/// Restricts the range to the code section. See [`matches`](#matches) for more information.
	pub fn matches_code(self, pat: &[self::pat::Atom]) -> Matches<P> {
		let optional_header = self.pe.optional_header();
		let range = optional_header.BaseOfCode..optional_header.BaseOfCode + optional_header.SizeOfCode;
		self.matches(pat, range)
	}
	/// Returns if the pattern matches the binary image at the given rva.
	pub fn exec(self, pat: &[self::pat::Atom], mut cursor: Rva) -> Option<pat::Match> {
		let ptr_skip = mem::size_of::<Va>() as i8;
		let mut stack = [0u32; pat::STACK_SIZE];
		let mut sp = 0;
		let mut result = pat::Match::default();
		let mut mask = 0xFF;
		let mut iter = pat.iter();
		while let Some(&atom) = iter.next() {
			match atom {
				pat::Atom::Byte(p_byte) => {
					match self.pe.derva_copy::<u8>(cursor) {
						Ok(byte) if byte & mask == p_byte & mask => {},
						_ => return None,
					}
					mask = 0xFF;
					cursor += 1;
				},
				pat::Atom::Save(slot) => {
					if slot < pat::MAX_SAVE as u8 {
						result.as_mut()[slot as usize] = cursor;
					}
				},
				pat::Atom::Push(skip) => {
					if sp < pat::STACK_SIZE {
						let skip = if skip == pat::PTR_SKIP { ptr_skip } else { skip };
						stack[sp] = cursor.wrapping_add(skip as Rva);
						sp += 1;
					}
				},
				pat::Atom::Pop => {
					if sp > 0 {
						sp -= 1;
						cursor = stack[sp];
					}
				},
				pat::Atom::Fuzzy(fuzz) => {
					mask = fuzz;
				},
				pat::Atom::Skip(skip) => {
					let skip = if skip == pat::PTR_SKIP { ptr_skip } else { skip };
					cursor = cursor.wrapping_add(skip as Rva);
				},
				pat::Atom::Many(limit) => {
					// Use `matches().next()` or just `find()`?
					return self.matches(iter.as_slice(), cursor..cursor + limit as Rva).next().map(|m| result.merge(&m));
				},
				pat::Atom::Jump1 => {
					if let Ok(sbyte) = self.pe.derva_copy::<i8>(cursor) {
						cursor = cursor.wrapping_add(sbyte as Rva).wrapping_add(1);
					}
					else {
						return None;
					}
				},
				pat::Atom::Jump4 => {
					if let Ok(sdword) = self.pe.derva_copy::<i32>(cursor) {
						cursor = cursor.wrapping_add(sdword as Rva).wrapping_add(4);
					}
					else {
						return None;
					}
				},
				pat::Atom::Ptr => {
					cursor = match self.pe.derva_copy(cursor).and_then(|va| self.pe.va_to_rva(va)) {
						Ok(cursor) => cursor,
						Err(_) => return None,
					};
				},
				pat::Atom::Pir(slot) => {
					if let Ok(sdword) = self.pe.derva_copy::<i32>(cursor) {
						let &base = result.as_ref().get(slot as usize).unwrap_or(&cursor);
						cursor = base.wrapping_add(sdword as Rva);
					}
					else {
						return None;
					}
				},
			}
		}
		// Pattern matches
		Some(result)
	}
	// Invokes the callback for all sections overlapping the range.
	// TODO! Specialze me for PeView?
	fn sections<F>(&self, range: Range<Rva>, mut f: F) -> Option<pat::Match> where F: FnMut(Rva, &'a [u8]) -> Option<pat::Match> {
		let image = self.pe.image();
		for it in self.pe.section_headers() {
			if range.start < (it.VirtualAddress + it.SizeOfRawData) && range.end >= it.VirtualAddress {
				use std::cmp::{min, max};
				let start = max(range.start, it.VirtualAddress);
				let end = min(range.end, it.VirtualAddress + it.SizeOfRawData);
				if let Some(slice) = image.get((start - it.VirtualAddress + it.PointerToRawData) as FileOffset..(end - it.VirtualAddress + it.PointerToRawData) as FileOffset) {
					let m = f(start, slice);
					if m.is_some() {
						return m;
					}
				}
			}
		}
		None
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
	/// Performance.
	///
	/// Number of times the slow [`exec`](struct.Scanner.html#method.exec) was invoked.
	pub hits: u32,
}

impl<'a, 'u, P: Pe<'a> + Copy> Matches<'u, P> {
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
	// Strategy:
	//  Cannot optimize the search, just brute-force it.
	//  Note that this is (relatively) slow...
	fn strategy0(&mut self, _qsbuf: &[u8]) -> Option<pat::Match> {
		let scanner = self.scanner;
		scanner.sections(self.range.clone(), |mut it, slice| {
			let end = it + slice.len() as Rva;
			while it < end {
				self.hits += 1;
				let m = scanner.exec(self.pat, it);
				if m.is_some() {
					self.range.start = it + 1;
					return m;
				}
				it += 1;
			}
			self.range.start = it;
			None
		})
	}
	// Strategy:
	//  Prefix is too small for full blown quicksearch.
	//  Memchr for the first byte and only eval pattern on potential matches.
	fn strategy1(&mut self, qsbuf: &[u8]) -> Option<pat::Match> {
		let byte = qsbuf[0];
		let scanner = self.scanner;
		scanner.sections(self.range.clone(), |it, slice| {
			// Find all places with matching byte
			// TODO! Replace with actual memchr
			for cursor in slice.iter().enumerate().filter_map(|(i, &a)| if a == byte { Some(it + i as Rva) } else { None }) {
				self.hits += 1;
				let m = scanner.exec(self.pat, cursor);
				if m.is_some() {
					self.range.start = cursor + 1;
					return m;
				}
			}
			self.range.start = it + slice.len() as Rva;
			None
		})
	}
	// Strategy:
	//  Full blown quicksearch for the prefix.
	//  Most likely completely unnecessary but oh well... it was fun to write!
	fn strategy2(&mut self, qsbuf: &[u8]) -> Option<pat::Match> {
		// Initialize jump table for quicksearch
		let qslen = qsbuf.len();
		let mut jumps = [qslen as u8; 256];
		for i in 0..qslen - 1 {
			jumps[qsbuf[i] as usize] = qslen as u8 - i as u8 - 1;
		}
		let jumps = jumps;
		let scanner = self.scanner;
		scanner.sections(self.range.clone(), |it, slice| {
			// Quicksearch baby!
			let mut i = 0;
			while i + qslen <= slice.len() {
				let tbuf = &slice[i..i + qslen];
				let last = tbuf[qslen - 1];
				let jump = jumps[last as usize] as Rva;
				if qsbuf[qslen - 1] == last && tbuf == qsbuf {
					self.hits += 1;
					let cursor = it + i as Rva;
					let m = scanner.exec(self.pat, cursor);
					if m.is_some() {
						self.range.start = cursor + jump;
						return m;
					}
				}
				i += jump as usize;
			}
			// FIXME! Quicksearch stops too soon!
			// It assumes there can't be another match in the last `qsbuf.len()` bytes
			// Even though there clearly can since the scan range can be artificially limited
			// For now let's ignore this edge case...
			self.range.start = it + slice.len() as Rva;
			None
		})
	}
}
impl<'a, 'u, P: Pe<'a> + Copy> Iterator for Matches<'u, P> {
	type Item = pat::Match;
	fn next(&mut self) -> Option<pat::Match> {
		// Build the quicksearch buffer
		let mut qsbuf = [0u8; QS_BUF_LEN];
		let qsbuf = self.setup(&mut qsbuf);

		// Select search strategy
		// FIXME! Profile the performance!
		if qsbuf.len() == 0 {
			self.strategy0(qsbuf)
		}
		else if qsbuf.len() < 4 {
			self.strategy1(qsbuf)
		}
		else {
			self.strategy2(qsbuf)
		}
	}
}
