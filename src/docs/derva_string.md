Reads a string at `rva`.

# Errors

* [`Null`][crate::Error::Null]: `rva` is zero.
* [`Encoding`][crate::Error::Encoding]: The bytes do not form a valid `T`.
* [`Bounds`][crate::Error::Bounds], [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The string cannot be read from the image.
