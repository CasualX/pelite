use std::path::PathBuf;

use clap::{Arg, ArgAction, ArgMatches, Command};
use pelite::resources::ResourceName;
use pelite::resources::version_info::{VersionInfo, VersionInfoData};
use serde::Serialize;

use crate::{OutputFormat, Result, print_json};

#[derive(Serialize)]
struct VersionOutput<'a> {
	resource_id: u32,
	#[serde(skip_serializing_if = "Option::is_none")]
	resource_language: Option<u16>,
	info: VersionInfoData<'a>,
	#[serde(skip_serializing_if = "Option::is_none")]
	source: Option<String>,
}

pub fn command() -> Command {
	Command::new("version-info")
		.visible_alias("version_info")
		.about("Inspect the version-information resource")
		.arg(Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(Arg::new("id")
			.long("id")
			.value_name("ID")
			.value_parser(clap::value_parser!(u32))
			.default_value("1")
			.help("Select the version resource identifier"))
		.arg(Arg::new("resource-language")
			.long("resource-language")
			.short('l')
			.value_name("LANG")
			.value_parser(parse_u16)
			.help("Select a resource language ID, in decimal or 0x-prefixed hexadecimal"))
		.arg(Arg::new("source")
			.long("source")
			.action(ArgAction::SetTrue)
			.help("Include reconstructed resource-script source"))
}

pub fn run(matches: &ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let resource_id = *matches.get_one::<u32>("id").expect("defaulted by clap");
	let language = matches.get_one::<u16>("resource-language").copied();
	let include_source = matches.get_flag("source");

	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let resources = pe.resources()?;
	let bytes = match language {
		Some(language) => resources.find_resource_ex(&[
			ResourceName::VERSION,
			ResourceName::Id(resource_id),
			ResourceName::Id(language as u32),
		])?,
		None => resources.find_resource(&[ResourceName::VERSION, ResourceName::Id(resource_id)])?,
	};
	let version = VersionInfo::try_from(bytes)?;
	let output = VersionOutput {
		resource_id,
		resource_language: language,
		info: version.file_info(),
		source: include_source.then(|| version.source_code()),
	};

	match format {
		OutputFormat::Text => print_text(&output),
		OutputFormat::Json => print_json(&output, false),
		OutputFormat::JsonPretty => print_json(&output, true),
	}
}

fn print_text(output: &VersionOutput<'_>) -> Result {
	if let Some(fixed) = output.info.fixed {
		println!(
			"{:<20} {}.{}.{}.{}",
			"FileVersion",
			fixed.dwFileVersion.Major,
			fixed.dwFileVersion.Minor,
			fixed.dwFileVersion.Patch,
			fixed.dwFileVersion.Build
		);
		println!(
			"{:<20} {}.{}.{}.{}",
			"ProductVersion",
			fixed.dwProductVersion.Major,
			fixed.dwProductVersion.Minor,
			fixed.dwProductVersion.Patch,
			fixed.dwProductVersion.Build
		);
		println!("{:<20} {:#x}", "FileFlagsMask", fixed.dwFileFlagsMask);
		println!("{:<20} {:#x}", "FileFlags", fixed.dwFileFlags);
		println!("{:<20} {}, {}", "FileOS", fixed.dwFileOS >> 16, fixed.dwFileOS & 0xffff);
		println!("{:<20} {}", "FileType", fixed.dwFileType);
		println!("{:<20} {}", "FileSubtype", fixed.dwFileSubtype);
	}

	let mut tables: Vec<_> = output.info.strings.iter().collect();
	tables.sort_by_key(|(language, _)| **language);
	for (language, strings) in tables {
		println!("\n[{language}]");
		let mut strings: Vec<_> = strings.iter().collect();
		strings.sort_by_key(|(key, _)| key.as_str());
		for (key, value) in strings {
			println!("{key:<20} {value:?}");
		}
	}
	if let Some(source) = &output.source {
		println!("\n{source}");
	}
	Ok(())
}

fn parse_u16(value: &str) -> std::result::Result<u16, String> {
	if let Some(value) = value.strip_prefix("0x").or_else(|| value.strip_prefix("0X")) {
		u16::from_str_radix(value, 16).map_err(|error| error.to_string())
	}
	else {
		value.parse().map_err(|error: std::num::ParseIntError| error.to_string())
	}
}

#[cfg(test)]
mod tests {
	use super::parse_u16;

	#[test]
	fn parses_resource_languages() {
		assert_eq!(parse_u16("1033"), Ok(1033));
		assert_eq!(parse_u16("0x0409"), Ok(0x0409));
		assert!(parse_u16("0x10000").is_err());
	}
}
