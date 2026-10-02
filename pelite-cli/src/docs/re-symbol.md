Look up symbols in a PE address range

Usage:

    pelite-cli re symbol FILE RANGE [--symbols SYMBOLS.txt]... [--format=text|json|json-pretty|nul]

Look up nearby symbols within an address range in the given symbol files.
FILE is the reference PE file used to convert addresses; the command does not
discover symbols on its own. Results are sorted by RVA and include names and
types. With no symbol files, the result is empty.

See `pelite-cli re --help` for RANGE syntax.

Use `--symbols` to load a `#symtext` symbol file, or repeat it to load several.
Load `auto.symbols.txt` first and `user.symbols.txt` last so your analysis takes
precedence. Entries and files are applied in order: the last entry at an RVA
replaces the earlier name and type. Entries do not need to be sorted. An entry
such as `0x1000 unk undef` removes an earlier symbol at that RVA; a later entry
can define it again. These rules also apply to symbol labels in `re disasm` and
`re disasm-raw`.

To record analysis, create `user.symbols.txt` with `#symtext` on the first line,
then one `RVA TYPE NAME` entry per line. RVAs must be `0x`-prefixed hexadecimal.
TYPE uses the syntax from `re read --help`; quote a type containing spaces using JSON
string syntax. NAME is a JSON-quoted string or one of `C`, `D`, `fn`, `thunk`,
and `undef`. Generic names display as `code_1000`, `data_2000`, and so on.
Blank lines and full lines beginning with `#` after the header are allowed.

```text
#symtext
# Confirmed by following callers and reading the referenced data.
0x1000 code "parse_config"
0x2000 "struct { count: u32, values: *[u32; count] }" "config_table"
# Discard a false candidate from autoanalysis.
0x2010 unk undef
```

Add discoveries and corrections to this small file while keeping the generated
baseline intact. Use comment lines to record evidence or uncertainty; a name
or type from automatic analysis is only a hint until you verify it.

Examples:

    pelite-cli re symbol sample.dll rva:0x1000..0x1100 --symbols auto.symbols.txt --symbols user.symbols.txt
    pelite-cli re symbol sample.dll va:0x180001000..+0x100 --symbols auto.symbols.txt

Example JSON output:

```json
{
  "start_rva": 4096,
  "end_rva": 4352,
  "symbols": [
    {
      "rva": 4096,
      "name": "main",
      "ty": "code"
    }
  ]
}
```

When to use:

Inspect names and type hints near an address before choosing a disassembly
range or a type for `re read`. Select a range beginning before the address of
interest to include preceding symbols, and widen it if needed. This is a range
query, not a nearest-symbol lookup or a function-boundary detector.
