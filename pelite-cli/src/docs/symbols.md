Discover candidate code and data symbols in a PE image

Usage:

    pelite-cli symbols FILE [--format=text|json|json-pretty]

The command scans executable sections of i386 and AMD64 images for direct
branches, calls, static memory references, and address-like immediates. It
also uses the entry point, exports, section starts, and base relocations.
Results are candidates: embedded data can decode as instructions, and targets
computed from registers cannot be resolved.

An interpretation is null when unknown, a type name when
there is one hint, or an array when hints disagree. References record a
`kind` and the `source_rva` of the instruction or pointer; metadata such as
an entry point or export has a null source. All addresses are RVAs.

Text is the default. Use `json` for compact machine-readable output or
`json-pretty` for indented output. Inspect a candidate before relying on its label or type.

The output can be large. Run the scan once, save the JSON, then query it with
`jq` instead of rescanning for each address:

    pelite-cli symbols sample.dll --format=json > symbols.json
    jq '.[] | select(.rva == 4096)' symbols.json
    jq '.[] | select(any(.references[]; .kind == "export")) | {rva, label}' symbols.json

Example JSON output (one symbol shown):

```json
[
  {
    "rva": 4096,
    "label": "code_00001000",
    "interpretations": "code",
    "references": [
      { "source_rva": 6448, "kind": "call" }
    ]
  }
]
```

When to use:

Map likely code and data targets before investigating specific addresses. For
references to one target, use `xref`.
