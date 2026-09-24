use super::*;

pub(crate) unsafe fn dos_header(image: &[u8]) -> &IMAGE_DOS_HEADER { unsafe {
	&*(image.as_ptr() as *const IMAGE_DOS_HEADER)
}}
pub(crate) unsafe fn dos_image(image: &[u8]) -> &[u8] { unsafe {
	image.get_unchecked(..dos_header(image).e_lfanew as usize)
}}
pub(crate) unsafe fn nt_headers(image: &[u8]) -> &IMAGE_NT_HEADERS { unsafe {
	&*(image.as_ptr().offset(dos_header(image).e_lfanew as isize) as *const IMAGE_NT_HEADERS)
}}
pub(crate) unsafe fn file_header(image: &[u8]) -> &IMAGE_FILE_HEADER { unsafe {
	&nt_headers(image).FileHeader
}}
pub(crate) unsafe fn optional_header(image: &[u8]) -> &IMAGE_OPTIONAL_HEADER { unsafe {
	&nt_headers(image).OptionalHeader
}}
pub(crate) unsafe fn data_directory(image: &[u8]) -> &[IMAGE_DATA_DIRECTORY] { unsafe {
	let opt = optional_header(image);
	let len = cmp::min(opt.NumberOfRvaAndSizes as usize, IMAGE_NUMBEROF_DIRECTORY_ENTRIES);
	let offset = dos_header(image).e_lfanew as usize
		+ mem::offset_of!(IMAGE_NT_HEADERS, OptionalHeader)
		+ mem::offset_of!(IMAGE_OPTIONAL_HEADER, DataDirectory);
	let data = image.as_ptr().add(offset).cast();
	slice::from_raw_parts(data, len)
}}
pub(crate) unsafe fn section_headers(image: &[u8]) -> &super::PeSectionHeaders { unsafe {
	let nt = nt_headers(image);
	let offset = dos_header(image).e_lfanew as usize
		+ mem::offset_of!(IMAGE_NT_HEADERS, OptionalHeader)
		+ nt.FileHeader.SizeOfOptionalHeader as usize;
	let data = image.as_ptr().add(offset).cast();
	let raw = slice::from_raw_parts(data, nt.FileHeader.NumberOfSections as usize);
	super::PeSectionHeaders::new(raw)
}}

pub(crate) unsafe fn slice_section(image: &[u8], rva: Rva, min_size_of: usize, align_of: usize) -> Result<&[u8]> {
	let start = rva as usize;
	if rva == 0 {
		Err(Error::Null)
	}
	else if start > image.len() {
		Err(Error::Bounds)
	}
	else if !usize::wrapping_add(image.as_ptr() as usize, start).aligned_to(align_of) {
		Err(Error::Misaligned)
	}
	else {
		match image.get(start..) {
			Some(bytes) if bytes.len() >= min_size_of => Ok(bytes),
			_ => Err(Error::Bounds),
		}
	}
}
pub(crate) unsafe fn read_section(image: &[u8], image_base: Va, va: Va, min_size_of: usize, align_of: usize) -> Result<&[u8]> { unsafe {
	let image_size = optional_header(image).SizeOfImage;

	if va == 0 {
		Err(Error::Null)
	}
	else if va < image_base || va - image_base > image_size as Va {
		Err(Error::Bounds)
	}
	else {
		let start = (va - image_base) as usize;
		if !usize::wrapping_add(image.as_ptr() as usize, start).aligned_to(align_of) {
			Err(Error::Misaligned)
		}
		else {
			match image.get(start..) {
				Some(bytes) if bytes.len() >= min_size_of => Ok(bytes),
				_ => Err(Error::Bounds),
			}
		}
	}
}}

