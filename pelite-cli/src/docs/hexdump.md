Display bytes from a PE image

Usage:

    pelite-cli hexdump FILE RANGE [--format=text|json|json-pretty|nul]

RANGE is `kind:START..END` and accepts decimal or hex: `rva:4096..5000`,
`va:0x180001000..0x180001004`, or `fo:512..576`; the end is excluded.
Prefix the second number with `+` to give a length instead, for example
`rva:0x1000..+10` ends at `0x100a`.

One prefix applies to both endpoints. File offset ranges must map
to contiguous PE image bytes.

Text output shows 16 bytes per row with virtual addresses, hex bytes, and an
ASCII view. Use it to inspect data or check the bytes behind a disassembly.
Unprintable bytes appear as dots in the ASCII view.

Use format `json` for a compact array of byte values or `json-pretty` for an
indented array. JSON contains only the selected bytes, without addresses or
ASCII text.

Examples:

    pelite-cli hexdump sample.dll rva:0x1000..0x1040
    pelite-cli hexdump sample.dll fo:1024..1088
    pelite-cli hexdump sample.dll va:0x180001000..0x180001040 --format=json

Example JSON output (four bytes shown):

```json
[184, 1, 0, 0]
```

When to use:

Inspect exact bytes and nearby ASCII to check a signature match or
disassembly. Use `read` when you know the type or want to follow pointers.
