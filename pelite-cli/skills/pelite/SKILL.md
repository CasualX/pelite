---
name: pelite
description: "Use pelite-cli whenever a task calls for looking inside a Windows PE file (.exe, .dll, .sys), including inspecting its structure or contents and reverse engineering its code or data, even if the user does not name pelite."
---

# pelite

`pelite-cli` does static analysis of Windows PE32/PE32+ images. It parses bytes and never runs the sample.
Keep the analysis static; run a sample only if the user explicitly asks and names a sandbox.

## Availability

Check whether `pelite-cli` is available on the PATH using the appropriate command for the current environment.

If unavailable, inform the user and ask for permission to install it:

```sh
cargo install --git https://github.com/CasualX/pelite pelite-cli
```

Do not install or update the tool without user approval.

## Usage

Start with `pelite-cli --help` to discover what the installed version can do for the task.
Then read the full `--help` for relevant commands and subcommands before using them for the first time in a session.
The help is the manual: it provides guidance, examples and caveats as well as argument syntax.
Use it as the source of truth for current capabilities and usage.
