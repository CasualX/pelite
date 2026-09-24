Dereferences `ptr` as an aligned `T`.

# Errors

* [`Null`][crate::Error::Null]: `ptr` is null.
* [`Bounds`][crate::Error::Bounds], [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The value cannot be read from the image.
