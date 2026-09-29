Calculate the MD5 import hash of PE files

Usage:

    pelite-cli imphash FILE... [--show-imports] [--format=text|json|json-pretty]

The hash is MD5 of the ordered import names joined by commas. Library and
symbol names are lowercased; `.dll`, `.sys`, and `.ocx` are removed from
library names. Ordinal imports use `ord` followed by the ordinal number.

Text output prints one hash and path per file. `--show-imports` also prints
the normalized names used to calculate the hash. JSON output is an array of
objects with `file`, `hash`, and the full `imports` list; `--show-imports`
affects text output only.

Examples:

    pelite-cli imphash sample.dll
    pelite-cli imphash sample.dll other.dll --show-imports
    pelite-cli imphash sample.dll --format=json-pretty

Example JSON excerpt (first three imports shown):

```json
[
  {
    "file": "demo/Demo64.dll",
    "hash": "e9706463db6949081c4ae567e6564296",
    "imports": [
      "kernel32.getcurrentthreadid",
      "kernel32.getcurrentprocessid",
      "kernel32.queryperformancecounter"
    ]
  }
]
```
