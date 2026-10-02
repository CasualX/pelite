When to use:

Investigate Rust formatting templates, panic locations, or trait vtables in a
PE binary.

The data scans support PE32 and PE32+ images; `format-args` requires x64.
These analyses recognize toolchain-specific layouts and report candidates;
matches can be missed or unrelated data can be reported.

Examples:

    pelite-cli re rust fmt-template sample.exe
    pelite-cli re rust panic-locations sample.exe --format=json-pretty
