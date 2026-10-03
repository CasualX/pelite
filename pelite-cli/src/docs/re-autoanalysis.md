Discover candidate code and data symbols in a PE image

Usage:

    pelite-cli re autoanalysis FILE [-o FACTS.txt] [--format=text|json|json-pretty|nul]

The command scans executable sections of i386 and AMD64 images for direct
branches, calls, static memory references, and address-like immediates. It
also uses the entry point, exports, section starts, and base relocations.
Results are candidates: embedded data can decode as instructions, and targets
computed from registers cannot be resolved.

An interpretation is null when unknown, a type name when there is one hint,
or an array when hints disagree. All addresses are RVAs. Text is the default;
JSON and indented JSON are available with `--format=json` and
`--format=json-pretty`.

Use `-o auto.facts.txt` to write a `#factmap` database of automatically
discovered candidates, using `SxRVA TYPE NAME` symbol facts. Treat it as a
generated baseline, which can be large;
keep names, types, and corrections from your investigation in a separate
`user.facts.txt` instead of editing the generated file. These filenames are
conventions, not defaults. The output file must not already exist; choose a
new path or remove the old generated file before regenerating it.

The analysis report is still printed when `-o` is used. Use `--format=nul` to
write only the database. Generic code and data symbols use the factmap names
`C` and `D`; direct jump stubs use `thunk`. Specialized labels are quoted names.
Conflicting numeric type hints become a union; code wins over other hints.
Named import labels omit the DLL name; ordinal import labels retain it.

    pelite-cli re autoanalysis sample.dll -o auto.facts.txt --format=nul

Use `re symbol` to query a small address range rather than reading the whole
database. See `re symbol --help` for the user-file format and override rules,
and `re disasm --help` for using both files to label code.

When to use:

Map likely code and data targets before investigating specific addresses. For
references to one target, use `re xref`.
