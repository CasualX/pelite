/*!
PE32+ binary files.

[Peering Inside the PE: A Tour of the Win32 Portable Executable File Format](https://msdn.microsoft.com/en-us/library/ms809762.aspx)

# Getting started

PE files exist in two states:

* When loaded in memory ready to be executed the sections are aligned at the requested section alignment in the optional header, typically 4 KiB.

  This alignment relates to the page size of virtual memory, since each section may want different memory protection settings.

* When stored on disk this may waste space so the sections are aligned at the specified file alignment in the optional header, typically 512 bytes.

  This alignment relates to the block size of the underlying file system storage.

This library provides a way to interact with either format through the [`Pe`](trait.Pe.html) trait.

## Executable files on disk

To load a file on disk, read it into memory using the [filesystem API](https://doc.rust-lang.org/std/fs/),
read it using the [`memmap` crate](https://crates.io/crates/memmap) or get it done quickly and without any
additional dependencies using the provided [`FileMap`](../struct.FileMap.html) loader.

Take a byte slice of the entire file contents and construct it with [`PeFile::from_bytes`](struct.PeFile.html#method.from_bytes).

Import the [`Pe`](trait.Pe.html) trait to continue from here.

## Executable images in memory

To simulate the system loading and mapping images with virtual memory alignment use the [`ImageMap`](../struct.ImageMap.html) loader.
Note however that this is only available on Windows targets as this maps the image using Windows file mapping facilities.

Take a byte slice of the entire image and construct it with [`PeView::from_bytes`](struct.PeView.html#method.from_bytes).

Import the [`Pe`](trait.Pe.html) trait to continue from here.

If you don't know which to choose, go with [`PeFile`](struct.PeFile.html).

# Advanced usage

When working with already loaded libraries in your own process there is the `pelite::pe` module alias which points to the correct module for your target.
[`pelite::pe32`](../pe32/index.html) if compiled for 32-bit targets and [`pelite::pe64`](../pe64/index.html) for 64-bit targets.

Evidently this alias is only available on Windows targets.

Access the your own module with [`PeView::new`](struct.PeView.html#method.new) to construct a view into your own image.
This is mostly safe, but be cautious when using it to read from writable sections.

Access other modules in the process with [`PeView::module`](struct.PeView.html#method.module) to construct a view into other images in the process.
This is mostly safe, but be even more cautious when using it to read from writable sections since other libraries written in other languages such as C/C++ respect rust memory aliasing rules even less.

# Examples

```
# #![allow(dead_code)]
use std::{io, error, fmt};
use std::path::Path;

//----------------------------------------------------------------
// Be a good rustacean and handle errors

#[derive(Debug)]
enum Error {
	Pe(pelite::Error),
	Io(io::Error),
}
impl From<pelite::Error> for Error {
	fn from(err: pelite::Error) -> Error { Error::Pe(err) }
}
impl From<io::Error> for Error {
	fn from(err: io::Error) -> Error { Error::Io(err) }
}
impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		error::Error::description(self).fmt(f)
	}
}
impl error::Error for Error {
	fn description(&self) -> &str {
		match *self {
			Error::Pe(ref err) => err.description(),
			Error::Io(ref err) => err.description(),
		}
	}
	fn cause(&self) -> Option<&error::Error> {
		match *self {
			Error::Pe(ref err) => Some(err),
			Error::Io(ref err) => Some(err),
		}
	}
}

//----------------------------------------------------------------

fn file_map<P: AsRef<Path> + ?Sized>(path: &P) -> Result<(), Error> {
	use pelite::FileMap;
	use pelite::pe64::{Pe, PeFile};

	let path = path.as_ref();
	let file_map = FileMap::open(path)?;
	let file = PeFile::from_bytes(&file_map)?;

	// Access the file contents through the Pe trait
	let image_base = file.optional_header().ImageBase;
	println!("The preferred load address of {:?} is {}.", path, image_base);

	// See the respective modules to access other parts of the PE file.
	Ok(())
}

#[cfg(windows)]
fn image_map<P: AsRef<Path> + ?Sized>(path: &P) -> Result<(), Error> {
	use pelite::ImageMap;
	use pelite::pe64::{Pe, PeView};

	let path = path.as_ref();
	let image = ImageMap::open(path)?;
	let view = PeView::from_bytes(&image)?;

	// Access the image contents through the Pe trait
	let image_size = view.optional_header().SizeOfImage;
	println!("The size of image in memory of {:?} is {}", path, image_size);

	// See the respective modules to access other parts of the PE image.
	Ok(())
}

#[cfg(windows)]
fn image_base() {
	use pelite::pe::{Pe, PeView};

	let view = unsafe { PeView::new() };

	// Access the image contents through the Pe trait
	let image_size = view.optional_header().SizeOfImage;
	println!("The size of our image is {}", image_size);

	// See the respective modules to access other parts of the PE image.
}
```
*/

#[macro_use]
mod macros;

pub mod image;

mod pe;
mod view;
mod file;
pub mod exports;
pub mod imports;
pub mod base_relocs;
pub mod resources;
pub mod tls;
pub mod debug;
mod ptr;
pub mod scanner;

pub use self::image::{Va, Rva};
pub use self::pe::{Align, Pe};
pub use self::view::{PeView};
pub use self::file::{PeFile};
pub use self::ptr::Ptr;
