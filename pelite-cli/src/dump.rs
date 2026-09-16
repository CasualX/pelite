use std::collections::BTreeMap;
use std::fmt::Debug;
use std::path::PathBuf;

use clap::{Arg, ArgAction, ArgMatches, Command};
use pelite::{PeFile, Wrap};
use serde::Serialize;
use serde_json::Value;

use crate::{OutputFormat, Result, print_json};

#[derive(Clone, Default)]
struct Selection {
	dos: bool,
	headers: bool,
	sections: bool,
	imports: bool,
	exports: bool,
	relocs: bool,
	load_config: bool,
	tls: bool,
	exceptions: bool,
	resources: bool,
	debug: bool,
}

impl Selection {
	fn any(&self) -> bool {
		self.dos || self.headers || self.sections || self.imports || self.exports || self.relocs || self.load_config || self.tls || self.exceptions || self.resources || self.debug
	}

	fn all() -> Self {
		Self {
			dos: true,
			headers: true,
			sections: true,
			imports: true,
			exports: true,
			relocs: true,
			load_config: true,
			tls: true,
			exceptions: true,
			resources: true,
			debug: true,
		}
	}

	fn from_matches(matches: &ArgMatches) -> Self {
		Self {
			dos: matches.get_flag("dos"),
			headers: matches.get_flag("headers"),
			sections: matches.get_flag("sections"),
			imports: matches.get_flag("imports"),
			exports: matches.get_flag("exports"),
			relocs: matches.get_flag("relocs"),
			load_config: matches.get_flag("load-config"),
			tls: matches.get_flag("tls"),
			exceptions: matches.get_flag("exceptions"),
			resources: matches.get_flag("resources"),
			debug: matches.get_flag("debug-info"),
		}
	}
}

fn flag(id: &'static str, short: char, help: &'static str) -> Arg {
	Arg::new(id).short(short).long(id).action(ArgAction::SetTrue).help(help)
}

pub fn command() -> Command {
	Command::new("dump")
		.about("Dump PE headers and data directories")
		.after_help("With no content options, every supported item is dumped.")
		.arg(Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(flag("dos", 'd', "Dump the DOS header"))
		.arg(flag("headers", 'n', "Dump the NT headers").visible_alias("nt"))
		.arg(flag("sections", 's', "Dump section headers"))
		.arg(flag("imports", 'i', "Dump the import directory"))
		.arg(flag("exports", 'e', "Dump the export directory"))
		.arg(flag("relocs", 'r', "Dump base relocations"))
		.arg(flag("load-config", 'l', "Dump the load configuration"))
		.arg(flag("tls", 't', "Dump the TLS directory"))
		.arg(flag("exceptions", 'c', "Dump the PE32+ x64/ARM64 exception directory"))
		.arg(flag("resources", 'x', "Dump the resource tree"))
		.arg(flag("debug-info", 'g', "Dump the debug directory"))
}

pub fn run(matches: &ArgMatches, format: OutputFormat) -> Result {
	let mut selection = Selection::from_matches(matches);
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	if !selection.any() {
		selection = Selection::all();
	}
	let map = pelite::FileMap::open(path)?;
	let pe = PeFile::from_bytes(&map)?;
	match format {
		OutputFormat::Text => dump_text(pe, &selection),
		OutputFormat::Json => dump_json(pe, &selection, false),
		OutputFormat::JsonPretty => dump_json(pe, &selection, true),
	}
}

fn section(name: &str, value: impl Debug) {
	println!("----------------------------------------------------------------");
	println!("{name}:");
	println!("{value:#?}");
}

fn directory<T: Debug>(name: &str, value: pelite::Result<T>) {
	match value {
		Ok(value) => section(name, value),
		Err(_) => {
			println!("----------------------------------------------------------------");
			println!("No {name} found.");
		},
	}
}

fn dump_text(pe: PeFile<'_>, selected: &Selection) -> Result {
	if selected.dos {
		section("DOS Header", pe.dos_header());
	}
	if selected.headers {
		section("NT Headers", pe.nt_headers());
	}
	if selected.sections {
		section("Sections", pe.section_headers());
	}
	if selected.imports {
		directory("Import Directory", pe.imports());
	}
	if selected.exports {
		directory("Export Directory", pe.exports());
	}
	if selected.relocs {
		directory("Base Relocation Directory", pe.base_relocs());
	}
	if selected.load_config {
		directory("Load Config Directory", pe.load_config());
	}
	if selected.tls {
		directory("TLS Directory", pe.tls());
	}
	if selected.exceptions {
		match pe {
			Wrap::T32(_) => {
				println!("----------------------------------------------------------------");
				println!("Exception directories are not supported for PE32 images.");
			},
			Wrap::T64(file) => {
				use pelite::pe64::Pe;
				if let Ok(value) = file.exception_x64() {
					section("x64 Exception Directory", value);
				}
				else if let Ok(value) = file.exception_arm64() {
					section("ARM64 Exception Directory", value);
				}
				else {
					directory::<()>("Exception Directory", Err(pelite::Error::Null));
				}
			},
		}
	}
	if selected.resources {
		directory("Resource Directory", pe.resources());
	}
	if selected.debug {
		directory("Debug Directory", pe.debug());
	}
	Ok(())
}

fn json_value<T: Serialize>(value: T) -> Result<Value> {
	Ok(serde_json::to_value(value)?)
}

fn json_optional<T: Serialize>(value: pelite::Result<T>) -> Result<Value> {
	match value {
		Ok(value) => json_value(value),
		Err(_) => Ok(Value::Null),
	}
}

fn dump_json(pe: PeFile<'_>, selected: &Selection, pretty: bool) -> Result {
	let mut output = BTreeMap::new();
	output.insert(
		"format",
		json_value(match pe {
			Wrap::T32(_) => "PE32",
			Wrap::T64(_) => "PE32+",
		})?,
	);
	if selected.dos {
		output.insert("dos", json_value(pe.dos_header())?);
	}
	if selected.headers {
		output.insert("headers", json_value(pe.headers())?);
	}
	if selected.sections {
		output.insert("sections", json_value(pe.section_headers())?);
	}
	if selected.imports {
		output.insert("imports", json_optional(pe.imports())?);
	}
	if selected.exports {
		output.insert("exports", json_optional(pe.exports())?);
	}
	if selected.relocs {
		output.insert("relocs", json_optional(pe.base_relocs())?);
	}
	if selected.load_config {
		output.insert("load_config", json_optional(pe.load_config())?);
	}
	if selected.tls {
		output.insert("tls", json_optional(pe.tls())?);
	}
	if selected.exceptions {
		let value = match pe {
			Wrap::T32(_) => Value::Null,
			Wrap::T64(file) => {
				use pelite::pe64::Pe;
				if let Ok(value) = file.exception_x64() {
					json_value(value)?
				}
				else if let Ok(value) = file.exception_arm64() {
					json_value(value)?
				}
				else {
					Value::Null
				}
			},
		};
		output.insert("exceptions", value);
	}
	if selected.resources {
		output.insert("resources", json_optional(pe.resources())?);
	}
	if selected.debug {
		output.insert("debug", json_optional(pe.debug())?);
	}
	print_json(&output, pretty)
}
