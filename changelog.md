# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [0.7.2] 2020-04-15

Small patch to remove the `pocs/blob` file from the published crate.
This avoids issues with antivirus false-positive detecting this file as malicious.

## [0.7.1] 2018-09-20

- Tweak the patterns documentation.
- Fix compile error with features=serde.
- Remove virtual_address method on base relocations block.

## [0.7.0] 2018-09-14

### Export and IAT Directory

- **[breaking]** Exports iter_names' iterator Item tuple is has swapped its elements to `(name, export)` to better match (_key_, _value_) semantics.
- **[breaking]** Exported symbol no longer contains the `None` value, it is returned in the outer result as error `Null`.
- Added Exports API to query non-sorted name table. The exports name table must be sorted according to the PE specification to allow binary search, an API was added to search by name in a linear fashion.
- Added IAT Directory support. Access all the imported functions directly through the IAT.

### Base Relocations Directory

- **[breaking]** Streamlined the base relocations API for the common use case, iterating over all base relocations.
- **[breaking]** Removed the main `Iterator`. Instead internal iterator methods are provided directly on the `BaseRelocations` struct.

### Exception and Security Directory

- **[major]** Implemented basic access to the exception directory!
- Fixed the Security reading from the wrong Directory index.

### Resources Directory

- **[breaking]** Removed explicit API for printing the resources filesystem, instead visualizing has moved to the `Display` implementation of `Resources` and `Directory` and changed the visualization to be more visually simple and consistent.
- **[breaking]** Changed the method to get the bytes from a data entry to the name `bytes()`.
- Added a simple file system consistency check method to the resources, checks that the internal structure of the resources filesystem is not broken.
- Cleanup use of `unsafe` in the resources code by refactoring them into a few methods in close proximity.
- **[major]** Added parser for Version Information. Read the parsed version information from the `Resources` struct.
- Added convenience method to get the Application Manifest directly from the `Resources` struct.

### Debug Directory

- **[breaking]** Redesigned the API, again. I'm almost happy with it now.
- Added support to parse the PGO debug entry.

### Scanner and Patterns

- **[breaking]** Removed the deprecated `Match` code in favour of passing the save array as a mut reference and removed the `Iterator` support for matches. Renamed the `next_match` method to `next`.
- Expanded the pattern atoms to support larger and many skips.
- Added pattern atoms to read data and write directly to the save array instead of only having bookmarks.
- Added pattern atoms to support alternation subpattern matching. This allows far more flexible patterns that can match slight variations in code.
- Rewrote the pattern parser syntax documentation with examples.
- Improve pattern parser error messages and indicate where in the string the parse went wrong.
- Added pattern string syntax for skipping a large or many number of bytes inspired by the YARA syntax.
- Added pattern string syntax for reading data and writing directly to the save array.
- Added pattern string syntax for alternation subpattern matching inspired by the YARA syntax (very similar to regular expressions).
- Added some basic testing of the scanner execution as things are getting a little more complex.

### Miscellaneous

- Added a top-level `PeFile` parser that works with both PE32 and PE32+ formats. A start to provide a better API to better support a more seamless experience handling both formats at the same time.
- Added a method to get the entire DOS image, no further inspection of the DOS image is provided.
- **[major]** Fixed potential overflow panics in address calculations. This is now a good candidate for fuzzing to ensure no further unintentional overflows are possible. When section header data overflows, the corrupt sections are ignored.
- **[breaking]** Added more strict alignment checks. Before only the offset into the image was checked for alignment, but this didn't work if the image itself wasn't align (it's a byte slice after all). Now the final pointer value is checked for alignment instead, this may result in unexpected `Misaligned` errors.
- **[breaking]** Renamed and added some `Error` values and write better documentation and error strings. Some APIs also got better documentation about the kinds of errors they return.
- **[breaking]** Group major and minor version fields in image structs in a separate struct to enable more fancy automatic formatting and serialization.
- **[breaking]** Deref functions are no longer generic over their pointer parameter. Convert bare `Va` manually by calling `.into()` when passing to the deref APIs.
- **[breaking]** Changed to use existential Iterators wherever possible, some `Iterator` types no longer exist.
- Added API to convert between the file view and section view, respectively `to_file` and `to_view`.
- Improved the text of some of the stringify strings.
- Added API to read unaligned arrays from the PE image.
- Improved documentation and examples in general.

### Cargo features

- **[major]** Optional Serde support! Enables serialization of datastructures making exporting PE details really easy. Just enable the `serde` feature, disabled by default.
- With `serde` support, optionally enable base64 encoding of certain data fields for a more compact and human friendly experience. Enable with the `base64` feature, disabled by default.
- Convenient OS memory mapping features are now optional (and enabled by default), using the `mmap` feature.

### Continuous integration

- Smarter usage of appveyor resources to reduce CI times to 20-30 minutes.
- Now supporting Travis-CI! Fixing the unix mmap code and keeping it working.
- Support coverage with OpenCppCoverage, coverage publishing is pending.

## [0.6.0] 2018-08-01

### Added

- Add convenient Error is_null.
- Add stringify module with functions to stringify many PE constants and bitflags.
- Implement LoadConfig Directory reader.
- Implement Security Directory reader.
- Implement Copy and Clone for msvc structs.
- Implement Send and Sync for Ptr type.
- Accept and test against curiously valid tiny PE files.
- Implement generic string reading functions in Pe derva_string and deref_string.
- Implement imphash example.
  - Hashes the import functions of a binary.

### Changed

- More generic GetProcAddress.
- Split overloaded Pe slice_len functions.
- **BREAKING** Rework the Debug interface completely.
- **BREAKING** Play with the Exports API.
  - Rename exports By hint_name to name_of_hint.
  - Reuse export By hint_name to get an export by its hint with fallback to its name.
- **BREAKING** Rename reading a C string to Pe derva_c_str and deref_c_str.
- **BREAKING** Reworked pointer offsetting with signed offsets.
- **BREAKING** Soft deprecate BADVA and BADRVA.
  - Their values have changed from all bits clear to all bits set.
- Rework all debug formatting code to the standard debug format.
  - This makes pedump output very ugly, but is a necessary step to rework proper serialization.
- Clean up pe32 and pe64 examples documentation.
- Fix deref_slice bounds to slice of T.
- Upgrade dependencies to their latest.

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
