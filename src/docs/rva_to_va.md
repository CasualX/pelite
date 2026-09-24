Converts a relative virtual address to a virtual address.

# Errors

* [`Null`][crate::Error::Null]: `rva` is zero.
* [`Bounds`][crate::Error::Bounds]: `rva` exceeds the virtual image size.
* [`Overflow`][crate::Error::Overflow]: Adding `rva` to the image base overflows.
