When to use:

Investigate code, data, symbols, and compiler patterns in binaries.
Select a subcommand to inspect an address or discover targets for analysis.

Examples:

    pelite-cli re disasm sample.dll rva:0x1000..+0x100
    pelite-cli re xref sample.dll rva:0x1000
    pelite-cli re msvc rtti sample.dll
    pelite-cli re rust panic-locations sample.exe

Use `pelite-cli re COMMAND --help` for arguments and examples.
