Returns the image bytes starting at `rva` without a size or alignment requirement.

Shorthand for [`slice(rva, 0, 1)`][Self::slice].

# Errors

* [`Bounds`][crate::Error::Bounds]: The address lies outside the available data.
* [`ZeroFill`][crate::Error::ZeroFill]: A file image has no raw data at the address.
* [`Invalid`][crate::Error::Invalid]: In a file image, the containing section's declared raw data range is invalid or extends beyond the file.
