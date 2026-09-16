use std::path::{Path, PathBuf};

use clap::{Arg, ArgAction, ArgMatches, Command};
use serde::Serialize;

use crate::{OutputFormat, Result, print_json};

#[derive(Serialize)]
struct ImportHash {
	file: String,
	hash: String,
	imports: Vec<String>,
}

pub fn command() -> Command {
	Command::new("imphash")
		.about("Calculate the conventional MD5 import hash")
		.after_help("DLL names are lowercased and .dll, .sys, and .ocx suffixes are removed. Ordinal imports use the portable fallback name ord<NUMBER>.")
		.arg(Arg::new("files")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.num_args(1..)
			.required(true))
		.arg(Arg::new("show-imports")
			.long("show-imports")
			.action(ArgAction::SetTrue)
			.help("Show the normalized import sequence in text output"))
}

pub fn run(matches: &ArgMatches, format: OutputFormat) -> Result {
	let files = matches.get_many::<PathBuf>("files").expect("required by clap");
	let show_imports = matches.get_flag("show-imports");
	let mut hashes = Vec::new();
	for path in files {
		hashes.push(calculate(path)?);
	}

	match format {
		OutputFormat::Text => {
			for result in &hashes {
				println!("{}  {}", result.hash, result.file);
				if show_imports {
					for import in &result.imports {
						println!("  {import}");
					}
				}
			}
			Ok(())
		},
		OutputFormat::Json => print_json(&hashes, false),
		OutputFormat::JsonPretty => print_json(&hashes, true),
	}
}

fn calculate(path: &Path) -> Result<ImportHash> {
	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let mut names = Vec::new();
	let imports = match pe.imports() {
		Ok(imports) => Some(imports),
		Err(error) if error.is_null() => None,
		Err(error) => return Err(error.into()),
	};

	if let Some(imports) = imports {
		for descriptor in imports {
			let dll = descriptor.dll_name()?.to_str()?.to_ascii_lowercase();
			let dll = strip_library_extension(&dll);
			for import in descriptor.int()? {
				let symbol = match import? {
					pelite::ImportSymbol::ByName { name, .. } => name.to_str()?.to_ascii_lowercase(),
					pelite::ImportSymbol::ByOrdinal { ord } => format!("ord{ord}"),
				};
				names.push(format!("{dll}.{symbol}"));
			}
		}
	}

	let input = names.join(",");
	let hash = format!("{:x}", md5::compute(input.as_bytes()));
	Ok(ImportHash {
		file: path.to_string_lossy().into_owned(),
		hash,
		imports: names,
	})
}

fn strip_library_extension(name: &str) -> &str {
	match name.rsplit_once('.') {
		Some((stem, "dll" | "sys" | "ocx")) => stem,
		_ => name,
	}
}

#[test]
fn normalizes_library_extensions() {
	assert_eq!(strip_library_extension("kernel32.dll"), "kernel32");
	assert_eq!(strip_library_extension("driver.sys"), "driver");
	assert_eq!(strip_library_extension("library.with.dots.ocx"), "library.with.dots");
	assert_eq!(strip_library_extension("api-ms-win-core"), "api-ms-win-core");
}
