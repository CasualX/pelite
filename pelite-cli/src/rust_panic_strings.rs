use pelite::FileMap;
use pelite::pe64::{image, PeFile};

use crate::rust_msvc::*;
use super::*;

#[derive(Clone)]
struct RawLocation {
	rva: u32,
	file: String,
	line: u32,
	column: u32,
}

#[derive(serde::Serialize)]
struct PanicOutput {
	location_rva: u32,
	file: String,
	line: u32,
	column: u32,
	messages: Vec<String>,
	code_rvas: Vec<u32>,
}

struct PanicReferences<'a> {
	xrefs: &'a [Xref],
	calls: &'a [Xref],
	immediates: &'a [Immediate],
	compact_argument_calls: &'a BTreeSet<u32>,
}

pub fn command() -> clap::Command {
	clap::Command::new("rust-panic-strings")
		.about("Find Rust panic messages and Location records in an x64 MSVC PE")
		.after_help(
			"Recognizes the current Rust toolchain's 64-bit core::panic::Location layout and recovers\n\
			 directly referenced string or compact format templates near each code reference.",
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
	let xrefs = candidate_relative_xrefs(file);
	let calls = relative_call_xrefs(file);
	let immediates = edx_immediates(file);
	let locations = find_locations(file);
	let mut by_target = BTreeMap::<u32, Vec<u32>>::new();
	for xref in &xrefs {
		by_target.entry(xref.target_rva).or_default().push(xref.code_rva);
	}
	let mut compact_argument_calls = BTreeSet::new();
	for location in &locations {
		let Some(code_rvas) = by_target.get(&location.rva) else {
			continue;
		};
		for &code_rva in code_rvas {
			let Some(call_target) = nearby_direct_call(&calls, code_rva) else {
				continue;
			};
			let start = code_rva.saturating_sub(32);
			if xrefs
				.iter()
				.any(|xref| xref.code_rva >= start && xref.code_rva <= code_rva.saturating_add(16) && read_format_template(file, xref.target_rva).is_some())
			{
				compact_argument_calls.insert(call_target);
			}
		}
	}
	let mut output = Vec::new();
	let references = PanicReferences {
		xrefs: &xrefs,
		calls: &calls,
		immediates: &immediates,
		compact_argument_calls: &compact_argument_calls,
	};
	for location in locations {
		let Some(code_rvas) = by_target.get(&location.rva) else {
			continue;
		};
		let mut messages = Vec::new();
		for &code_rva in code_rvas {
			recover_nearby_messages(file, &references, code_rva, location.rva, &mut messages);
		}
		messages.sort();
		messages.dedup();
		let mut code_rvas = code_rvas.clone();
		code_rvas.sort_unstable();
		code_rvas.dedup();
		output.push(PanicOutput {
			location_rva: location.rva,
			file: location.file,
			line: location.line,
			column: location.column,
			messages,
			code_rvas,
		});
	}
	match format {
		OutputFormat::Json => print_json(&output, false),
		OutputFormat::JsonPretty => print_json(&output, true),
		OutputFormat::Text => {
			let image_name = path.file_name().and_then(|name| name.to_str()).unwrap_or("<input>");
			for item in output {
				println!("{image_name}!{:#010x} {}:{}:{}", item.location_rva, item.file, item.line, item.column);
				for message in item.messages {
					println!("  message: {message:?}");
				}
				for code_rva in item.code_rvas {
					println!("  referenced at {image_name}!{code_rva:#010x}");
				}
			}
			Ok(())
		},
	}
}

fn find_locations(file: PeFile<'_>) -> Vec<RawLocation> {
	let mut output = Vec::new();
	for section in file.section_headers() {
		if section.Characteristics & image::IMAGE_SCN_MEM_EXECUTE != 0 {
			continue;
		}
		let Ok(bytes) = file.get_section_bytes(section) else {
			continue;
		};
		for offset in (0..bytes.len().saturating_sub(23)).step_by(8) {
			let record = &bytes[offset..offset + 24];
			let pointer = u64::from_le_bytes(record[0..8].try_into().unwrap());
			let Ok(file_rva) = file.va_to_rva(pointer) else {
				continue;
			};
			let length64 = u64::from_le_bytes(record[8..16].try_into().unwrap());
			let Ok(length) = usize::try_from(length64) else {
				continue;
			};
			let line = u32::from_le_bytes(record[16..20].try_into().unwrap());
			let column = u32::from_le_bytes(record[20..24].try_into().unwrap());
			if line == 0 || line > 10_000_000 || column == 0 || column > 1_000_000 {
				continue;
			}
			let Some(source_file) = read_utf8(file, file_rva, length) else {
				continue;
			};
			if !looks_like_rust_source_path(source_file) {
				continue;
			}
			output.push(RawLocation {
				rva: section.VirtualAddress.saturating_add(offset as u32),
				file: source_file.to_owned(),
				line,
				column,
			});
		}
	}
	output.sort_unstable_by_key(|location| location.rva);
	output.dedup_by_key(|location| location.rva);
	output
}

fn looks_like_rust_source_path(value: &str) -> bool {
	!value.is_empty() && value.len() <= 4096 && value.contains(".rs") && value.chars().all(|ch| !ch.is_control())
}

fn recover_nearby_messages(file: PeFile<'_>, references: &PanicReferences<'_>, location_code_rva: u32, location_rva: u32, output: &mut Vec<String>) {
	let start = location_code_rva.saturating_sub(32);
	let end = location_code_rva.saturating_add(16);
	for candidate in references.xrefs {
		if candidate.code_rva < start || candidate.code_rva > end || candidate.target_rva == location_rva {
			continue;
		}
		if let Some(template) = read_format_template(file, candidate.target_rva) {
			output.push(template.rendered);
			continue;
		}
		let Some(mut length) = nearby_edx_immediate(references.immediates, candidate.code_rva, end) else {
			continue;
		};
		if nearby_direct_call(references.calls, location_code_rva).is_some_and(|target| references.compact_argument_calls.contains(&target)) {
			if length & 1 == 0 {
				continue;
			}
			length >>= 1;
		}
		if let Some(message) = read_utf8(file, candidate.target_rva, length as usize)
			&& message.chars().all(|ch| ch == '\n' || ch == '\r' || ch == '\t' || !ch.is_control())
		{
			output.push(message.to_owned());
		}
	}
}

fn nearby_direct_call(calls: &[Xref], from: u32) -> Option<u32> {
	let end = from.saturating_add(32);
	calls.iter().find(|call| call.code_rva >= from && call.code_rva <= end).map(|call| call.target_rva)
}

fn nearby_edx_immediate(immediates: &[Immediate], from: u32, to: u32) -> Option<u32> {
	immediates
		.iter()
		.find(|immediate| immediate.code_rva >= from && immediate.code_rva <= to && immediate.value > 0 && immediate.value <= 64 * 1024)
		.map(|immediate| immediate.value)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rust_source_path_filter_is_conservative() {
		assert!(looks_like_rust_source_path("src/main.rs"));
		assert!(looks_like_rust_source_path("C:\\work\\src\\lib.rs"));
		assert!(!looks_like_rust_source_path("ordinary message"));
		assert!(!looks_like_rust_source_path("bad.rs\n"));
	}
}
