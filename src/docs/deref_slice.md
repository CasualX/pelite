Dereferences `ptr` as `len` aligned values of `T`.

# Errors

* [`Overflow`][crate::Error::Overflow]: `len` times the size of `T` overflows.
* [`Null`][crate::Error::Null]: `ptr` is null.
* [`Bounds`][crate::Error::Bounds], [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The array cannot be read from the image.
