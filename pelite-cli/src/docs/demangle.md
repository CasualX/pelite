Demangle compiler-generated symbol names

Usage:

    pelite-cli demangle SYMBOL... --abi msvc [--flags FLAGS] [--format=text|json|json-pretty|nul]

`--abi` is required and selects the symbol naming scheme. Currently only
`msvc` is supported. No PE file is needed. Quote the symbol to prevent the
shell from interpreting characters in its name.

For `--abi msvc`, `--flags` accepts comma-separated `DemangleFlags` names.
Omit it to use the default LLVM flags.

Provide one or more symbols. Text output prints one demangled name per line,
in input order. JSON output is an array of objects, each mapping the original
symbol to its demangled name. Duplicate symbols retain separate entries.
Invalid symbols are reported on stderr and omitted from the output. Valid
symbols are still printed, and the command exits with a nonzero status if any
symbol could not be demangled.

Examples:

    pelite-cli demangle '?func@@YAHH@Z' --abi msvc
    pelite-cli demangle '?func@@YAHH@Z' --abi msvc --format=json-pretty
    pelite-cli demangle '?func@@YAHH@Z' --abi msvc --flags NAME_ONLY,NO_MS_KEYWORDS
    pelite-cli demangle '?func@@YAHH@Z' '?other@@YAXXZ' --abi msvc --format=json

Example JSON output:

    [{"?func@@YAHH@Z":"int __cdecl func(int)"},{"?other@@YAXXZ":"void __cdecl other(void)"}]

When to use:

Decode mangled symbols into readable names and signatures using an explicit
compiler naming scheme.
