use super::*;

mod rtti;
mod throws;

pub fn command() -> clap::Command {
	clap::Command::new("msvc")
		.about("Analyze Microsoft C++ compiler patterns in PE binaries")
		.after_help(include_str!("../docs/msvc.md"))
		.arg_required_else_help(true)
		.subcommand(rtti::command())
		.subcommand(throws::command())
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	match matches.subcommand() {
		Some(("rtti", matches)) => rtti::run(matches, format),
		Some(("throws", matches)) => throws::run(matches, format),
		_ => unreachable!("all MSVC subcommands are handled"),
	}
}

fn demangle_name(name: &str) -> String {
	let flags = msvc_demangler::DemangleFlags::llvm() | msvc_demangler::DemangleFlags::NO_CLASS_TYPE;
	if let Some(ty) = name.strip_prefix('.') {
		// The demangler accepts RTTI descriptor symbols, but not the type
		// names stored in descriptors. Wrap the encoded type in a symbol.
		let qualifier = if ty.starts_with('?') { "" } else { "?A" };
		msvc_demangler::demangle(&format!("??_R0{qualifier}{ty}@8"), flags)
			.ok()
			.and_then(|name| name.strip_suffix("::`RTTI Type Descriptor'").map(str::to_owned))
			.unwrap_or_else(|| name.to_owned())
	}
	else {
		msvc_demangler::demangle(name, flags).unwrap_or_else(|_| name.to_owned())
	}
}
