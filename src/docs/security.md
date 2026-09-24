Returns the security directory.

# Errors

* [`Null`][crate::Error::Null]: The image has no security directory.
* [`Unmapped`][crate::Error::Unmapped]: Security data is unavailable in a mapped image.
* [`Misaligned`][crate::Error::Misaligned]: The file offset or size is not eight-byte aligned.
* [`Bounds`][crate::Error::Bounds]: The directory entry is absent, its size is zero, or its range lies outside the file.
* [`Overflow`][crate::Error::Overflow]: The file offset and size overflow when added.
