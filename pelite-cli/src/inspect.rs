use pelite::{image, PeFile, Wrap};

use super::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Topic {
	Dos = 0,
	RichStructure = 1,
	Headers = 2,
	Sections = 3,
	Imports = 4,
	Exports = 5,
	Relocations = 6,
	LoadConfig = 7,
	Tls = 8,
	Exceptions = 9,
	Debug = 10,
}

impl Topic {
	const ALL: [Topic; 11] = [
		Topic::Dos,
		Topic::RichStructure,
		Topic::Headers,
		Topic::Sections,
		Topic::Imports,
		Topic::Exports,
		Topic::Relocations,
		Topic::LoadConfig,
		Topic::Tls,
		Topic::Exceptions,
		Topic::Debug,
	];

	fn flag(self) -> &'static str {
		match self {
			Topic::Dos => "dos",
			Topic::RichStructure => "rich-structure",
			Topic::Headers => "headers",
			Topic::Sections => "sections",
			Topic::Imports => "imports",
			Topic::Exports => "exports",
			Topic::Relocations => "relocations",
			Topic::LoadConfig => "load-config",
			Topic::Tls => "tls",
			Topic::Exceptions => "exceptions",
			Topic::Debug => "debug",
		}
	}
}

#[derive(Clone, Copy, Default)]
struct TopicSet(u16);

impl TopicSet {
	fn insert(&mut self, topic: Topic) {
		self.0 |= 1u16 << (topic as u8);
	}

	fn is_empty(self) -> bool {
		self.0 == 0
	}

	fn all() -> Self {
		let mut set = Self::default();
		for &topic in &Topic::ALL {
			set.insert(topic);
		}
		set
	}

	fn iter(self) -> TopicIter {
		TopicIter(self.0)
	}
}

struct TopicIter(u16);

impl Iterator for TopicIter {
	type Item = Topic;

	fn next(&mut self) -> Option<Self::Item> {
		if self.0 == 0 {
			return None;
		}

		let index = self.0.trailing_zeros() as usize;
		self.0 &= self.0 - 1;

		Topic::ALL.get(index).copied()
	}
}

pub fn command() -> clap::Command {
	let mut command = clap::Command::new("inspect")
		.about("Inspect selected PE headers and directories")
		.after_help(
			"With no flags, every supported structure is inspected. \
			 --all also selects everything.\n\n\
			 Examples:\n  \
			 pelite-cli inspect program.exe --sections\n  \
			 pelite-cli inspect program.exe --imports --exports\n  \
			 pelite-cli inspect program.exe --all --format=json-pretty",
		)
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("all")
			.long("all")
			.action(clap::ArgAction::SetTrue)
			.help("Include every supported structure"));

	for topic in &Topic::ALL {
		let name = topic.flag();
		command = command.arg(clap::Arg::new(name)
			.long(name)
			.action(clap::ArgAction::SetTrue)
			.help("Include this structure in the output"));
	}

	command
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");

	let mut selected = TopicSet::default();

	for &topic in &Topic::ALL {
		if matches.get_flag(topic.flag()) {
			selected.insert(topic);
		}
	}

	if selected.is_empty() || matches.get_flag("all") {
		selected = TopicSet::all();
	}

	let map = pelite::FileMap::open(path)?;
	let pe = PeFile::from_bytes(&map)?;

	let mut output = BTreeMap::new();

	let format_str = match pe {
		Wrap::T32(_) => "PE32",
		Wrap::T64(_) => "PE32+",
	};
	output.insert("format", value(format_str)?);

	for topic in selected.iter() {
		match topic {
			Topic::Dos => {
				output.insert("dos", value(pe.dos_header())?);
			}
			Topic::RichStructure => {
				output.insert("rich_structure", value_opt(pe.rich_structure())?);
			}
			Topic::Headers => {
				output.insert("headers", value(pe.headers())?);
			}
			Topic::Sections => {
				output.insert("sections", value(pe.section_headers())?);
			}
			Topic::Imports => {
				output.insert("imports", value_opt(pe.imports())?);
			}
			Topic::Exports => {
				output.insert("exports", value_opt(pe.exports())?);
			}
			Topic::Relocations => {
				output.insert("relocations", value_opt(pe.base_relocs())?);
			}
			Topic::LoadConfig => {
				output.insert("load_config", value_opt(pe.load_config())?);
			}
			Topic::Tls => {
				output.insert("tls", value_opt(pe.tls())?);
			}
			Topic::Exceptions => {
				output.insert("exceptions", exceptions(pe)?);
			}
			Topic::Debug => {
				output.insert("debug", value_opt(pe.debug())?);
			}
		}
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
		Wrap::T64(file) => match file.file_header().Machine {
			image::IMAGE_FILE_MACHINE_AMD64 => value_opt(file.exception_x64()),
			image::IMAGE_FILE_MACHINE_ARM64 => value_opt(file.exception_arm64()),
			_ => Err(pelite::Error::Invalid.into()),
		},
	}
}
