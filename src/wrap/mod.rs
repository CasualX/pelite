use core::{fmt, mem, ops, slice, str};

use crate::Pod;
use crate::util::{CStr, FromBytes};
use crate::{Error, Result, image, pattern, pe32, pe64, rich_structure};

/// Wraps 32-bit and 64-bit variants.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum Wrap<T32, T64> {
	T32(T32),
	T64(T64),
}

impl<Iter32: Iterator, Iter64: Iterator> Iterator for Wrap<Iter32, Iter64> {
	type Item = Wrap<Iter32::Item, Iter64::Item>;
	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		match self {
			Wrap::T32(iter32) => iter32.next().map(Wrap::T32),
			Wrap::T64(iter64) => iter64.next().map(Wrap::T64),
		}
	}
	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		match self {
			Wrap::T32(iter32) => iter32.size_hint(),
			Wrap::T64(iter64) => iter64.size_hint(),
		}
	}
	#[inline]
	fn count(self) -> usize {
		match self {
			Wrap::T32(iter32) => iter32.count(),
			Wrap::T64(iter64) => iter64.count(),
		}
	}
	#[inline]
	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		match self {
			Wrap::T32(iter32) => iter32.nth(n).map(Wrap::T32),
			Wrap::T64(iter64) => iter64.nth(n).map(Wrap::T64),
		}
	}
}

impl<Iter32: DoubleEndedIterator, Iter64: DoubleEndedIterator> DoubleEndedIterator for Wrap<Iter32, Iter64> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		match self {
			Wrap::T32(iter32) => iter32.next_back().map(Wrap::T32),
			Wrap::T64(iter64) => iter64.next_back().map(Wrap::T64),
		}
	}
}

impl<Iter32: ExactSizeIterator, Iter64: ExactSizeIterator> ExactSizeIterator for Wrap<Iter32, Iter64> {
	#[inline]
	fn len(&self) -> usize {
		match self {
			Wrap::T32(iter32) => iter32.len(),
			Wrap::T64(iter64) => iter64.len(),
		}
	}
}

impl<Iter32: core::iter::FusedIterator, Iter64: core::iter::FusedIterator> core::iter::FusedIterator for Wrap<Iter32, Iter64> {}

impl<T32, T64> Wrap<Result<T32>, Result<T64>> {
	/// Transposes a wrapped result into a result of a wrap.
	#[inline]
	pub fn transpose(self) -> Result<Wrap<T32, T64>> {
		match self {
			Wrap::T32(Ok(ok)) => Ok(Wrap::T32(ok)),
			Wrap::T32(Err(err)) => Err(err),
			Wrap::T64(Ok(ok)) => Ok(Wrap::T64(ok)),
			Wrap::T64(Err(err)) => Err(err),
		}
	}
}
impl<T32, T64> Wrap<Option<T32>, Option<T64>> {
	/// Transposes wrapped options into an option of a wrap.
	#[inline]
	pub fn transpose(self) -> Option<Wrap<T32, T64>> {
		match self {
			Wrap::T32(Some(some)) => Some(Wrap::T32(some)),
			Wrap::T32(None) => None,
			Wrap::T64(Some(some)) => Some(Wrap::T64(some)),
			Wrap::T64(None) => None,
		}
	}
}
impl<T> Wrap<T, T> {
	/// Unwraps the wrapped value of equal types.
	#[inline]
	pub fn into(self) -> T {
		match self {
			Wrap::T32(val) => val,
			Wrap::T64(val) => val,
		}
	}
}

pub(crate) mod exports;
mod file;
mod headers;
pub(crate) mod imports;
mod load_config;
mod pe;
mod scanner;
pub(crate) mod sections;
mod tls;
mod view;

pub use self::file::PeFile;
pub use self::pe::PeLayout;
pub use self::view::PeView;

pub use self::exports::Export;
pub use self::imports::Import;
