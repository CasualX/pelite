Returns the base relocation directory.

See the [base relocations][crate::base_relocs] module for its API.

# Errors

* [`Null`][crate::Error::Null]: The image has no base relocation directory.
* [`Bounds`][crate::Error::Bounds], [`Misaligned`][crate::Error::Misaligned], [`ZeroFill`][crate::Error::ZeroFill], or [`Invalid`][crate::Error::Invalid]: The declared directory range cannot be read from the image.
