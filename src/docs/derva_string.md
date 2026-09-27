Reads a string at `rva`.

# Errors

* [`Encoding`][crate::Error::Encoding]: The bytes do not form a valid `T`.
* [`Bounds`][crate::Error::Bounds]: The address or requested size lies outside the mapped image. For a file image, the read is outside the headers and sections, or extends past the containing region.
* [`Misaligned`][crate::Error::Misaligned]: The string does not satisfy the alignment required by `T`.
* [`ZeroFill`][crate::Error::ZeroFill]: The requested data includes zero-filled bytes in a file section that are not backed by raw file data.
* [`Invalid`][crate::Error::Invalid]: In a file image, the containing section's declared raw data range is invalid or extends beyond the file.
