Returns the load configuration directory.

# Errors

* [`Null`][crate::Error::Null]: The image has no load configuration directory.
* [`Invalid`][crate::Error::Invalid]: The directory or its embedded size is too small, or its raw data range is invalid.
* [`Bounds`][crate::Error::Bounds], [`Misaligned`][crate::Error::Misaligned], or [`ZeroFill`][crate::Error::ZeroFill]: The directory cannot be read from the image.
