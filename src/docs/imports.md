Returns the import directory.

# Errors

* [`Null`][crate::Error::Null]: The image has no import directory.
* [`Bounds`][crate::Error::Bounds]: The directory entry or descriptor array lies outside the image, or the array has no terminating null descriptor.
* [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The descriptor array cannot be read from the image.
