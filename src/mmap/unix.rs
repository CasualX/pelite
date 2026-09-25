use std::fs::File;
use std::os::unix::fs::FileExt;
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::{io, mem, ptr, slice};

use crate::util::AlignTo;

/// Memory mapped file.
pub struct FileMap {
	ptr: *mut libc::c_void,
	size: usize,
}
impl FileMap {
	/// Maps the whole file into memory.
	pub fn open<P: AsRef<Path> + ?Sized>(path: &P) -> io::Result<FileMap> {
		Self::_open(path.as_ref())
	}
	fn _open(path: &Path) -> io::Result<FileMap> {
		// Open the file and get its fd
		let file = File::open(path)?;
		let fd = file.as_raw_fd();

		// Find its file size aligned to page boundary
		let size = unsafe {
			let mut stat = mem::MaybeUninit::uninit();
			if libc::fstat(fd, stat.as_mut_ptr()) < 0 {
				return Err(io::Error::last_os_error());
			}
			let stat = stat.assume_init();
			// Round up to nearest multiple of page_size
			let page_size = libc::sysconf(libc::_SC_PAGE_SIZE) as usize;
			(stat.st_size as usize).align_to(page_size)
		};

		// Mmap the file
		unsafe {
			let ptr = libc::mmap(ptr::null_mut(), size as libc::size_t, libc::PROT_READ, libc::MAP_PRIVATE, fd, 0);
			if ptr == libc::MAP_FAILED { Err(io::Error::last_os_error()) } else { Ok(FileMap { ptr, size }) }
		}
	}
}
impl Drop for FileMap {
	fn drop(&mut self) {
		unsafe {
			let _result = libc::munmap(self.ptr, self.size as libc::size_t);
			debug_assert_eq!(_result, 0, "unable to munmap: {}", io::Error::last_os_error());
		}
	}
}
impl AsRef<[u8]> for FileMap {
	fn as_ref(&self) -> &[u8] {
		unsafe { slice::from_raw_parts(self.ptr as *const _, self.size) }
	}
}


