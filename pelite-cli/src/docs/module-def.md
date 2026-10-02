Generate a module-definition file from a DLL

Usage:

    pelite-cli module-def DLL [--format=text|json|json-pretty|nul]

Text output writes `LIBRARY` using the name stored in the DLL's export
directory, followed by `EXPORTS` and its named exports. The stored name may
differ from the input filename. Ordinal-only exports are not included.

Redirect text output to save a `.def` file. Use `json` or `json-pretty` to get
an object with `library` and `exports` instead.

Examples:

    pelite-cli module-def sample.dll > sample.def
    pelite-cli module-def sample.dll --format=json-pretty

Example text excerpt:

```text
LIBRARY Demo.dll
EXPORTS
??0Passwds@@QEAA@PEBD@Z
??1Passwds@@QEAA@XZ
```

When to use:

Generate a `.def` file from named DLL exports, for example to create an import
library.
