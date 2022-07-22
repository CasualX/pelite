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
#![allow(ellipsis_inclusive_range_patterns)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate no_std_compat as std;

#[macro_use]
pub mod util;

pub mod image;

pub mod stringify;

#[path = "proc-macros/pattern.rs"]
pub mod pattern;

pub use pelite_macros::pattern;

mod error;
pub use self::error::{Error, Result};

#[cfg(feature = "mmap")]
mod mmap;
#[cfg(feature = "mmap")]
pub use self::mmap::*;

pub mod pe32;
pub mod pe64;
pub(crate) mod wrap;
pub use self::wrap::*;

#[cfg(feature = "unstable")]
mod pir;
#[cfg(feature = "unstable")]
pub use self::pir::Pir;

pub use dataview::Pod;

/// Defaults to the current platform if it is available.
#[cfg(all(windows, target_pointer_width = "32"))]
pub use self::pe32 as pe;
/// Defaults to the current platform if it is available.
#[cfg(all(windows, target_pointer_width = "64"))]
pub use self::pe64 as pe;

pub mod base_relocs;

#[cfg(any(feature = "std", feature = "resources_nostd"))]
pub mod resources;
pub mod rich_structure;
pub mod security;
pub mod strings;

#[cfg(test)]
mod tests;
