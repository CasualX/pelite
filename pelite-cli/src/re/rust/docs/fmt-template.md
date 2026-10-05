Find Rust formatting templates in read-only PE data

Usage:

    pelite-cli re rust fmt-template FILE [--format=text|json|json-pretty|nul]

The command scans readable, non-writable, non-discardable sections for
compact `format_args!` templates. It supports PE32 and PE32+ and does not
inspect code. A candidate must have a placeholder and at least two literal
bytes. Literal-only and very short formats are omitted.

Matches are heuristic: unrelated bytes can decode as templates, and
overlapping candidates are resolved in favor of the longest one. JSON
output includes `template_rva`, `encoded_len`, `format`, `argument_count`,
and `placeholders`. Text is the default.

Examples:

    pelite-cli re rust fmt-template sample.exe
    pelite-cli re rust fmt-template sample.exe --format=json-pretty

Example JSON output (one template shown):

```json
[
  {
    "template_rva": 651312,
    "encoded_len": 34,
    "format": "example value={0} and hex={1}\n",
    "argument_count": 2,
    "placeholders": [{ "argument": 0 }, { "argument": 1, "flags": 1619001376 }]
  }
]
```

When to use:

Find candidate Rust formatting templates in PE data; use `xref` to search for
references.
