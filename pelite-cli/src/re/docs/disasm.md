Disassemble instructions from a PE image

Usage:

    pelite-cli re disasm FILE ADDRESS (BYTES | --trace [--trace-limit N]) [--facts FACTS.txt|auto]... [--layout none|indent|rva|fo|va] [--arch x86_16|x86_32|x86_64] [--hex] [--lookback BYTES] [--format=text|json|json-pretty|nul]

See `pelite-cli re --help` for ADDRESS syntax (`rva:`, `va:`, `fo:`, or `sym:`).

Disassembly starts at ADDRESS and proceeds in address order.

Choose one of two modes:

    pelite-cli re disasm FILE ADDRESS BYTES
    pelite-cli re disasm FILE ADDRESS --trace

With positional BYTES, decode instructions starting within that byte window.
BYTES accepts decimal or 0xhex. The end is excluded; zero bytes produces empty
output. All requested bytes must be readable.

With `--trace`, decode from ADDRESS through the first control transfer. Use it
for unknown or obfuscated code boundaries: it follows straight-line execution
to the next control-flow decision without guessing a byte count. Limit with
`--trace-limit N`; use 0 to disable the limit. `--trace` cannot be combined
with BYTES or `--lookback` and must start at a known instruction boundary.

Start at an instruction boundary when possible. If ADDRESS falls inside an
instruction, `--lookback BYTES` decodes up to BYTES earlier and includes the
overlapping instruction, but does not guarantee correct alignment. Embedded
data may decode misleadingly.

The PE machine header selects `x86_32` or `x86_64` by default. Use `--arch` to
override the decoding mode.

`--layout` controls text output and defaults to `indent`. Use `none`
for no prefix, `rva` or `va` to include the section name and address, or `fo`
for the file offset. Layout does not affect JSON. Add `--hex` to include
instruction bytes in text and JSON output; JSON omits `bytes` without it.

`--format` defaults to `text`. Use `json` or `json-pretty` for machine-readable
output. Both include the virtual address in `ip`. `prefix` is the section name
at the requested start, shared by all instructions. The symbol name is `label`, omitted without
a symbol. Annotations are separate strings in `comments`, omitted when empty.
`instruction` contains only the formatted instruction. Use `nul` to suppress
successful output while still reporting errors.

Facts
-----

Use `--facts FACTS.txt` to apply `#factmap` symbols and comments, or
`--facts auto` to run `analysis` in memory. Repeat in override order:

    --facts auto --facts user.facts.txt

Comments at instruction starts and symbols inside instructions appear as `;`
comments in text and entries in the JSON `comments` array.
Symbols at instruction starts appear as text labels and JSON `label` values.
Reference facts do not affect disassembly. Symbol types do not control decoding.

Relative branches and IP-relative memory references can resolve to symbols
without relocations. Absolute immediates and memory displacements require a
matching base relocation. If the PE has no usable relocation table, these
absolute values remain numeric even when they match a known symbol.

Examples
--------

    pelite-cli re disasm sample.dll rva:0x1000 0x100
    pelite-cli re disasm sample.dll rva:0x1000 0x100 --layout rva
    pelite-cli re disasm sample.dll rva:0x1000 --trace --trace-limit 64
    pelite-cli re disasm sample.dll fo:1024 128 --hex
    pelite-cli re disasm sample.dll va:0x180002000 0x80 --lookback 16
    pelite-cli re disasm sample.dll rva:0x1000 0x100 --hex --format=json-pretty

Example JSON output with `--hex`:

```json
[
  {
    "ip": 6442455040,
    "prefix": ".text",
    "bytes": [184, 1, 0, 0, 0],
    "instruction": "mov eax,1"
  }
]
```

When to use:

Inspect instructions at a known PE address. Use `--trace` for unknown or
obfuscated code boundaries when you want to stop at the next control transfer.
For raw bytes, use `disasm-raw`.
