Returns the import address table.

# Errors

* [`Null`][crate::Error::Null]: The image has no import address table.
* [`Invalid`][crate::Error::Invalid]: The table size is not a multiple of the entry size, or its raw data range is invalid.
* [`Bounds`][crate::Error::Bounds], [`Misaligned`][crate::Error::Misaligned], or [`ZeroFill`][crate::Error::ZeroFill]: The table cannot be read from the image.
