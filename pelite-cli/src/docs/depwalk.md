Check PE dependencies and imported symbols

Usage:

    pelite-cli depwalk FILE [-L DIR]... [--format=text|json|json-pretty]

The command follows imported modules and forwarded exports, checks that
requested symbols exist, and reports missing modules, wrong architectures,
invalid PE data, and dependency cycles.

It searches beside FILE, then in each `-L` (`--search-directory`) directory.
On Windows it checks system directories next. It then checks the current
directory, followed by PATH directories on Windows. Repeat `-L` to add
directories containing the target's DLLs.

Text is the default. A missing module has a null path. The report is printed
even when issues are found; any issue makes the command exit with a failure status.

Examples:

    pelite-cli depwalk sample.exe -L /path/to/dlls
    pelite-cli depwalk sample.exe -L /path/to/dlls --format=json-pretty

Example JSON excerpt:

```json
{
  "modules": [
    { "name": "Demo64.dll", "path": "demo/Demo64.dll", "imports": ["KERNEL32.dll", "MSVCR120.dll"] },
    { "name": "KERNEL32.dll", "path": null, "imports": [] }
  ],
  "issues": [
    { "kind": "module_not_found", "module": "KERNEL32.dll", "detail": "no matching file in search directories" }
  ]
}
```

When to use:

Diagnose missing DLLs or imports, architecture mismatches, and dependency
cycles that may prevent loading.
