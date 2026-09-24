use pelite::Wrap;

use super::*;

#[derive(Clone, Copy, Debug)]
enum Address {
	Rva(u32),
	Va(u64),
	Fo(usize),
}

#[derive(serde::Serialize)]
struct ConvertedAddress {
	rva: u32,
	va: u64,
	fo: usize,
}

pub fn command() -> clap::Command {
	clap::Command::new("addr")
		.about("Convert a PE address between RVA, VA, and file offset")
		.after_help("Addresses are hexadecimal, with or without a 0x prefix. Example: pelite-cli addr file.exe rva:1000")
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("address")
			.value_name("rva:HEX|va:HEX|fo:HEX")
			.value_parser(parse_address)
			.required(true))
}

fn parse_address(value: &str) -> result::Result<Address, String> {
	let (kind, digits) = value.split_once(':').ok_or("expected rva:HEX, va:HEX, or fo:HEX")?;
	let digits = digits.strip_prefix("0x").or_else(|| digits.strip_prefix("0X")).unwrap_or(digits);
	if digits.is_empty() {
		return Err("missing hexadecimal address".to_owned());
	}
	match kind {
		"rva" => u32::from_str_radix(digits, 16).map(Address::Rva).map_err(|_| "RVA must be a 32-bit hexadecimal value".to_owned()),
		"va" => u64::from_str_radix(digits, 16).map(Address::Va).map_err(|_| "VA must be a 64-bit hexadecimal value".to_owned()),
		"fo" => usize::from_str_radix(digits, 16).map(Address::Fo).map_err(|_| "file offset must be a hexadecimal value".to_owned()),
		_ => Err("expected rva:HEX, va:HEX, or fo:HEX".to_owned()),
	}
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let address = *matches.get_one::<Address>("address").expect("required by clap");
	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let headers = pe.headers();
	let image_base = match pe.optional_header() {
		Wrap::T32(header) => u64::from(header.ImageBase),
		Wrap::T64(header) => header.ImageBase.get(),
	};
	let rva = match address {
		Address::Rva(rva) => rva,
		Address::Va(va) => u32::try_from(va.checked_sub(image_base).ok_or_else(|| err("VA is below the image base"))?)
			.map_err(|_| err("VA is outside the 32-bit RVA range"))?,
		Address::Fo(fo) => match headers {
			Wrap::T32(headers) => headers.file_offset_to_rva(fo)?,
			Wrap::T64(headers) => headers.file_offset_to_rva(fo)?,
		},
	};
	let fo = match headers {
		Wrap::T32(headers) => headers.rva_to_file_offset(rva)?,
		Wrap::T64(headers) => headers.rva_to_file_offset(rva)?,
	};
	let va = image_base.checked_add(u64::from(rva)).ok_or_else(|| err("image base plus RVA overflows"))?;
	if matches!(pe.optional_header(), Wrap::T32(_)) && va > u64::from(u32::MAX) {
		return Err(err("VA exceeds the 32-bit address range"));
	}
	let converted = ConvertedAddress { rva, va, fo };
	match format {
		OutputFormat::Text => {
			println!("rva:{:#x} va:{:#x} fo:{:#x}", converted.rva, converted.va, converted.fo);
			Ok(())
		},
		OutputFormat::Json => print_json(&converted, false),
		OutputFormat::JsonPretty => print_json(&converted, true),
	}
}
