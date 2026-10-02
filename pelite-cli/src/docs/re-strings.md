Find strings in PE sections

Usage:

    pelite-cli re strings FILE [--filter PATTERN]... [--regex] [--ignore-case] [--min-confidence SCORE] [--format=text|json|json-pretty|nul]

The command finds ASCII, UTF-8, and UTF-16LE text. Each result includes its
section, RVA `address`, encoding, confidence score, and value. Matches are
heuristic, so some bytes may be mistaken for text.

`--filter` keeps values containing a literal substring and can be repeated;
any matching filter includes the string. Use `--regex` (`-E`) to treat all
filters as regular expressions, or `--ignore-case` (`-i`) to ignore case.
`--min-confidence` defaults to 40. The score ranks candidates; it is not a probability.

Text is the default. Use `json` or `json-pretty` for structured output.

Examples:

    pelite-cli re strings sample.dll --filter Demo -i
    pelite-cli re strings sample.dll --filter 'error|warning' -E --min-confidence 60 --format=json

Example JSON output (one string shown):

```json
[
  {
    "section": ".rdata",
    "address": 15136,
    "encoding": "ascii",
    "confidence": 90,
    "value": "Demo.dll"
  }
]
```

When to use:

Find embedded messages, paths, URLs, or other readable text without knowing
their addresses. Filter large results.
