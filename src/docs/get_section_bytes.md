Returns a section's bytes in the image's layout.

This is raw data for a file image and virtual data for a mapped image.

# Errors

* [`Null`][crate::Error::Null]: The section's raw data pointer or virtual address is zero.
* [`Bounds`][crate::Error::Bounds]: The section's data range lies outside the image.
