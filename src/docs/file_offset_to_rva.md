Converts a file offset to a relative virtual address.

# Errors

* [`Overflow`][crate::Error::Overflow]: A matching section's virtual range overflows.
* [`Unmapped`][crate::Error::Unmapped]: The offset refers to raw data absent from the mapped image.
* [`Bounds`][crate::Error::Bounds]: The offset lies outside the headers and section raw data ranges.
