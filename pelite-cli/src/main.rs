use std::error::Error;
use std::fmt;
use std::process::ExitCode;

use clap::{Arg, ArgMatches, Command};

mod dump;
mod disasm;
mod extract_icons;
mod findsig;
mod hexdump;
mod imphash;
mod markov;
mod module_def;
mod msrtti;
mod rust_format_args;
mod rust_msvc;
mod rust_panic_strings;
mod rva_range;
mod strings;
mod version_info;

type Result<T = ()> = std::result::Result<T, Box<dyn Error>>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum OutputFormat {
	Text,
	Json,
	JsonPretty,
}

impl OutputFormat {
	fn from_matches(matches: &ArgMatches) -> Self {
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

impl Error for CliError {}

fn err(message: impl Into<String>) -> Box<dyn Error> {
	Box::new(CliError(message.into()))
}

fn cli() -> Command {
	Command::new("pelite-cli")
		.about("Inspect Windows PE binaries")
		.arg_required_else_help(true)
		.subcommand_required(true)
		.arg(Arg::new("format")
			.long("format")
			.value_name("FORMAT")
			.value_parser(["text", "json", "json-pretty"])
			.default_value("text")
			.global(true)
			.help("Select the output format"))
		.subcommand(dump::command())
		.subcommand(disasm::command())
		.subcommand(extract_icons::command())
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
	let stdout = std::io::stdout();
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
		Some(("dump", matches)) => dump::run(matches, format),
		Some(("disasm", matches)) => disasm::run(matches, format),
		Some(("extract-icons", matches)) => extract_icons::run(matches, format),
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
		_ => unreachable!("clap requires a subcommand"),
	}
}

fn main() -> ExitCode {
	match run() {
		Ok(()) => ExitCode::SUCCESS,
		Err(error) => {
			eprintln!("pelite-cli: {error}");
			ExitCode::FAILURE
		},
	}
}
