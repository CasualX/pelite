Returns the virtual address of an exported symbol.

Forwarded exports are not resolved. Repeated lookups are less efficient than caching an `ExportBy` instance.

# Errors

* [`Null`][crate::Error::Null]: The image has no exports, the requested name is absent, the export has no address, or the export is forwarded.
* [`Bounds`][crate::Error::Bounds]: The ordinal or export table index is invalid, or the export address lies outside the image.
* [`Overflow`][crate::Error::Overflow]: Converting the export address to a virtual address overflows.
* Other errors indicate malformed export tables or data that cannot be read.
