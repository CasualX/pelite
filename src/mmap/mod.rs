/*!
Memory map files from disk.
*/

#[cfg(all(windows, feature = "windows-sys"))]
mod windows;
#[cfg(all(windows, feature = "windows-sys"))]
pub use self::windows::{FileMap, ImageMap};

#[cfg(all(unix, feature = "libc"))]
mod unix;
#[cfg(all(unix, feature = "libc"))]
pub use self::unix::FileMap;