/// Memory mapped PE image with its sections at their virtual addresses.
pub struct ImageMap {
	ptr: *mut libc::c_void,
	size: usize,
}
impl ImageMap {
	/// Loads a PE32 or PE32+ file into a zero-filled mapping.
	/// Full pages are mapped from the file when file and image offsets have matching page alignment.
	/// Other bytes are read directly into anonymous pages.
	pub fn open<P: AsRef<Path> + ?Sized>(path: &P) -> io::Result<ImageMap> {
		use crate::image::{IMAGE_DOS_HEADER, IMAGE_FILE_HEADER, IMAGE_OPTIONAL_HEADER64, IMAGE_SECTION_HEADER};

		let source = File::open(path)?;
		let file_size = source.metadata()?.len();
		let invalid = || io::Error::new(io::ErrorKind::InvalidData, "invalid PE image");
		let mut dos: IMAGE_DOS_HEADER = dataview::zeroed();
		source.read_exact_at(dataview::bytes_mut(&mut dos), 0)?;
		if dos.e_magic != crate::image::IMAGE_DOS_SIGNATURE || dos.e_lfanew > 0x0100_0000 {
			return Err(invalid());
		}

		let nt = dos.e_lfanew as usize;
		let mut signature: u32 = dataview::zeroed();
		source.read_exact_at(dataview::bytes_mut(&mut signature), nt as u64)?;
		if signature != crate::image::IMAGE_NT_HEADERS_SIGNATURE {
			return Err(invalid());
		}

		let mut file_header: IMAGE_FILE_HEADER = dataview::zeroed();
		source.read_exact_at(dataview::bytes_mut(&mut file_header), (nt + 4) as u64)?;
		let optional = nt.checked_add(24).ok_or_else(invalid)?;
		if file_header.SizeOfOptionalHeader < 64 {
			return Err(invalid());
		}

		// These fields have identical offsets in PE32 and PE32+.
		let mut opt: IMAGE_OPTIONAL_HEADER64 = dataview::zeroed();
		source.read_exact_at(&mut dataview::bytes_mut(&mut opt)[..64], optional as u64)?;
		if opt.Magic != crate::image::IMAGE_NT_OPTIONAL_HDR32_MAGIC && opt.Magic != crate::image::IMAGE_NT_OPTIONAL_HDR64_MAGIC {
			return Err(invalid());
		}
		let section_alignment = opt.SectionAlignment as usize;
		let size = opt.SizeOfImage as usize;
		let headers = opt.SizeOfHeaders as usize;
		let section_table = optional.checked_add(file_header.SizeOfOptionalHeader as usize).ok_or_else(invalid)?;
		let section_count = file_header.NumberOfSections as usize;
		let section_bytes = section_count.checked_mul(mem::size_of::<IMAGE_SECTION_HEADER>()).ok_or_else(invalid)?;
		let section_end = section_table.checked_add(section_bytes).ok_or_else(invalid)?;
		if size == 0 || size > isize::MAX as usize || headers > size || headers as u64 > file_size ||
			section_end > headers || !section_alignment.is_power_of_two() {
			return Err(invalid());
		}
		let mut sections: Vec<IMAGE_SECTION_HEADER> = vec![dataview::zeroed(); section_count];
		source.read_exact_at(dataview::bytes_mut(sections.as_mut_slice()), section_table as u64)?;

		// Validate all ranges before mapping any file pages.
		let mut copies = Vec::with_capacity(section_count);
		for section in &sections {
			let virtual_size = if section.VirtualSize == 0 { section.SizeOfRawData } else { section.VirtualSize } as usize;
			let aligned_size = virtual_size.checked_add(section_alignment - 1).ok_or_else(invalid)? & !(section_alignment - 1);
			let count = (section.SizeOfRawData as usize).min(aligned_size);
			let dest = section.VirtualAddress as usize;
			let src = section.PointerToRawData as usize;
			if dest.checked_add(count).is_none_or(|end| end > size) ||
				(src as u64).checked_add(count as u64).is_none_or(|end| end > file_size) {
				return Err(invalid());
			}
			copies.push((dest, src, count));
		}

		let page = unsafe { libc::sysconf(libc::_SC_PAGE_SIZE) };
		if page <= 0 {
			return Err(io::Error::last_os_error());
		}
		let page = page as usize;
		let ptr = unsafe {
			libc::mmap(ptr::null_mut(), size, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_PRIVATE | libc::MAP_ANONYMOUS, -1, 0)
		};
		if ptr == libc::MAP_FAILED {
			return Err(io::Error::last_os_error());
		}
		let map = ImageMap { ptr, size };
		let header_page_end = headers.div_ceil(page) * page;
		map_file_or_read(&source, ptr.cast(), 0, 0, headers, page, 0)?;
		for (dest, src, count) in copies {
			map_file_or_read(&source, ptr.cast(), dest, src, count, page, header_page_end)?;
		}
		if unsafe { libc::mprotect(ptr, size, libc::PROT_READ) } < 0 {
			return Err(io::Error::last_os_error());
		}
		Ok(map)
	}
}

/// Map whole matching pages; read short, unaligned or unsupported ranges into the image.
fn map_file_or_read(source: &File, base: *mut u8, dest: usize, src: usize, count: usize, page: usize, min_mapped_dest: usize) -> io::Result<()> {
	let mut lead = if dest % page == src % page { (page - dest % page) % page } else { count };
	if dest + lead < min_mapped_dest { lead = min_mapped_dest - dest; }
	let lead = lead.min(count);
	let mapped_len = (count - lead) / page * page;
	if lead != 0 {
		let bytes = unsafe { slice::from_raw_parts_mut(base.add(dest), lead) };
		source.read_exact_at(bytes, src as u64)?;
	}
	if mapped_len != 0 {
		let address = unsafe { base.add(dest + lead) };
		let offset = libc::off_t::try_from(src + lead).map_err(|_| io::ErrorKind::InvalidData)?;
		let mapped = unsafe { libc::mmap(address.cast(), mapped_len, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_PRIVATE | libc::MAP_FIXED, source.as_raw_fd(), offset) };
		if mapped == libc::MAP_FAILED {
			let bytes = unsafe { slice::from_raw_parts_mut(address, mapped_len) };
			source.read_exact_at(bytes, (src + lead) as u64)?;
		}
	}
	let tail = lead + mapped_len;
	if tail < count {
		let bytes = unsafe { slice::from_raw_parts_mut(base.add(dest + tail), count - tail) };
		source.read_exact_at(bytes, (src + tail) as u64)?;
	}
	Ok(())
}
impl AsRef<[u8]> for ImageMap {
	fn as_ref(&self) -> &[u8] {
		unsafe { slice::from_raw_parts(self.ptr.cast(), self.size) }
	}
}
impl Drop for ImageMap {
	fn drop(&mut self) {
		unsafe { libc::munmap(self.ptr, self.size); }
	}
}
