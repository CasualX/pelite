Edit a PE file

Usage:

    pelite-cli edit FILE [-o OUTPUT] [--image] [--fix-baserelocs] [--fix-section-names]

Without `-o` (`--output`), the command rewrites FILE. With `-o`, it writes
the edited bytes to OUTPUT and leaves FILE unchanged. Select one or more
changes:

* `--image` (alias `--raw`) sets each section's file offset to its virtual
  address and its raw size to its aligned virtual size. Use it for a memory
  image laid out by virtual address.
* `--fix-baserelocs` points the base relocation directory at the `.reloc`
  section. It requires a nonempty `.reloc` section and a directory entry.
* `--fix-section-names` replaces section names with unique names inferred
  from their data directories and permissions.

When combined, the changes run in the order shown above.

Examples:

    pelite-cli edit sample.dll --fix-section-names -o sample-edited.dll
    pelite-cli edit sample.dll --fix-section-names
    pelite-cli edit image.dll --image --fix-baserelocs

When to use:

Convert a PE memory image or repair its relocation directory or section names
before analysis.
