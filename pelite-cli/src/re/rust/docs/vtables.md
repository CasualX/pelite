Find candidate Rust trait vtables in a PE image

Usage:

    pelite-cli re rust vtables FILE [--format=text|json|json-pretty|nul]

The command scans readable, non-writable sections for a drop pointer,
size, alignment, and consecutive function pointers. It uses base relocations
when available; otherwise it scans aligned pointer-sized words. It supports
PE32 and PE32+ images.

Results are heuristic and may include unrelated pointer tables. Each
`address` is the RVA of the drop-pointer slot. `functions` counts the
function pointers, and `comments` describe any recognized constant or
static string returns. Text is the default.

Examples:

    pelite-cli re rust vtables sample.exe
    pelite-cli re rust vtables sample.exe --format=json-pretty

Example JSON output (one vtable shown):

```json
[
  { "address": 8192, "size": 16, "align": 8, "functions": 2, "comments": [] }
]
```

When to use:

Investigate candidate Rust trait vtables, including type size, alignment, and
method pointers; use `xref` to check references.
