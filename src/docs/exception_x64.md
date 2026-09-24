Returns the x64 exception directory.

# Errors

* [`Null`][crate::Error::Null]: The image has no exception directory.
* [`Invalid`][crate::Error::Invalid]: The machine is not x64 or the table size is not a multiple of the entry size.
* [`Bounds`][crate::Error::Bounds], [`Misaligned`][crate::Error::Misaligned], or [`ZeroFill`][crate::Error::ZeroFill]: The function table cannot be read.
