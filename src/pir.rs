use crate::Pod;
use std::marker::PhantomData;
use std::{cmp, fmt, hash, mem, str};

/// Typed relative virtual address.
#[repr(transparent)]
pub struct Pir<T: ?Sized = ()> {
    rva: u32,
    marker: PhantomData<fn() -> T>,
}

impl<T: ?Sized> Pir<T> {
    /// Null pointer constant.
    pub const NULL: Pir<T> = Pir {
        rva: 0,
        marker: PhantomData,
    };
    /// Creates a null pointer.
    pub fn null() -> Pir<T> {
        Pir::NULL
    }
    /// Returns true if the pointer is null.
    pub fn is_null(self) -> bool {
        self.rva == 0
    }
    /// Casts the pointer to a different type keeping the pointer address fixed.
    pub fn cast<U: ?Sized>(self) -> Pir<U> {
        Pir {
            rva: self.rva,
            marker: PhantomData,
        }
    }
    /// Offsets and casts the pointer to a differen type.
    ///
    /// Because the type of the current and the target may be unrelated, this is a byte offset.
    pub fn offset<U: ?Sized>(self, offset: i32) -> Pir<U> {
        let rva = self.rva.wrapping_add(offset as u32);
        Pir {
            rva,
            marker: PhantomData,
        }
    }
    /// Returns the raw integer, type ascription helper.
    pub fn into_raw(self) -> u32 {
        self.rva
    }
    /// Formats the pointer.
    fn fmt(self) -> [u8; 10] {
        let mut s = [0; 10];
        s[0] = b'0';
        s[1] = b'x';
        let mut rva = self.rva;
        for i in 0..8 {
            rva = rva.rotate_left(4);
            let digit = (rva & 0xf) as u8;
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
impl<T> Pir<[T]> {
    /// Decays the pointer from `[T]` to `T`.
    pub fn decay(self) -> Pir<T> {
        Pir {
            rva: self.rva,
            marker: PhantomData,
        }
    }
    /// Pointer arithmetic, gets the pointer of an element at the specified index.
    pub fn at(self, i: usize) -> Pir<T> {
        let rva = self.rva + (i * mem::size_of::<T>()) as u32;
        Pir {
            rva,
            marker: PhantomData,
        }
    }
}

unsafe impl<T: ?Sized> Pod for Pir<T> where Pir<T>: 'static {}

impl<T: ?Sized> Copy for Pir<T> {}
impl<T: ?Sized> Clone for Pir<T> {
    fn clone(&self) -> Pir<T> {
        *self
    }
}
impl<T: ?Sized> Default for Pir<T> {
    fn default() -> Pir<T> {
        Pir::NULL
    }
}
impl<T: ?Sized> Eq for Pir<T> {}
impl<T: ?Sized> PartialEq for Pir<T> {
    fn eq(&self, rhs: &Pir<T>) -> bool {
        self.rva == rhs.rva
    }
}
impl<T: ?Sized> PartialOrd for Pir<T> {
    fn partial_cmp(&self, rhs: &Pir<T>) -> Option<cmp::Ordering> {
        self.rva.partial_cmp(&rhs.rva)
    }
}
impl<T: ?Sized> Ord for Pir<T> {
    fn cmp(&self, rhs: &Pir<T>) -> cmp::Ordering {
        self.rva.cmp(&rhs.rva)
    }
}
impl<T: ?Sized> hash::Hash for Pir<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.rva.hash(state)
    }
}
impl<T: ?Sized> From<u32> for Pir<T> {
    fn from(rva: u32) -> Pir<T> {
        Pir {
            rva,
            marker: PhantomData,
        }
    }
}
impl<T: ?Sized> From<Pir<T>> for u32 {
    fn from(pir: Pir<T>) -> u32 {
        pir.rva
    }
}
impl<T: ?Sized> AsRef<u32> for Pir<T> {
    fn as_ref(&self) -> &u32 {
        &self.rva
    }
}
impl<T: ?Sized> AsMut<u32> for Pir<T> {
    fn as_mut(&mut self) -> &mut u32 {
        &mut self.rva
    }
}
impl<T: ?Sized> fmt::Debug for Pir<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let buf = Pir::fmt(*self);
        f.write_str(unsafe { str::from_utf8_unchecked(&buf) })
    }
}
impl<T: ?Sized> fmt::UpperHex for Pir<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.rva.fmt(f)
    }
}
impl<T: ?Sized> fmt::LowerHex for Pir<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.rva.fmt(f)
    }
}
impl<T: ?Sized> fmt::Display for Pir<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let buf = Pir::fmt(*self);
        f.write_str(unsafe { str::from_utf8_unchecked(&buf) })
    }
}

#[cfg(feature = "serde")]
impl<T: ?Sized> serde::Serialize for Pir<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u32(self.rva)
    }
}
