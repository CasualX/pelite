use pelite::Wrap;

use super::*;

const ROW_WIDTH: usize = 16;

struct HexRow<'a> {
	address: u64,
	offset: usize,
	bytes: &'a [u8],
}

struct HexRows<'a> {
	address: u64,
	offset: usize,
	bytes: &'a [u8],
}

impl<'a> Iterator for HexRows<'a> {
	type Item = HexRow<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.bytes.is_empty() {
			return None;
		}
		let row_len = (ROW_WIDTH - self.offset).min(self.bytes.len());
		let (bytes, remaining) = self.bytes.split_at(row_len);
		let row = HexRow { address: self.address, offset: self.offset, bytes };
		self.bytes = remaining;
		if !self.bytes.is_empty() {
			self.address += ROW_WIDTH as u64;
			self.offset = 0;
		}
		Some(row)
	}
}

pub fn command() -> clap::Command {
	clap::Command::new("hexdump")
		.about("Hexdump an RVA range")
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("range")
			.value_name("START..END")
			.value_parser(RvaRange::parse)
			.required(true))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
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
			let stdout = io::stdout();
			let mut output = stdout.lock();
			for row in build_rows(start_address, bytes)? {
				write!(output, "{:#0address_width$x}  ", row.address)?;
				for column in 0..ROW_WIDTH {
					match column.checked_sub(row.offset).and_then(|index| row.bytes.get(index)) {
						Some(byte) => write!(output, "{byte:02x} ")?,
						None => write!(output, "   ")?,
					}
					if column == 7 {
						write!(output, " ")?;
					}
				}
				write!(output, " |")?;
				for column in 0..ROW_WIDTH {
					match column.checked_sub(row.offset).and_then(|index| row.bytes.get(index)) {
						Some(byte) if byte.is_ascii_graphic() || *byte == b' ' => write!(output, "{}", char::from(*byte))?,
						Some(_) => write!(output, ".")?,
						None => write!(output, " ")?,
					}
				}
				writeln!(output, "|")?;
			}
			Ok(())
		},
	}
}

fn build_rows(start_address: u64, bytes: &[u8]) -> Result<HexRows<'_>> {
	start_address.checked_add(bytes.len() as u64).ok_or_else(|| err("hexdump address range overflows"))?;
	let address = start_address & !(ROW_WIDTH as u64 - 1);
	let offset = (start_address - address) as usize;
	Ok(HexRows { address, offset, bytes })
}
