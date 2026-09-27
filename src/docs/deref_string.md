Dereferences `ptr` as a string.

# Errors

* [`Null`][crate::Error::Null]: `ptr` is null.
* [`Encoding`][crate::Error::Encoding]: The bytes do not form a valid `T`.
* [`Bounds`][crate::Error::Bounds]: The address or requested size lies outside the mapped image. For a file image, no section contains the address, or the read extends past that section.
* [`Misaligned`][crate::Error::Misaligned]: The string does not satisfy the alignment required by `T`.
* [`ZeroFill`][crate::Error::ZeroFill]: The requested data includes zero-filled bytes in a file section that are not backed by raw file data.
* [`Invalid`][crate::Error::Invalid]: In a file image, the containing section's declared raw data range is invalid or extends beyond the file.
