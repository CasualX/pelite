Find Rust formatting templates referenced by x64 code

Usage:

    pelite-cli rust format-args FILE [--format=text|json|json-pretty]

The command finds code references to compact `format_args!` templates in an
x64 PE image, then decodes their text and placeholders. Each result has
a template RVA and the RVAs of code references. Literal-only formats use a
different representation and are omitted.

Text is the default. JSON output includes `template_rva`, `encoded_len`,
`format`, `argument_count`, `placeholders`, and `code_rvas`. The template
layout depends on the Rust toolchain version.

Examples:

    pelite-cli rust format-args sample.exe
    pelite-cli rust format-args sample.exe --format=json-pretty

Example JSON output (one template shown):

```json
[
  {
    "template_rva": 651312,
    "encoded_len": 34,
    "format": "example value={0} and hex={1}\n",
    "argument_count": 2,
    "placeholders": [{ "argument": 0 }, { "argument": 1, "flags": 1619001376 }],
    "code_rvas": [5672]
  }
]
```

When to use:

Find Rust formatting templates and the x64 code locations that reference them.
