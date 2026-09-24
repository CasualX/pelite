Reads a null-terminated C string at `rva`.

# Errors

* [`Null`][crate::Error::Null]: `rva` is zero.
* [`Encoding`][crate::Error::Encoding]: No null terminator occurs in the available bytes.
* [`Bounds`][crate::Error::Bounds], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The string cannot be read from the image.
