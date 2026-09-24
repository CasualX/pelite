use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{error, fmt, fs, num, process, result};
use std::io::{self, BufRead, IsTerminal, Write};

mod addr;
mod disasm;
mod edit;
mod findsig;
mod hexdump;
mod imphash;
mod inspect;
mod markov;
mod module_def;
mod msrtti;
mod resources;
mod rust_format_args;
mod rust_msvc;
mod rust_panic_strings;
mod strings;
mod summary;
mod value_parser;
mod version_info;

use value_parser::*;

type Result<T = ()> = result::Result<T, Box<dyn error::Error>>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum OutputFormat {
	Text,
	Json,
	JsonPretty,
}

impl OutputFormat {
	fn from_matches(matches: &clap::ArgMatches) -> Self {
		match matches.get_one::<String>("format").map(String::as_str) {
			Some("json") => Self::Json,
			Some("json-pretty") => Self::JsonPretty,
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
	clap::Command::new("pelite-cli")
		.about("Inspect Windows PE binaries")
		.arg_required_else_help(true)
		.arg(summary::file_arg())
		.arg(clap::Arg::new("format")
			.long("format")
			.value_name("FORMAT")
			.value_parser(["text", "json", "json-pretty"])
			.default_value("text")
			.global(true)
			.help("Select the output format"))
		.subcommand(inspect::command())
		.subcommand(edit::command())
		.subcommand(summary::command())
		.subcommand(resources::command())
		.subcommand(addr::command())
		.subcommand(disasm::command())
		.subcommand(hexdump::command())
		.subcommand(strings::command())
		.subcommand(findsig::command())
		.subcommand(imphash::command())
		.subcommand(markov::command())
		.subcommand(module_def::command())
		.subcommand(msrtti::command())
		.subcommand(rust_format_args::command())
		.subcommand(rust_panic_strings::command())
		.subcommand(version_info::command())
}

fn print_json<T: serde::Serialize>(value: &T, pretty: bool) -> Result {
	let stdout = io::stdout();
	let writer = stdout.lock();
	match pretty {
		false => serde_json::to_writer(writer, value)?,
		true => serde_json::to_writer_pretty(writer, value)?,
	}
	println!();
	Ok(())
}

fn run() -> Result {
	let matches = cli().get_matches();
	let format = OutputFormat::from_matches(&matches);
	match matches.subcommand() {
		Some(("inspect", matches)) => inspect::run(matches, format),
		Some(("edit", matches)) => edit::run(matches),
		Some(("summary", matches)) => summary::run(matches, format),
		Some(("resources", matches)) => resources::run(matches, format),
		Some(("addr", matches)) => addr::run(matches, format),
		Some(("disasm", matches)) => disasm::run(matches, format),
		Some(("hexdump", matches)) => hexdump::run(matches, format),
		Some(("strings", matches)) => strings::run(matches, format),
		Some(("findsig", matches)) => findsig::run(matches, format),
		Some(("imphash", matches)) => imphash::run(matches, format),
		Some(("markov", matches)) => markov::run(matches, format),
		Some(("module-def", matches)) => module_def::run(matches, format),
		Some(("msrtti", matches)) => msrtti::run(matches, format),
		Some(("rust-format-args", matches)) => rust_format_args::run(matches, format),
		Some(("rust-panic-strings", matches)) => rust_panic_strings::run(matches, format),
		Some(("version-info", matches)) => version_info::run(matches, format),
		None => summary::run(&matches, format),
		_ => unreachable!("all subcommands are handled"),
	}
}

fn main() -> process::ExitCode {
	match run() {
		Ok(()) => process::ExitCode::SUCCESS,
		Err(error) => {
			eprintln!("pelite-cli: {error}");
			process::ExitCode::FAILURE
		},
	}
}
