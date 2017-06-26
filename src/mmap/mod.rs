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
