/*!
Podness.
*/

use std::{mem, slice};

/// Defines types which can be safely `transmute`d from any bit pattern.
///
/// Types which need to be read from PE files should implement this.
///
/// # Safety
///
/// It must be safe to transmute any bit pattern (with length equal to the size of the type) to this type.
///
/// The type must be annotated by `#[repr(C)]` or equivalent and should not have any padding between its members.
///
/// # Auto derive
///
/// To help with safely implementing this trait, a proc-macro is provided to implement the `Pod` trait if the requirements are satisfied.
///
/// ```
/// #[derive(pelite::Pod)]
/// #[repr(C)]
/// struct Foo {
/// 	foo: i32,
/// }
/// ```
pub unsafe trait Pod: 'static {
	fn zeroed() -> Self where Self: Sized {
		unsafe { mem::zeroed() }
	}
	unsafe fn uninit() -> Self where Self: Sized {
		mem::uninitialized()
	}
	fn as_bytes(&self) -> &[u8] {
		unsafe { slice::from_raw_parts(self as *const _ as *const u8, mem::size_of_val(self)) }
	}
	fn as_bytes_mut(&mut self) -> &mut [u8] {
		unsafe { slice::from_raw_parts_mut(self as *mut _ as *mut u8, mem::size_of_val(self)) }
	}
	#[doc(hidden)]
	fn _static_assert() {}
}

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

unsafe impl<T: Pod> Pod for [T] {}

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
impl_pod_array!(32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47);
impl_pod_array!(48 56 64 80 100 128 160 192 256 512 768 1024 2048 4096);

/// Pod derive proc-macro implementation helper.
#[doc(hidden)]
#[macro_export]
macro_rules! derive_pod {
	(
		$(#[$meta:meta])*
		$vis:vis struct $name:ident {
			$(
				$(#[$field_meta:meta])*
				$field_vis:vis $field_name:ident: $field_ty:ty,
			)+
		}
	) => {
		unsafe impl $crate::Pod for $name
			where Self: 'static $(, $field_ty: $crate::Pod)+
		{
			#[doc(hidden)]
			fn _static_assert() {
				// TODO: Is this assertion correct?
				// let _ = std::mem::transmute::<$name, [u8; 0 $(+ mem::size_of::<$field_ty>())+]>;
			}
		}
	};
}
