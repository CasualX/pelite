Look up symbols in a PE address range.

Usage:

    pelite-cli re symbol FILE ADDRESS BYTES [--facts FACTS.txt|auto]... [--format=text|json|json-pretty|nul]

Looks up symbols within an address range using one or more #factmap files.
Results are sorted by RVA and include symbol names and types. Without --facts, the result is empty.

ADDRESS accepts rva:, va:, or fo: prefixes (see `re --help`).
BYTES is the range size in decimal or 0xhex. The end is exclusive.

Symbol sources:

  --facts FACTS.txt   Load symbols from a #factmap file.
  --facts auto        Run automatic analysis without saving a database.

Multiple --facts arguments are applied in order. Later definitions override
earlier ones at the same RVA. Load auto.facts.txt first and user.facts.txt
last to preserve your corrections.

Factmap format:

  Files begin with #factmap, followed by one fact per line.
  Blank lines and comments beginning with # are allowed.

  SxRVA TYPE NAME      Symbol name and type.
  CxRVA "COMMENT"      Comment at an address.
  RxRVA 0xTARGET       Reference between two RVAs.
  FxRVA CONTENT        Function metadata (JSON).

  RVAs are hexadecimal. Names are JSON-quoted strings or special markers.
  Types follow the syntax described in `read --help`.

  Symbol name markers:
    C       Generic code      (code_RVA)
    D       Generic data      (data_RVA)
    R       Read-only data    (rdata_RVA)
    fn      Generic function  (fn_RVA)
    thunk   Jump stub         (thunk_RVA)
    _       Weak name; does not override existing symbols.
    undef   Remove an earlier symbol at this RVA.

  Quote names to use them literally, e.g. "fn" or "_".
  Use `SxRVA unk undef` to discard a false candidate.
  Function metadata at the same RVA merges JSON objects across files;
  other values are replaced by later definitions.

Example user.facts.txt:

    #factmap
    # Confirmed symbols and corrections
    Sx1000 code "parse_config"
    Sx2000 "struct { count: u32, values: *[u32; count] }" "config_table"
    Cx1000 "Reads the count before following the values pointer."
    Rx1000 0x2000
    Sx2010 unk undef

Keep manually verified symbols and corrections in user.facts.txt,
separate from the generated auto.facts.txt baseline. Record evidence
and uncertainty in comments.

Output:

  --format=text         Text report (default).
  --format=json         JSON report.
  --format=json-pretty  Indented JSON report.
  --format=nul          Suppress the report.

Examples:

    # Query symbols from automatic analysis
    pelite-cli re symbol sample.dll rva:0x1000 200 --facts auto

    # Query generated symbols with user corrections
    pelite-cli re symbol sample.dll rva:0x1000 200 --facts auto.facts.txt --facts user.facts.txt

See also:
  analysis --help   Discover symbols and generate a factmap database.
  disasm --help     Disassemble code using symbol annotations.
  read --help       Inspect typed data and type syntax.

When to use:

Inspect known or automatically discovered names and type
hints within a PE address range, typically before disassembly or typed data
inspection with `read`. Load generated and user factmaps together to see
corrected symbols, or use `--facts auto` for on-demand analysis. This is a
range query, not a nearest-symbol lookup or function-boundary detector.
