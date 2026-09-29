Dump Microsoft C++ RTTI from a PE32 image

Usage:

    pelite-cli msrtti FILE [--format=text|json|json-pretty]

The command finds candidate vtables using base relocations, then reports
their type descriptors and class hierarchies. It supports PE32 images with
`.text`, `.rdata`, and base relocations; PE32+ is not supported.

Text is the default. JSON output is an array of types. Each type has a raw
RTTI `name`, an `inheritance` kind, `vtables` with RVAs and method counts,
and a `hierarchy` with base-class depth and offsets. A virtual base has a
null offset.

Examples:

    pelite-cli msrtti sample.dll
    pelite-cli msrtti sample.dll --format=json-pretty

Example JSON output (one type shown):

```json
[
  {
    "name": ".?AVA@@",
    "inheritance": "single",
    "vtables": [{ "rva": 8544, "for_type": ".?AVA@@", "methods": 2 }],
    "hierarchy": [{ "depth": 0, "offset": 0, "virtual_base": false, "name": ".?AVA@@" }]
  }
]
```
