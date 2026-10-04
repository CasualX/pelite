Disassemble instructions from raw x86 bytes

Usage:

    pelite-cli re disasm-raw FILE --arch x86_16|x86_32|x86_64 [--offset BYTES] [--length BYTES | --trace [--trace-limit N]] [--base ADDRESS] [--facts FACTS.txt]... [--layout none|indent|fo|va] [--hex] [--lookback BYTES] [--format=text|json|json-pretty|nul]

FILE is a binary file, or `-` for standard input.

Disassembly starts at `--offset` (default: 0) and proceeds in address order.

Choose one of two modes:

    pelite-cli re disasm-raw FILE --arch x86_64 --length BYTES
    pelite-cli re disasm-raw FILE --arch x86_64 --trace

With `--length BYTES`, decode instructions starting within that byte window.
BYTES accepts decimal or 0xhex. The end is excluded; zero bytes produces empty
output. All requested bytes must be readable. Without `--length` or `--trace`,
decode through the end of the input.

With `--trace`, decode from `--offset` through the first control transfer. Use it
for unknown or obfuscated code boundaries: it follows straight-line execution
to the next control-flow decision without guessing a byte count. Limit with
`--trace-limit N`; use 0 to disable the limit. `--trace` cannot be combined
with `--length` or `--lookback` and must start at a known instruction boundary.

Start at an instruction boundary when possible. If `--offset` falls inside an
instruction, `--lookback BYTES` decodes up to BYTES earlier and includes the
overlapping instruction, but does not guarantee correct alignment. Embedded
data may decode misleadingly.

Use `--arch` to select the decoding mode; it is required for raw input.
`x86` is an alias for `x86_32`.

`--base` is the instruction address at file offset zero (default: 0), so an
instruction's `ip` is `base + offset`. Offset and base accept decimal or 0xhex.

`--layout` controls text output and defaults to `indent`. Use `none` for no
prefix, or `fo` or `va` to include the file offset or instruction address. Add `--hex`
to show instruction bytes in text output. These options do not affect JSON.

`--format` defaults to `text`. Use `json` or `json-pretty` for machine-readable
output; both include the file offset in `offset` and instruction address in `ip`.
Use `nul` to suppress successful output while still reporting errors.

Facts
-----

Use `--facts FACTS.txt` to apply `#factmap` symbols and comments.
Repeat in override order:

    --facts raw.facts.txt --facts user.facts.txt

Comments at instruction starts are appended as `;` comments. Reference facts
do not affect disassembly. Symbol types do not control decoding.

The RVA field in each `#factmap` entry is treated as a file offset; its
instruction address is `base + offset`. PE symbol files from `re analysis`
use RVAs, so convert their entries to offsets in the raw input before using
them here; `--base` changes instruction addresses, not the meaning of the
entry offsets. Without symbol files, raw disassembly has no labels.
See `re symbol --help` for the entry syntax.

Examples
--------

    pelite-cli re disasm-raw code.bin --arch x86_64 --offset 0x100 --length 0x100
    pelite-cli re disasm-raw code.bin --arch x86_64 --offset 0x100 --length 0x100 --layout fo
    pelite-cli re disasm-raw code.bin --arch x86_64 --offset 0x100 --trace --trace-limit 64
    pelite-cli re disasm-raw code.bin --arch x86_64 --offset 1024 --length 128 --hex
    pelite-cli re disasm-raw code.bin --arch x86_64 --offset 0x100 --length 0x80 --base 0x180000000 --lookback 16
    pelite-cli re disasm-raw code.bin --arch x86_64 --facts raw.facts.txt --layout fo
    printf '\xb8\x01\x00\x00\x00\xc3' | pelite-cli re disasm-raw - --arch x86 --base 0x1000 --format=json-pretty

Example JSON output:

```json
[
  {
    "offset": 0,
    "ip": 4096,
    "bytes": [184, 1, 0, 0, 0],
    "instruction": "mov eax,1"
  },
  {
    "offset": 5,
    "ip": 4101,
    "bytes": [195],
    "instruction": "ret"
  }
]
```

When to use:

Inspect instructions in raw 16-, 32-, or 64-bit x86 bytes from a payload, file,
or stream. Use `--trace` for unknown or obfuscated code boundaries when you
want to stop at the next control transfer. For mapped PE addresses, use `re disasm`.
