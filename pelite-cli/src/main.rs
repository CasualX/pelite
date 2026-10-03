use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{error, fmt, fs, num, process, result};
use std::io::{self, BufRead, IsTerminal, Write};

mod depwalk;
mod edit;
mod imphash;
mod inspect;
mod markov;
mod module_def;
mod printer;
mod re;
mod resources;
mod summary;
mod version_info;

use printer::*;

type Result<T = ()> = result::Result<T, Box<dyn error::Error>>;

fn get_section_name_by_rva<'a>(pe: pelite::PeFile<'a>, rva: u32) -> &'a str {
	pe.section_headers().by_rva(rva)
		.and_then(|section| section.name().ok())
		.unwrap_or("<invalid>")
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum OutputFormat {
	Text,
	Json,
	JsonPretty,
	Nul,
}

impl OutputFormat {
	fn from_matches(matches: &clap::ArgMatches) -> Self {
		match matches.get_one::<String>("format").map(String::as_str) {
			Some("json") => Self::Json,
			Some("json-pretty") => Self::JsonPretty,
			Some("nul") => Self::Nul,
			_ => Self::Text,
		}
	}
}

#[derive(Debug)]
struct CliError(String);

impl fmt::Display for CliError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.0)
	}
}

impl error::Error for CliError {}

fn err(message: impl Into<String>) -> Box<dyn error::Error> {
	Box::new(CliError(message.into()))
}

fn cli() -> clap::Command {
	let command = clap::Command::new("pelite-cli")
		.about("Inspect Windows PE binaries")
		.override_usage("pelite-cli [COMMAND] [SUBCOMMAND] <FILE> [ARGS] [OPTIONS]")
		.arg_required_else_help(true)
		.arg(summary::file_arg())
		.arg(clap::Arg::new("format")
			.long("format")
			.value_name("FORMAT")
			.value_parser(["text", "json", "json-pretty", "nul"])
			.default_value("text")
			.global(true)
			.help("Select the output format"))
		.subcommand(inspect::command())
		.subcommand(edit::command())
		.subcommand(summary::command())
		.subcommand(resources::command())
		.subcommand(depwalk::command())
		.subcommand(imphash::command())
		.subcommand(markov::command())
		.subcommand(module_def::command())
		.subcommand(version_info::command())
		.subcommand(re::command());

	with_command_guide(command)
}

fn with_command_guide(command: clap::Command) -> clap::Command {
	let guide = command.get_subcommands()
		.map(|subcommand| guide_entry(subcommand.get_name(), &when_to_use(subcommand)))
		.collect::<Vec<_>>()
		.join("\n\n");
	let guide = format!("When to use:\n\n{guide}");
	let help = match command.get_after_help() {
		Some(documentation) => format!("{}\n\n{guide}", documentation.to_string().trim_end()),
		None => guide,
	};
	command.after_help(help)
}

fn when_to_use(command: &clap::Command) -> String {
	let help = command.get_after_help().expect("every subcommand has documentation").to_string();
	let (_, section) = help.split_once("When to use:").expect("documentation has a When to use section");
	let mut snippet = String::new();
	let words = section.trim_ascii_start().lines()
		.take_while(|line| !line.trim_ascii().is_empty())
		.flat_map(str::split_whitespace);
	for word in words {
		if !snippet.is_empty() {
			snippet.push(' ');
		}
		snippet.push_str(word);
	}
	return snippet;
}

const TERMINAL_WIDTH: usize = 80;

fn guide_entry(name: &str, snippet: &str) -> String {
	let indent = "    ";
	let mut entry = format!("  {name}:\n{indent}");
	let mut line_width = indent.len();
	for word in snippet.split_whitespace() {
		if line_width > indent.len() && line_width + word.len() + 1 > TERMINAL_WIDTH {
			entry.push('\n');
			entry.push_str(indent);
			line_width = indent.len();
		}
		else if line_width > indent.len() {
			entry.push(' ');
			line_width += 1;
		}
		entry.push_str(word);
		line_width += word.len();
	}
	entry
}

