use super::*;

#[path = "scanner/exec.rs"]
mod exec;
use self::exec::*;

/// Size of the prefix buffer for search optimization.
const QS_BUF_LEN: usize = 16;

//----------------------------------------------------------------

/// Pattern scanner.
///
/// See the [`pattern`][mod@crate::pattern] module for more information about patterns.
///
/// # Examples
///
/// ```
/// # #![allow(unused_variables)]
/// use pelite::pattern as pat;
/// use pelite::pe64::{Pe, PeFile};
///
/// # #[allow(dead_code)]
/// fn example(file: PeFile<'_>, pattern: &[pat::Atom]) {
/// 	// Get the pattern scanner interface
/// 	let scanner = file.scanner();
///
/// 	// Captured references from the pattern are written into this array
/// 	let mut save = [0; 8];
///
/// 	// Find a unique match across executable sections
/// 	if let Some(rva) = scanner.code().find(pattern, &mut save) {
/// 		println!("match at {rva:#x}: {save:x?}");
/// 	}
///
/// 	// Find all matches starting in executable sections
/// 	let mut matches = scanner.code().matches(pattern);
/// 	while let Some(rva) = matches.next(&mut save) {
/// 		println!("match at {rva:#x}: {save:x?}");
/// 	}
/// }
/// ```
#[derive(Copy, Clone)]
pub struct Scanner<P> {
	pe: P,
}
impl<'a, P: Pe<'a>> Scanner<P> {
	pub(crate) fn new(pe: P) -> Scanner<P> {
		Scanner { pe }
	}
	/// Selects candidate starting addresses within this RVA range across all sections.
	pub fn within(self, range: ops::Range<Rva>) -> ScanSections<'a, P> {
		let selection: ScanSections<'a, P> = self.sections(|_| true);
		selection.within(range)
	}
	/// Scan in executable sections.
	pub fn code(self) -> ScanSections<'a, P> {
		self.sections(|section| section.Characteristics & (IMAGE_SCN_MEM_EXECUTE | IMAGE_SCN_MEM_WRITE) == IMAGE_SCN_MEM_EXECUTE)
	}
	/// Selects sections using a predicate, evaluated once per section during iteration.
	pub fn sections<F: FnMut(&SectionHeader) -> bool>(self, filter: F) -> ScanSections<'a, P, F> {
		ScanSections {
			scanner: self,
			filter,
			sections: self.pe.section_headers().image(),
			range: 0..u32::MAX,
		}
	}
	/// Selects the section described by the given header.
	pub fn section(self, section: &'a SectionHeader) -> ScanSections<'a, P> {
		let mut selection: ScanSections<'a, P> = self.sections(|_| true);
		selection.sections = slice::from_ref(&*section);
		selection
	}
	/// Pattern interpreter, returns if the pattern matches the binary image at the given rva.
	///
	/// The pattern may contain instructions to capture interesting addresses, these are stored in the save array.
	/// Out-of-bounds save-slot reads return zero and stores are ignored.
	/// Supply at least [`save_len(pat)`](pat::save_len) slots for the given pattern.
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

/// Candidate locations for a PE pattern search.
///
/// - Sections are visited in section-table order, with increasing RVAs within each section.
/// - File searches use raw section bytes (including padding), without synthesizing zero-fill.
/// - Mapped searches use the declared virtual section bytes.
/// - Headers and gaps are not candidate locations.
/// - Pattern reads and references retain access to the whole PE, even outside the selection or `within` range.
/// - Invalid section extents and inaccessible section contents are skipped.
#[derive(Clone)]
pub struct ScanSections<'a, P, F = fn(&SectionHeader) -> bool> {
	scanner: Scanner<P>,
	filter: F,
	sections: &'a [IMAGE_SECTION_HEADER],
	range: ops::Range<Rva>,
}

impl<'a, P: Pe<'a>, F: FnMut(&SectionHeader) -> bool> ScanSections<'a, P, F> {
	/// Intersects the candidate starting addresses with this RVA range.
	pub fn within(mut self, range: ops::Range<Rva>) -> Self {
		self.range.start = cmp::max(self.range.start, range.start);
		self.range.end = cmp::min(self.range.end, range.end);
		self
	}
	/// Searches the selected sections.
	pub fn matches<'pat>(self, pat: &'pat [pat::Atom]) -> ScannerMatches<'a, 'pat, P, F> {
		ScannerMatches { selection: self, pat, active: None, cursor: 0, end: 0, hits: 0 }
	}
	/// Finds a unique match and returns its starting RVA.
	///
	/// After checking uniqueness, re-executes the pattern at the match to populate `save`.
	/// Returns `None` if there are zero or multiple matches, or re-execution fails.
	/// The contents of `save` are unspecified on failure.
	///
	/// Supply at least [`save_len(pat)`](pat::save_len) slots.
	pub fn find(self, pat: &[pat::Atom], save: &mut [Rva]) -> Option<Rva> {
		let mut matches = self.matches(pat);
		let first = matches.next(save)?;
		if matches.next(save).is_some() {
			return None;
		}
		matches.scanner().exec(first, pat, save).then_some(first)
	}
}

