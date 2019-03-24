use std::{cmp, fs, mem, slice};
use std::ops::Range;

use pelite::pe64::*;
use pelite::util::align_to;

fn main() {
	let outer_map = pelite::FileMap::open(r"D:\Projects\EmbedPE\x64\Release\Host.exe").unwrap();
	let outer_pe = PeFile::from_bytes(&outer_map).unwrap();
	let outer_image = outer_pe.image();

	let inner_map = pelite::FileMap::open(r"D:\Projects\EmbedPE\x64\Release\Dependency.dll").unwrap();
	let inner_pe = PeFile::from_bytes(&inner_map).unwrap();
	let inner_image = inner_pe.image();

	let host = outer_pe;
	let deps = &[inner_pe];

	// Calculate size of the new headers
	let (headers, _) = headers_size(host, deps);
	let mapping = create_mapping(headers, host, deps);
	eprintln!("{:#x?}", mapping);
	// let data = build_image_template(host, deps, &mapping);

	// let sections = build_section_headers(outer_pe, &[inner_pe]);
	// dbg!(sections);

	// // Figure out how much extra space we can stuff in the headers
	// // By finding the section with the lowest virtual address
	// let max_header_size = outer_pe.section_headers()
	// 	.iter()
	// 	.filter(|sect| sect.VirtualAddress != 0)
	// 	.min_by_key(|sect| sect.VirtualAddress)
	// 	.expect("No section headers found!?");

	// // Start by injecting extra space for headers
	// let header_size = outer_pe.optional_header().SizeOfHeaders as usize;
	// let sections = outer_pe.section_headers();

	// let sections_start = sections.as_ptr() as usize - outer_image.as_ptr() as usize;
	// let sections_end = sections[sections.len()..].as_ptr() as usize - outer_image.as_ptr() as usize;
	// if sections_end >= 0x1000 {
	// 	panic!("headers too large!");
	// }

	// let extra = 0x1000 - header_size;

	// let mut vec = vec![0u8; outer_image.len() + extra];

	// vec[..header_size].copy_from_slice(&outer_image[..header_size]);
	// vec[0x1000..].copy_from_slice(&outer_image[header_size..]);

	// let (_, vec_nt, _, vec_sections) = unsafe { headers_mut(&mut vec) };
	// for sect in vec_sections.iter_mut() {
	// 	if !(sect.PointerToRawData == 0 && sect.SizeOfRawData == 0) {
	// 		sect.PointerToRawData += extra as u32;
	// 	}
	// }
	// // Now add extra section headers
	// vec_nt.FileHeader.NumberOfSections += inner_pe.section_headers().len() as u16;
	// let vec_sections_len = vec_sections.len();
	// let vec_inner_sections_ptr = vec_sections[vec_sections_len..].as_mut_ptr();
	// let vec_inner_sections = unsafe { slice::from_raw_parts_mut(vec_inner_sections_ptr, inner_pe.section_headers().len()) };
	// vec_inner_sections.copy_from_slice(inner_pe.section_headers());

	// fs::write(r"D:\Projects\EmbedPE\x64\Release\New.exe", &data).unwrap();
}

#[derive(Copy, Clone, Debug)]
struct PeFileMapping {
	virtual_address: u32,
	file_offset: u32,
}

fn count_sections_len(host: PeFile, deps: &[PeFile]) -> usize {
	// Before we start figure out the total number of sections in the result
	// This is needed to figure out the SizeOfHeaders of the new image
	let mut count = host.section_headers().len();
	// Skip the base relocs section header
	if host.base_relocs().is_ok() {
		count -= 1;
	}
	// Calculate this for every dependency we wish to merge
	for dep in deps {
		count += dep.section_headers().len();
		if dep.base_relocs().is_ok() {
			count -= 1;
		}
	}
	// Add sections required for our implementation
	// base relocs, imports, exceptions, glue code
	count + 4
}

fn headers_size(host: PeFile, deps: &[PeFile]) -> (u32, usize) {
	let sections_len = count_sections_len(host, deps);
	let headers = (host.section_headers().as_ptr() as usize - host.image().as_ptr() as usize
		+ sections_len * mem::size_of::<image::IMAGE_SECTION_HEADER>()) as u32;
	(headers, sections_len)
}

fn strip_reloc_section(bin: PeFile) -> &[image::IMAGE_SECTION_HEADER] {
	let mut sections = bin.section_headers();
	if let Some(data_dir) = bin.data_directory().get(image::IMAGE_DIRECTORY_ENTRY_BASERELOC) {
		if data_dir.VirtualAddress > 0 {
			if let Some(last_section) = sections.last() {
				if data_dir.VirtualAddress >= last_section.VirtualAddress && data_dir.VirtualAddress + data_dir.Size <= last_section.VirtualAddress + last_section.VirtualSize {
					sections = &sections[..sections.len() - 1];
				}
			}
		}
	}
	sections
}

