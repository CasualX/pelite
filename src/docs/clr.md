Returns the Common Language Runtime directory from the COM descriptor entry.

The header exposes runtime version, flags, entry point token or RVA, and CLR
subdirectories. `metadata_root()` exposes the metadata stream directory,
table row counts, index widths, and string/GUID/blob heap values. Metadata
table rows and IL are not interpreted.

# Errors

* [`Null`][crate::Error::Null]: The image has no CLR directory.
* [`Bounds`][crate::Error::Bounds]: The directory is missing, too small, or outside the image.
* [`Invalid`][crate::Error::Invalid]: The header's `cb` is too small or exceeds the directory size, or the containing file section is malformed.
* [`Misaligned`][crate::Error::Misaligned]: The header does not satisfy the required alignment.
* [`ZeroFill`][crate::Error::ZeroFill]: The header includes bytes not backed by raw file data.
