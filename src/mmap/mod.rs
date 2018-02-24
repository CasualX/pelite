/*!
Memory map files from disk.
*/

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use self::windows::{FileMap, ImageMap};

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use self::unix::{FileMap};

/// Memory protection values.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Protect {
	ReadOnly,
	ReadWrite,
	ExecuteRead,
	ExecuteReadWrite,
}
