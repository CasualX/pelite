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
		.after_help(include_str!("docs/version-info.md"))
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("id")
			.long("id")
			.value_name("ID")
			.value_parser(value_parser::parse_u32)
			.default_value("1")
			.help("Select the version resource identifier"))
		.arg(clap::Arg::new("resource-language")
			.long("resource-language")
			.short('l')
			.value_name("LANG")
			.value_parser(value_parser::parse_u16)
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
	let resources = match pe.resources() {
		Ok(resources) => resources,
		Err(error) if error.is_null() => return print("Version information", &serde_json::Value::Null, format),
		Err(error) => return Err(error.into()),
	};
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

	print("Version information", &output, format)
}
