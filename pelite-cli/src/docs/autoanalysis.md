Discover candidate code and data symbols in a PE image

Usage:

    pelite-cli autoanalysis FILE [-o SYMBOLS.txt] [--format=text|json|json-pretty|nul]

The command scans executable sections of i386 and AMD64 images for direct
branches, calls, static memory references, and address-like immediates. It
also uses the entry point, exports, section starts, and base relocations.
Results are candidates: embedded data can decode as instructions, and targets
computed from registers cannot be resolved.

An interpretation is null when unknown, a type name when there is one hint,
or an array when hints disagree. All addresses are RVAs. Text is the default;
JSON and indented JSON are available with `--format=json` and
`--format=json-pretty`.

Use `-o symbols.txt` to write a `#symtext` database. The output file must not
already exist; remove an old database before regenerating it.
is still printed. Use `--format=nul` to suppress terminal output while writing
the database. Generic code and data symbols use the symtext names `C` and
`D`; direct jump stubs use `Thunk`. Specialized labels are quoted names.
Conflicting numeric type hints become a union; code wins over other hints.
Named import labels omit the DLL name; ordinal import labels retain it.

    pelite-cli autoanalysis sample.dll -o symbols.txt --format=nul

When to use:

Map likely code and data targets before investigating specific addresses. For
references to one target, use `xref`.