/// An iterator over section-oriented pattern matches.
///
/// Created with [`ScanSections::matches`]. Match RVAs are returned independently of capture slots.
#[derive(Clone)]
pub struct ScannerMatches<'a, 'pat, P, F = fn(&SectionHeader) -> bool> {
	selection: ScanSections<'a, P, F>,
	pat: &'pat [pat::Atom],
	active: Option<&'a IMAGE_SECTION_HEADER>,
	cursor: Rva,
	end: Rva,
	hits: u64,
}

impl<'a, 'pat, P: Pe<'a>, F: FnMut(&SectionHeader) -> bool> ScannerMatches<'a, 'pat, P, F> {
	/// Gets the scanner instance.
	pub fn scanner(&self) -> Scanner<P> {
		self.selection.scanner
	}
	/// Gets the pattern.
	pub fn pattern(&self) -> &'pat [pat::Atom] {
		self.pat
	}
	/// Number of calls to the pattern interpreter.
	pub fn hits(&self) -> u64 {
		self.hits
	}
	// Only use an initial literal prefix; references and masks require interpretation.
	fn prefix(&self) -> ([u8; QS_BUF_LEN], usize) {
		let mut bytes = [0; QS_BUF_LEN];
		let mut len = 0;
		for &atom in self.pat {
			match atom {
				pat::Atom::Byte(byte) => {
					bytes[len] = byte;
					len += 1;
					if len == bytes.len() {
						break;
					}
				},
				pat::Atom::Save(_) | pat::Atom::IsAlign(_) | pat::Atom::Nop => {},
				_ => break,
			}
		}
		(bytes, len)
	}
	/// Finds the next starting RVA, writing captures into `save`.
	///
	/// Supply at least `pattern::save_len(pattern)` slots. On `None`, the buffer
	/// contents are unspecified. Invalid section byte extents are skipped, even if
	/// the section was selected.
	pub fn next(&mut self, save: &mut [Rva]) -> Option<Rva> {
		assert!(
			save.len() > 255 || save.len() >= pat::save_len(self.pat),
			"save array too small: pattern requires {} slots, got {}",
			pat::save_len(self.pat),
			save.len()
		);
		self.next_impl(save)
	}
	fn next_impl(&mut self, save: &mut [Rva]) -> Option<Rva> {
		let pe = self.selection.scanner.pe;
		let (prefix, prefix_len) = self.prefix();
		let prefix = &prefix[..prefix_len];
		let mut jumps = [prefix_len as u8; 256];
		for i in 0..prefix_len.saturating_sub(1) {
			jumps[prefix[i] as usize] = (prefix_len - i - 1) as u8;
		}
		loop {
			if self.active.is_none() {
				let section = PeSectionHeaders::new(self.selection.sections).as_slice().first()?;
				self.selection.sections = &self.selection.sections[1..];
				if !(self.selection.filter)(section) {
					continue;
				}
				let size = match pe.layout() {
					PeLayout::File => section.SizeOfRawData,
					PeLayout::Section => section.VirtualSize,
				};
				if size == 0 {
					continue;
				}
				let Ok(bytes) = pe.get_section_bytes(section) else {
					continue;
				};
				let cursor = cmp::max(section.VirtualAddress, self.selection.range.start);
				let end = cmp::min(section.VirtualAddress.wrapping_add(bytes.len() as u32), self.selection.range.end);
				if cursor >= end {
					continue;
				}
				self.cursor = cursor;
				self.end = end;
				self.active = Some(section);
			}
			let section = self.active.unwrap();
			let Ok(bytes) = pe.get_section_bytes(section) else {
				self.active = None;
				continue;
			};
			while self.cursor < self.end {
				let cursor = self.cursor;
				let offset = (cursor - section.VirtualAddress) as usize;
				let available = &bytes[offset..];
				// TODO: Benchmark a memchr fast path for single-byte prefixes that avoids the jump table.
				let (candidate, jump) = if prefix_len != 0 && available.len() >= prefix_len {
					(available[..prefix_len] == *prefix, jumps[available[prefix_len - 1] as usize] as u32)
				}
				else {
					// At a backing-section boundary the interpreter may still match,
					// e.g. by reading individual bytes from the next section.
					(true, 1)
				};
				self.cursor += cmp::min(jump, self.end - self.cursor);
				if candidate {
					self.hits += 1;
					if self.selection.scanner.exec(cursor, self.pat, save) {
						return Some(cursor);
					}
				}
			}
			self.active = None;
		}
	}
}

//----------------------------------------------------------------

#[cfg(test)]
#[path = "scanner/tests.rs"]
mod tests;

#[cfg(test)]
pub(crate) use tests::test_scanner;
