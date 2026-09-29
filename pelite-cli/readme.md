# pelite-cli

A command-line companion to [pelite](https://crates.io/crates/pelite) for exploring Windows PE executables and DLLs. It reads both PE32 and PE32+ files and can print human-readable text or JSON.

## Get started

From the repository root, run the CLI with Cargo:

```console
cargo run -p pelite-cli -- demo/Demo64.dll
cargo run -p pelite-cli -- inspect demo/Demo64.dll --imports --exports
cargo run -p pelite-cli -- depwalk demo/Demo64.dll -L /path/to/windows/dlls
cargo run -p pelite-cli -- xref demo/Demo64.dll rva:0x1000
cargo run -p pelite-cli -- --help
```

Passing a file directly runs `summary`. The summary covers image details, sections, imports, hashes, and useful signals for a first look.

To install the binary from this checkout, run `cargo install --path pelite-cli`.

## Commands

| Command | What it does |
| --- | --- |
| `summary` | Show a first-look report with image details, hashes, sections, imports, and signals to review. |
| `inspect` | Inspect selected PE headers and directories, such as sections, imports, and exports. |
| `resources tree` | List the resource tree. |
| `resources cat` | Read a resource payload. |
| `resources fsck` | Validate the resource tree. |
| `resources icons list` | List and validate icon groups. |
| `resources icons extract` | Extract icon groups as `.ico` files. |
| `resources cursors list` | List and validate cursor groups. |
| `resources cursors extract` | Extract cursor groups as `.cur` files. |
| `version-info` | Read the version-information resource. |
| [`strings`](docs/strings.md) | Find ASCII, UTF-8, and UTF-16LE strings in PE sections with a heuristic confidence score. |
| [`findsig`](docs/findsig.md) | Search the image for byte patterns. |
| [`addr`](docs/addr.md) | Convert between an RVA, virtual address, and file offset. |
| [`xref`](docs/xref.md) | Find candidate pointers and relative references; confirm matches in surrounding code or data. |
| [`read`](docs/read.md) | Read scalars, strings, typed pointers, arrays, structs, and unions at an address. |
| [`symbols`](docs/symbols.md) | Discover candidate code and data symbols using disassembly and base relocations. |
| [`hexdump`](docs/hexdump.md) | Display bytes from an address range. |
| [`disasm`](docs/disasm.md) | Disassemble instructions from an address range. |
| [`disasm-raw`](docs/disasm-raw.md) | Disassemble arbitrary x86 or x86_64 bytes from a file or standard input. |
| [`imphash`](docs/imphash.md) | Calculate the conventional MD5 import hash. |
| [`depwalk`](docs/depwalk.md) | Walk imported modules and forwarded exports, check symbols, and report missing modules, malformed PE data, and dependency cycles. |
| [`module-def`](docs/module-def.md) | Generate a module-definition file from a DLL's exports. |
| [`msrtti`](docs/msrtti.md) | Dump Microsoft C++ RTTI, vtables, and class hierarchies. |
| `rust format-args` | Find Rust `format_args!` templates in x64 MSVC binaries. |
| `rust panic-strings` | Find Rust panic messages and location records in x64 MSVC binaries. |
| `rust vtables` | Find candidate Rust trait vtables in PE32 and PE32+ binaries. |
| [`markov`](docs/markov.md) | Generate bytes from executable PE sections using a Markov chain. |
| [`edit`](docs/edit.md) | Edit a PE file in place or write a separate file, including repair or conversion. |

Run `cargo run -p pelite-cli -- COMMAND --help` for arguments and options. The [demo examples](../demo/readme.md) show inspection, resource extraction, signature searches, and generated output files.

## Output formats

Text is the default. Use the global `--format=json` option for compact JSON or `--format=json-pretty` for indented JSON:

```console
cargo run -p pelite-cli -- summary demo/Demo64.dll --format=json-pretty
cargo run -p pelite-cli -- resources tree demo/Demo.dll --format=json
```
