use pelite::{FileMap, PeFile, Wrap};

use super::*;

mod throws32;
mod throws64;

pub fn command() -> clap::Command {
	clap::Command::new("throws")
		.about("Extract Microsoft C++ thrown types and exception conversion metadata")
		.after_help(include_str!("docs/throws.md"))
		.arg(clap::Arg::new("demangle").long("demangle")
			.help("Demangle C++ exception type names").action(clap::ArgAction::SetTrue))
		.arg(clap::Arg::new("file").value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf)).required(true))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let map = FileMap::open(matches.get_one::<PathBuf>("file").expect("required by clap"))?;
	let mut output = analyze(PeFile::from_bytes(&map)?)?;
	if matches.get_flag("demangle") {
		for item in &mut output {
			for ty in &mut item.catchable_types {
				ty.name = demangle_name(&ty.name);
			}
		}
	}
	match format {
		OutputFormat::Nul => return Ok(()),
		OutputFormat::Json => return print_json(&output, false),
		OutputFormat::JsonPretty => return print_json(&output, true),
		OutputFormat::Text => {},
	}
	let mut out = io::stdout().lock();
	for item in output {
		writeln!(out, "{:#010X}: throw {} (attributes {:#X})", item.rva, item.catchable_types[0].name, item.attributes)?;
		writeln!(out, "  destructor: {}, forward compatibility: {}", display_rva(item.destructor_rva), display_rva(item.forward_compat_rva))?;
		for reference in item.references {
			writeln!(out, "  metadata reference: {reference:#010X}")?;
		}
		for ty in item.catchable_types {
			writeln!(out, "  catch {}: size/offset {}, PMD ({}, {}, {}), properties {:#X}, copy {}",
				ty.name, ty.size_or_offset, ty.displacement.mdisp, ty.displacement.pdisp,
				ty.displacement.vdisp, ty.properties, display_rva(ty.copy_function_rva))?;
		}
		writeln!(out)?;
	}
	Ok(())
}

fn display_rva(rva: Option<u32>) -> impl fmt::Display {
	fmt::from_fn(move |f| match rva {
		Some(rva) => write!(f, "{rva:#010X}"),
		None => f.write_str("none"),
	})
}

#[derive(Debug, serde::Serialize)]
struct ThrowOutput {
	rva: u32,
	attributes: u32,
	destructor_rva: Option<u32>,
	forward_compat_rva: Option<u32>,
	catchable_type_array_rva: u32,
	/// Instructions loading this metadata, not verified throw calls.
	references: Vec<u32>,
	catchable_types: Vec<CatchableOutput>,
}

#[derive(Debug, serde::Serialize)]
struct CatchableOutput {
	rva: u32,
	type_descriptor_rva: u32,
	name: String,
	properties: u32,
	simple_type: bool,
	by_reference_only: bool,
	has_virtual_bases: bool,
	displacement: Displacement,
	size_or_offset: i32,
	copy_function_rva: Option<u32>,
}

#[derive(Debug, serde::Serialize)]
struct Displacement {
	mdisp: i32,
	pdisp: i32,
	vdisp: i32,
}

fn analyze(file: PeFile<'_>) -> Result<Vec<ThrowOutput>> {
	match file {
		Wrap::T32(file) => throws32::analyze(file),
		Wrap::T64(file) => throws64::analyze(file),
	}
}
