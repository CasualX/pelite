/// Helper to implement generic alignment operator.
pub trait AlignTo {
    type TAlign;
    /// Aligns the given value to a multiple of the align value.
    ///
    /// Debug asserts that align is a power of two, otherwise calculates an incorrect result.
    fn align_to(self, align: Self::TAlign) -> Self;
    /// Checks if the given value is aligned to a multiple of the align value.
    ///
    /// Debug asserts that align is a power of two, otherwise calculates an incorrect result.
    fn aligned_to(self, align: Self::TAlign) -> bool;
}

macro_rules! impl_align_to {
    ($ty:ty) => {
        impl AlignTo for $ty {
            type TAlign = $ty;
            #[inline(always)]
            fn align_to(self, align: $ty) -> $ty {
                debug_assert!(
                    align.is_power_of_two(),
                    "align ({:#x}) is not a power of two",
                    align
                );
                let mask = align - 1;
                self.wrapping_add(mask) & !mask
            }
            #[inline(always)]
            fn aligned_to(self, align: $ty) -> bool {
                debug_assert!(
                    align.is_power_of_two(),
                    "align ({:#x}) is not a power of two",
                    align
                );
                let mask = align - 1;
                self & mask == 0
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

impl<T> AlignTo for *const T {
    type TAlign = usize;
    #[inline(always)]
    fn align_to(self, align: usize) -> *const T {
        (self as usize).align_to(align) as *const T
    }
    #[inline(always)]
    fn aligned_to(self, align: usize) -> bool {
        (self as usize).aligned_to(align)
    }
}

impl<T> AlignTo for *mut T {
    type TAlign = usize;
    #[inline(always)]
    fn align_to(self, align: usize) -> *mut T {
        (self as usize).align_to(align) as *mut T
    }
    #[inline(always)]
    fn aligned_to(self, align: usize) -> bool {
        (self as usize).aligned_to(align)
    }
}

#[test]
fn test_align_to() {
    assert_eq!(0, 0_u32.align_to(256_u32));
    assert_eq!(0, (!0_u32).align_to(256_u32));
    assert_eq!(128, 65_u32.align_to(64_u32));
    assert_eq!(64, 64_u32.align_to(64_u32));
}
