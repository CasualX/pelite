/*!
Your adventure starts with a choice:

Do you wish to inspect 64-bit PE binares? ⟶ [continue](pe64/index.html)

Do you wish to inspect 32-bit PE binaries? ⟶ [continue](pe32/index.html)

The `pelite::pe` module is aliased to the target of the compiled crate.
Use it if you want to work with modules in your own process.
Evidently this is only available on Windows targets.

Due to small but incompatible differences the two formats are not unified.
*/

#![recursion_limit = "128"]

#[macro_use]
pub mod util;

pub mod image;

pub mod stringify;

pub mod pattern;

mod error;
pub use self::error::{Error, Result};

#[cfg(feature = "mmap")]
mod mmap;
#[cfg(all(feature = "mmap", windows))]
pub use self::mmap::{FileMap, ImageMap};
#[cfg(all(feature = "mmap", unix))]
pub use self::mmap::{FileMap};

pub mod pe64;
pub mod pe32;

mod pefile;
pub use self::pefile::PeFile;

mod pir;
pub use self::pir::Pir;

mod pod;
pub use self::pod::Pod;
pub use pelite_macros::Pod;
// Special versions of the derive proc macro for use with pelite itself
pub(crate) use self::pod::Pod as _Pod;
pub(crate) use pelite_macros::_Pod;

/// Defaults to the current platform if it is available.
#[cfg(all(windows, target_pointer_width = "32"))]
pub use self::pe32 as pe;
/// Defaults to the current platform if it is available.
#[cfg(all(windows, target_pointer_width = "64"))]
pub use self::pe64 as pe;

pub mod resources;

#[cfg(test)]
mod tests;
