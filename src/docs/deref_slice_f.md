Reads an array of `T` at `ptr` until `f` returns `true`.

The returned slice excludes the element for which `f` returns `true`.

# Errors

* [`Bounds`][crate::Error::Bounds]: The array ends before `f` returns `true`, or the address lies outside the image.
* [`Null`][crate::Error::Null]: The pointer is null.
* [`Misaligned`][crate::Error::Misaligned]: The array is not aligned for `T`.
* [`ZeroFill`][crate::Error::ZeroFill]: The requested data includes zero-filled bytes in a file section that are not backed by raw file data.
* [`Invalid`][crate::Error::Invalid]: In a file image, the containing section's declared raw data range is invalid or extends beyond the file.
