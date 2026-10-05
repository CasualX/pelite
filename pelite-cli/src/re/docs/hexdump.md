Display bytes from a PE image

Usage:

    pelite-cli re hexdump FILE ADDRESS BYTES [-o OUTPUT] [--format=text|json|json-pretty|nul]

See `pelite-cli re --help` for ADDRESS syntax (`rva:`, `va:`, or `fo:`).
BYTES is the number of bytes to dump, in decimal or 0xhex.

Text output shows 16 bytes per row with virtual addresses, hex bytes, and an
ASCII view. Use it to inspect data or check the bytes behind a disassembly.
Unprintable bytes appear as dots in the ASCII view.

Use format `json` for a compact array of byte values or `json-pretty` for an
indented array. JSON contains only the selected bytes, without addresses or
ASCII text.

Use `-o OUTPUT` (or `--output OUTPUT`) to write the selected raw bytes to a
new file instead of printing them. The file must not already exist. This
option takes precedence over `--format`.

Examples:

    pelite-cli re hexdump sample.dll rva:0x1000 0x1040
    pelite-cli re hexdump sample.dll fo:1024 1088
    pelite-cli re hexdump sample.dll va:0x180001000 0x40 --format=json
    pelite-cli re hexdump sample.dll rva:0x1000 64 -o bytes.bin --format=nul

Example JSON output (four bytes shown):

```json
[184, 1, 0, 0]
```

When to use:

Inspect exact bytes and nearby ASCII to check a signature match or
disassembly. Use `read` when you know the type or want to follow pointers.
Use `hexdump -o OUTPUT` to extract selected bytes to disk.
