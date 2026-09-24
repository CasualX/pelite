Returns the image bytes starting at `va` without a size or alignment requirement.

Shorthand for [`read(va, 0, 1)`][Self::read].

# Errors

* [`Null`][crate::Error::Null]: `va` is zero.
* [`Bounds`][crate::Error::Bounds]: The address lies outside the available data.
* [`ZeroFill`][crate::Error::ZeroFill]: A file image has no raw data at the address.
* [`Invalid`][crate::Error::Invalid]: A section's raw data range is invalid in a file image.
