PeLite
======

[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/pelite.svg)](https://crates.io/crates/pelite)
[![docs.rs](https://docs.rs/pelite/badge.svg)](https://docs.rs/pelite)
[![Changelog](https://img.shields.io/badge/docs-changelog-blue.svg)](changelog.md)
[![Gate](https://github.com/CasualX/pelite/actions/workflows/gate.yml/badge.svg)](https://github.com/CasualX/pelite/actions/workflows/gate.yml)

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

The workspace includes a unified CLI for inspecting binaries and version resources,
extracting strings and icons, hashing imports, finding signatures, generating binary
data and module-definition files, and dumping MSVC RTTI.
Run `cargo run -p pelite-cli -- --help` or [try it on the demos](demo).

For a compact first look at an unfamiliar binary, run
`cargo run -p pelite-cli -- summary suspicious.exe`. Add `--format=json` for
machine-readable output. The summary reports triage clues, not a malware verdict.

Library
-------

This library is available on [crates.io](https://crates.io/crates/pelite).

Documentation can be found on [docs.rs](https://docs.rs/pelite/).

In your Cargo.toml, put

```
[dependencies]
pelite = "0.10"
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

Coverage
--------

Coverage uses Rust's LLVM source-based instrumentation through [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov).
To generate and open a local HTML report:

```console
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov --locked
cargo llvm-cov --package pelite --all-features --html --open
```

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