pub(crate) unsafe fn range_file(image: &[u8], rva: Rva, min_size_of: usize) -> Result<&[u8]> { unsafe {
	// This code has been carefully designed to avoid panicking on overflow
	for it in section_headers(image) {
		// Compare if rva is contained within the virtual address space of a section
		// If the calculating the section end address overflows the corrupt section will be skipped
		#[allow(non_snake_case)]
		let VirtualEnd = it.VirtualAddress.wrapping_add(cmp::max(it.VirtualSize, it.SizeOfRawData));
		// $1
		if it.VirtualAddress <= rva && rva < VirtualEnd {
			// Isolate and range check the pointer and size of raw data
			// If this fails immediately abort and return an error
			let section_range = it.PointerToRawData as usize..it.PointerToRawData.wrapping_add(it.SizeOfRawData) as usize;
			let section_bytes = image.get(section_range).ok_or(Error::Invalid)?;
			// Calculate the offset in the section requested. cannot underflow, see $1
			let section_offset = (rva - it.VirtualAddress) as usize;
			return match section_bytes.get(section_offset..) {
				Some(bytes) if bytes.len() >= min_size_of => Ok(bytes),
				// Identify the reason the slice fails. cannot underflow, see $1
				_ => Err(if min_size_of > (VirtualEnd - rva) as usize { Error::Bounds } else { Error::ZeroFill }),
			};
		}
	}
	Err(Error::Bounds)
}}
#[inline(never)]
pub(crate) unsafe fn slice_file(image: &[u8], rva: Rva, min_size_of: usize, align_of: usize) -> Result<&[u8]> { unsafe {
	if rva == 0 {
		return Err(Error::Null);
	}

	let bytes = range_file(image, rva, min_size_of)?;

	if !(bytes.as_ptr() as usize).aligned_to(align_of) {
		return Err(Error::Misaligned);
	}

	Ok(bytes)
}}
#[inline(never)]
pub(crate) unsafe fn read_file(image: &[u8], image_base: Va, va: Va, min_size_of: usize, align_of: usize) -> Result<&[u8]> { unsafe {
	let size_of_image = optional_header(image).SizeOfImage;

	if va == 0 {
		return Err(Error::Null);
	}
	if va < image_base || va - image_base >= size_of_image as Va {
		return Err(Error::Bounds);
	}

	let rva = (va - image_base) as Rva;
	let bytes = range_file(image, rva, min_size_of)?;

	if !(bytes.as_ptr() as usize).aligned_to(align_of) {
		return Err(Error::Misaligned);
	}

	Ok(bytes)
}}

//----------------------------------------------------------------

// TODO: This code needs to be audited...
// The safety of `Pe` relies on it.
pub(crate) fn validate_headers(image: &[u8]) -> Result<u32> {
	// Grab the DOS header
	if mem::size_of::<IMAGE_DOS_HEADER>() > image.len() {
		return Err(Error::Bounds);
	}
	// Check basic alignment of the image bytes
	if !image.as_ptr().aligned_to(4) {
		return Err(Error::Misaligned);
	}
	let dos = unsafe { &*(image.as_ptr() as *const IMAGE_DOS_HEADER) };
	// Verify the DOS header
	if dos.e_magic != IMAGE_DOS_SIGNATURE {
		return Err(Error::BadMagic);
	}
	// "According to the PE specification, the PE header must be aligned on a 8 byte boundary, but the Windows loader requires only a 4 byte alignment."
	if !dos.e_lfanew.aligned_to(4) {
		return Err(Error::Misaligned);
	}
	// Prevent overflow the easy way...
	// When changing, take care of overflow in later offset calculations!
	if dos.e_lfanew > 0x01000000 {
		return Err(Error::Insanity);
	}

	// Grab the NT headers
	let nt_end = dos.e_lfanew as usize + mem::size_of::<IMAGE_NT_HEADERS>();
	if nt_end > image.len() {
		return Err(Error::Bounds);
	}
	if !image.as_ptr().wrapping_add(dos.e_lfanew as usize).aligned_to(mem::align_of::<IMAGE_NT_HEADERS>()) {
		return Err(Error::Misaligned);
	}
	let nt = unsafe { &*(image.as_ptr().offset(dos.e_lfanew as isize) as *const IMAGE_NT_HEADERS) };
	// Verify the NT headers
	if nt.Signature != IMAGE_NT_HEADERS_SIGNATURE || !(nt.OptionalHeader.Magic == IMAGE_NT_OPTIONAL_HDR32_MAGIC || nt.OptionalHeader.Magic == IMAGE_NT_OPTIONAL_HDR64_MAGIC) {
		return Err(Error::BadMagic);
	}
	if nt.OptionalHeader.SizeOfHeaders as usize > image.len() {
		return Err(Error::Bounds);
	}
	if nt.OptionalHeader.SizeOfHeaders > nt.OptionalHeader.SizeOfImage {
		return Err(Error::Insanity);
	}
	// Give the caller a chance to retry with the correct parser
	if nt.OptionalHeader.Magic != IMAGE_NT_OPTIONAL_HDR_MAGIC {
		return Err(Error::PeMagic);
	}

	// Verify that the standard data directories exposed by this parser are physically present.
	// Pelite follows the standard table size here; malformed but loadable images may advertise
	// a larger count without storing those extra entries.
	let num_rva_sizes = cmp::min(nt.OptionalHeader.NumberOfRvaAndSizes as usize, IMAGE_NUMBEROF_DIRECTORY_ENTRIES);
	let size_of_data_dir = mem::size_of::<IMAGE_DATA_DIRECTORY>().checked_mul(num_rva_sizes).ok_or(Error::Overflow)?;
	if nt_end.checked_add(size_of_data_dir).ok_or(Error::Overflow)? > image.len() {
		return Err(Error::Bounds);
	}

	// Verify the section headers
	if nt.FileHeader.NumberOfSections > 96 {
		return Err(Error::Insanity);
	}
	// u16 * sizeof(T) casted to usize, cannot reasonably overflow
	let size_of_sections = nt.FileHeader.NumberOfSections as usize * mem::size_of::<IMAGE_SECTION_HEADER>();
	// e_lfanew is checked for reasonable values, the others then cannot reasonably cause overflow
	#[rustfmt::skip]
	let start_of_sections = dos.e_lfanew as usize
		+ (mem::size_of::<IMAGE_NT_HEADERS>() - mem::size_of::<IMAGE_OPTIONAL_HEADER>())
		+ nt.FileHeader.SizeOfOptionalHeader as usize;
	if !image.as_ptr().wrapping_add(start_of_sections).aligned_to(mem::align_of::<IMAGE_SECTION_HEADER>()) {
		return Err(Error::Misaligned);
	}
	// then the sum of these cannot reasonably overflow
	if size_of_sections + start_of_sections > image.len() {
		return Err(Error::Bounds);
	}
	Ok(nt.OptionalHeader.SizeOfImage)
}

