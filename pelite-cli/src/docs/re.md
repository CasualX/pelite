When to use:

Investigate code, data, symbols, and compiler patterns in binaries.
Select a subcommand to inspect an address or discover targets for analysis.

ADDRESS is `kind:EXPR`, where `rva:` selects a relative virtual address,
`va:` a virtual address, and `fo:` a file offset. Examples: `rva:4096`,
`va:0x180001000`, and `fo:1024`. The result must fit the address kind:
32 bits for RVA, 64 bits for VA, and the platform's `usize` for file offsets.

RANGE is `kind:START..END` or `kind:START..+LENGTH`. One address kind applies
to both endpoints, and START, END, and LENGTH are expressions. Without `+`,
END is independent of START; with `+`, the end is START plus LENGTH.
For example, `rva:0x1000..+10` ends at `0x100a`. The end is excluded and must
be greater than or equal to the start. Equal endpoints produce an empty range.
Both endpoints must fit the address kind; adding a length must not overflow.

Expressions accept decimal and `0x`-prefixed hexadecimal literals,
and binary `+`, `-`, and `*`. Multiplication takes precedence; operations of
the same precedence are evaluated left to right. For example,
`rva:0x1000+0x20*8-4*4` evaluates to `rva:0x10f0`, and
`rva:0x1000+0x20*8..+32*8-4` selects a range of 252 bytes.
Literals and intermediate results must fit in `u64`; overflow and underflow
are errors, even if later operations would bring the result back into range.
Parentheses, unary operators, and whitespace inside expressions are rejected.
Quote arguments containing `*` to prevent shell expansion.

Examples:

    pelite-cli re disasm sample.dll rva:0x1000..+0x100
    pelite-cli re xref sample.dll rva:0x1000
    pelite-cli re msvc rtti sample.dll
    pelite-cli re rust panic-locations sample.exe

Use `pelite-cli re COMMAND --help` for arguments and examples.
