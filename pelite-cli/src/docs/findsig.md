Find patterns in a PE image

Usage:

    pelite-cli findsig FILE [PATTERN] [--section SECTION] [--format=text|json|json-pretty]

PATTERN uses the [pelite pattern syntax](https://github.com/CasualX/pelite/blob/master/src/pattern/syntax.md).
Quote it in the shell. With no PATTERN, enter one pattern per line on standard
input; an interactive terminal shows a prompt.

By default, the command scans sections marked executable. `--section` scans
only the named section or its 0-based index, regardless of its flags.
An unknown name or index is an error.

The command reports matching RVAs. Each JSON `matches`
entry is an array: the first number is the match RVA, followed by any captures.
Text is the default; use `json` or `json-pretty` for structured output.

Examples:

    pelite-cli findsig sample.dll '48 8B ? 48 85 C0'
    pelite-cli findsig sample.dll 'E8${B8 save ???? C3}' --format=json-pretty
    pelite-cli findsig sample.dll '30 3E 00 00' --section .rdata

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
