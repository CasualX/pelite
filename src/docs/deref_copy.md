Copies an unaligned `T` from `ptr`.

# Errors

* [`Null`][crate::Error::Null]: `ptr` is null.
* [`Bounds`][crate::Error::Bounds], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The value cannot be read from the image.
