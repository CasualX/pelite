Convert a PE address between RVA, VA, and file offset

Usage:

    pelite-cli re addr FILE ADDRESS [--format=text|json|json-pretty|nul]

See `pelite-cli re --help` for ADDRESS syntax.

The command reports the matching RVA, VA, file offset (`fo`), and section.
`fo` is null if the RVA has no corresponding file bytes. An address that
cannot be converted to an RVA causes an error. Use this command to translate
an address from a disassembler or hex editor before passing it on.

Use format `json` for compact machine-readable output or `json-pretty` for
indented output. Text is the default.

Examples:

    pelite-cli re addr sample.dll rva:4096
    pelite-cli re addr sample.dll fo:0x400
    pelite-cli re addr sample.dll va:0x180001000 --format=json-pretty

Example JSON output:

```json
{
  "rva": 4096,
  "va": 6442455040,
  "fo": 1024,
  "section": ".text"
}
```

When to use:

Convert between RVA, VA, and file offset when following an address across tools.
