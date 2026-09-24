Returns the TLS directory.

# Errors

* [`Null`][crate::Error::Null]: The image has no TLS directory.
* [`Bounds`][crate::Error::Bounds]: The declared directory is too small or lies outside the image.
* [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The directory cannot be read from the image.
