Display code and typed data from a PE range using loaded factmaps

Usage:

    pelite-cli re text-view FILE ADDRESS BYTES [--facts FACTS.txt|auto]...

Examples:

    pelite-cli re text-view sample.dll rva:0x1000 0x100 --facts auto
    pelite-cli re text-view sample.dll sym:Entry 64 --facts auto --facts user.txt
    pelite-cli re text-view sample.dll rva:0x2000 128 --facts user.txt --format=json-pretty

ADDRESS and BYTES follow the common re conventions, except that text-view retains
multiple symbols at an RVA. Each symbol contributes its own type interpretation.
An undef symbol removes all earlier symbols at that RVA; later symbols can add
new interpretations.
sym:NAME uses the shared symbol lookup used by other commands: later symbols at
an RVA replace earlier ones for lookup, while the listing still displays them all.
References show all known target names. Later comments override earlier comments;
decode regions and references accumulate independently.
Instructions are decoded from each region's own start, even before the requested
range. Whole instructions and typed fields intersecting the range are displayed.
Instruction operands and pointer targets use RVAs; pointers outside the image
are displayed as virtual addresses. References retain numeric targets and add
known symbol names.

Scalar symbols use `Name: Type = value`; aggregate fields and array elements
use `.field: Type = value` and `[index]: Type = value`. Types are part of the
declaration, not comments. Array headers use `Name: [Type;count]`. Top-level
`fn`, `unk`, `struct`, and `union` keep their keyword before the name.
The `code` type renders as a bare label (`Name:`), without a keyword prefix.
Scalar declarations are single data rows; aggregate and opaque headers are
zero-length symbol rows. Failed scalar reads retain a declaration header.

Structs, unions, and fixed arrays are flattened into field paths using the type
system's layout. Pointers occupy only their pointer slots. Dynamic arrays are
currently unsupported and produce an error annotation with raw-byte fallback.
Strings include their terminators in coverage and display at most 256 units.

Uninterpreted file bytes appear as `.bytes`. Type layout padding uses
`.pad`, and explicit alignment facts use `.align`. Unavailable raw bytes (including
virtual zero-fill) appear as `.bytes ?? ??`, with at most 16 bytes per row.
Fixed-size typed fields without enough file-backed bytes display `= ?` and
retain their declared size and type; structs and arrays preserve field paths.
These unknown values consume their field bytes without producing read errors.
Strings still require a readable terminator to determine their coverage.
Other failed interpretations produce error annotations and leave bytes uncovered.
Labels, comments, and references do not consume bytes; raw runs split at them.

JSON formats emit an array of rows with rva, length, kind, text, syntax, and an
optional overlaps RVA. Each syntax entry is {start, end, class}: a half-open
UTF-8 byte range into the unescaped row text. Ranges are sorted, nonempty, and
non-overlapping, and their boundaries never split a UTF-8 character. Unstyled
gaps (such as whitespace) inherit the default style. The printed section/address
prefix, indentation, and overlap suffix are outside the row text and its ranges.

Classes are keyword, type, symbol, field, mnemonic, register, number, address,
string, comment, directive, punctuation, unknown, and error. Type expressions
and field paths are each styled as a whole. Instruction classes come directly
from the iced-x86 formatter; typed declarations are tagged as they are emitted.
JSON always includes syntax metadata and never includes ANSI escapes. Text
uses these same ranges for ANSI colors only when stdout is a terminal;
redirected text stays plain. --format=nul suppresses output.

Repository examples with manual factmaps, saved listings, and reproduction
commands live in `pelite-cli/examples/text-view/README.md`. They cover clean
PE32/PE32+ DLLs, Deadlock CreateInterface, and EA AntiCheat opaque predicates
and a proven register-jump target annotated with RefFact.

When to use:

Inspect a PE range using the interpretations already declared by loaded factmaps.
Unlike disasm, text-view only decodes DecodeFact regions. Unlike read, it combines
code, typed fields, annotations, and raw gaps without following pointers. It does
not discover facts. Overlapping interpretations remain visible, marked with `~`.
