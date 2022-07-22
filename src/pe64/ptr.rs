/*!
Typed virtual address.
*/

use std::marker::PhantomData;
use std::{cmp, fmt, hash, mem, str};

use crate::Pod;

use super::image::{SignedVa, Va};

/// Typed virtual address.
#[repr(transparent)]
pub struct Ptr<T: ?Sized = ()> {
    va: Va,
    marker: PhantomData<fn() -> T>,
}

impl<T: ?Sized> Ptr<T> {
    // Work around unstable const fn features
    const MARKER: PhantomData<fn() -> T> = PhantomData;
    /// Null pointer constant.
    pub const NULL: Ptr<T> = Ptr {
        va: 0,
        marker: PhantomData,
    };
    /// Creates a null pointer.
    pub const fn null() -> Ptr<T> {
        Ptr::NULL
    }
    /// Returns true if the pointer is null.
    pub const fn is_null(self) -> bool {
        self.va == 0
    }
    /// Constructs a pointer with an offset.
    pub const fn member(va: Va, offset: u32) -> Ptr<T> {
        let va = va + offset as Va;
        Ptr {
            va,
            marker: Self::MARKER,
        }
    }
    /// Casts the pointer to a different type keeping the pointer address fixed.
    pub const fn cast<U: ?Sized>(self) -> Ptr<U> {
        Ptr {
            va: self.va,
            marker: Ptr::<U>::MARKER,
        }
    }
    /// Offsets and casts the pointer.
    ///
    /// Because the type of the current and the target may be unrelated, this is a byte offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use pelite::pe64::Ptr;
    ///
    /// #[repr(C)]
    /// struct Composite {
    /// 	int: i32,
    /// 	float: f32,
    /// }
    ///
    /// let p: Ptr<Composite> = Ptr::from(0x2000);
    /// let target = p.offset::<f32>(4);
    ///
    /// assert_eq!(target, Ptr::from(0x2004));
    /// ```
    pub const fn offset<U: ?Sized>(self, offset: SignedVa) -> Ptr<U> {
        let va = self.va.wrapping_add(offset as Va);
        Ptr {
            va,
            marker: Ptr::<U>::MARKER,
        }
    }
    /// Returns the raw integer, type ascription helper.
    pub const fn into_raw(self) -> Va {
        self.va
    }
    /// Formats the pointer.
    fn fmt(self) -> [u8; mem::size_of::<Va>() * 2 + 2] {
        let mut s = [0; mem::size_of::<Va>() * 2 + 2];
        s[0] = b'0';
        s[1] = b'x';
        let mut va = self.va;
        for i in 0..mem::size_of::<Va>() * 2 {
            va = va.rotate_left(4);
            let digit = (va & 0xf) as u8;
            let chr = if digit < 10 {
                b'0' + digit
            } else {
                b'a' + (digit - 10)
            };
            s[i + 2] = chr;
        }
        return s;
    }
}
impl<T> Ptr<[T]> {
    /// Decays the pointer from `[T]` to `T`.
    pub const fn decay(self) -> Ptr<T> {
        Ptr {
            va: self.va,
            marker: Ptr::<T>::MARKER,
        }
    }
    /// Pointer arithmetic, gets the pointer of an element at the specified index.
    pub const fn at(self, i: usize) -> Ptr<T> {
        let va = self.va + (i * mem::size_of::<T>()) as Va;
        Ptr {
            va,
            marker: Ptr::<T>::MARKER,
        }
    }
}

unsafe impl<T: ?Sized> Pod for Ptr<T> where Ptr<T>: 'static {}

impl<T: ?Sized> Copy for Ptr<T> {}
impl<T: ?Sized> Clone for Ptr<T> {
    fn clone(&self) -> Ptr<T> {
        *self
    }
}
impl<T: ?Sized> Default for Ptr<T> {
    fn default() -> Ptr<T> {
        Ptr::NULL
    }
}
impl<T: ?Sized> Eq for Ptr<T> {}
impl<T: ?Sized> PartialEq for Ptr<T> {
    fn eq(&self, rhs: &Ptr<T>) -> bool {
        self.va == rhs.va
    }
}
impl<T: ?Sized> PartialOrd for Ptr<T> {
    fn partial_cmp(&self, rhs: &Ptr<T>) -> Option<cmp::Ordering> {
        self.va.partial_cmp(&rhs.va)
    }
}
impl<T: ?Sized> Ord for Ptr<T> {
    fn cmp(&self, rhs: &Ptr<T>) -> cmp::Ordering {
        self.va.cmp(&rhs.va)
    }
}
impl<T: ?Sized> hash::Hash for Ptr<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.va.hash(state)
    }
}
impl<T: ?Sized> From<Va> for Ptr<T> {
    fn from(va: Va) -> Ptr<T> {
        Ptr {
            va,
            marker: PhantomData,
        }
    }
}
impl<T: ?Sized> From<Ptr<T>> for Va {
    fn from(ptr: Ptr<T>) -> Va {
        ptr.va
    }
}
impl<T: ?Sized> AsRef<Va> for Ptr<T> {
    fn as_ref(&self) -> &Va {
        &self.va
    }
}
impl<T: ?Sized> AsMut<Va> for Ptr<T> {
    fn as_mut(&mut self) -> &mut Va {
        &mut self.va
    }
}
impl<T: ?Sized> fmt::Debug for Ptr<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let buf = Ptr::fmt(*self);
        f.write_str(unsafe { str::from_utf8_unchecked(&buf) })
    }
}
impl<T: ?Sized> fmt::UpperHex for Ptr<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.va.fmt(f)
    }
}
impl<T: ?Sized> fmt::LowerHex for Ptr<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.va.fmt(f)
    }
}
impl<T: ?Sized> fmt::Display for Ptr<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let buf = Ptr::fmt(*self);
        f.write_str(unsafe { str::from_utf8_unchecked(&buf) })
    }
}

#[cfg(feature = "serde")]
impl<T: ?Sized> serde::Serialize for Ptr<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        branch! {
            pe32 { serializer.serialize_u32(self.va) }
            pe64 { serializer.serialize_u64(self.va) }
        }
    }
}
