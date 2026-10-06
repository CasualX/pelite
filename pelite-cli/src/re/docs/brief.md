Summarize a code range for quick reverse-engineering triage

Usage:

    pelite-cli re brief FILE ADDRESS BYTES [--arch ARCH] [--zerofill] [--facts FACTS.txt|auto]... [--format=json-pretty]

See `pelite-cli re --help` for ADDRESS syntax (`rva:`, `va:`, or `fo:`).

Start at an instruction boundary. BYTES accepts decimal or 0xhex; instructions
starting inside the range are included even if they extend past its end.

References and constants contain distinct entries in the order they
first appear in the disassembly:

  references: Calls, external branch targets, and static memory references.
              Fact names when available, otherwise signed integer RVAs.
  values: Typed reads of referenced symbols with non-opaque fact types, keyed
          by symbol name. Uses the same reader as `read`, with fixed defaults.
          String previews end with `…` when truncated.
  constants:
    immediates:    Literal values used by instructions.
    comparisons:   Values and masks used by CMP and TEST.
    displacements: Memory addressing offsets.

The control_flow section is a compact fingerprint from the same linear scan:

  direct_jumps:         Direct unconditional and conditional jumps, excluding calls.
  conditional_branches: Conditional branches, including LOOP and JCXZ variants.
  indirect_branches:    Calls and jumps through registers or memory, including
                        imports, tail calls, and dispatch jumps. Counts repeats.
  internal_targets:     Unique direct branch targets inside the requested range.
  external_branches:    Direct branches targeting outside the range.
  leaders:              Unique candidate block starts inside the range: entry,
                        internal direct branch targets, and conditional fallthroughs.
                        An empty range has no leaders. Calls add no leaders.
  returns:              Return instructions encountered, including far returns.
  return_pop:           Stack-pop values from RET: 0 without an immediate,
                        otherwise the imm16 value. A number for one unique value,
                        a deduplicated array for multiple, or [] with no RET.

Targets are candidate starts; instruction boundaries and reachability are not
validated. This does not reconstruct a CFG.

The instructions section aggregates iced-x86 architectural metadata. All lists
are unique and sorted alphabetically:

  encodings:         Encoding names from iced-x86, such as Legacy, VEX, and EVEX.
  cpuid_features:    Required feature names from iced-x86. Filtered for brevity;
                     this report is not a complete CPU compatibility requirements list.
  segment_overrides: Explicit segment prefixes only; default segments are excluded.
  register_classes:  Accessed operand and used-register families from iced-x86,
                     including implicit registers and memory address registers.
  privileged:        True if any instruction is marked privileged by iced-x86.

Internal jumps and constants used to address or adjust the stack are filtered.
Values stored or compared on the stack remain. Frame-pointer filtering requires
its setup to appear in the range; once detected, it applies for the rest of the
range, including blocks after early returns. This is a linear scan, not data-flow
analysis.

Use `--facts auto` for automatic symbol discovery. Add `--facts user.txt` after
it to apply your own names. Text is the default; JSON preserves the same shape:

    {"values": {}, "references": ["__imp_CreateFileW", 8192], "control_flow": {"direct_jumps": 3, "conditional_branches": 2, "indirect_branches": 2, "internal_targets": 2, "external_branches": 1, "leaders": 5, "returns": 1, "return_pop": 0}, "instructions": {"encodings": ["Legacy"], "cpuid_features": [], "segment_overrides": [], "register_classes": ["GPR"], "privileged": false}, "constants": {"immediates": [128], "comparisons": [-1], "displacements": [8]}}

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