fn run() -> Result {
	let matches = cli().get_matches();
	let format = OutputFormat::from_matches(&matches);
	match matches.subcommand() {
		Some(("inspect", matches)) => inspect::run(matches, format),
		Some(("edit", matches)) => edit::run(matches),
		Some(("summary", matches)) => summary::run(matches, format),
		Some(("resources", matches)) => resources::run(matches, format),
		Some(("depwalk", matches)) => depwalk::run(matches, format),
		Some(("imphash", matches)) => imphash::run(matches, format),
		Some(("markov", matches)) => markov::run(matches, format),
		Some(("module-def", matches)) => module_def::run(matches, format),
		Some(("version-info", matches)) => version_info::run(matches, format),
		Some(("re", matches)) => re::run(matches, format),
		None => summary::run(&matches, format),
		_ => unreachable!("all subcommands are handled"),
	}
}

fn main() -> process::ExitCode {
	match run().and_then(|()| io::stdout().flush().map_err(Into::into)) {
		Ok(()) => process::ExitCode::SUCCESS,
		Err(error) if is_broken_pipe(error.as_ref()) => process::ExitCode::SUCCESS,
		Err(error) => {
			let _ = writeln!(io::stderr().lock(), "pelite-cli: {error}");
			process::ExitCode::FAILURE
		},
	}
}

fn is_broken_pipe(error: &(dyn error::Error + 'static)) -> bool {
	let mut current = Some(error);
	while let Some(error) = current {
		if error.downcast_ref::<io::Error>().is_some_and(|error| error.kind() == io::ErrorKind::BrokenPipe)
			|| error.downcast_ref::<serde_json::Error>().is_some_and(|error| error.io_error_kind() == Some(io::ErrorKind::BrokenPipe)) {
			return true;
		}
		current = error.source();
	}
	false
}

#[cfg(test)]
mod cli_tests {
	use super::*;

	#[test]
	fn nested_commands_have_valid_help() {
		cli().debug_assert();
		let commands = [
			vec!["addr"], vec!["symbol"], vec!["read"], vec!["scan"],
			vec!["disasm"], vec!["disasm-raw"], vec!["trace"], vec!["hexdump"],
			vec!["strings"], vec!["findsig"], vec!["xref"],
			vec!["analysis"], vec!["demangle"], vec!["msvc", "rtti"],
			vec!["rust", "format-args"], vec!["rust", "fmt-template"],
			vec!["rust", "panic-locations"], vec!["rust", "vtables"],
		];
		for command in commands {
			let mut args = vec!["pelite-cli", "re"];
			args.extend(&command);
			args.push("--help");
			let error = cli().try_get_matches_from(args).unwrap_err();
			assert_eq!(error.kind(), clap::error::ErrorKind::DisplayHelp);
			assert!(error.to_string().contains(&format!("pelite-cli re {}", command.join(" "))));
		}
	}

	#[test]
	fn global_format_reaches_nested_commands() {
		for path in [
			vec!["re", "strings", "sample.exe"],
			vec!["re", "msvc", "rtti", "sample.exe"],
			vec!["re", "rust", "panic-locations", "sample.exe"],
		] {
			for position in 0..=path.len() {
				let mut args = path.clone();
				args.insert(position, "--format=json");
				args.insert(0, "pelite-cli");
				let matches = cli().try_get_matches_from(args).unwrap();
				let mut leaf = &matches;
				while let Some((_, child)) = leaf.subcommand() {
					leaf = child;
				}
				assert!(matches!(OutputFormat::from_matches(&matches), OutputFormat::Json));
				assert!(matches!(OutputFormat::from_matches(leaf), OutputFormat::Json));
			}
		}
		let matches = cli().try_get_matches_from(["pelite-cli", "sample.exe"]).unwrap();
		assert!(matches.subcommand().is_none());
		assert_eq!(matches.get_one::<PathBuf>("file"), Some(&PathBuf::from("sample.exe")));
	}
}
