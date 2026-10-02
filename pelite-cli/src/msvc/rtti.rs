use pelite::{FileMap, PeFile, Wrap};

use super::*;

mod rtti32;
mod rtti64;

pub fn command() -> clap::Command {
	clap::Command::new("rtti")
		.about("Dump Microsoft C++ RTTI, vtables, and class hierarchies")
		.after_help(include_str!("../docs/msvc-rtti.md"))
		.arg(clap::Arg::new("demangle")
			.long("demangle")
			.help("Demangle C++ type names")
			.action(clap::ArgAction::SetTrue))
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let map = FileMap::open(path)?;
	let mut output = analyze(PeFile::from_bytes(&map)?)?;
	let demangle = matches.get_flag("demangle");
	if matches!(format, OutputFormat::Text) {
		return print_text(&output, demangle);
	}
	if demangle {
		for item in &mut output {
			item.name = demangle_name(&item.name);
			for vtable in &mut item.vtables {
				if let Some(name) = &mut vtable.for_type {
					*name = demangle_name(name);
				}
			}
			for base in &mut item.hierarchy {
				base.name = demangle_name(&base.name);
			}
		}
	}
	match format {
		OutputFormat::Json => print_json(&output, false),
		OutputFormat::JsonPretty => print_json(&output, true),
		OutputFormat::Text => unreachable!(),
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

fn demangle_name(name: &str) -> String {
	let flags = msvc_demangler::DemangleFlags::llvm() | msvc_demangler::DemangleFlags::NO_CLASS_TYPE;
	if let Some(ty) = name.strip_prefix('.') {
		// The demangler accepts RTTI descriptor symbols, but not the type
		// names stored in descriptors. Wrap the encoded type in a symbol.
		msvc_demangler::demangle(&format!("??_R0{ty}@8"), flags)
			.ok()
			.and_then(|name| name.strip_suffix("::`RTTI Type Descriptor'").map(str::to_owned))
			.unwrap_or_else(|| name.to_owned())
	}
	else {
		msvc_demangler::demangle(name, flags).unwrap_or_else(|_| name.to_owned())
	}
}

#[test]
fn test_demangle_name() {
	assert_eq!(demangle_name(".?AVRoot@fixture@@"), "fixture::Root");
	assert_eq!(demangle_name(".?AURecord@fixture@@"), "fixture::Record");
	assert_eq!(demangle_name(".?ATValue@fixture@@"), "fixture::Value");
	assert_eq!(demangle_name(".?AW4Color@fixture@@"), "fixture::Color");
	assert_eq!(demangle_name(".?AV?$Template@H@fixture@@"), "fixture::Template<int>");
	for name in ["?", "", ".?AV", "invalid", ".invalid"] {
		assert_eq!(demangle_name(name), name);
	}
}

fn print_text(types: &[TypeOutput], demangle: bool) -> Result {
	fn display_name(name: &str, demangle: bool) -> impl fmt::Display + '_ {
		fmt::from_fn(move |f| {
			if demangle {
				f.write_str(&demangle_name(name))
			}
			else {
				f.write_str(name)
			}
		})
	}

	let mut output = io::stdout().lock();
	for item in types {
		let kind = match item.inheritance {
			"single" => " (SI)",
			"multiple" => " (MI)",
			"virtual" => " (VI)",
			_ => " (MI VI)",
		};
		writeln!(output, "class {}{}", display_name(&item.name, demangle), kind)?;
		let symbol_name = item.name.get(4..).unwrap_or(&item.name);
		for vtable in &item.vtables {
			let symbol = format!("??_7{}6B@", symbol_name);
			writeln!(output,
				"{:#010X}: {} {{for '{}'}} ({} methods)",
				vtable.rva,
				display_name(&symbol, demangle),
				display_name(vtable.for_type.as_deref().unwrap_or("?"), demangle),
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
			writeln!(output, "{}", display_name(&base.name, demangle))?;
		}
		writeln!(output)?;
	}
	Ok(())
}
