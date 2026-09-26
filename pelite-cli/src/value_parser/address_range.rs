use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AddressRange {
	start: Address,
	end: Address,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RvaRange {
	pub start: u32,
	pub end: u32,
}

impl AddressRange {
	pub fn parse(value: &str) -> result::Result<Self, String> {
		let (kind, range) = value.split_once(':').ok_or("expected rva:START..END, va:START..END, or fo:START..END")?;
		let (start, end) = range.split_once("..").ok_or("expected a range in the form KIND:START..END")?;
		if end.contains("..") {
			return Err("expected exactly one '..' range separator".to_owned());
		}
		let start = Address::parse_parts(kind, start.trim()).map_err(|error| format!("invalid range start: {error}"))?;
		let end = Address::parse_parts(kind, end.trim()).map_err(|error| format!("invalid range end: {error}"))?;
		let ordered = match (start, end) {
			(Address::Rva(start), Address::Rva(end)) => start < end,
			(Address::Va(start), Address::Va(end)) => start < end,
			(Address::Fo(start), Address::Fo(end)) => start < end,
			_ => false,
		};
		if !ordered {
			return Err("range start must be less than range end".to_owned());
		}
		Ok(Self { start, end })
	}

	/// Converts a half-open address range using the PE image's address mapping.
	pub fn to_rva(self, pe: pelite::PeFile<'_>) -> Result<RvaRange> {
		let (start, end) = match (self.start, self.end) {
			(Address::Rva(start), Address::Rva(end)) => (start, end),
			(Address::Va(start), Address::Va(end)) => (pe.va_to_rva(start)?, pe.va_to_rva(end)?),
			(Address::Fo(start), Address::Fo(end)) => {
				let headers = pe.headers();
				let start_rva = headers.file_offset_to_rva(start)?;
				// The exclusive end can be just past a section's mapped raw data.
				let end_rva = headers.file_offset_to_rva(end - 1)?.checked_add(1).ok_or_else(|| err("range end RVA overflows"))?;
				if end_rva.checked_sub(start_rva).map(u64::from) != Some((end - start) as u64) {
					return Err(err("file offset range does not map to a contiguous RVA range"));
				}
				(start_rva, end_rva)
			},
			_ => unreachable!("parsed range endpoints have the same address kind"),
		};
		Ok(RvaRange { start, end })
	}
}

#[test]
fn parses_address_ranges() {
	for (value, start, end) in [
		("rva:1000..1100", Address::Rva(0x1000), Address::Rva(0x1100)),
		("rva:0x1000..0X1100", Address::Rva(0x1000), Address::Rva(0x1100)),
		("va:180001000h..180001100h", Address::Va(0x180001000), Address::Va(0x180001100)),
		("fo: 0x400 .. 0x500 ", Address::Fo(0x400), Address::Fo(0x500)),
	] {
		assert_eq!(AddressRange::parse(value), Ok(AddressRange { start, end }));
	}
}

#[test]
fn rejects_invalid_address_ranges() {
	for value in [
		"1000..1100", "rva:1000", "rva:1000..1000", "va:2000..1000", "fo:400..400",
		"rva:xyz..1000", "rva:..1000", "rva:1000..", "rva:1000..1100..1200",
		"rva:1000..rva:1100", "rva:1000..va:1100", "offset:400..500",
		"rva:1000..100000000", "va:0..10000000000000000",
	] {
		assert!(AddressRange::parse(value).is_err(), "accepted {value}");
	}
}
