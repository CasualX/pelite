Disassemble instructions from a PE image

Usage:

    pelite-cli disasm FILE RANGE [--symbols SYMBOLS.txt]... [--layout none|indent|rva|va] [--arch x86|x86_64] [--hex] [--lookback BYTES] [--format=text|json|json-pretty|nul]

RANGE is `kind:START..END` and accepts decimal or hex: `rva:4096..5000`,
`va:0x180001000..0x180001004`, or `fo:512..576`; the end is excluded.
Prefix the second number with `+` to give a length instead, for example
`rva:0x1000..+10` ends at `0x100a`.

One prefix applies to both endpoints. File offset ranges must map
to contiguous PE image bytes.

Disassembly starts at START and proceeds through the range in address order.
The PE machine header selects x86 or x86_64 by default. Use `--arch` to override
the decoding mode.

Use `--symbols SYMBOLS.txt` to label instruction addresses and operands from a
`#symtext` database. Repeat the option to load multiple files in order; the
last entry for an RVA wins. With no symbol files, addresses are shown without
symbol labels. Generate a starting database with `autoanalysis -o symbols.txt`.

In text output, a symbol inside an instruction appears after it as a `;` comment
with its byte offset, for example `; +0x2: data_1002`. The instruction stays intact.

Text uses `--layout indent` by default, with four spaces before each
instruction. Use `none` for no prefix (recommended) or `rva` and `va` for the
section name and an hexadecimal address. JSON output always includes
the instruction RVA in its `address` field.

Start at an instruction boundary: decoding from the middle of an instruction
can produce misleading results. Branches are not followed, and embedded data
may be decoded as instructions.

Use `--hex` to show instruction bytes alongside text, for example to check
alignment or recognize padding. It has no effect on JSON output.

Use `--lookback BYTES` when START may fall inside an instruction. It decodes
up to BYTES earlier (decimal) and includes an instruction overlapping START.
The earlier position must itself be correctly aligned. Lookback stays within
START's section; if the earlier bytes cannot be read, decoding starts at START.

Use format `json` for compact machine-readable output or `json-pretty` for
indented output. Each instruction has an RVA `address`, a `bytes` array,
and an `instruction` string. Text is the default.

See `disasm-raw` to decode raw bytes.

Examples:

    pelite-cli disasm sample.dll rva:0x1000..0x1100
    pelite-cli disasm sample.dll rva:0x1000..0x1100 --symbols auto.txt --symbols edits.txt
    pelite-cli disasm sample.dll rva:0x1000..0x1100 --layout rva
    pelite-cli disasm sample.dll rva:0x1000..+0x100
    pelite-cli disasm sample.dll fo:1024..1152 --hex
    pelite-cli disasm sample.dll rva:0x1000..0x1100 --arch x86
    pelite-cli disasm sample.dll va:0x180001000..0x180001080 --lookback 16
    pelite-cli disasm sample.dll rva:0x1000..0x1100 --format=json-pretty

Example JSON output (one instruction shown):

```json
[
  {
    "address": 4096,
    "bytes": [184, 1, 0, 0, 0],
    "instruction": "mov eax,1"
  }
]
```

When to use:

Inspect instructions at a known PE address, such as an entry point or export.
For raw bytes, use `disasm-raw`.
