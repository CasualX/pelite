# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

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
