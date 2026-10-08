Returns the relative virtual address of `symbol` within the image.

`symbol` must refer to data in `self.image()`, rather than a copy of that data.
The address is calculated from the start of `symbol`, taking the image layout into account.
For file layout, the byte offset is translated through the headers and section table.
For mapped layout, the byte offset is the RVA.

# Errors

* [`Overflow`][crate::Error::Overflow]: A matching section's virtual range overflows, or the mapped offset does not fit in an RVA.
* [`Unmapped`][crate::Error::Unmapped]: The symbol starts in raw section data absent from the mapped image.
* [`Bounds`][crate::Error::Bounds]: The symbol starts outside the headers and section raw data ranges in file layout.

# Panics

Panics if `symbol` does not refer to data within the image, just like `offset_of`.
