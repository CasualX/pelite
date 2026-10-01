use super::*;

mod rtti;

pub fn command() -> clap::Command {
	clap::Command::new("msvc")
		.about("Analyze Microsoft C++ compiler patterns in PE binaries")
		.after_help(include_str!("../docs/msvc.md"))
		.arg_required_else_help(true)
		.subcommand(rtti::command())
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	match matches.subcommand() {
		Some(("rtti", matches)) => rtti::run(matches, format),
		_ => unreachable!("all MSVC subcommands are handled"),
	}
}
