Returns the debug directory.

# Errors

* [`Null`][crate::Error::Null]: The image has no debug directory.
* [`Invalid`][crate::Error::Invalid]: The directory size is not a multiple of the entry size, or its raw data range is invalid.
* [`Bounds`][crate::Error::Bounds], [`Misaligned`][crate::Error::Misaligned], or [`ZeroFill`][crate::Error::ZeroFill]: The directory cannot be read from the image.
