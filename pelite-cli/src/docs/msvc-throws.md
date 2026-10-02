Usage:

    pelite-cli msvc throws FILE [--demangle] [--format=text|json|json-pretty]

Extract thrown exception types independently of vtable RTTI analysis. Supports
x86 PE32 (absolute metadata pointers) and x64 PE32+ (32-bit image-relative
metadata fields). Does not require base relocations, /GR, symbols, or named
.text/.rdata sections. Other machine types are rejected.

The analyzer decodes executable sections and validates ThrowInfo candidates
referenced by x86 immediate pushes/moves or x64 RIP-relative LEA into RDX and
immediate MOV into EDX. Each result includes metadata and instruction RVAs,
raw attributes, destructor and forward compatibility helper RVAs, and all
catchable types with names, properties, displacement recipes, size/offset
fields, and copy helper RVAs. Null helper RVAs mean no helper is recorded.
ThrowInfo attribute bits 0 and 1 encode const and volatile respectively; pointer
throws encode pointee qualifiers here rather than in their shared type names.

References are candidate metadata loads, not proven calls to _CxxThrowException.
Linear disassembly and these argument patterns can miss other code generation
patterns or metadata reached through indirect loads. Only referenced, validated
metadata is reported; unreferenced records are not exhaustively scanned.

The first catchable type describes the thrown type; subsequent entries describe
permitted exception conversions, including bases and pointer conversions. These
are runtime catchability records, not a list of catch clauses in the program.
The size_or_offset field is retained as encoded and is not always a class size.
Rethrows have no new ThrowInfo and are not reported. Function EH tables, catch
regions, and unwind state maps are outside this command's scope.
