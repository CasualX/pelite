Generate decodable x86 code from executable PE sections

Usage:

    pelite-cli markov FILE... COUNT [--seed SEED] [--raw] [--format=text|json|json-pretty|nul]

The command learns adjacent-byte transitions from executable sections in the
input PE files, then generates exactly COUNT bytes of complete instructions.
All inputs must have the same Machine value in their PE headers:
i386 (32-bit x86) or AMD64 (64-bit x86). Other machines are rejected.

The output is syntactically decodable code, not a runnable program. Branch targets,
memory accesses, register values, and CPU feature requirements are not validated.

Text output is space-separated hex by default. Add `--raw` to write raw bytes
directly to standard output, without a trailing newline, for piping or redirection.
`--raw` only affects `--format=text` (the default); JSON and nul output are
unchanged. Use `--seed` for repeatable output.

Examples:

    pelite-cli markov sample.dll 64 --seed 1
    pelite-cli markov sample.dll other.dll 64 --seed 1 --raw > generated.bin
    pelite-cli markov sample.dll 256 --seed 1 --raw | pelite-cli re disasm-raw - --arch x86_64 --hex --layout fo

Match `--arch` to the training inputs: use `x86_32` for i386 or `x86_64` for AMD64.

Example JSON output:

```json
[144, 195]
```

When to use:

Generate synthetic instruction streams from executable PE sections for
disassembler experiments or test data.
