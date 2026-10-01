use pelite::{FileMap, PeFile, Wrap};

use super::*;

mod rtti32;
mod rtti64;

pub fn command() -> clap::Command {
	clap::Command::new("rtti")
		.about("Dump Microsoft C++ RTTI, vtables, and class hierarchies")
		.after_help(include_str!("../docs/msvc-rtti.md"))
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let map = FileMap::open(path)?;
	let output = analyze(PeFile::from_bytes(&map)?)?;
	match format {
		OutputFormat::Json => print_json(&output, false),
		OutputFormat::JsonPretty => print_json(&output, true),
		OutputFormat::Text => print_text(&output),
	}
}

fn analyze(file: PeFile<'_>) -> Result<Vec<TypeOutput>> {
	match file {
		Wrap::T32(file) => rtti32::analyze(file),
		Wrap::T64(file) => rtti64::analyze(file),
	}
}

fn inheritance_kind(attributes: u32, has_virtual_base: bool) -> &'static str {
	// MSVC can leave CHD_VIRTINH unset for single virtual inheritance.
	match (attributes & 1 != 0, attributes & 2 != 0 || has_virtual_base) {
		(false, false) => "single",
		(true, false) => "multiple",
		(false, true) => "virtual",
		(true, true) => "multiple virtual",
	}
}

#[derive(serde::Serialize)]
struct TypeOutput {
	name: String,
	inheritance: &'static str,
	vtables: Vec<VTableOutput>,
	hierarchy: Vec<BaseClassOutput>,
}

#[derive(serde::Serialize)]
struct VTableOutput {
	rva: u32,
	for_type: Option<String>,
	methods: usize,
}

#[derive(serde::Serialize)]
struct BaseClassOutput {
	depth: usize,
	offset: Option<i32>,
	virtual_base: bool,
	name: String,
}

fn print_text(types: &[TypeOutput]) -> Result {
	let mut output = io::stdout().lock();
	for item in types {
		let kind = match item.inheritance {
			"single" => " (SI)",
			"multiple" => " (MI)",
			"virtual" => " (VI)",
			_ => " (MI VI)",
		};
		writeln!(output, "class {}{}", item.name, kind)?;
		let symbol_name = item.name.get(4..).unwrap_or(&item.name);
		for vtable in &item.vtables {
			writeln!(output,
				"{:#010X}: ??_7{}6B@ {{for '{}'}} ({} methods)",
				vtable.rva,
				symbol_name,
				vtable.for_type.as_deref().unwrap_or("?"),
				vtable.methods
			)?;
		}
		for (index, base) in item.hierarchy.iter().enumerate() {
			match base.offset {
				Some(offset) => write!(output, "{offset:04X}: ")?,
				None => write!(output, "****: ")?,
			}
			if base.depth > 0 {
				for _ in 1..base.depth {
					write!(output, "|   ")?;
				}
				let is_last = item.hierarchy.get(index + 1).is_none_or(|next| next.depth < base.depth);
				write!(output, "{}", if is_last { "`-- " } else { "+-- " })?;
			}
			writeln!(output, "{}", base.name)?;
		}
		writeln!(output)?;
	}
	Ok(())
}
