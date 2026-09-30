Summarize a PE file

Usage:

    pelite-cli summary FILE [--format=text|json|json-pretty]
    pelite-cli FILE [--format=text|json|json-pretty]

Passing FILE without a command also runs `summary`. The report covers file
hashes, image and section details, mitigations, imports, and findings that may
merit closer inspection. Text is the default; use `json` or `json-pretty` for
structured output.

Findings are clues, not a malware verdict. The compile timestamp comes from
the file itself, and certificate-table presence does not validate a signature.

Examples:

    pelite-cli summary sample.dll
    pelite-cli sample.dll --format=json-pretty

When to use:

Get a first look at an unfamiliar PE's layout, imports, hashes, and findings.
For exact headers or directories, use `inspect`.
