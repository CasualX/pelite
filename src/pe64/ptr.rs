/*!
Typed virtual address.
*/

use std::{cmp, fmt, mem, ops, str};
use std::marker::PhantomData;

use util::{Pod};

use super::image::{Va, SignedVa};

//----------------------------------------------------------------

/// Typed virtual address.
pub struct Ptr<T: ?Sized = ()>(Va, PhantomData<fn() -> T>);

unsafe impl<T: ?Sized + 'static> Pod for Ptr<T> {}

impl<T: ?Sized> Ptr<T> {
	/// Creates a null pointer.
	pub fn null() -> Ptr<T> {
		Ptr(0, PhantomData)
	}
	/// Returns true if the pointer is null.
	pub fn is_null(self) -> bool {
		self.0 == 0
	}
	/// Casts the pointer to a different type keeping the pointer address fixed.
	pub fn cast<U: ?Sized>(self) -> Ptr<U> {
		Ptr(self.0, PhantomData)
	}
	/// Offset and cast the pointer.
	///
	/// Access nearby values (such as data members) by offsetting and casting the pointer.
	///
	/// Because the type of the current and the target may be unrelated, this is a byte offset.
	///
	/// # Examples
	///
	/// ```
	/// use pelite::pe64::Ptr;
	///
	/// # #[allow(dead_code)]
	/// #[repr(C)]
	/// struct Composite {
	/// 	int: i32,
	/// 	float: f32,
	/// }
	///
	/// let p: Ptr<Composite> = Ptr::from(0x2000);
	/// let target: Ptr<f32> = p.shift(4);
	///
	/// assert_eq!(target, Ptr::from(0x2004));
	/// ```
	pub fn shift<U: ?Sized>(self, offset: Va) -> Ptr<U> {
		Ptr(self.0 + offset, PhantomData)
	}
	/// Simply formats the pointer.
	///
	/// # Examples
	///
	/// ```
	/// use std::str;
	/// use pelite::pe32::Ptr;
	///
	/// let p: Ptr<i32> = Ptr::from(0x1f00);
	/// let fmts = p.fmt();
	///
	/// // Guaranteed to be safe
	/// let s = unsafe { str::from_utf8_unchecked(&fmts) };
	/// assert_eq!(s, "0x00001f00");
	/// ```
	pub fn fmt(self) -> [u8; mem::size_of::<Va>() * 2 + 2] {
		let mut s = [0; mem::size_of::<Va>() * 2 + 2];
		s[0] = b'0';
		s[1] = b'x';
		let mut va = self.0;
		for i in 0..mem::size_of::<Va>() * 2 {
			va = va.rotate_left(4);
			let digit = (va & 0xf) as u8;
			let chr = if digit < 10 { b'0' + digit } else { b'a' + (digit - 10) };
			s[i + 2] = chr;
		}
		return s;
	}
}

impl<T: ?Sized> Copy for Ptr<T> {}
impl<T: ?Sized> Clone for Ptr<T> {
	fn clone(&self) -> Ptr<T> {
		*self
	}
}

impl<T: ?Sized> Default for Ptr<T> {
	fn default() -> Ptr<T> {
		Ptr(0, PhantomData)
	}
}

impl<T: ?Sized> Eq for Ptr<T> {}
impl<T: ?Sized> PartialEq for Ptr<T> {
	fn eq(&self, rhs: &Ptr<T>) -> bool {
		self.0 == rhs.0
	}
}

impl<T: ?Sized> PartialOrd for Ptr<T> {
	fn partial_cmp(&self, rhs: &Ptr<T>) -> Option<cmp::Ordering> {
		self.0.partial_cmp(&rhs.0)
	}
}
impl<T: ?Sized> Ord for Ptr<T> {
	fn cmp(&self, rhs: &Ptr<T>) -> cmp::Ordering {
		self.0.cmp(&rhs.0)
	}
}

impl<T: ?Sized> From<Va> for Ptr<T> {
	fn from(va: Va) -> Ptr<T> {
		Ptr(va, PhantomData)
	}
}
impl<T: ?Sized> From<Ptr<T>> for Va {
	fn from(ptr: Ptr<T>) -> Va {
		ptr.0
	}
}

impl<T: ?Sized> AsRef<Va> for Ptr<T> {
	fn as_ref(&self) -> &Va {
		&self.0
	}
}
impl<T: ?Sized> AsMut<Va> for Ptr<T> {
	fn as_mut(&mut self) -> &mut Va {
		&mut self.0
	}
}

impl<T: Pod> ops::Add<SignedVa> for Ptr<T> {
	type Output = Ptr<T>;
	fn add(self, rhs: SignedVa) -> Ptr<T> {
		let va = self.0.wrapping_add((rhs * mem::size_of::<T>() as SignedVa) as Va);
		Ptr(va, PhantomData)
	}
}
impl<T: Pod> ops::Add<SignedVa> for Ptr<[T]> {
	type Output = Ptr<T>;
	fn add(self, rhs: SignedVa) -> Ptr<T> {
		let va = self.0.wrapping_add((rhs * mem::size_of::<T>() as SignedVa) as Va);
		Ptr(va, PhantomData)
	}
}
impl<T: Pod> ops::Sub<SignedVa> for Ptr<T> {
	type Output = Ptr<T>;
	fn sub(self, rhs: SignedVa) -> Ptr<T> {
		let va = self.0.wrapping_sub((rhs * mem::size_of::<T>() as SignedVa) as Va);
		Ptr(va, PhantomData)
	}
}
impl<T: Pod> ops::Sub<SignedVa> for Ptr<[T]> {
	type Output = Ptr<T>;
	fn sub(self, rhs: SignedVa) -> Ptr<T> {
		let va = self.0.wrapping_sub((rhs * mem::size_of::<T>() as SignedVa) as Va);
		Ptr(va, PhantomData)
	}
}

impl<T: ?Sized> fmt::Debug for Ptr<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let buf = Ptr::fmt(*self);
		f.write_str(unsafe { str::from_utf8_unchecked(&buf) })
	}
}
impl<T: ?Sized> fmt::Display for Ptr<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let buf = Ptr::fmt(*self);
		f.write_str(unsafe { str::from_utf8_unchecked(&buf) })
	}
}

#[cfg(feature = "serde")]
impl<T: ?Sized> ::serde::Serialize for Ptr<T> {
	fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		branch! {
			pe32 { serializer.serialize_u32(self.0) }
			pe64 { serializer.serialize_u64(self.0) }
		}
	}
}
