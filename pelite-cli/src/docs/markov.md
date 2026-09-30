Generate bytes from executable PE sections

Usage:

    pelite-cli markov COUNT FILE... [--seed SEED] [-o FILE] [--format=text|json|json-pretty]

The command learns adjacent-byte transitions from executable sections in the
input PE files, then generates COUNT bytes. The result is synthetic byte data;
it is not guaranteed to be valid code.

Text output is space-separated hex. Use `--seed` for repeatable output and
`-o` (`--output`) to also write the raw bytes to a file.

Examples:

    pelite-cli markov 64 sample.dll --seed 1
    pelite-cli markov 64 sample.dll other.dll --seed 1 -o generated.bin --format=json-pretty

Example JSON output:

```json
[207, 232, 31, 132, 0, 0, 255, 76]
```

When to use:

Generate synthetic bytes from executable PE sections for experiments or test
data; the bytes may not be valid code.
