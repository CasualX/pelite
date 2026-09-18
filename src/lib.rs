/*!
Your adventure starts with a choice:

Do you wish to inspect 64-bit PE binaries? ⟶ [continue][crate::pe64]

Do you wish to inspect 32-bit PE binaries? ⟶ [continue][crate::pe32]

The `pelite::pe` module is aliased to the target of the compiled crate.
Use it if you want to work with modules in your own process.
Evidently this is only available on Windows targets.

## Format-agnostic API

When the format is not known in advance, start with [`PeFile`] for a file on
disk or [`PeView`] for an image mapped into memory. Their constructors detect
the format and return a [`Wrap`] containing the 32-bit or 64-bit variant. Most
operations can be used directly without matching on that variant:

```rust
# fn print_export_names(image: &[u8]) -> pelite::Result<()> {
let file = pelite::PeFile::from_bytes(image)?;
let exports = file.exports()?;

for &name_rva in exports.names()? {
	println!("{}", file.derva_c_str(name_rva)?);
}
# Ok(())
# }
```

Some structures genuinely differ between the formats. Operations returning
such structures preserve the distinction in another [`Wrap`], which can be
matched as [`Wrap::T32`] or [`Wrap::T64`].

When the format is known in advance, the [`pe32`] and [`pe64`] modules expose
the complete format-specific APIs.
*/

#![recursion_limit = "128"]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

#[macro_use]
pub mod util;

pub mod image;

pub mod stringify;

pub mod pattern;

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
pub mod debug;

pub mod resources;
pub mod rich_structure;
pub mod security;
pub mod strings;

#[cfg(test)]
mod tests;
