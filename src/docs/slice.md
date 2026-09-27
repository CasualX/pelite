Returns the image bytes starting at `rva`.

The slice contains at least `min_size_of` bytes and extends to the end of the available data. For a file image, it ends at the headers boundary or the section's raw data boundary. `align` specifies the required alignment of the returned bytes. RVA zero starts at the image headers.

# Errors

* [`Bounds`][crate::Error::Bounds]: The address or requested size lies outside the available data.
* [`ZeroFill`][crate::Error::ZeroFill]: A file image has no raw data at the requested address.
* [`Invalid`][crate::Error::Invalid]: In a file image, the containing section's declared raw data range is invalid or extends beyond the file.
* [`Misaligned`][crate::Error::Misaligned]: The returned bytes do not satisfy `align`.
