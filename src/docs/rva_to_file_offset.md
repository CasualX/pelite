Converts a relative virtual address to a file offset.

# Errors

* [`Overflow`][crate::Error::Overflow]: A matching section's raw data range overflows.
* [`ZeroFill`][crate::Error::ZeroFill]: The address refers to zero-filled data absent from the file.
* [`Bounds`][crate::Error::Bounds]: The address lies outside the headers and mapped section ranges.
