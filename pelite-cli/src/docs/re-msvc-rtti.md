Dump Microsoft C++ RTTI from a PE32 or PE32+ image

Usage:

    pelite-cli re msvc rtti FILE [--demangle] [--format=text|json|json-pretty|nul]

The command finds candidate vtables using base relocations, then reports
their type descriptors and class hierarchies. It selects the PE32 or PE32+
scanner from the input image. Both scanners require `.text`, `.rdata`, and
base relocations. The PE32+ scanner validates each locator's self RVA and
resolves image-relative RTTI descriptor offsets.

Text is the default. JSON output is an array of types. Each type has a raw
RTTI `name`, an `inheritance` kind, `vtables` with RVAs and method counts,
and a `hierarchy` with base-class depth and offsets. A virtual base has a
null offset.

Use `--demangle` to demangle C++ type names in text and JSON output, including
vtable target types and base classes. Text vtable symbols are also demangled.
Names that cannot be demangled are left unchanged.

Examples:

    pelite-cli re msvc rtti sample.dll
    pelite-cli re msvc rtti sample64.dll --format=json-pretty
    pelite-cli re msvc rtti sample64.dll --demangle --format=json-pretty

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

When to use:

Investigate class names, inheritance, or virtual methods in a Microsoft C++
binary with RTTI and base relocations.
