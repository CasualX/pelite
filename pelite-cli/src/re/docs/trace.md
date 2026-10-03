Trace forwards from a PE address to the first control transfer

Usage:

    pelite-cli re trace FILE ADDRESS [--arch x86_16|x86_32|x86_64] [--disasm] [--disasm-limit N] [--facts FACTS.txt|auto]... [--layout none|indent|rva|va] [--hex] [--format=text|json|json-pretty|nul]

See `pelite-cli re --help` for ADDRESS syntax (rva:, va:, or fo:).
The PE machine header selects x86_32 or x86_64 by default; `--arch` overrides it.
`x86` is an alias for `x86_32`.

Decode forwards at instruction boundaries until a call, jump, conditional
branch (including loops), interrupt, return, system control transfer, trap,
transactional branch, or halt. Branches are not followed. Start at a known
instruction boundary; embedded data may decode as instructions.

Report the number of instructions and bytes, including the stopping instruction,
and its RVA and disassembly.

Use `--disasm` to output the traced range through `pelite-cli re disasm`.
By default limited to 256 instructions by default. Use `--disasm-limit N` to change
that limit, or `--disasm-limit 0` to disable it. Regular trace reports always run
without an instruction limit; `--disasm-limit` has no effect without `--disasm`.

An invalid or truncated instruction produces a nonzero exit status and a `(bad)`
error with the failing RVA and successfully decoded counts. The
invalid instruction is excluded from those counts. Reaching the end of available
section or header data before a control transfer or the `--disasm` instruction
limit also fails. Format `nul` suppresses successful output but still checks for errors.

Examples:

    pelite-cli re trace sample.dll rva:0x1000
    pelite-cli re trace sample.dll va:0x180001000 --format=json-pretty
    pelite-cli re trace sample.dll fo:1024 --arch x86
    pelite-cli re trace sample.dll rva:0x1000 --disasm --layout rva --hex

Example JSON output (without --disasm):

```json
{
  "instructions": 3,
  "bytes": 6,
  "stop_address": 4100,
  "instruction": "call rax"
}
```

When to use:

Measure a straight-line sequence of instructions from a known PE address up to
its first control transfer, or check whether decoding fails before reaching one.
