Discover candidate code and data symbols in a PE image

Usage:

    pelite-cli re analysis FILE [-o FACTS.txt] [--format=text|json|json-pretty|nul]

The command scans executable sections of i386 and AMD64 images for direct
branches, calls, static memory references, and address-like immediates. It
also uses the entry point, TLS callbacks, exports, imports, base relocations, and
x64 exception records. Function starts from x64 exception records are added
or upgraded to type `fn`, preserving specific names such as export names,
import stubs, and thunk/return labels. Generic code/data names become `fn` names.
Direct exports retain their names; executable targets receive type `code`
unless stronger function metadata establishes type `fn`. Data exports are
not assumed to be functions, and forwarded exports are skipped. Ordinal-only
exports still seed their target addresses. When names share an RVA, the last
name in the export name table is selected by the current analysis index.
Passes run from weaker heuristics to stronger metadata: linear disassembly,
relocations, label refinement, exports, strings, imports, x64 exception
records, then entry points (TLS callbacks and the PE entry point).
Later generic code hints preserve specific labels; exception records establish
function symbols. The entry-point function is named `EntryPoint`; TLS callback
functions are named `TlsCallback_0`, `TlsCallback_1`, and so on in callback-array
order. These functions have type `fn`; their names override generic exception labels.

Finally, an experimental forward feeder makes a second linear scan of
executable sections. It tracks `mov` loads of static global dwords/qwords and
`lea` references to known symbols in general-purpose registers. A subsequent
`call REG` receives a comment naming that global, for example
`call rax ; __imp_Function`. Any explicit, implicit, or conditional write to
any register alias ends propagation (`al` invalidates `rax`). Tracking resets
between sections and after invalid instructions. It does not follow branches,
propagate register copies, or model calling-convention clobbers; comments are
heuristic evidence, not proof of the target.
Pass errors are reported on stderr; analysis continues with the remaining
passes and keeps any candidates already found.
Results are candidates: embedded data can decode as instructions, and targets
computed from registers cannot be resolved.

The string pass checks existing generic data symbols in readable sections
that are neither writable nor executable. It accepts NUL-terminated printable
ASCII strings of at least three bytes: strings of six bytes or fewer must be entirely
alphanumeric; longer strings must be at least 50% alphanumeric. The terminator
must lie within the section's backed, mapped bytes. Pointer-aligned candidates
are skipped when their first pointer-sized value is a VA within the image.
Matching symbols get type
`cstr` and `szPascalCase` names, with spaces and punctuation removed. Names
are capped at 64 characters, including a trailing `...` when truncated;
labels may repeat. UTF-16 strings are not recognized by this pass.

Reports contain symbol entries and comment entries (`rva`, `comment`).
Symbol entries use the factmap fields: `rva`, `name`, and `ty`. Types are
`unk` when unknown, a type name for one hint, or a union when numeric hints
disagree. Names use factmap syntax, including quotes for named symbols.
All addresses are RVAs. Text is the default;
JSON and indented JSON are available with `--format=json` and
`--format=json-pretty`.

Use `-o auto.facts.txt` to write a `#factmap` database of automatically
discovered candidates, using `SxRVA TYPE NAME` symbol facts and `CxRVA "COMMENT"` comment facts. Treat it as a
generated baseline, which can be large;
keep names, types, and corrections from your investigation in a separate
`user.facts.txt` instead of editing the generated file. These filenames are
conventions, not defaults. The output file must not already exist; choose a
new path or remove the old generated file before regenerating it.

The analysis report is still printed when `-o` is used. Use `--format=nul` to
write only the database. Generic code and data symbols use the factmap names
`C`, `D`, and `R`; direct jump stubs use `thunk`. `R` generates `rdata_RVA`
names for candidates in readable, non-writable, non-executable sections,
regardless of section name. Writable data uses `D` (`data_RVA`). Specialized labels are quoted names.
Conflicting numeric type hints become a union; code wins over data hints,
and confirmed functions win over generic code hints. Linear branch scanning
and label heuristics provide type `code`; function metadata provides type `fn`.
Import address table slots use `__imp_NAME` with type `u32` in PE32 or `u64` in PE32+, overriding
heuristic type hints. These values are read as raw integers because imported
addresses may refer to another module. Import jump stubs use `imp_NAME`. Named import labels
omit the DLL name; ordinal import labels retain it.

    pelite-cli re analysis sample.dll -o auto.facts.txt --format=nul

Use `symbol` to query a small address range rather than reading the whole
database. See `symbol --help` for the user-file format and override rules,
and `disasm --help` for using both files to label code.

When to use:

Map likely code and data targets before investigating specific addresses. For
references to one target, use `xref`.

For live analysis without a database file, use `--facts auto` with
`disasm` or `symbol`, optionally followed by `--facts user.facts.txt`
to apply your corrections. This runs the same complete analysis pipeline.
