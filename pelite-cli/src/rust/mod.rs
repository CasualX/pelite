use super::*;

mod format_args;
mod fmt_template;
mod panic_locations;
mod vtables;

pub fn command() -> clap::Command {
	clap::Command::new("rust")
		.about("Analyze Rust-specific patterns in PE binaries")
		.after_help(include_str!("../../docs/rust.md"))
		.arg_required_else_help(true)
		.subcommand(format_args::command())
		.subcommand(fmt_template::command())
		.subcommand(panic_locations::command())
		.subcommand(vtables::command())
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	match matches.subcommand() {
		Some(("format-args", matches)) => format_args::run(matches, format),
		Some(("fmt-template", matches)) => fmt_template::run(matches, format),
		Some(("panic-locations", matches)) => panic_locations::run(matches, format),
		Some(("vtables", matches)) => vtables::run(matches, format),
		_ => unreachable!("all Rust subcommands are handled"),
	}
}
