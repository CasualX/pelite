use super::*;

struct CopyRange {
	/// Byte offsets in the file.
	source: ops::Range<usize>,
	/// Byte offsets relative to the requested RVA.
	dest: ops::Range<usize>,
}

/// Clip file-backed header and section ranges to the requested virtual bounds.
/// Missing file bytes are excluded. Widen offsets to avoid address wraparound.
fn copy_ranges(
	sections: &[pelite::image::IMAGE_SECTION_HEADER],
	header_size: u32,
	file_size: usize,
	bounds: ops::Range<u64>,
	mut copy: impl FnMut(CopyRange),
) {
	let mut visit_range = |virtual_start: u32, file_start: u32, size: u32| {
		let available = (file_size as u64).saturating_sub(u64::from(file_start));
		let virtual_start = u64::from(virtual_start);
		let start = bounds.start.max(virtual_start);
		let end = bounds.end.min(virtual_start + u64::from(size).min(available));
		if start >= end {
			return;
		}
		let source = (u64::from(file_start) + start - virtual_start) as usize;
		let dest = (start - bounds.start) as usize;
		let len = (end - start) as usize;
		copy(CopyRange { source: source..source + len, dest: dest..dest + len });
	};
	visit_range(0, 0, header_size);
	for section in sections {
		visit_range(section.VirtualAddress, section.PointerToRawData, section.VirtualSize.min(section.SizeOfRawData));
	}
}

#[unsafe(export_name = "pefileHexDumpBytes")]
pub unsafe fn hex_dump_bytes(pefile: *mut PeFile, rva: u32, bytes: u32) {
	let pe = match pelite::PeFile::from_bytes(unsafe { &*pefile }.as_ref()) {
		Ok(pe) => pe,
		Err(err) => return return_error(err),
	};
	let (image_size, header_size) = match pe.optional_header() {
		pelite::Wrap::T32(header) => (header.SizeOfImage, header.SizeOfHeaders),
		pelite::Wrap::T64(header) => (header.SizeOfImage, header.SizeOfHeaders),
	};
	let start = u64::from(rva);
	let end = start + u64::from(bytes);
	if end > u64::from(image_size) {
		return return_error(pelite::Error::Bounds);
	}

	let image = pe.image();
	let mut dump = vec![0; bytes as usize];
	copy_ranges(pe.section_headers().image(), header_size, image.len(), start..end, |range| {
		dump[range.dest].copy_from_slice(&image[range.source]);
	});
	return_bytes(&dump);
}

#[unsafe(export_name = "pefileHexDumpMask")]
pub unsafe fn hex_dump_mask(pefile: *mut PeFile, rva: u32, bytes: u32) {
	let pe = match pelite::PeFile::from_bytes(unsafe { &*pefile }.as_ref()) {
		Ok(pe) => pe,
		Err(err) => return return_error(err),
	};
	let (image_size, header_size) = match pe.optional_header() {
		pelite::Wrap::T32(header) => (header.SizeOfImage, header.SizeOfHeaders),
		pelite::Wrap::T64(header) => (header.SizeOfImage, header.SizeOfHeaders),
	};
	let start = u64::from(rva);
	let end = start + u64::from(bytes);
	if end > u64::from(image_size) {
		return return_error(pelite::Error::Bounds);
	}

	let image = pe.image();
	let mut mask = vec![0; (bytes as usize).div_ceil(8)];
	copy_ranges(pe.section_headers().image(), header_size, image.len(), start..end, |range| {
		// Bit i describes byte i relative to the requested RVA, LSB first.
		let first = range.dest.start / 8;
		let last = (range.dest.end - 1) / 8;
		let first_bits = 0xffu8 << (range.dest.start % 8);
		let last_bits = 0xffu8 >> (7 - (range.dest.end - 1) % 8);
		if first == last {
			mask[first] |= first_bits & last_bits;
		}
		else {
			mask[first] |= first_bits;
			mask[first + 1..last].fill(0xff);
			mask[last] |= last_bits;
		}
	});
	return_bytes(&mask);
}
