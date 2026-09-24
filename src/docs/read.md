Returns the image bytes starting at `va`.

The slice contains at least `min_size_of` bytes and extends to the end of the available data. For a file image, it ends at the section's raw data boundary. `align` specifies the required alignment of the returned bytes.

# Errors

* [`Null`][crate::Error::Null]: `va` is zero.
* [`Bounds`][crate::Error::Bounds]: The address or requested size lies outside the available data.
* [`ZeroFill`][crate::Error::ZeroFill]: A file image has no raw data at the requested address.
* [`Invalid`][crate::Error::Invalid]: A section's raw data range is invalid in a file image.
* [`Misaligned`][crate::Error::Misaligned]: The returned bytes do not satisfy `align`.
