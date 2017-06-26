
/// Offset operator for address types.
pub trait Offset<I> {
	fn offset(self, int: I) -> Self;
}

// Offset 32-bit Va.
use ::pe32::Va as Va32;
impl Offset<i32> for Va32 { fn offset(self, int: i32) -> Va32 { self.wrapping_add(int as Va32) } }
impl Offset<u32> for Va32 { fn offset(self, int: u32) -> Va32 { self.wrapping_add(int as Va32) } }
impl Offset<isize> for Va32 { fn offset(self, int: isize) -> Va32 { self.wrapping_add(int as Va32) } }
impl Offset<usize> for Va32 { fn offset(self, int: usize) -> Va32 { self.wrapping_add(int as Va32) } }

// Offset 64-bit Va.
use ::pe64::Va as Va64;
impl Offset<i32> for Va64 { fn offset(self, int: i32) -> Va64 { self.wrapping_add(int as Va64) } }
impl Offset<u32> for Va64 { fn offset(self, int: u32) -> Va64 { self.wrapping_add(int as Va64) } }
impl Offset<i64> for Va64 { fn offset(self, int: i64) -> Va64 { self.wrapping_add(int as Va64) } }
impl Offset<u64> for Va64 { fn offset(self, int: u64) -> Va64 { self.wrapping_add(int as Va64) } }
impl Offset<isize> for Va64 { fn offset(self, int: isize) -> Va64 { self.wrapping_add(int as Va64) } }
impl Offset<usize> for Va64 { fn offset(self, int: usize) -> Va64 { self.wrapping_add(int as Va64) } }
