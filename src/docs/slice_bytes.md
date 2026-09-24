Returns the image bytes starting at `rva` without a size or alignment requirement.

Shorthand for [`slice(rva, 0, 1)`][Self::slice].

# Errors

* [`Null`][crate::Error::Null]: `rva` is zero.
* [`Bounds`][crate::Error::Bounds]: The address lies outside the available data.
* [`ZeroFill`][crate::Error::ZeroFill]: A file image has no raw data at the address.
* [`Invalid`][crate::Error::Invalid]: A section's raw data range is invalid in a file image.
