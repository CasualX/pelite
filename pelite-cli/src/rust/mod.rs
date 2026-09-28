use super::*;

mod format_args;
mod msvc;
mod panic_strings;
mod vtables;

pub fn command() -> clap::Command {
	clap::Command::new("rust")
		.about("Analyze Rust-specific patterns in PE binaries")
		.arg_required_else_help(true)
		.subcommand(format_args::command())
		.subcommand(panic_strings::command())
		.subcommand(vtables::command())
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	match matches.subcommand() {
		Some(("format-args", matches)) => format_args::run(matches, format),
		Some(("panic-strings", matches)) => panic_strings::run(matches, format),
		Some(("vtables", matches)) => vtables::run(matches, format),
		_ => unreachable!("all Rust subcommands are handled"),
	}
}
