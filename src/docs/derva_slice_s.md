Reads an array of `T` at `rva` up to a sentinel value.

The returned slice excludes the sentinel.

# Errors

* [`Bounds`][crate::Error::Bounds]: The array ends before the sentinel, or the address lies outside the image.
* [`Null`][crate::Error::Null]: `rva` is zero.
* [`Misaligned`][crate::Error::Misaligned]: The array is not aligned for `T`.
* [`ZeroFill`][crate::Error::ZeroFill] or [`Invalid`][crate::Error::Invalid]: The array refers to unavailable or invalid raw data in a file image.
