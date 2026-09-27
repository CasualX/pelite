Dereferences `ptr` as `len` aligned values of `T`.

# Errors

* [`Overflow`][crate::Error::Overflow]: `len` times the size of `T` overflows.
* [`Null`][crate::Error::Null]: `ptr` is null.
* [`Bounds`][crate::Error::Bounds]: The address or requested size lies outside the mapped image. For a file image, no section contains the address, or the read extends past that section.
* [`Misaligned`][crate::Error::Misaligned]: The array is not aligned for `T`.
* [`ZeroFill`][crate::Error::ZeroFill]: The requested data includes zero-filled bytes in a file section that are not backed by raw file data.
* [`Invalid`][crate::Error::Invalid]: In a file image, the containing section's declared raw data range is invalid or extends beyond the file.
