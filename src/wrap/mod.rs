use crate::Result;

/// Wraps 32-bit and 64-bit variants.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize), serde(untagged))]
pub enum Wrap<T32, T64> {
    T32(T32),
    T64(T64),
}

impl<Iter32: Iterator, Iter64: Iterator> Iterator for Wrap<Iter32, Iter64> {
    type Item = Wrap<Iter32::Item, Iter64::Item>;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Wrap::T32(iter32) => iter32.next().map(Wrap::T32),
            Wrap::T64(iter64) => iter64.next().map(Wrap::T64),
        }
    }
}

impl<T32, T64> Wrap<Result<T32>, Result<T64>> {
    /// Transposes a wrap of results in a result of a wrap.
    #[inline]
    pub fn transpose(self) -> Result<Wrap<T32, T64>> {
        match self {
            Wrap::T32(Ok(ok)) => Ok(Wrap::T32(ok)),
            Wrap::T32(Err(err)) => Err(err),
            Wrap::T64(Ok(ok)) => Ok(Wrap::T64(ok)),
            Wrap::T64(Err(err)) => Err(err),
        }
    }
}
impl<T32, T64> Wrap<Option<T32>, Option<T64>> {
    /// Transposes a wrap of options in an option of a wrap.
    #[inline]
    pub fn transpose(self) -> Option<Wrap<T32, T64>> {
        match self {
            Wrap::T32(Some(some)) => Some(Wrap::T32(some)),
            Wrap::T32(None) => None,
            Wrap::T64(Some(some)) => Some(Wrap::T64(some)),
            Wrap::T64(None) => None,
        }
    }
}
impl<T> Wrap<T, T> {
    /// Unwraps the wrapped value of equal types.
    #[inline]
    pub fn into(self) -> T {
        match self {
            Wrap::T32(val) => val,
            Wrap::T64(val) => val,
        }
    }
}

pub(crate) mod debug;
pub(crate) mod exports;
mod file;
mod headers;
pub(crate) mod imports;
mod load_config;
mod pe;
mod scanner;
pub(crate) mod sections;
mod tls;
mod view;

pub use self::file::PeFile;
pub(crate) use self::pe::get_section_bytes;
pub use self::pe::Align;
pub use self::view::PeView;
