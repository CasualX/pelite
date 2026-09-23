use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{error, fmt, fs, num, process, result};
use std::io::{self, BufRead, IsTerminal, Write};

mod addr;
mod disasm;
mod depwalk;
mod edit;
mod findsig;
mod hex;
mod hexdump;
mod imphash;
mod import_map;
mod inspect;
mod markov;
mod module_def;
mod msrtti;
mod printer;
mod resources;
mod rust_format_args;
mod rust_msvc;
mod rust_panic_strings;
mod strings;
mod summary;
mod symbols;
mod value_parser;
mod version_info;
mod xref;

use hex::*;
use printer::*;
use value_parser::*;

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
		.subcommand(depwalk::command())
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
		.subcommand(xref::command())
		.subcommand(symbols::command())
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
		Some(("depwalk", matches)) => depwalk::run(matches, format),
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
		Some(("xref", matches)) => xref::run(matches, format),
		Some(("symbols", matches)) => symbols::run(matches, format),
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
