Returns the base relocation directory.

See the [base relocations][crate::base_relocs] module for its API.

# Errors

* [`Null`][crate::Error::Null]: The image has no base relocation directory.
* [`Bounds`][crate::Error::Bounds]: The directory entry is missing, or the declared directory range extends outside the mapped image or containing file section.
* [`Misaligned`][crate::Error::Misaligned]: The directory data is not aligned to a 4-byte boundary.
* [`ZeroFill`][crate::Error::ZeroFill]: The requested data includes zero-filled bytes in a file section that are not backed by raw file data.
* [`Invalid`][crate::Error::Invalid]: In a file image, the containing section's declared raw data range is invalid or extends beyond the file.
