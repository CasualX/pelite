Find candidate Rust panic locations in PE data

Usage:

    pelite-cli re rust panic-locations FILE [--format=text|json|json-pretty|nul]

The command scans readable, non-writable sections for `core::panic::Location`
records in PE32 and PE32+ images. A record contains a pointer and length for
a UTF-8 source path, followed by line and column numbers. The path must be
in read-only data, contain `/`, and end in `.rs`; line and column must be
positive and within reasonable bounds.

Results are candidates and can include unrelated data. Each `rva` is
the RVA of a record. Text is the default; JSON output is an array with
`rva`, `file`, `line`, and `column` for each match.

Examples:

    pelite-cli re rust panic-locations sample.exe
    pelite-cli re rust panic-locations sample.exe --format=json-pretty

Example JSON output (one location shown):

```json
[
  { "rva": 9088, "file": "src/main.rs", "line": 123, "column": 7 }
]
```

When to use:

Find Rust source paths and line numbers embedded in a PE binary; use `xref` to
check references.
