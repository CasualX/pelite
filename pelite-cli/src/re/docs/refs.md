Summarize a function's references and constants

Usage:

    pelite-cli re refs FILE ADDRESS BYTES [--facts FACTS.txt|auto]... [--format=json-pretty]

See `pelite-cli re --help` for ADDRESS syntax (`rva:`, `va:`, or `fo:`).

Start at an instruction boundary. BYTES accepts decimal or 0xhex; instructions
starting inside the range are included even if they extend past its end.

Each category lists distinct values in the order they first appear in the
disassembly, preserving a hint of the function's progression:

  references: Calls, external branch targets, and static memory references.
              Fact names when available, otherwise signed integer RVAs.
  indirect_calls: Number of calls and jumps through registers or memory, including
                  imports, tail calls, and dispatch jumps. Counts occurrences.
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

    {"references": ["__imp_CreateFileW", 8192], "indirect_calls": 2, "constants": {"immediates": [128], "comparisons": [-1], "displacements": [8]}}

Use `re findsig` to search for encoded constants as a heuristic for finding
candidate code; use `re xref` to find references to an address.

Example:

    pelite-cli re refs sample.dll rva:0x1000 0x100 --facts auto --format=json-pretty

When to use:

Screen a function for interesting APIs, data references, and constants before
spending time on full disassembly. If it looks interesting, use `re disasm`
with the same range and facts to inspect how those values are used.
