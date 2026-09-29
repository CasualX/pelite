Disassemble raw x86 bytes

Usage:

    pelite-cli disasm-raw FILE --machine x86|x86_64 [--offset BYTES] [--length BYTES] [--base ADDRESS] [--lookback BYTES] [--hex] [--format=text|json|json-pretty]

FILE is a binary file, or `-` for standard input. `--offset` starts at a file
offset (default: 0); `--length` sets the selected byte count (default: through
the end of the input). `--base` is the instruction address at file offset zero,
so an instruction's `ip` is `base + offset`. Offset, length, and base accept
decimal or `0x`-prefixed hexadecimal numbers.

Start at an instruction boundary. `--lookback` decodes up to that many bytes
before the offset and includes an instruction overlapping it; the earlier
position must also be aligned. Lookback is decimal. `--hex` shows opcode
bytes in text output only.

Text is the default. JSON output contains each instruction's file `offset`,
`ip`, `bytes`, and formatted `instruction`.

Examples:

    pelite-cli disasm-raw code.bin --machine x86_64 --offset 0x100 --length 64 --base 0x180000000 --hex
    printf '\xb8\x01\x00\x00\x00\xc3' | pelite-cli disasm-raw - --machine x86 --base 0x1000 --format=json

Example JSON output for the second command:

```json
[
  { "offset": 0, "ip": 4096, "bytes": [184, 1, 0, 0, 0], "instruction": "mov eax,1" },
  { "offset": 5, "ip": 4101, "bytes": [195], "instruction": "ret" }
]
```
