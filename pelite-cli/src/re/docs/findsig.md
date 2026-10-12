Find patterns in a PE image

Usage:

    pelite-cli re findsig FILE [PATTERN] [--section SECTION] [--format=text|json|json-pretty|nul]

PATTERN uses the [pelite pattern syntax](https://github.com/CasualX/pelite/blob/master/src/pattern/syntax.md).
Quote it in the shell. With no PATTERN, enter one pattern per line on standard
input; an interactive terminal shows a prompt.

By default, the command scans readable, executable, non-discardable sections,
including writable code, regardless of content flags. `--section` scans
only the named section or its 0-based index, regardless of its flags.
An unknown name or index is an error.

The command reports matching RVAs. Each JSON `matches`
entry is an array: the first number is the match RVA, followed by any captures.
Text is the default; use `json` or `json-pretty` for structured output.

To help a pattern work across future versions of a binary, keep it as short as
possible while still identifying the intended match. Mask operands likely to
change, such as field offsets, and retain constants likely to stay the same.
When updating a pattern, aim to support both the current and previous versions.
Compatibility with the previous version can be dropped after significant
structural changes, such as a compiler update.

Examples:

    pelite-cli re findsig sample.dll '48 8B ? 48 85 C0'
    pelite-cli re findsig sample.dll 'E8${B8 save ???? C3}' --format=json-pretty
    pelite-cli re findsig sample.dll '30 3E 00 00' --section .rdata

Example JSON output:

```json
{
  "pattern": "E8${B8 save ???? C3}",
  "matches": [[6448, 4097], [6476, 4097]]
}
```

When to use:

Locate a known byte or instruction pattern in executable code or section; and
capture values from each match.
