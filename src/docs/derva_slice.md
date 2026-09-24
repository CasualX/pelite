Reads `len` aligned values of `T` at `rva`.

# Errors

* [`Overflow`][crate::Error::Overflow]: `len` times the size of `T` overflows.
* [`Null`][crate::Error::Null]: `rva` is zero.
* [`Bounds`][crate::Error::Bounds], [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The array cannot be read from the image.
