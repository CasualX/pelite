Copies bytes at `ptr` into `dest`.

Allows reading of an unaligned array of data.

# Errors

* [`Null`][crate::Error::Null]: `ptr` is null.
* [`Bounds`][crate::Error::Bounds], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The destination's bytes cannot be read from the image.
