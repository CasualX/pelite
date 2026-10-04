Edit a PE file

Usage:

    pelite-cli edit FILE [-o OUTPUT] [--image] [--fix-baserelocs] [--rebase N] [--fix-section-names]

Without `-o` (`--output`), the command rewrites FILE. With `-o`, it writes
the edited bytes to OUTPUT and leaves FILE unchanged. Select one or more
changes:

* `--image` (alias `--raw`) sets each section's file offset to its virtual
  address and its raw size to its aligned virtual size. Use it for a memory
  image laid out by virtual address.
* `--fix-baserelocs` points the base relocation directory at the `.reloc`
  section. It requires a nonempty `.reloc` section and a directory entry.
* `--rebase N` applies base relocations using the difference between N and
  the current `ImageBase`. N accepts decimal or `0x` hexadecimal addresses.
  PE32 applies `HIGHLOW` fixups; PE32+ applies `DIR64` fixups. Both update
  `ImageBase`. Requires a valid relocation directory.
* `--fix-section-names` replaces section names with unique names inferred
  from their data directories and permissions.

When combined, the changes run in the order shown above.

Examples:

    pelite-cli edit sample.dll --fix-section-names -o sample-edited.dll
    pelite-cli edit sample.dll --fix-section-names
    pelite-cli edit image.dll --image --fix-baserelocs
    pelite-cli edit sample.dll --rebase 0x180000000 -o rebased.dll

When to use:

Convert a PE memory image, rebase pointers, or repair its relocation directory
or section names before analysis.
