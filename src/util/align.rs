
/// Ensures alignment of at least 16 bytes.
#[repr(align(16))]
pub struct Align16<T>(pub T);

/// Ensures alignment of at least 512 bytes.
#[repr(align(512))]
pub struct Align512<T>(pub T);

/// Ensures alignment of at least 4K bytes.
#[repr(align(4096))]
pub struct Align4K<T>(pub T);

/// Helper to implement generic alignment operator.
pub trait AlignTo {
	fn align_to(self, align: Self) -> Self;
}

macro_rules! impl_align_to {
	($ty:ty) => {
		impl AlignTo for $ty {
			fn align_to(self, align: $ty) -> $ty {
				debug_assert!(align.is_power_of_two());
				(self.wrapping_sub(1) & !align.wrapping_sub(1)).wrapping_add(align)
			}
		}
	};
}
impl_align_to!(u8);
impl_align_to!(u16);
impl_align_to!(u32);
impl_align_to!(u64);
impl_align_to!(u128);
impl_align_to!(usize);

/// Aligns the given value to a multiple of the align value.
///
/// Debug asserts that align is a power of two, otherwise calculates an incorrect result.
pub fn align_to<T: AlignTo>(value: T, align: T) -> T {
	value.align_to(align)
}

#[test]
fn test_align_to() {
	assert_eq!(0, align_to(0_u32, 256_u32));
	assert_eq!(0, align_to(!0_u32, 256_u32));
	assert_eq!(128, align_to(65_u32, 64_u32));
	assert_eq!(64, align_to(64_u32, 64_u32));
}
