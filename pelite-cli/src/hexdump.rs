use std::path::PathBuf;

use clap::{Arg, ArgMatches, Command};
use pelite::Wrap;

use crate::rva_range::{RvaRange, parse as parse_rva_range};
use crate::{OutputFormat, Result, err, print_json};

const ROW_WIDTH: usize = 16;

struct HexdumpRow {
	address: u64,
	bytes: Vec<Option<u8>>,
	ascii: String,
}

pub fn command() -> Command {
	Command::new("hexdump")
		.about("Hexdump an RVA range")
		.after_help("The range is half-open and its endpoints are hexadecimal RVAs (for example, 1000..1100 or 0x1000..0x1100).")
		.arg(Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(Arg::new("range")
			.value_name("START..END")
			.value_parser(parse_rva_range)
			.required(true))
}

pub fn run(matches: &ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let range = *matches.get_one::<RvaRange>("range").expect("required by clap");
	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let (image_base, address_width) = match pe.optional_header() {
		Wrap::T32(header) => (u64::from(header.ImageBase), 10),
		Wrap::T64(header) => (header.ImageBase.get(), 18),
	};
	let len = usize::try_from(range.end - range.start)?;
	let bytes = pe.slice(range.start, len, 1)?;
	let bytes = &bytes[..len];
	let start_address = image_base.checked_add(u64::from(range.start)).ok_or_else(|| err("image base plus start RVA overflows"))?;

	match format {
		OutputFormat::Json => print_json(&bytes, false),
		OutputFormat::JsonPretty => print_json(&bytes, true),
		OutputFormat::Text => {
			for row in build_rows(start_address, bytes)? {
				print!("{:#0address_width$x}  ", row.address);
				for (index, byte) in row.bytes.iter().enumerate() {
					match byte {
						Some(byte) => print!("{byte:02x} "),
						None => print!("   "),
					}
					if index == 7 {
						print!(" ");
					}
				}
				println!(" |{}|", row.ascii);
			}
			Ok(())
		},
	}
}

fn build_rows(start_address: u64, bytes: &[u8]) -> Result<Vec<HexdumpRow>> {
	let end_address = start_address.checked_add(bytes.len() as u64).ok_or_else(|| err("hexdump address range overflows"))?;
	let mut address = start_address & !(ROW_WIDTH as u64 - 1);
	let mut rows = Vec::new();
	while address < end_address {
		let mut row_bytes = vec![None; ROW_WIDTH];
		let mut ascii = String::with_capacity(ROW_WIDTH);
		for (column, slot) in row_bytes.iter_mut().enumerate() {
			let byte_address = address + column as u64;
			if byte_address >= start_address && byte_address < end_address {
				let byte = bytes[(byte_address - start_address) as usize];
				*slot = Some(byte);
				ascii.push(if byte.is_ascii_graphic() || byte == b' ' { char::from(byte) } else { '.' });
			}
			else {
				ascii.push(' ');
			}
		}
		rows.push(HexdumpRow { address, bytes: row_bytes, ascii });
		address = address.checked_add(ROW_WIDTH as u64).ok_or_else(|| err("hexdump row address overflows"))?;
	}
	Ok(rows)
}
