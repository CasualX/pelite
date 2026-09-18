use pelite::FileMap;

use crate::rust_msvc::*;
use super::*;

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
	clap::Command::new("rust-format-args")
		.about("Find current-toolchain Rust format_args! templates in an x64 MSVC PE")
		.after_help(
			"Decodes the compact fmt::Arguments template emitted by the current Rust toolchain.\n\
			 Literal-only format_args! values use the optimized &str representation and are not distinguishable reliably.",
		)
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
		if !is_rust_location(file, xref.target_rva) && read_format_template(file, xref.target_rva).is_some() {
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
		OutputFormat::Json => print_json(&output, false),
		OutputFormat::JsonPretty => print_json(&output, true),
		OutputFormat::Text => {
			let file_name = path.file_name().and_then(|name| name.to_str()).unwrap_or("<input>");
			for item in output {
				println!("{file_name}!{:#010x} {:?} (arguments={})", item.template_rva, item.display, item.argument_count);
				for code_rva in item.code_rvas {
					println!("  used at {file_name}!{code_rva:#010x}");
				}
			}
			Ok(())
		},
	}
}
