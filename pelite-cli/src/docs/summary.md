Summarize a PE file

Usage:

    pelite-cli summary FILE [--format=text|json|json-pretty|nul]
    pelite-cli FILE [--format=text|json|json-pretty|nul]

Passing FILE without a command also runs `summary`. The report covers file
hashes, image and section details, mitigations, imports, and findings that may
merit closer inspection. Text is the default; use `json` or `json-pretty` for
structured output.

The image details indicate CLR (.NET) images when the COM Runtime Descriptor
directory has a nonzero RVA and size. This does not validate CLR metadata.

If import data is damaged, the report includes the imports it can read and
marks the import hash unavailable.

Findings are clues, not a malware verdict. The compile timestamp comes from
the file itself, and certificate-table presence does not validate a signature.

Examples:

    pelite-cli summary sample.dll
    pelite-cli sample.dll --format=json-pretty

When to use:

Get a first look at an unfamiliar PE's layout, imports, hashes, and findings.
For exact headers or directories, use `inspect`.
