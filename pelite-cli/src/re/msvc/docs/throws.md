Extract Microsoft C++ thrown exception types

Usage:

    pelite-cli re msvc throws FILE [--demangle] [--format=text|json|json-pretty|nul]

The command follows code references to exception metadata and reports thrown
types, their catchable conversions, and copy or destruction helpers. It works
independently of vtable RTTI, including binaries compiled with `/GR-`, and does
not require symbols or base relocations.

Text is the default. JSON output is an array of exception records with metadata
RVAs and instruction `references`. The first `catchable_types` entry describes
the thrown type; later entries describe permitted conversions, such as base
classes. Use `--demangle` for readable C++ names in text and JSON output.

Only referenced, validated metadata is reported, so some thrown types may be
missed. References identify metadata loads, not verified throw calls. The
command does not list catch clauses or analyze function unwind tables.

Examples:

    pelite-cli re msvc throws sample.exe
    pelite-cli re msvc throws sample64.dll --demangle --format=json-pretty

Example JSON output (one exception shown, with demangled names):

```json
[
  {
    "rva": 118656,
    "attributes": 0,
    "destructor_rva": null,
    "forward_compat_rva": null,
    "catchable_type_array_rva": 118688,
    "references": [4108],
    "catchable_types": [{
      "rva": 118704,
      "type_descriptor_rva": 125600,
      "name": "int",
      "properties": 1,
      "simple_type": true,
      "by_reference_only": false,
      "has_virtual_bases": false,
      "displacement": { "mdisp": 0, "pdisp": -1, "vdisp": 0 },
      "size_or_offset": 4,
      "copy_function_rva": null
    }]
  }
]
```

When to use:

Investigate which exception types a Microsoft C++ binary throws, how they can
be caught through base or pointer conversions, and where their metadata and
cleanup helpers reside, including when vtable RTTI is unavailable.
