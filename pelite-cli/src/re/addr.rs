use super::*;

#[derive(serde::Serialize)]
struct ConvertedAddress<'a> {
	rva: u32,
	va: u64,
	fo: Option<usize>,
	section: &'a str,
}

pub fn command() -> clap::Command {
	clap::Command::new("addr")
		.about("Convert a PE address between RVA, VA, and file offset")
		.after_help(include_str!("docs/addr.md"))
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("address")
			.value_name("ADDRESS")
			.value_parser(Address::parse)
			.required(true))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let address = *matches.get_one::<Address>("address").expect("required by clap");
	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let rva = address.to_rva(pe)?;
	let va = pe.rva_to_va(rva)?;
	let fo = pe.headers().rva_to_file_offset(rva).ok();
	let section = get_section_name_by_rva(pe, rva);
	let converted = ConvertedAddress { rva, va, fo, section };
	print("Address", &converted, format)
}
