Copies an unaligned `T` from `rva`.

# Errors

* [`Null`][crate::Error::Null]: `rva` is zero.
* [`Bounds`][crate::Error::Bounds], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The value cannot be read from the image.
