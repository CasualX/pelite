Converts a relative virtual address to a virtual address.

RVA zero converts to the image base.

# Errors

* [`Bounds`][crate::Error::Bounds]: `rva` exceeds the virtual image size.
* [`Overflow`][crate::Error::Overflow]: Adding `rva` to the image base overflows.
