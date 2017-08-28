use std::{mem, slice};

use util::Pod;

/// Helper trait representing the length of an array.
///
/// The length of an array can be specified by a known length of type `usize`.
///
/// Sometimes the length of an array is determined by a [sentinel value](https://en.wikipedia.org/wiki/Sentinel_value), a special value of `T` which marks the end of the array.
/// The length of the array is then specified by a callable with parameter `&'a T` returning a `bool` indicating if this value is the sentinel.
///
/// This trait has no usefulness outside `Pe::deref_slice` and `Pe::derva_slice` and should be considered an implementation detail...
pub trait SliceLen<'a, T: Pod> {
	fn min_size(&self) -> Option<usize>;
	unsafe fn slice_len(self, bytes: &'a [u8]) -> Option<&'a [T]>;
}

impl<'a, T: Pod> SliceLen<'a, T> for usize {
	#[inline]
	fn min_size(&self) -> Option<usize> { mem::size_of::<T>().checked_mul(*self) }
	#[inline]
	unsafe fn slice_len(self, bytes: &'a [u8]) -> Option<&'a [T]> {
		// Safe because the bytes slice should have at least `min_size()` length
		Some(slice::from_raw_parts(bytes.as_ptr() as *const T, self))
	}
}
impl<'a, T: Pod, F: FnMut(&'a T) -> bool> SliceLen<'a, T> for F {
	#[inline]
	fn min_size(&self) -> Option<usize> { Some(0) }
	#[inline]
	unsafe fn slice_len(mut self, bytes: &'a [u8]) -> Option<&'a [T]> {
		let size_of_t = mem::size_of::<T>();
		let mut len = 0;
		let mut size = 0;
		loop {
			// Safety critical OOB check
			if size + size_of_t > bytes.len() {
				return None;
			}
			// Safe because size is checked above and T is Pod
			let s = bytes.as_ptr().offset(size as isize) as *const T;
			if self(&*s) {
				let p = slice::from_raw_parts(bytes.as_ptr() as *const T, len);
				return Some(p);
			}
			size += size_of_t;
			len += 1;
		}
	}
}
// Needs Specialization
// impl<'a, 'v, T: Pod + PartialEq> SliceLen<'a, T> for &'v T {}
