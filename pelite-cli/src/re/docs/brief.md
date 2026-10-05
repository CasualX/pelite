Summarize a function's references and constants

Usage:

    pelite-cli re brief FILE ADDRESS BYTES [--arch ARCH] [--zerofill] [--facts FACTS.txt|auto]... [--format=json-pretty]

See `pelite-cli re --help` for ADDRESS syntax (`rva:`, `va:`, or `fo:`).

Start at an instruction boundary. BYTES accepts decimal or 0xhex; instructions
starting inside the range are included even if they extend past its end.

References and constants contain distinct entries in the order they
first appear in the disassembly:

  references: Calls, external branch targets, and static memory references.
              Fact names when available, otherwise signed integer RVAs.
  indirect_branches: Number of calls and jumps through registers or memory,
                     including imports, tail calls, and dispatch jumps.
                     Counts occurrences, including repeats.
  values: Typed reads of referenced symbols with non-opaque fact types, keyed
          by symbol name. Uses the same reader as `read`, with fixed default
          limits of 256 string bytes and 1024 elements per field-length array.
  constants:
    immediates:    Literal values used by instructions.
    comparisons:   Values and masks used by CMP and TEST.
    displacements: Memory addressing offsets.

Internal jumps and constants used to address or adjust the stack are filtered.
Values stored or compared on the stack remain. Frame-pointer filtering requires
its setup to appear in the range; once detected, it applies for the rest of the
range, including blocks after early returns. This is a linear scan, not data-flow
analysis.

Use `--facts auto` for automatic symbol discovery. Add `--facts user.txt` after
it to apply your own names. Text is the default; JSON preserves the same shape:

    {"values": {}, "references": ["__imp_CreateFileW", 8192], "indirect_branches": 2, "constants": {"immediates": [128], "comparisons": [-1], "displacements": [8]}}

Read failures remain as `$error` objects in `values`, including errors inside
arrays or structs. Reading continues for other symbols; these errors do not
make `brief` return a failure status. Symbols typed `code`, `fn`, or `unk`,
weak anchors, and unnamed RVA references have no values.

Use `--zerofill` to allow typed scalar and pointer reads, including dynamic
array lengths, from a section's virtual zero-filled tail. String reads still
require file-backed bytes, as does the instruction range. Zero filling is
disabled by default.

Use `findsig` to search for encoded constants as a heuristic for finding
candidate code; use `xref` to find references to an address.

Example:

    pelite-cli re brief sample.dll rva:0x1000 0x100 --facts auto --format=json-pretty

When to use:

Screen a function for interesting APIs, data references, and constants before
spending time on full disassembly. If it looks interesting, use `disasm`
with the same range and facts to inspect how those values are used.
