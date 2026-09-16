use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use clap::{Arg, ArgAction, ArgMatches, Command};
use pelite::resources::ResourceName;
use serde::Serialize;

use crate::{OutputFormat, Result, print_json};

#[derive(Serialize)]
struct Extracted {
	kind: &'static str,
	name: String,
	path: String,
	images: usize,
	bytes: usize,
}

pub fn command() -> Command {
	Command::new("extract-icons")
		.visible_alias("icons")
		.about("Extract icon and cursor groups from PE resources")
		.arg(Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(Arg::new("destination")
			.value_name("DESTINATION")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(Arg::new("kind")
			.long("kind")
			.value_name("KIND")
			.value_parser(["icons", "cursors", "all"])
			.default_value("icons")
			.help("Select which resource groups to extract"))
		.arg(Arg::new("overwrite")
			.long("overwrite")
			.short('f')
			.action(ArgAction::SetTrue)
			.help("Overwrite existing output files"))
}

pub fn run(matches: &ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let destination = matches.get_one::<PathBuf>("destination").expect("required by clap");
	let kind = matches.get_one::<String>("kind").expect("defaulted by clap").as_str();
	let overwrite = matches.get_flag("overwrite");

	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let resources = pe.resources()?;
	fs::create_dir_all(destination)?;

	let mut extracted = Vec::new();
	if kind == "icons" || kind == "all" {
		for entry in resources.icons() {
			let (name, group) = entry?;
			extracted.push(write_group(destination, name, group, "icon", "ico", overwrite)?);
		}
	}
	if kind == "cursors" || kind == "all" {
		for entry in resources.cursors() {
			let (name, group) = entry?;
			extracted.push(write_group(destination, name, group, "cursor", "cur", overwrite)?);
		}
	}

	match format {
		OutputFormat::Text => {
			for entry in &extracted {
				println!("{}", entry.path);
			}
			Ok(())
		},
		OutputFormat::Json => print_json(&extracted, false),
		OutputFormat::JsonPretty => print_json(&extracted, true),
	}
}

fn write_group(
	destination: &Path,
	name: ResourceName<'_>,
	group: pelite::resources::group::ResourceGroup<'_>,
	kind: &'static str,
	extension: &str,
	overwrite: bool,
) -> Result<Extracted> {
	let display_name = name.to_string();
	let file_name = format!("{}.{}", safe_name(name), extension);
	let path = destination.join(file_name);
	let mut contents = Vec::new();
	group.write(&mut contents)?;

	if overwrite {
		fs::write(&path, &contents)?;
	}
	else {
		let mut file = OpenOptions::new().write(true).create_new(true).open(&path)?;
		file.write_all(&contents)?;
	}

	Ok(Extracted {
		kind,
		name: display_name,
		path: path.to_string_lossy().into_owned(),
		images: group.entries().len(),
		bytes: contents.len(),
	})
}

fn safe_name(name: ResourceName<'_>) -> String {
	let name = match name {
		ResourceName::Id(id) => id.to_string(),
		ResourceName::Wide(words) => String::from_utf16_lossy(words),
		ResourceName::Str(name) => name.to_owned(),
	};
	let name: String = name
		.chars()
		.map(|character| {
			if character.is_alphanumeric() || matches!(character, '-' | '_' | '.') {
				character
			}
			else {
				'_'
			}
		})
		.collect();
	if name.is_empty() || name == "." || name == ".." {
		"resource".to_owned()
	}
	else {
		name
	}
}
