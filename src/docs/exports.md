Returns the export directory.

# Errors

* [`Null`][crate::Error::Null]: The image has no export directory.
* [`Bounds`][crate::Error::Bounds]: The directory entry or export header lies outside the image.
* [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The export header cannot be read from the image.
