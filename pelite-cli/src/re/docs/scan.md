Find patterns and read typed data at their matches

Usage:

    pelite-cli re scan FILE TYPE [PATTERN] [--section SECTION] [--zerofill] [--string-preview-length N] [--max-dynamic-array-length N] [--format=text|json|json-pretty|nul]

PATTERN uses the syntax and section scanning described by `findsig --help`.
By default, scan searches sections not marked executable. `--section` searches
only the named section or its 0-based index, regardless of its flags.
An unknown name or index is an error. With no PATTERN, every byte position in
the selected sections is considered a match.
TYPE and the read limits work as described by `read --help`.

Use `--zerofill` to allow typed scalar and pointer reads, including dynamic
array lengths, from a section's virtual zero-filled tail. This also applies to
values reached through pointers. String reads still require file-backed bytes.
Zero filling is disabled by default.

For each pattern match, scan reads TYPE at the match RVA. It omits matches
whose RVA is misaligned for TYPE or whose read contains any `$error`, including
errors in nested fields and array elements. Captured values do not change the
read address.

Each output is an array of objects with `captures` (the match RVA followed by
captures, as in `findsig`) and `value` (the typed result, as in `read`).

Examples:

    pelite-cli re scan sample.dll '[u8; 7]' '48 8B ? 48 85 C0' --section .text --format=json
    pelite-cli re scan sample.dll '[u8; 5]' 'E8 save ????' --section .text --format=json-pretty
    pelite-cli re scan sample.dll '*unk' --format=json

Example JSON output (one result shown):

```json
[
  {
    "captures": [12288],
    "value": 15920
  }
]
```

When to use:

Find values by type across data sections, optionally narrowing the search with
a byte pattern. Use `findsig` when you need only match addresses or captures;
use `read` to examine one result in detail. Successful reads establish that
bytes fit TYPE, not that the inferred structure is correct.
