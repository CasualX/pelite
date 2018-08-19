use std::{iter, fmt};
use util::WideStr;

#[derive(Copy, Clone)]
pub struct StringTable<'a> {
	words: &'a [u16],
}
impl<'a> StringTable<'a> {
	pub fn new(words: &'a [u16]) -> StringTable<'a> {
		StringTable { words }
	}
}
impl<'a> IntoIterator for StringTable<'a> {
	type Item = &'a WideStr;
	type IntoIter = StringTableIter<'a>;
	fn into_iter(self) -> StringTableIter<'a> {
		StringTableIter { words: self.words }
	}
}
impl<'a> fmt::Debug for StringTable<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_tuple("StringTable").field(&self.into_iter()).finish()
	}
}

#[derive(Clone)]
pub struct StringTableIter<'a> {
	words: &'a [u16],
}
impl<'a> iter::FusedIterator for StringTableIter<'a> {}
impl<'a> Iterator for StringTableIter<'a> {
	type Item = &'a WideStr;
	fn next(&mut self) -> Option<&'a WideStr> {
		WideStr::from_words(self.words).map(|ws| {
			self.words = &self.words[ws.len() + 1..];
			ws
		})
	}
}
impl<'a> StringTableIter<'a> {
	pub fn words(&self) -> &'a [u16] {
		self.words
	}
}
impl<'a> fmt::Debug for StringTableIter<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_list().entries(self.clone()).finish()
	}
}
