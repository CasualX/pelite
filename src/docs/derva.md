Reads an aligned `T` at `rva`.

# Errors

* [`Bounds`][crate::Error::Bounds]: The address or requested size lies outside the mapped image. For a file image, the read is outside the headers and sections, or extends past the containing region.
* [`Misaligned`][crate::Error::Misaligned]: The value is not aligned for `T`.
* [`ZeroFill`][crate::Error::ZeroFill]: The requested data includes zero-filled bytes in a file section that are not backed by raw file data.
* [`Invalid`][crate::Error::Invalid]: In a file image, the containing section's declared raw data range is invalid or extends beyond the file.
