Browse, validate, and extract PE resources

Usage:

    pelite-cli resources tree FILE [PATH] [--format=text|json|json-pretty|nul]
    pelite-cli resources cat FILE PATH [--format=text|json|json-pretty|nul]
    pelite-cli resources fsck FILE [--format=text|json|json-pretty|nul]
    pelite-cli resources KIND list FILE [--format=text|json|json-pretty|nul]
    pelite-cli resources KIND extract FILE DESTINATION [NAME]... [--force] [--format=text|json|json-pretty|nul]

Resource paths are absolute, for example `/#MANIFEST/#1/#1033`. `tree` lists
entries beneath PATH (default `/`) with sizes and code pages. `cat` writes the
selected resource's exact bytes in text mode, so it can be redirected to a
file. In JSON modes, `cat` returns base64 data with its path, size, and code
page. `fsck` checks the full tree and reports directory, file, and byte counts.
If the PE has no resource directory, every resource command returns null.
Extraction creates no destination in this case.

KIND is `icons` or `cursors`. They work with reconstructed `.ico` and `.cur` files. `list`
shows available groups. `extract` writes named groups (for example `MAIN` or
`#103`), or every group when no NAME is given. It creates DESTINATION if
needed and refuses to overwrite existing files unless `--force` is set.

Text is the default. Use `json` or `json-pretty` for structured output.

Examples:

    pelite-cli resources tree sample.dll /#MANIFEST
    pelite-cli resources cat sample.dll /#MANIFEST/#1/#1033 > manifest.xml
    pelite-cli resources fsck sample.dll --format=json
    pelite-cli resources icons list sample.dll
    pelite-cli resources icons extract sample.dll icons '#103'
    pelite-cli resources cursors extract sample.dll cursors

When to use:

Browse, validate, or extract PE resources, including raw payloads, icons, and
cursors.
