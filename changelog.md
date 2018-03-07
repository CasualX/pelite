# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [0.5.0] 2018-02-21

### Added

- Add pattern atoms
  - Fuzzy masking atom allows bytes to be masked before being compared.
  - Many atom skips up to a limited number of bytes, similar to `.{0,limit}` regex.
  - Pir atom enables following pointer references in position independent executables (PIE).
- Pattern scanner accepts a `&mut [Rva]` save array instead of returning a `Match` result of fixed size.
- Convenience methods for the `Export` enum to make the common case more ergonomic.
- Implement convenient iteration over exported symbols and names.
- Introduce convenient `get_proc_address` to easily query exports with a small performance trade-off.
- Introduce alignment query to `Pe` trait, allows to know if the image has file alignment or section alignment.
- Findsig binary tool to interactively scan binaries for patterns using the powerful pattern scanner.
- Examples
  - Generate random bytes with a markov chain over bytes based on x86 instructions.
  - Demonstrate how to use pelite to automate analysis of [Team Fortress 2](https://en.wikipedia.org/wiki/Team_Fortress_2) binaries.
  - Demonstrate how to use pelite to automate analysis of [Counter-Strike: Global Offensive](https://en.wikipedia.org/wiki/Counter-Strike:_Global_Offensive) binaries.

### Changed

- Improved pattern scanner by optimizing it for `PeView`.
- Intent to deprecate `Match` and all related methods.
- Renamed specific resources find error to `FindError::Bad8Path`.
- Renamed pemoddef binary to module-def.
- Allow references to virtual address space in section padding.
- Allow slicing one byte past the end of a section with length zero.
- Fix incorrect error when slicing `PeFile` and overlapping sections.
- Guide the compiler to generate slightly more efficient code reading or slicing the image.
- When reading or slicing `debug_assert!` that align is a power of 2.
- Better testing of image struct sizes are correct.
- Use winapi in the mmap module instead of doing it yolo.

## [0.4.0] 2017-09-06

### Added

- Typed virtual address pointers.
  - Dereference these typed virtual address pointers in the context of a PE image.
- Pattern Scanning for PE images.
  - Signatures capable of following jumps to subroutines and data access.
  - Textual and binary representations of signatures.
  - Optimized quicksearch makes even advanced patterns fast to find.
- PE Module-Definition Generator
  - Generates a `.def` file for a DLL which can be used to create an Import Library to link against.

### Changed

- Simplify the readme error handling.
- **BREAKING** Refactor PE reading functions:
  - Unify the slice reading functions with trait-based overloading.
  - Equivalent functions for both `Va` and `Rva` addresses.
  - Fixes subtle semantic differences between null addresses.
- AppVeyor now also tests `--release` versions in case of unsafe code regressions.
- Refactoring to improve code quality without changing semantics.
- Documentation typos and wordings fixed.
- MSVC data structures now have documentation.
- Made the `Pe` trait unsafe, as it makes unsafe assumptions that the PE headers are valid.

## [0.3.0] 2017-06-07

Completely rewrote the library with proper error handling in mind.
