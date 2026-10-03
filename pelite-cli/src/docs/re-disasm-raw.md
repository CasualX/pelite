Disassemble raw x86 bytes

Usage:

    pelite-cli re disasm-raw FILE --arch x86_16|x86_32|x86_64 [--offset BYTES] [--length BYTES] [--base ADDRESS] [--facts FACTS.txt]... [--layout none|indent|fo|va] [--lookback BYTES] [--hex] [--format=text|json|json-pretty|nul]

`--arch` selects 16-, 32-, or 64-bit x86 decoding. `x86` is an alias for `x86_32`.

FILE is a binary file, or `-` for standard input. `--offset` starts at a file
offset (default: 0); `--length` sets the selected byte count (default: through
the end of the input). `--base` is the instruction address at file offset zero,
so an instruction's `ip` is `base + offset`. Offset, length, and base accept
decimal or `0x`-prefixed hexadecimal numbers.

Use `--facts FACTS.txt` to label instructions and referenced addresses.
The RVA field in each `#factmap` symbol entry is treated as a file offset; its
instruction address is `base + offset`. Repeat `--facts` to load files in
order, with the last symbol for an offset winning; `undef` removes an earlier
label. Weak `_` entries preserve existing symbols and do not create labels.
Comment and reference facts are accepted but do not affect the output.
See `re symbol --help` for the entry syntax. Without symbol files, raw
disassembly has no labels. PE symbol files from `re autoanalysis` use RVAs, so
convert their entries to offsets in the raw input before using them here;
`--base` changes instruction addresses, not the meaning of the entry offsets.

In text output, a symbol inside an instruction appears after it as a `;` comment
with its byte offset. The instruction stays intact.

Text uses `--layout indent` by default, with four spaces before each
instruction. Use `none` for no prefix (recommended), `fo` for a file offset,
or `va` for the instruction address.

Start at an instruction boundary. `--lookback` decodes up to that many bytes
before the offset and includes an instruction overlapping it; the earlier
position must also be aligned. Lookback is decimal. `--hex` shows opcode
bytes in text output only.

Text is the default. JSON output contains each instruction's file `offset`,
`ip`, `bytes`, and formatted `instruction`.

Examples:

    pelite-cli re disasm-raw code.bin --arch x86_64 --offset 0x100 --length 64 --base 0x180000000 --hex
    pelite-cli re disasm-raw code.bin --arch x86_64 --facts raw.facts.txt --layout fo
    printf '\xb8\x01\x00\x00\x00\xc3' | pelite-cli re disasm-raw - --arch x86 --base 0x1000 --format=json

Example JSON output for the third command:

```json
[
  { "offset": 0, "ip": 4096, "bytes": [184, 1, 0, 0, 0], "instruction": "mov eax,1" },
  { "offset": 5, "ip": 4101, "bytes": [195], "instruction": "ret" }
]
```

When to use:

Disassemble raw 16-, 32-, or 64-bit x86 bytes from a payload, file, or stream. For
mapped PE addresses, use `re disasm`.
