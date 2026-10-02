Find candidate references to a PE address

Usage:

    pelite-cli xref FILE ADDRESS [--format=text|json|json-pretty|nul]

ADDRESS is `kind:number` and accepts decimal or hex: `rva:4096`, `va:0x180001000`, or `fo:1024`.
The command converts the target to an RVA before searching.

The search checks base relocations for pointers to the target, or scans raw
pointer bytes if relocations are unavailable. It also checks executable
sections for 32-bit relative displacements. Use this to find places that may
refer to a function or data item.

Each reference's `address` is the RVA of the pointer or displacement field.
Matches are candidates: raw bytes can match by chance, and the search does
not decode instructions. For code matches, query a surrounding range with
`symbol` to find possible starting points, then use `disasm` to verify the
instruction and its target. A reference address can lie inside an instruction;
starting disassembly exactly there can misdecode it. `--lookback` can include
the containing instruction only if the earlier decode position is aligned.
For data matches, use `read` with `ptr` or a suitable type.

Use format `json` for compact machine-readable output or `json-pretty` for
indented output. Duplicate locations appear once. Text is the default.
An empty list means no candidates were found.

Examples:

    pelite-cli xref sample.dll rva:4096
    pelite-cli xref sample.dll fo:0x400 --format=json-pretty
    pelite-cli xref sample.dll va:0x180001000

Example JSON output (one reference shown):

```json
{
  "target_rva": 4096,
  "target_va": 6442455040,
  "references": [
    {
      "address": 6449,
      "section": ".text"
    }
  ]
}
```

When to use:

Find candidate references to a known function or data address.
