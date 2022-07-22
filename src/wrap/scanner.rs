use super::Wrap;
use crate::*;
use std::ops::Range;

/// Pattern scanner.
impl<'a, Pe32: pe32::Pe<'a>, Pe64: pe64::Pe<'a>>
    Wrap<pe32::scanner::Scanner<Pe32>, pe64::scanner::Scanner<Pe64>>
{
    /// Finds the unique match for the pattern in the given range.
    #[inline]
    pub fn finds(&self, pat: &[pattern::Atom], range: Range<u32>, save: &mut [u32]) -> bool {
        match self {
            Wrap::T32(scanner) => scanner.finds(pat, range, save),
            Wrap::T64(scanner) => scanner.finds(pat, range, save),
        }
    }
    /// Finds the unique code match for the pattern.
    #[inline]
    pub fn finds_code(&self, pat: &[pattern::Atom], save: &mut [u32]) -> bool {
        match self {
            Wrap::T32(scanner) => scanner.finds_code(pat, save),
            Wrap::T64(scanner) => scanner.finds_code(pat, save),
        }
    }
    /// Returns an iterator over the matches of a pattern within the given range.
    #[inline]
    pub fn matches<'pat>(
        &self,
        pat: &'pat [pattern::Atom],
        range: Range<u32>,
    ) -> Wrap<pe32::scanner::Matches<'pat, Pe32>, pe64::scanner::Matches<'pat, Pe64>> {
        match self {
            Wrap::T32(scanner) => Wrap::T32(scanner.matches(pat, range)),
            Wrap::T64(scanner) => Wrap::T64(scanner.matches(pat, range)),
        }
    }
    /// Returns an iterator over the code matches of a pattern.
    #[inline]
    pub fn matches_code<'pat>(
        &self,
        pat: &'pat [pattern::Atom],
    ) -> Wrap<pe32::scanner::Matches<'pat, Pe32>, pe64::scanner::Matches<'pat, Pe64>> {
        match self {
            Wrap::T32(scanner) => Wrap::T32(scanner.matches_code(pat)),
            Wrap::T64(scanner) => Wrap::T64(scanner.matches_code(pat)),
        }
    }
    /// Pattern interpreter, returns if the pattern matches the binary image at the given rva.
    #[inline]
    pub fn exec(&self, cursor: u32, pat: &[pattern::Atom], save: &mut [u32]) -> bool {
        match self {
            Wrap::T32(scanner) => scanner.exec(cursor, pat, save),
            Wrap::T64(scanner) => scanner.exec(cursor, pat, save),
        }
    }
}

impl<'a, 'pat, Pe32: pe32::Pe<'a>, Pe64: pe64::Pe<'a>>
    Wrap<pe32::scanner::Matches<'pat, Pe32>, pe64::scanner::Matches<'pat, Pe64>>
{
    /// Gets the scanner instance.
    #[inline]
    pub fn scanner(&self) -> Wrap<pe32::scanner::Scanner<Pe32>, pe64::scanner::Scanner<Pe64>> {
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
    /// Gets the remaining range to scan.
    #[inline]
    pub fn range(&self) -> Range<u32> {
        match self {
            Wrap::T32(matches) => matches.range(),
            Wrap::T64(matches) => matches.range(),
        }
    }
    /// Performance counter.
    #[inline]
    pub fn hits(&self) -> u32 {
        match self {
            Wrap::T32(matches) => matches.hits(),
            Wrap::T64(matches) => matches.hits(),
        }
    }
    /// Finds the next match with the given save array.
    #[inline]
    pub fn next(&mut self, save: &mut [u32]) -> bool {
        match self {
            Wrap::T32(matches) => matches.next(save),
            Wrap::T64(matches) => matches.next(save),
        }
    }
}
