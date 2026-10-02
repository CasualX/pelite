Look up symbols in a PE address range

Usage:

    pelite-cli symbol FILE RANGE [--symbols SYMBOLS.txt] [--format=text|json|json-pretty|nul]

Look up nearby symbols within an address range in the given symbol files.
FILE is the reference PE file used to convert addresses.

RANGE is `kind:START..END` and accepts decimal or hex: `rva:4096..5000`,
`va:0x180001000..0x180001004`, or `fo:512..576`; the end is excluded.
Prefix the second number with `+` to give a length instead, for example
`rva:0x1000..+10` ends at `0x100a`.

Use --symbols to load a `#symtext` symbol file, or repeat it to load several.
The symbols do not need to be sorted.

Examples:

    pelite-cli symbol sample.dll rva:0x1000..0x1100 --symbols SYMBOLS.txt
    pelite-cli symbol sample.dll va:0x180001000..+0x100 --symbols SYMBOLS.txt

Example JSON output:

```json
{
  "start_rva": 4096,
  "end_rva": 4352,
  "symbols": [
    {
      "rva": 4096,
      "name": "main",
      "ty": "code"
    }
  ]
}
```

When to use:

Find nearby symbols in symbol files for a known address range.
