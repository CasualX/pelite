Disassemble instructions from a PE image

Usage:

    pelite-cli re disasm FILE RANGE [--facts FACTS.txt|auto]... [--layout none|indent|rva|va] [--arch x86_16|x86_32|x86_64] [--hex] [--lookback BYTES] [--format=text|json|json-pretty|nul]

See `pelite-cli re --help` for RANGE syntax.

Disassembly starts at START and proceeds through the range in address order.
The PE machine header selects x86_32 or x86_64 by default. Use `--arch` to override
the decoding mode with `x86_16`, `x86_32`, or `x86_64`. `x86` is an alias for `x86_32`.

Use `--facts FACTS.txt` to label instruction addresses and operands from a
`#factmap` database. Use `--facts auto` to run the full `re analysis` pipeline
on the PE image and use its symbols and forward-fed comments in memory,
without writing a file. Mix `auto` and file paths in the desired override order;
`--facts auto --facts user.facts.txt` applies your corrections last. Use
`./auto` to load a file literally named `auto`. Repeat the option to load multiple files in order; the
last symbol for an RVA wins, and `undef` removes an earlier label. Weak `_`
entries preserve existing symbols and do not create labels. Comment facts at instruction starts appear after the instruction as `;`
comments in text and in the JSON `instruction` string. Later comments at the
same RVA override earlier ones. Reference facts do not affect the output. Pass the generated
baseline first and your analysis file last:

    pelite-cli re disasm sample.dll rva:0x1000..0x1100 --facts auto.facts.txt --facts user.facts.txt

With no symbol files, addresses are shown without symbol labels. See
`re analysis --help` to generate the baseline and `re symbol --help` to query
nearby symbols or record corrections. Symbol types do not control decoding:
a data label does not prevent those bytes from being decoded as instructions.

In text output, a symbol inside an instruction appears after it as a `;` comment
with its byte offset, for example `; +0x2: data_1002`. The instruction stays intact.

Text uses `--layout indent` by default, with four spaces before each
instruction. Use `none` for no prefix (recommended) or `rva` and `va` for the
section name and a hexadecimal address. JSON output always includes
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

See `re disasm-raw` to decode raw bytes.

Examples:

    pelite-cli re disasm sample.dll rva:0x1000..0x1100
    pelite-cli re disasm sample.dll rva:0x1000..0x1100 --layout rva
    pelite-cli re disasm sample.dll rva:0x1000..+0x100
    pelite-cli re disasm sample.dll fo:1024..1152 --hex
    pelite-cli re disasm sample.dll rva:0x1000..0x1100 --arch x86
    pelite-cli re disasm sample.dll va:0x180001000..0x180001080 --lookback 16
    pelite-cli re disasm sample.dll rva:0x1000..0x1100 --format=json-pretty

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
For raw bytes, use `re disasm-raw`.
