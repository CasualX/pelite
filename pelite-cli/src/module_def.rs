use std::path::PathBuf;

use clap::{Arg, ArgMatches, Command};
use serde::Serialize;

use crate::{OutputFormat, Result, print_json};

#[derive(Serialize)]
struct ModuleDefinition {
	library: String,
	exports: Vec<String>,
}

pub fn command() -> Command {
	Command::new("module-def")
		.about("Generate a module-definition file from exports")
		.arg(Arg::new("dll")
			.value_name("DLL")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
}

pub fn run(matches: &ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("dll").expect("required by clap");
	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let exports = pe.exports()?.by()?;
	let library = exports.dll_name()?.to_str()?.to_owned();
	let mut names = Vec::new();
	for (name, _) in exports.iter_names() {
		names.push(name?.to_str()?.to_owned());
	}
	let definition = ModuleDefinition { library, exports: names };
	match format {
		OutputFormat::Json => print_json(&definition, false),
		OutputFormat::JsonPretty => print_json(&definition, true),
		OutputFormat::Text => {
			println!("LIBRARY {}\nEXPORTS", definition.library);
			for name in &definition.exports {
				println!("{name}");
			}
			Ok(())
		},
	}
}
