use pelite::FileMap;
use pelite::pe64::{image, PeFile, Rva};

use super::fmt_template::{FormatTemplate, Placeholder, parse_format_template};
use super::panic_locations::is_location64;
use crate::*;

#[derive(serde::Serialize)]
struct FormatArgsOutput {
	template_rva: u32,
	encoded_len: usize,
	format: String,
	#[serde(skip)]
	display: String,
	argument_count: u16,
	placeholders: Vec<Placeholder>,
	code_rvas: Vec<u32>,
}

pub fn command() -> clap::Command {
	clap::Command::new("format-args")
		.visible_alias("format_args")
		.about("Find current-toolchain Rust format_args! templates in an x64 PE")
		.after_help(include_str!("../docs/rust-format-args.md"))
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let map = FileMap::open(path)?;
	let file = open_x64(map.as_ref())?;
	let mut grouped = BTreeMap::<u32, Vec<u32>>::new();
	for xref in candidate_relative_xrefs(file) {
		// The same LEA search also finds references to panic locations.
		if !is_location64(file, xref.target_rva) && read_format_template(file, xref.target_rva).is_some() {
			grouped.entry(xref.target_rva).or_default().push(xref.code_rva);
		}
	}
	let mut output = Vec::new();
	for (template_rva, mut code_rvas) in grouped {
		let template = read_format_template(file, template_rva).expect("validated above");
		code_rvas.sort_unstable();
		code_rvas.dedup();
		output.push(FormatArgsOutput {
			template_rva,
			encoded_len: template.encoded_len,
			format: template.rendered,
			display: template.detailed,
			argument_count: template.argument_count,
			placeholders: template.placeholders,
			code_rvas,
		});
	}
	match format {
		OutputFormat::Nul => Ok(()),
		OutputFormat::Json => print_json(&output, false),
		OutputFormat::JsonPretty => print_json(&output, true),
		OutputFormat::Text => {
			let file_name = path.file_name().and_then(|name| name.to_str()).unwrap_or("<input>");
			let mut writer = io::stdout().lock();
			for item in output {
				writeln!(writer, "{file_name}!{:#010x} {:?} (arguments={})", item.template_rva, item.display, item.argument_count)?;
				for code_rva in item.code_rvas {
					writeln!(writer, "  used at {file_name}!{code_rva:#010x}")?;
				}
			}
			Ok(())
		},
	}
}

//----------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Xref {
	pub code_rva: Rva,
	pub target_rva: Rva,
}

pub fn open_x64<'a>(bytes: &'a [u8]) -> Result<PeFile<'a>> {
	let file = PeFile::from_bytes(bytes).map_err(|error| err(format!("input is not a PE32+ image: {error}")))?;
	if file.file_header().Machine != image::IMAGE_FILE_MACHINE_AMD64 {
		return Err(err("input is not an x86-64 PE image"));
	}
	Ok(file)
}

/// Finds candidate relative references made by `lea` instructions.
///
/// The pattern intentionally leaves the REX and ModR/M details to semantic
/// filtering of the referenced Rust data structure.
pub fn candidate_relative_xrefs(file: PeFile<'_>) -> Vec<Xref> {
	let pattern = pelite::pattern!("8D ? $'");
	let mut output = Vec::new();
	let mut save = [0; 3];
	let mut matches = file.scanner().code().matches(pattern);
	while matches.next(&mut save).is_some() {
		if !file.section_headers().by_rva(save[1]).is_some_and(|section| section.Characteristics & image::IMAGE_SCN_MEM_EXECUTE != 0) && file.slice_bytes(save[1]).is_ok() {
			output.push(Xref {
				code_rva: save[0],
				target_rva: save[1],
			});
		}
	}
	output.sort_unstable_by_key(|xref| (xref.target_rva, xref.code_rva));
	output.dedup_by_key(|xref| (xref.target_rva, xref.code_rva));
	output
}

pub fn read_format_template(file: PeFile<'_>, rva: Rva) -> Option<FormatTemplate> {
	parse_format_template(file.slice_bytes(rva).ok()?)
}
