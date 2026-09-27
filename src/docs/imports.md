Returns the import directory.

# Errors

* [`Null`][crate::Error::Null]: The image has no import directory.
* [`Bounds`][crate::Error::Bounds]: The directory entry or descriptor array lies outside the image, or the array has no terminating null descriptor.
* [`Misaligned`][crate::Error::Misaligned]: The directory data does not satisfy the required alignment.
* [`ZeroFill`][crate::Error::ZeroFill]: The requested data includes zero-filled bytes in a file section that are not backed by raw file data.
* [`Invalid`][crate::Error::Invalid]: In a file image, the containing section's declared raw data range is invalid or extends beyond the file.
