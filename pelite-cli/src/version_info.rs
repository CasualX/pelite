use pelite::resources::ResourceName;
use pelite::resources::version_info::{VersionInfo, VersionInfoData};

use super::*;

#[derive(serde::Serialize)]
struct VersionOutput<'a> {
	resource_id: u32,
	#[serde(skip_serializing_if = "Option::is_none")]
	resource_language: Option<u16>,
	info: VersionInfoData<'a>,
	#[serde(skip_serializing_if = "Option::is_none")]
	source: Option<String>,
}

pub fn command() -> clap::Command {
	clap::Command::new("version-info")
		.visible_alias("version_info")
		.about("Inspect the version-information resource")
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("id")
			.long("id")
			.value_name("ID")
			.value_parser(clap::value_parser!(u32))
			.default_value("1")
			.help("Select the version resource identifier"))
		.arg(clap::Arg::new("resource-language")
			.long("resource-language")
			.short('l')
			.value_name("LANG")
			.value_parser(parse_u16)
			.help("Select a resource language ID, in decimal or 0x-prefixed hexadecimal"))
		.arg(clap::Arg::new("source")
			.long("source")
			.action(clap::ArgAction::SetTrue)
			.help("Include reconstructed resource-script source"))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
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

	printer::print("Version information", &output, format)
}

fn parse_u16(value: &str) -> result::Result<u16, String> {
	if let Some(value) = value.strip_prefix("0x").or_else(|| value.strip_prefix("0X")) {
		u16::from_str_radix(value, 16).map_err(|error| error.to_string())
	}
	else {
		value.parse().map_err(|error: num::ParseIntError| error.to_string())
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
