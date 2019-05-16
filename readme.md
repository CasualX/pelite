PeLite
======

[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/pelite.svg)](https://crates.io/crates/pelite)
[![docs.rs](https://docs.rs/pelite/badge.svg)](https://docs.rs/pelite)
[![Changelog](https://img.shields.io/badge/docs-changelog-blue.svg)](changelog.md)
[![Build status](https://ci.appveyor.com/api/projects/status/6obf6w6awf4b540r/branch/master?svg=true)](https://ci.appveyor.com/project/CasualX/pelite/branch/master)
[![Build Status](https://travis-ci.org/CasualX/pelite.svg?branch=master)](https://travis-ci.org/CasualX/pelite)

Lightweight, memory-safe, zero-allocation library for reading and navigating PE binaries.

Design
------

The purpose of this library is inspecting PE binaries (whether on disk or already loaded in memory).

A trade-off was made to not unify the 32-bit (PE32) and 64-bit (PE32+) formats for two reasons:

* There are small but incompatible differences, which would add overhead by requiring constant matching even if at source code level the match arms look identical.

* Most of the time you know (at build time) what format you're working with anyway.

This makes it rather awkward to work with both formats together transparently.

Note that while the correct name is PE32+, the name PE64 is used as it is a valid identifier; they are otherwise synonymous.

Tools
-----

Included are bins showing some uses for the library, [try them out on the demos](demo)!

Library
-------

This library is available on [crates.io](https://crates.io/crates/pelite).

Documentation can be found on [docs.rs](https://docs.rs/pelite/).

In your Cargo.toml, put

```
[dependencies]
pelite = "0.8"
```

Examples
--------

Try this example: `cargo run --example readme`.

```rust
use pelite::FileMap;
use pelite::pe64::{Pe, PeFile};

fn main() {
	// Load the desired file into memory
	let file_map = FileMap::open("demo/Demo64.dll").unwrap();
	// Process the image file
	dll_deps(file_map.as_ref()).unwrap();
}

fn dll_deps(image: &[u8]) -> pelite::Result<()> {
	// Interpret the bytes as a PE32+ executable
	let file = PeFile::from_bytes(image)?;

	// Let's read the DLL dependencies
	let imports = file.imports()?;
	for desc in imports {
		// Get the DLL name being imported from
		let dll_name = desc.dll_name()?;
		// Get the number of imports for this dll
		let iat = desc.iat()?;
		println!("imported {} functions from {}", iat.len(), dll_name);
	}

	Ok(())
}
```

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
