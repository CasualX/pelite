Read a PE version-information resource

Usage:

    pelite-cli version-info FILE [--id ID] [-l LANG] [--source] [--format=text|json|json-pretty]

`--id` selects the version resource identifier (default 1). Use
`--resource-language` (`-l`) to select a language ID in decimal or
`0x`-prefixed hexadecimal. Without it, the command uses the first matching
version resource. `--source` includes reconstructed resource-script source.

Text is the default. Use `json` or `json-pretty` for structured output.

Examples:

    pelite-cli version-info sample.dll
    pelite-cli version-info sample.dll --id 1 -l 0x409 --source
    pelite-cli version-info sample.dll --format=json-pretty

When to use:

Inspect product names, version strings, or other metadata in a PE version
resource. Select an ID or language when needed.
