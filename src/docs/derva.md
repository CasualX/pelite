Reads an aligned `T` at `rva`.

# Errors

* [`Null`][crate::Error::Null]: `rva` is zero.
* [`Bounds`][crate::Error::Bounds], [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The value cannot be read from the image.
