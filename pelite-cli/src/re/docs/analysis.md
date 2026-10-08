Discover candidate code and data symbols in a PE image.

Usage:

    pelite-cli re analysis FILE [-o FACTS.txt] [--measure-load-time] [--timings] [--format=text|json|json-pretty|nul]

Scans i386 and AMD64 PE images for likely code, functions, data, strings,
and references. Combines disassembly heuristics with PE metadata, including
exports, imports, relocations, TLS callbacks, entry points, and x64 exception
records.

Analysis includes:

  - Direct calls, branches, memory references, and address-like immediates.
  - Exports, imports, relocations, and entry-point functions.
  - ASCII strings in read-only data sections (type `cstr`).
  - x64 RUNTIME_FUNCTION records, adding comments at BeginAddress.
  - Import jump stubs (`imp_NAME`) and indirect call annotations.
  - Generic code (`C`), data (`D`), and read-only data (`R`) symbols.
  - Named entry points (`EntryPoint`, `TlsCallback_N`) with type `fn`.

Results are heuristic candidates, not guaranteed symbols. Embedded data may
resemble instructions, and indirect targets may remain unresolved.
Pass failures are reported to stderr without stopping the remaining analysis.

Output:

  By default, prints a text report of symbols and comments using RVAs.
  Symbols contain an address, name, and type; comments contain an address
  and text.

  --format=text         Text report (default).
  --format=json         JSON report.
  --format=json-pretty  Indented JSON report.
  --format=nul          Suppress the report.

  -o FACTS.txt          Write a #factmap database (file must not exist).
  --timings             Report analysis pass timings to stderr.
  --measure-load-time   Measure loading the generated database (requires -o).

The report is still printed with -o unless --format=nul is specified.

Examples:

    # Discover symbols and print a report
    pelite-cli re analysis sample.dll

    # Generate a database without printing the report
    pelite-cli re analysis sample.dll -o auto.facts.txt --format=nul

    # Benchmark analysis and database loading
    pelite-cli re analysis sample.dll -o auto.facts.txt --timings --measure-load-time --format=nul

Workflow:

  Treat auto.facts.txt as a generated baseline. Keep manually verified
  symbols, names, and corrections in a separate user.facts.txt.
  Regenerate the automatic database as needed.

  Use --facts auto with `disasm` or `symbol` to run analysis on demand,
  without writing a database. Add --facts user.facts.txt to apply overrides.

  See also:
    symbol --help   Query symbols and manage factmap overrides.
    disasm --help   Disassemble code with symbol annotations.
    xref --help     Find references to a specific address.

When to use:

Discover likely code, functions, data, and symbol names
across a PE image before investigating specific addresses. Generate a
reusable factmap database with `-o`, or use `--facts auto` with `disasm`
or `symbol` for on-demand analysis without saving a file. For finding
references to a specific address, use `xref` instead.
