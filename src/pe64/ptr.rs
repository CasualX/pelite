/*!
Typed virtual address.
*/

use std::{cmp, fmt};
use std::marker::PhantomData;

use util::{Offset, Pod};

use super::image::Va;

//----------------------------------------------------------------

/// Typed virtual address.
pub struct Ptr<T: ?Sized>(Va, PhantomData<*const T>);

unsafe impl<T: ?Sized + 'static> Pod for Ptr<T> {}

unsafe impl<T: ?Sized> Send for Ptr<T> {}
unsafe impl<T: ?Sized> Sync for Ptr<T> {}

impl<T: ?Sized> Ptr<T> {
	/// Creates a null pointer.
	pub fn null() -> Ptr<T> {
		Ptr(0, PhantomData)
	}
	/// Returns true if the pointer is null.
	pub fn is_null(self) -> bool {
		self.0 == 0
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
	pub fn shift<U: ?Sized, I>(self, offset: I) -> Ptr<U> where Va: Offset<I> {
		Ptr(self.0.offset(offset), PhantomData)
	}
}

impl<T: ?Sized> Copy for Ptr<T> {}
impl<T: ?Sized> Clone for Ptr<T> {
	fn clone(&self) -> Ptr<T> {
		Ptr(self.0, self.1)
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
/*
impl<T: ?Sized> ops::Deref for Ptr<T> {
	type Target = Va;
	fn deref(&self) -> &Va {
		&self.0
	}
}
impl<T: ?Sized> ops::DerefMut for Ptr<T> {
	fn deref_mut(&mut self) -> &mut Va {
		&mut self.0
	}
}
*/
impl<T: ?Sized> fmt::Debug for Ptr<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		branch! {
			pe32 { write!(f, "{:#010X}", self.0) }
			pe64 { write!(f, "{:#018X}", self.0) }
		}
	}
}
impl<T: ?Sized> fmt::Display for Ptr<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		branch! {
			pe32 { write!(f, "{:#010X}", self.0) }
			pe64 { write!(f, "{:#018X}", self.0) }
		}
	}
}
