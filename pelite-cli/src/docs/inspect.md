Inspect PE headers and directories

Usage:

    pelite-cli inspect FILE [--all] [--dos] [--rich-structure] [--headers] [--sections] [--imports] [--exports] [--relocations] [--load-config] [--tls] [--exceptions] [--debug] [--clr] [--security] [--format=text|json|json-pretty|nul]

With no topic flags, the command includes every supported structure. Select
one or more flags to limit the output, or use `--all` to include everything.
The result includes the PE format and the selected structures. An absent
optional directory appears as null. A directory-level parsing error appears
as a `$error` object for that structure.

Text is the default. Use `json` or `json-pretty` for structured output.

Examples:

    pelite-cli inspect sample.dll --sections
    pelite-cli inspect sample.dll --imports --exports --format=json
    pelite-cli inspect sample.dll --all --format=json-pretty

When to use:

Inspect exact PE headers or directories, including imports, exports,
relocations, TLS, CLR metadata, and the certificate table. For a quick first
look, use `summary`.
