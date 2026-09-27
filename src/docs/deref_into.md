Copies bytes at `ptr` into `dest`.

Allows reading of an unaligned array of data.

For file-aligned images, copies the file-backed prefix and zero-fills the remainder
within the same section. Reads cannot cross the end of that section (the larger of
its virtual and raw sizes). Missing or truncated raw file data is an error, not zero fill.
The destination is unchanged on error.

# Errors

* [`Null`][crate::Error::Null]: `ptr` is null.
* [`Bounds`][crate::Error::Bounds]: The address or requested size lies outside the mapped image. For a file image, the read is outside the headers and sections, or extends past the containing region.
* [`Invalid`][crate::Error::Invalid]: In a file image, the containing section's declared raw data range is invalid or extends beyond the file.
