/*!
Podness.
*/

use std::cell;

/// Defines types which can be safely `transmute`d from any bit pattern.
///
/// Types which need to be read from PE files should implement this.
///
/// # Safety
///
/// It must be safe to transmute any byte array (with length equal to the size of the type) to this type.
///
/// The type should be annotated by `#[repr(C)]`.
pub unsafe trait Pod: 'static {}

unsafe impl Pod for i8 {}
unsafe impl Pod for i16 {}
unsafe impl Pod for i32 {}
unsafe impl Pod for i64 {}

unsafe impl Pod for u8 {}
unsafe impl Pod for u16 {}
unsafe impl Pod for u32 {}
unsafe impl Pod for u64 {}

unsafe impl Pod for f32 {}
unsafe impl Pod for f64 {}

unsafe impl<T: Pod> Pod for cell::UnsafeCell<T> {}
unsafe impl<T: Pod> Pod for cell::Cell<T> {}

macro_rules! impl_pod_array {
	($n:tt $($tail:tt)+) => {
		unsafe impl<T: Pod> Pod for [T; $n] {}
		impl_pod_array!($($tail)+);
	};
	($n:tt) => {
		unsafe impl<T: Pod> Pod for [T; $n] {}
	};
}
impl_pod_array!(0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15);
impl_pod_array!(16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31);
impl_pod_array!(32 48 64 80 100 128 160 192 256 512 768 1024 2048 4096);
