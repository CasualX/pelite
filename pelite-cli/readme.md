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

## Install the agent skill

After installing the binary above, make sure `pelite-cli` is on your agent's `PATH`. Copy the [pelite skill](skills/pelite) into a skills directory so the agent can choose it automatically for PE inspection and reverse engineering, without you naming the tool.

Choose a destination (project paths are relative to the project where you want to use the skill):

| Agent | All your projects | One project |
| --- | --- | --- |
| [Codex](https://developers.openai.com/codex/skills/) | `~/.agents/skills/pelite` | `.agents/skills/pelite` |
| [Claude Code](https://code.claude.com/docs/en/skills) | `~/.claude/skills/pelite` | `.claude/skills/pelite` |
| [GitHub Copilot](https://docs.github.com/en/copilot/how-tos/copilot-on-github/customize-copilot/customize-cloud-agent/add-skills) | `~/.agents/skills/pelite` | `.agents/skills/pelite` |

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
