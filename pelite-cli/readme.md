# pelite-cli

A command-line companion to [pelite](https://crates.io/crates/pelite) for exploring Windows PE executables and DLLs. It reads both PE32 and PE32+ files and can print human-readable text or JSON.

## Get started

From the repository root, run the CLI with Cargo:

```console
cargo run -p pelite-cli -- demo/Demo64.dll
cargo run -p pelite-cli -- inspect demo/Demo64.dll --imports --exports
cargo run -p pelite-cli -- depwalk demo/Demo64.dll -L /path/to/windows/dlls
cargo run -p pelite-cli -- re xref demo/Demo64.dll rva:0x1000
cargo run -p pelite-cli -- --help
```

Reverse engineering commands live under `re`. Run `cargo run -p pelite-cli -- re --help` to browse them.

Passing a file directly runs `summary`. The summary covers image details, sections, imports, hashes, and useful signals for a first look.

To install the binary from this checkout, run `cargo install --path pelite-cli`.

Run `cargo run -p pelite-cli -- COMMAND --help` for arguments and examples. The [demo examples](../demo/readme.md) show inspection, resource extraction, signature searches, and generated output files.

## Output formats

Text is the default. Use the global `--format=json` option for compact JSON or `--format=json-pretty` for indented JSON:

```console
cargo run -p pelite-cli -- summary demo/Demo64.dll --format=json-pretty
cargo run -p pelite-cli -- resources tree demo/Demo.dll --format=json
```

## Invocation tracing

Set `PELITE_CLI_TRACE_LOGFILE` to a log file path to append every invocation before
argument parsing, including help requests and invalid commands. The file is created
if needed. Each line contains a Unix timestamp in seconds with millisecond precision,
followed by the executable and arguments. Arguments are quoted and escaped when
needed to keep the entry on one line. Logging failures are silently ignored.

## Tests

Run the CLI tests from the repository root:

```sh
cargo test -p pelite-cli
```

Snapshot tests compare command output with checked-in expected text and show a
colored line diff on mismatch. To update snapshots after an intentional change:

```sh
UPDATE_SNAPSHOTS=1 cargo test -p pelite-cli --test snapshot
```

License
-------

Licensed under [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.html), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
