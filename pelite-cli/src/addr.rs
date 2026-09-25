use super::*;

#[derive(serde::Serialize)]
struct ConvertedAddress {
	rva: u32,
	va: u64,
	fo: Option<usize>,
	section: Option<String>,
}

pub fn command() -> clap::Command {
	clap::Command::new("addr")
		.about("Convert a PE address between RVA, VA, and file offset")
		.after_help("Addresses are hexadecimal. Example: pelite-cli addr file.exe rva:0x1000")
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("address")
			.value_name("rva:HEX|va:HEX|fo:HEX")
			.value_parser(Address::parse)
			.required(true))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let address = *matches.get_one::<Address>("address").expect("required by clap");
	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let rva = match address {
		Address::Rva(rva) => rva,
		Address::Va(va) => pe.va_to_rva(va)?,
		Address::Fo(fo) => pe.headers().file_offset_to_rva(fo)?,
	};
	let va = pe.rva_to_va(rva)?;
	let fo = pe.headers().rva_to_file_offset(rva).ok();
	let section = pe.section_headers().by_rva(rva).and_then(|section| section.name().ok()).filter(|name| !name.is_empty()).map(str::to_owned);
	let converted = ConvertedAddress { rva, va, fo, section };
	printer::print("Address", &converted, format)
}
