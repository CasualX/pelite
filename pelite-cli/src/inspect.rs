use pelite::{PeFile, Wrap, image};

use super::*;

const KEYWORDS: [&str; 12] = [
	"all",
	"dos",
	"rich-structure",
	"headers",
	"sections",
	"imports",
	"exports",
	"relocations",
	"load-config",
	"tls",
	"exceptions",
	"debug",
];

pub fn command() -> clap::Command {
	clap::Command::new("inspect")
		.about("Inspect selected PE headers and directories")
		.after_help("With no keywords, every supported structure is inspected. The 'all' keyword also selects everything.\n\nExamples:\n  pelite-cli inspect program.exe sections\n  pelite-cli inspect program.exe imports exports\n  pelite-cli inspect program.exe all --format=json-pretty")
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("topics")
			.value_name("KEYWORD")
			.value_parser(KEYWORDS)
			.num_args(0..)
			.help("Structures to include in the output"))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let requested = matches.get_many::<String>("topics")
		.into_iter()
		.flatten()
		.map(String::as_str)
		.collect::<BTreeSet<_>>();
	let inspect_all = requested.is_empty() || requested.contains("all");
	let selected = |topic| inspect_all || requested.contains(topic);

	let map = pelite::FileMap::open(path)?;
	let pe = PeFile::from_bytes(&map)?;
	let mut output = BTreeMap::new();
	output.insert("format", value(match pe {
		Wrap::T32(_) => "PE32",
		Wrap::T64(_) => "PE32+",
	})?);
	if selected("dos") {
		output.insert("dos", value(pe.dos_header())?);
	}
	if selected("rich-structure") {
		output.insert("rich_structure", value_opt(pe.rich_structure())?);
	}
	if selected("headers") {
		output.insert("headers", value(pe.headers())?);
	}
	if selected("sections") {
		output.insert("sections", value(pe.section_headers())?);
	}
	if selected("imports") {
		output.insert("imports", value_opt(pe.imports())?);
	}
	if selected("exports") {
		output.insert("exports", value_opt(pe.exports())?);
	}
	if selected("relocations") {
		output.insert("relocations", value_opt(pe.base_relocs())?);
	}
	if selected("load-config") {
		output.insert("load_config", value_opt(pe.load_config())?);
	}
	if selected("tls") {
		output.insert("tls", value_opt(pe.tls())?);
	}
	if selected("exceptions") {
		output.insert("exceptions", exceptions(pe)?);
	}
	if selected("debug") {
		output.insert("debug", value_opt(pe.debug())?);
	}

	printer::print("PE image", &output, format)
}

fn value<T: serde::Serialize>(value: T) -> Result<serde_json::Value> {
	Ok(serde_json::to_value(value)?)
}

fn value_opt<T: serde::Serialize>(result: pelite::Result<T>) -> Result<serde_json::Value> {
	match result {
		Ok(directory) => value(directory),
		Err(error) if error.is_null() => Ok(serde_json::Value::Null),
		Err(error) => Err(error.into()),
	}
}
fn exceptions(pe: PeFile<'_>) -> Result<serde_json::Value> {
	match pe {
		Wrap::T32(_) => Ok(serde_json::Value::Null),
		Wrap::T64(file) => {
			match file.file_header().Machine {
				image::IMAGE_FILE_MACHINE_AMD64 => value_opt(file.exception_x64()),
				image::IMAGE_FILE_MACHINE_ARM64 => value_opt(file.exception_arm64()),
				_ => Err(pelite::Error::Invalid.into()),
			}
		},
	}
}