pub(crate) unsafe fn rva_to_file_offset(image: &[u8], rva: Rva) -> Result<usize> {
	// Consider rva inside headers to be valid
	if rva < unsafe { optional_header(image) }.SizeOfHeaders {
		return Ok(rva as usize);
	}
	// This code has been carefully designed to avoid panicking on overflow
	for it in unsafe { section_headers(image) } {
		// Compare if rva is contained within the virtual address space of a section
		// If the calculating the section end address overflows the corrupt section will be skipped
		#[allow(non_snake_case)]
		let VirtualEnd = it.VirtualAddress.wrapping_add(cmp::max(it.VirtualSize, it.SizeOfRawData));
		// $1
		if it.VirtualAddress <= rva && rva < VirtualEnd {
			// Check if the raw data reference is sane
			// $2
			if let None = it.PointerToRawData.checked_add(it.SizeOfRawData) {
				return Err(Error::Overflow);
			}
			// Calculate the offset in the section. cannot underflow, see $1
			let section_offset = rva - it.VirtualAddress;
			// $3
			return if section_offset < it.SizeOfRawData {
				// Calculate the final offset in the file. cannot overflow, see $2 and $3
				Ok((section_offset + it.PointerToRawData) as usize)
			}
			// Identify the reason the conversion fails
			else if section_offset < it.VirtualSize {
				Err(Error::ZeroFill)
			}
			else {
				Err(Error::Bounds)
			};
		}
	}
	Err(Error::Bounds)
}

pub(crate) unsafe fn file_offset_to_rva(image: &[u8], file_offset: usize) -> Result<Rva> {
	// Consider rva inside headers to be valid
	if file_offset < unsafe { optional_header(image) }.SizeOfHeaders as usize {
		return Ok(file_offset as Rva);
	}
	// This code has been carefully designed to avoid panicking on overflow
	for it in unsafe { section_headers(image) } {
		// Compare if file_offset is contained within the raw data of a section
		// If the calculating the section end address overflows the corrupt section will be skipped
		#[allow(non_snake_case)]
		let EndOfRawData = it.PointerToRawData.wrapping_add(it.SizeOfRawData);
		// $1
		if it.PointerToRawData as usize <= file_offset && file_offset < EndOfRawData as usize {
			// Check if the virtual reference is sane
			// $2
			if let None = it.VirtualAddress.checked_add(it.VirtualSize) {
				return Err(Error::Overflow);
			}
			// Calculate the offset in the section. cannot underflow, see $1
			let section_offset = file_offset as Rva - it.PointerToRawData;
			// $3
			return if section_offset < it.VirtualSize {
				// Calculate the final virtual address. cannot overflow, see $2 and $3
				Ok(section_offset + it.VirtualAddress)
			}
			// Identify the reason the conversion fails
			else if section_offset < it.SizeOfRawData {
				Err(Error::Unmapped)
			}
			else {
				Err(Error::Bounds)
			};
		}
	}
	Err(Error::Bounds)
}

pub(crate) unsafe fn rva_to_va(image: &[u8], image_base: Va, rva: Rva) -> Result<Va> {
	if rva == 0 {
		Err(Error::Null)
	}
	else {
		let size_of_image = unsafe { optional_header(image) }.SizeOfImage;

		if rva <= size_of_image {
			image_base.checked_add(rva as Va).ok_or(Error::Overflow)
		}
		else {
			Err(Error::Bounds)
		}
	}
}

pub(crate) unsafe fn va_to_rva(image: &[u8], image_base: Va, va: Va) -> Result<Rva> {
	if va == 0 {
		Err(Error::Null)
	}
	else {
		let size_of_image = unsafe { optional_header(image) }.SizeOfImage;

		// Carefully avoid panicking overflow
		if va < image_base || va - image_base > size_of_image as Va {
			Err(Error::Bounds)
		}
		else {
			Ok((va - image_base) as Rva)
		}
	}
}
