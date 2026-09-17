use super::*;
use crate::wrap::sections::SectionHeader;

/// Pattern scanner.
impl<'a, Pe32: pe32::Pe<'a>, Pe64: pe64::Pe<'a>> Wrap<pe32::Scanner<Pe32>, pe64::Scanner<Pe64>> {
	/// Selects candidate starting addresses within this RVA range across all sections.
	#[inline]
	pub fn within(self, range: ops::Range<u32>) -> Wrap<pe32::ScanSections<'a, Pe32>, pe64::ScanSections<'a, Pe64>> {
		match self {
			Wrap::T32(scanner) => Wrap::T32(scanner.within(range)),
			Wrap::T64(scanner) => Wrap::T64(scanner.within(range)),
		}
	}
	/// Scan in executable sections.
	#[inline]
	pub fn code(self) -> Wrap<pe32::ScanSections<'a, Pe32>, pe64::ScanSections<'a, Pe64>> {
		match self {
			Wrap::T32(scanner) => Wrap::T32(scanner.code()),
			Wrap::T64(scanner) => Wrap::T64(scanner.code()),
		}
	}
	/// Selects sections using a predicate, evaluated once per section during iteration.
	#[inline]
	pub fn sections<F: FnMut(&SectionHeader) -> bool>(self, filter: F) -> Wrap<pe32::ScanSections<'a, Pe32, F>, pe64::ScanSections<'a, Pe64, F>> {
		match self {
			Wrap::T32(scanner) => Wrap::T32(scanner.sections(filter)),
			Wrap::T64(scanner) => Wrap::T64(scanner.sections(filter)),
		}
	}
	/// Selects the section described by the given header.
	#[inline]
	pub fn section(self, section: &'a SectionHeader) -> Wrap<pe32::ScanSections<'a, Pe32>, pe64::ScanSections<'a, Pe64>> {
		match self {
			Wrap::T32(scanner) => Wrap::T32(scanner.section(section)),
			Wrap::T64(scanner) => Wrap::T64(scanner.section(section)),
		}
	}
	/// Pattern interpreter, returns if the pattern matches the binary image at the given rva.
	///
	/// The pattern may contain instructions to capture interesting addresses, these are stored in the save array.
	/// Out-of-bounds save-slot reads return zero and stores are ignored.
	/// Supply at least [`save_len(pat)`](pattern::save_len) slots for the given pattern.
	///
	/// If this returns `false`, the contents of the save array are unspecified.
	#[inline]
	pub fn exec(&self, cursor: u32, pat: &[pattern::Atom], save: &mut [u32]) -> bool {
		match self {
			Wrap::T32(scanner) => scanner.exec(cursor, pat, save),
			Wrap::T64(scanner) => scanner.exec(cursor, pat, save),
		}
	}
}

impl<'a, Pe32: pe32::Pe<'a>, Pe64: pe64::Pe<'a>, F: FnMut(&SectionHeader) -> bool> Wrap<pe32::ScanSections<'a, Pe32, F>, pe64::ScanSections<'a, Pe64, F>> {
	/// Intersects the candidate starting addresses with this RVA range.
	#[inline]
	pub fn within(self, range: ops::Range<u32>) -> Self {
		match self {
			Wrap::T32(selection) => Wrap::T32(selection.within(range)),
			Wrap::T64(selection) => Wrap::T64(selection.within(range)),
		}
	}
	/// Searches the selected sections.
	#[inline]
	pub fn matches<'pat>(self, pat: &'pat [pattern::Atom]) -> Wrap<pe32::ScannerMatches<'a, 'pat, Pe32, F>, pe64::ScannerMatches<'a, 'pat, Pe64, F>> {
		match self {
			Wrap::T32(selection) => Wrap::T32(selection.matches(pat)),
			Wrap::T64(selection) => Wrap::T64(selection.matches(pat)),
		}
	}
	/// Finds a unique match and returns its starting RVA.
	///
	/// After checking uniqueness, re-executes the pattern at the match to populate `save`.
	/// Returns `None` if there are zero or multiple matches, or re-execution fails.
	/// The contents of `save` are unspecified on failure.
	///
	/// Supply at least [`save_len(pat)`](pattern::save_len) slots.
	#[inline]
	pub fn find(self, pat: &[pattern::Atom], save: &mut [u32]) -> Option<u32> {
		match self {
			Wrap::T32(selection) => selection.find(pat, save),
			Wrap::T64(selection) => selection.find(pat, save),
		}
	}
}

impl<'a, 'pat, Pe32: pe32::Pe<'a>, Pe64: pe64::Pe<'a>, F: FnMut(&SectionHeader) -> bool> Wrap<pe32::ScannerMatches<'a, 'pat, Pe32, F>, pe64::ScannerMatches<'a, 'pat, Pe64, F>> {
	/// Gets the scanner instance.
	#[inline]
	pub fn scanner(&self) -> Wrap<pe32::Scanner<Pe32>, pe64::Scanner<Pe64>> {
		match self {
			Wrap::T32(matches) => Wrap::T32(matches.scanner()),
			Wrap::T64(matches) => Wrap::T64(matches.scanner()),
		}
	}
	/// Gets the pattern.
	#[inline]
	pub fn pattern(&self) -> &'pat [pattern::Atom] {
		match self {
			Wrap::T32(matches) => matches.pattern(),
			Wrap::T64(matches) => matches.pattern(),
		}
	}
	/// Number of calls to the pattern interpreter.
	#[inline]
	pub fn hits(&self) -> u64 {
		match self {
			Wrap::T32(matches) => matches.hits(),
			Wrap::T64(matches) => matches.hits(),
		}
	}
	/// Finds the next starting RVA, writing captures into the supplied scratch buffer.
	#[inline]
	pub fn next(&mut self, save: &mut [u32]) -> Option<u32> {
		match self {
			Wrap::T32(matches) => matches.next(save),
			Wrap::T64(matches) => matches.next(save),
		}
	}
}
