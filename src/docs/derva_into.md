Copies bytes at `rva` into `dest`.

Allows reading of an unaligned array of data.

For file-aligned images, when `zerofill` is `true`, copies the file-backed prefix
and zero-fills the remainder within the same section. When `zerofill` is `false`,
returns `ZeroFill` if any requested bytes require zero fill.
Reads cannot cross the end of that section (the larger of its virtual and raw sizes).
Missing or truncated raw file data is an error, not zero fill.
The destination is unchanged on error.
RVA zero reads the start of the image headers.

# Errors

* [`ZeroFill`][crate::Error::ZeroFill]: `zerofill` is `false` and the read includes a section's virtual zero-filled tail.
* [`Bounds`][crate::Error::Bounds]: The address or requested size lies outside the mapped image. For a file image, the read is outside the headers and sections, or extends past the containing region.
* [`Invalid`][crate::Error::Invalid]: In a file image, the containing section's declared raw data range is invalid or extends beyond the file.
