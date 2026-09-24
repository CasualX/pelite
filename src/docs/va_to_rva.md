Converts a virtual address to a relative virtual address.

# Errors

* [`Null`][crate::Error::Null]: `va` is zero.
* [`Bounds`][crate::Error::Bounds]: `va` lies outside the virtual image.