fn sections_range_fo(sections: &[image::IMAGE_SECTION_HEADER]) -> Range<u32> {
	let mut range = !0..0;
	for sect in sections {
		if sect.PointerToRawData != 0 {
			range.start = cmp::min(range.start, sect.PointerToRawData);
			range.end = cmp::max(range.end, sect.PointerToRawData + sect.SizeOfRawData);
		}
	}
	range
}
fn sections_range_rva(sections: &[image::IMAGE_SECTION_HEADER]) -> Range<u32> {
	let mut range = !0..0;
	for sect in sections {
		if sect.VirtualAddress != 0 {
			range.start = cmp::min(range.start, sect.VirtualAddress);
			range.end = cmp::max(range.end, sect.VirtualAddress + sect.VirtualSize);
		}
	}
	range
}

// Figures out where the put each binary in the final image.
fn create_mapping(headers: u32, host: PeFile, deps: &[PeFile]) -> Vec<PeFileMapping> {
	let file_align = host.optional_header().FileAlignment;
	let section_align = host.optional_header().SectionAlignment;

	let mut mapping = Vec::with_capacity(deps.len() + 1);

	let mut file_offset = align_to(headers, file_align);
	let mut virtual_address = align_to(headers, section_align);

	// The host image goes first after the headers
	mapping.push(PeFileMapping { virtual_address, file_offset });
	let host_sections = strip_reloc_section(host);
	file_offset += align_to(sections_range_fo(host_sections).len() as u32, file_align);
	virtual_address += align_to(sections_range_rva(host_sections).len() as u32, section_align);

	for &dep in deps {
		mapping.push(PeFileMapping { virtual_address, file_offset });
		let dep_sections = strip_reloc_section(dep);
		file_offset += align_to(sections_range_fo(dep_sections).end, file_align);
		virtual_address += align_to(sections_range_rva(dep_sections).end, section_align);
	}

	mapping
}

struct Section<'a> {
	file: PeFile<'a>,
	section: Option<&'a image::IMAGE_SECTION_HEADER>, // None if header
	data: Option<&'a [u8]>, // None if virtual
	virtual_address: u32,
	file_offset: u32,
}

fn create_sections<'a>(host: PeFile<'a>, deps: &[PeFile<'a>]) -> Vec<Section<'a>> {
	let mut sections = Vec::new();

	let section_align = host.optional_header().SectionAlignment;
	let file_align = host.optional_header().FileAlignment;

	let mut next_rva = 0;
	let mut next_fo = 0;

	sections.push(Section {
		file: host,
		section: None,
		data: Some(host.headers().image()),
		virtual_address: 0,
		file_offset: 0,
	});

	for sect in host.section_headers() {
		if &sect.Name == b".reloc\0\0" {
			continue;
		}
		sections.push(Section {
			file: host,
			section: Some(sect),
			data: host.get_section_data(sect).ok(),
			virtual_address: sect.VirtualAddress,
			file_offset: sect.PointerToRawData,
		});
		if sect.VirtualAddress != 0 {
			next_rva = cmp::max(next_rva, align_to(sect.VirtualAddress + sect.VirtualSize, section_align));
		}
		if sect.PointerToRawData != 0 {
			next_fo = cmp::max(next_fo, align_to(sect.PointerToRawData + sect.SizeOfRawData, file_align));
		}
	}

	for &dep in deps {
		let virtual_address = next_rva;
		let file_offset = next_fo;
		sections.push(Section {
			file: dep,
			section: None,
			data: Some(dep.headers().image()),
			virtual_address,
			file_offset,
		});
		for sect in dep.section_headers() {
			if &sect.Name == b".reloc\0\0" {
				continue;
			}
			sections.push(Section {
				file: dep,
				section: Some(sect),
				data: dep.get_section_data(sect).ok(),
				virtual_address: virtual_address + sect.VirtualAddress,
				file_offset: file_offset + sect.FileOffset,
			});
			next_rva = cmp::max(next_rva, align_to(virtual_address + sect.VirtualAddress + sect.VirtualSize, section_align));
			next_fo = cmp::max(next_fo, align_to(file_offset + sect.PointerToRawData + sect.SizeOfRawData, file_align));
		}
	}

	// FIXME! Update stuff if header doesn't fit in 4K!

	sections
}

fn create_image(sections: &[Section]) -> Vec<u8> {
	unimplemented!()
}
