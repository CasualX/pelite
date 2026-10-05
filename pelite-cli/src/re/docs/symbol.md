Look up symbols in a PE address range

Usage:

    pelite-cli re symbol FILE ADDRESS BYTES [--facts FACTS.txt|auto]... [--format=text|json|json-pretty|nul]

Look up nearby symbols within an address range in the given symbol files.
FILE is the reference PE file used to convert addresses and for automatic
analysis when requested. Results are sorted by RVA and include names and
types. With no symbol files, the result is empty.

See `pelite-cli re --help` for ADDRESS syntax (`rva:`, `va:`, or `fo:`).
BYTES is the size of the search range, in decimal or 0xhex.
The range includes ADDRESS and excludes ADDRESS + BYTES.

Use `--facts` to load a `#factmap` symbol file, or repeat it to load several.
Load `auto.facts.txt` first and `user.facts.txt` last so your analysis takes
precedence. Symbol entries and files are applied in order: a later symbol at an
RVA replaces the earlier name and type. Entries do not need to be sorted.
`Sx1000 unk undef` removes an earlier symbol at that RVA; a later entry can
define it again. The weak name `_` fills an undefined RVA without replacing
an existing symbol. Weak entries do not contribute labels in disassembly.
These rules also apply to symbol labels in `disasm` and `disasm-raw`.

To record analysis, create `user.facts.txt` with `#factmap` on the first line,
then one fact per line. Symbol facts use `SxRVA TYPE NAME`, with hexadecimal
RVAs fitting in 32 bits. TYPE uses the syntax from `read --help`; quote a
type containing spaces using JSON string syntax. NAME is a JSON-quoted string
or one of `C`, `D`, `R`, `_`, `fn`, `thunk`, and `undef`. Generic names display as
`code_1000`, `data_2000`, `rdata_3000`, and so on. `R` marks read-only data;
for example, `Sx3000 unk R` defines `rdata_3000`. Quoted names such as `"fn"`
or `"_"` are literal names rather than special markers.

Comment facts use `CxRVA "COMMENT"`, with a JSON-quoted string. Reference facts
use `RxRVA 0xTARGET`, with both RVAs in hexadecimal. Symbol lookup uses only symbol
facts. Blank lines and full lines beginning with `#` after the header are allowed.
Parse errors include the source filename and line number.

```text
#factmap
# Confirmed by following callers and reading the referenced data.
Sx1000 code "parse_config"
Sx2000 "struct { count: u32, values: *[u32; count] }" "config_table"
Cx1000 "Reads the count before following the values pointer."
Rx1000 0x2000
# Discard a false candidate from analysis.
Sx2010 unk undef
```

Add discoveries and corrections to this small file while keeping the generated
baseline intact. Use comment lines to record evidence or uncertainty; a name
or type from automatic analysis is only a hint until you verify it.

Examples:

    pelite-cli re symbol sample.dll rva:0x1000 200 --facts auto.facts.txt --facts user.facts.txt
    pelite-cli re symbol sample.dll va:0x180001000 0x100 --facts auto.facts.txt

Example JSON output:

```json
{
  "start_rva": 4096,
  "end_rva": 4296,
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
range or a type for `read`. Select a range beginning before the address of
interest to include preceding symbols, and widen it if needed. This is a range
query, not a nearest-symbol lookup or a function-boundary detector.
