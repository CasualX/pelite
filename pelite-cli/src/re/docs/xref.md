Find candidate references to a PE address

Usage:

    pelite-cli re xref FILE ADDRESS [--format=text|json|json-pretty|nul]

See `pelite-cli re --help` for ADDRESS syntax.

The command converts the target to an RVA before searching.

The search checks base relocations for pointers to the target, or scans raw
pointer bytes if relocations are unavailable. It also checks executable
sections for 32-bit relative displacements. Use this to find places that may
refer to a function or data item.

Each reference's `rva` is the RVA of the pointer or displacement field.
Matches are candidates: raw bytes can match by chance, and the search does
not decode instructions. For code matches, investigate nearby symbols with
`symbol --facts auto` or explicit factmap(s). Query a range
beginning before the reference and look for a credible preceding code boundary.
Disassemble from that boundary with `disasm` to verify the instruction and
its target; symbol facts are hints, not guaranteed boundaries. A reference RVA
can lie inside an instruction; starting disassembly exactly there can misdecode it.
`--lookback` can include the containing instruction only if the earlier decode
position is aligned. For data matches, use `read` with `*unk` or a suitable type.

Use format `json` for compact machine-readable output or `json-pretty` for
indented output. Duplicate locations appear once. Text is the default.
An empty list means no candidates were found.

Examples:

    pelite-cli re xref sample.dll rva:4096
    pelite-cli re xref sample.dll fo:0x400 --format=json-pretty
    pelite-cli re xref sample.dll va:0x180001000

Example JSON output (one reference shown):

```json
{
  "target_rva": 4096,
  "target_va": 6442455040,
  "references": [
    {
      "rva": 6449,
      "section": ".text"
    }
  ]
}
```

When to use:

Find candidate references to a known function or data address.
