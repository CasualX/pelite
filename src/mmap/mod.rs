/*!
Memory map files from disk.
*/

#[cfg(all(windows, feature = "winapi"))]
mod windows;
#[cfg(all(windows, feature = "winapi"))]
pub use self::windows::{FileMap, ImageMap};

#[cfg(all(unix, feature = "libc"))]
mod unix;
#[cfg(all(unix, feature = "libc"))]
pub use self::unix::FileMap;
