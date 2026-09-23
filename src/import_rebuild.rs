//! Rebuild imports from resolved addresses in a PE image.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::{cmp, mem};
use crate::{image, Error, PeFile, PeMemory, Result, Wrap};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Symbol { Name(Vec<u8>), Ordinal(u16) }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedImport {
	/// DLL filename, without directory components.
	pub dll: Vec<u8>,
	pub symbol: Symbol,
}

/// A resolved VA can have several import identities (aliases or forwarders).
pub type ImportMap = BTreeMap<u64, Vec<ResolvedImport>>;

#[derive(Clone, Debug)]
pub struct ImportRun {
	pub first_thunk: u32,
	pub dll: Vec<u8>,
	pub symbols: Vec<Symbol>,
}

/// Import data prepared independently from the output image.
pub struct BuiltImports {
	pub bytes: Vec<u8>,
	pub directory_rva: u32,
	pub directory_size: u32,
	base_rva: u32,
	pointer_width: usize,
	patches: Vec<(u32, Vec<u64>)>,
}

fn align(n: usize, a: usize) -> Result<usize> {
	if !a.is_power_of_two() { return Err(Error::Invalid); }
	n.checked_add(a - 1).map(|x| x & !(a - 1)).ok_or(Error::Overflow)
}
fn ptr_width(file: &PeFile<'_>) -> usize {
	match file { Wrap::T32(_) => 4, Wrap::T64(_) => 8 }
}
fn read_ptr(data: &[u8], offset: usize, width: usize) -> Option<u64> {
	let s = data.get(offset..offset.checked_add(width)?)?;
	Some(if width == 4 { u32::from_le_bytes(s.try_into().ok()?) as u64 }
		 else { u64::from_le_bytes(s.try_into().ok()?) })
}

/// Detect null-terminated pointer runs in initialized, non-executable sections.
/// Existing import address tables are excluded.
pub fn find_runs(bytes: &[u8], map: &ImportMap) -> Result<Vec<ImportRun>> {
	let pe = PeFile::from_bytes(bytes)?;
	let width = ptr_width(&pe);
	let mut occupied = Vec::new();
	let dir = pe.data_directory().get(image::IMAGE_DIRECTORY_ENTRY_IMPORT).ok_or(Error::Bounds)?;
	if dir.VirtualAddress != 0 {
		for desc in pe.imports()?.image() {
			let start = desc.FirstThunk.get();
			let mut end = start;
			loop {
				let offset = match pe {
					Wrap::T32(file) => crate::pe32::Pe::rva_to_file_offset(file, end)?,
					Wrap::T64(file) => crate::pe64::Pe::rva_to_file_offset(file, end)?,
				};
				if read_ptr(bytes, offset, width).ok_or(Error::Bounds)? == 0 { break; }
				end = end.checked_add(width as u32).ok_or(Error::Overflow)?;
			}
			occupied.push(start..end.checked_add(width as u32).ok_or(Error::Overflow)?);
		}
	}
	let mut runs = Vec::new();
	for section in pe.section_headers() {
		if section.Characteristics & image::IMAGE_SCN_CNT_INITIALIZED_DATA == 0 ||
		   section.Characteristics & image::IMAGE_SCN_MEM_EXECUTE != 0 { continue; }
		let size = cmp::min(section.VirtualSize, section.SizeOfRawData) as usize;
		let raw = section.PointerToRawData as usize;
		let data = bytes.get(raw..raw.checked_add(size).ok_or(Error::Overflow)?).ok_or(Error::Bounds)?;
		let mut pos = (width - (section.VirtualAddress as usize % width)) % width;
		while pos + width <= data.len() {
			let start = pos;
			let mut addresses = Vec::new();
			while pos + width <= data.len() {
				let address = read_ptr(data, pos, width).ok_or(Error::Bounds)?;
				if address == 0 || !map.contains_key(&address) { break; }
				addresses.push(address);
				pos += width;
			}
			if addresses.is_empty() { pos += width; continue; }
			if read_ptr(data, pos, width) != Some(0) { continue; }
			let first_thunk = section.VirtualAddress.checked_add(start as u32).ok_or(Error::Overflow)?;
			if occupied.iter().any(|range: &core::ops::Range<u32>| range.contains(&first_thunk)) { continue; }
			let mut dlls = Vec::new();
			for candidate in &map[&addresses[0]] {
				if !dlls.contains(&candidate.dll) && addresses.iter().all(|a| map[a].iter().any(|x| x.dll == candidate.dll)) {
					dlls.push(candidate.dll.clone());
				}
			}
			if dlls.len() != 1 { continue; }
			let dll = &dlls[0];
			let symbols = addresses.iter().map(|a| map[a].iter().find(|x| x.dll == *dll).unwrap().symbol.clone()).collect();
			runs.push(ImportRun { first_thunk, dll: dll.clone(), symbols });
		}
	}
	Ok(runs)
}

/// The RVA where an appended section will be loaded.
pub fn section_rva(bytes: &[u8]) -> Result<u32> {
	let pe = PeFile::from_bytes(bytes)?;
	let (alignment, mut end) = match pe.optional_header() {
		Wrap::T32(h) => (h.SectionAlignment, h.SizeOfImage as usize),
		Wrap::T64(h) => (h.SectionAlignment, h.SizeOfImage as usize),
	};
	for section in pe.section_headers() {
		end = cmp::max(end, (section.VirtualAddress as usize)
			.checked_add(cmp::max(section.VirtualSize, section.SizeOfRawData) as usize).ok_or(Error::Overflow)?);
	}
	u32::try_from(align(end, alignment as usize)?).map_err(|_| Error::Overflow)
}
fn rva(base: u32, offset: usize) -> Result<u32> {
	base.checked_add(u32::try_from(offset).map_err(|_| Error::Overflow)?).ok_or(Error::Overflow)
}
fn pad(bytes: &mut Vec<u8>, a: usize) -> Result<()> {
	bytes.resize(align(bytes.len(), a)?, 0);
	Ok(())
}
fn put32(bytes: &mut Vec<u8>, x: u32) { bytes.extend_from_slice(&x.to_le_bytes()); }

/// Build the new section in a separate Vec, including existing descriptors.
pub fn build(base: u32, width: usize, old: &[image::IMAGE_IMPORT_DESCRIPTOR], runs: &[ImportRun]) -> Result<BuiltImports> {
	if width != 4 && width != 8 { return Err(Error::Invalid); }
	let mut bytes = Vec::new();
	let mut descriptors = Vec::new();
	let mut patches = Vec::new();
	for d in old { descriptors.push([d.OriginalFirstThunk.get(), d.TimeDateStamp.get(), d.ForwarderChain.get(), d.Name.get(), d.FirstThunk.get()]); }
	for run in runs {
		if run.dll.is_empty() || run.dll.contains(&0) || run.symbols.is_empty() { return Err(Error::Invalid); }
		let dll_rva = rva(base, bytes.len())?;
		bytes.extend_from_slice(&run.dll);
		bytes.push(0);
		let mut thunks = Vec::new();
		for symbol in &run.symbols {
			let thunk = match symbol {
				Symbol::Ordinal(ord) => (if width == 4 { image::IMAGE_ORDINAL_FLAG32 as u64 } else { image::IMAGE_ORDINAL_FLAG64 }) | *ord as u64,
				Symbol::Name(name) => {
					if name.is_empty() || name.contains(&0) { return Err(Error::Invalid); }
					pad(&mut bytes, 2)?;
					let name_rva = rva(base, bytes.len())?;
					bytes.extend_from_slice(&0u16.to_le_bytes());
					bytes.extend_from_slice(name);
					bytes.push(0);
					name_rva as u64
				},
			};
			thunks.push(thunk);
		}
		pad(&mut bytes, width)?;
		let int_rva = rva(base, bytes.len())?;
		patches.push((run.first_thunk, thunks.clone()));
		for thunk in thunks {
			if width == 4 { put32(&mut bytes, u32::try_from(thunk).map_err(|_| Error::Overflow)?); }
			else { bytes.extend_from_slice(&thunk.to_le_bytes()); }
		}
		bytes.resize(bytes.len() + width, 0);
		descriptors.push([int_rva, 0, 0, dll_rva, run.first_thunk]);
	}
	pad(&mut bytes, 4)?;
	let directory_rva = rva(base, bytes.len())?;
	for d in &descriptors { for x in d { put32(&mut bytes, *x); } }
	bytes.resize(bytes.len() + mem::size_of::<image::IMAGE_IMPORT_DESCRIPTOR>(), 0);
	let directory_size = u32::try_from((descriptors.len() + 1) * mem::size_of::<image::IMAGE_IMPORT_DESCRIPTOR>()).map_err(|_| Error::Overflow)?;
	Ok(BuiltImports { bytes, directory_rva, directory_size, base_rva: base, pointer_width: width, patches })
}

/// Append prepared data and publish the new section and import directory.
pub fn install(memory: &mut PeMemory, built: BuiltImports) -> Result<()> {
	let pe = PeFile::from_bytes(&*memory)?;
	let base = section_rva(memory)?;
	if built.base_rva != base || built.pointer_width != ptr_width(&pe) { return Err(Error::Invalid); }
	let (file_align, section_align, size_headers, old_init, opt_offset, size_image_field, size_init_field) = match pe.optional_header() {
		Wrap::T32(h) => (h.FileAlignment, h.SectionAlignment, h.SizeOfHeaders, h.SizeOfInitializedData,
			pe.offset_of(h), mem::offset_of!(image::IMAGE_OPTIONAL_HEADER32, SizeOfImage), mem::offset_of!(image::IMAGE_OPTIONAL_HEADER32, SizeOfInitializedData)),
		Wrap::T64(h) => (h.FileAlignment, h.SectionAlignment, h.SizeOfHeaders, h.SizeOfInitializedData,
			pe.offset_of(h), mem::offset_of!(image::IMAGE_OPTIONAL_HEADER64, SizeOfImage), mem::offset_of!(image::IMAGE_OPTIONAL_HEADER64, SizeOfInitializedData)),
	};
	if file_align < 16 || !file_align.is_power_of_two() || !section_align.is_power_of_two() || section_align < file_align { return Err(Error::Invalid); }
	let header = pe.file_header();
	let file_header_offset = pe.offset_of(header);
	let sections = pe.section_headers().as_slice();
	let last = sections.last().ok_or(Error::Invalid)?;
	let slot = pe.offset_of(&**last) + mem::size_of::<image::IMAGE_SECTION_HEADER>();
	let first_raw = sections.iter().filter(|s| s.PointerToRawData != 0).map(|s| s.PointerToRawData as usize).min().ok_or(Error::Invalid)?;
	let slot_end = slot.checked_add(mem::size_of::<image::IMAGE_SECTION_HEADER>()).ok_or(Error::Overflow)?;
	if slot_end > cmp::min(size_headers as usize, first_raw) || slot_end > memory.len() { return Err(Error::Bounds); }
	if memory[slot..slot_end].iter().any(|&x| x != 0) { return Err(Error::Invalid); }
	let dir = pe.data_directory().get(image::IMAGE_DIRECTORY_ENTRY_IMPORT).ok_or(Error::Bounds)?;
	let dir_offset = pe.offset_of(dir);
	let raw_offset = align(memory.append_offset(), file_align as usize)?;
	let raw_size = align(built.bytes.len(), file_align as usize)?;
	let image_size = align((base as usize).checked_add(built.bytes.len()).ok_or(Error::Overflow)?, section_align as usize)?;
	let count = header.NumberOfSections.checked_add(1).ok_or(Error::Overflow)?;
	let init_size = old_init.checked_add(u32::try_from(raw_size).map_err(|_| Error::Overflow)?).ok_or(Error::Overflow)?;
	let section = image::IMAGE_SECTION_HEADER {
		Name: *b".impreb\0", VirtualSize: u32::try_from(built.bytes.len()).map_err(|_| Error::Overflow)?, VirtualAddress: base,
		SizeOfRawData: u32::try_from(raw_size).map_err(|_| Error::Overflow)?, PointerToRawData: u32::try_from(raw_offset).map_err(|_| Error::Overflow)?,
		PointerToRelocations: 0, PointerToLinenumbers: 0, NumberOfRelocations: 0, NumberOfLinenumbers: 0,
		Characteristics: image::IMAGE_SCN_CNT_INITIALIZED_DATA | image::IMAGE_SCN_MEM_READ,
	};
	// Turn each recovered IAT slot back into an import thunk before writing.
	let mut patch_offsets = Vec::new();
	for (first, thunks) in &built.patches {
		for (index, thunk) in thunks.iter().enumerate() {
			let slot_rva = first.checked_add(u32::try_from(index * built.pointer_width).map_err(|_| Error::Overflow)?).ok_or(Error::Overflow)?;
			let offset = match pe {
				Wrap::T32(file) => crate::pe32::Pe::rva_to_file_offset(file, slot_rva)?,
				Wrap::T64(file) => crate::pe64::Pe::rva_to_file_offset(file, slot_rva)?,
			};
			if offset.checked_add(built.pointer_width).ok_or(Error::Overflow)? > memory.len() { return Err(Error::Bounds); }
			patch_offsets.push((offset, *thunk));
		}
	}
	let mut payload = alloc::vec![0; raw_offset - memory.append_offset()];
	payload.extend_from_slice(&built.bytes);
	payload.resize(payload.len() + raw_size - built.bytes.len(), 0);
	memory.append(&payload);
	for (offset, thunk) in patch_offsets {
		if built.pointer_width == 4 { memory[offset..offset + 4].copy_from_slice(&(thunk as u32).to_le_bytes()); }
		else { memory[offset..offset + 8].copy_from_slice(&thunk.to_le_bytes()); }
	}
	let view = memory.view_mut();
	view.write(slot, &section);
	view.write(file_header_offset + mem::offset_of!(image::IMAGE_FILE_HEADER, NumberOfSections), &count);
	view.write(opt_offset + size_image_field, &u32::try_from(image_size).map_err(|_| Error::Overflow)?);
	view.write(opt_offset + size_init_field, &init_size);
	view.write(dir_offset, &image::IMAGE_DATA_DIRECTORY { VirtualAddress: built.directory_rva, Size: built.directory_size });
	Ok(())
}


#[cfg(all(test, feature = "std"))]
mod tests {
	use super::*;
	use crate::pe64::Pe;

	#[test]
	fn rebuilds_a_pe32_run() {
		use crate::pe32::Pe;
		let path = concat!(env!("CARGO_MANIFEST_DIR"), "/demo/Demo.dll");
		let mut memory = PeMemory::open(path).unwrap();
		let section = {
			let pe = crate::pe32::PeFile::from_bytes(&memory).unwrap();
			**pe.section_headers().by_name(".data").unwrap()
		};
		let size = cmp::min(section.VirtualSize, section.SizeOfRawData) as usize;
		let offset = (0..size - 8).step_by(4).find(|&i| {
			memory[section.PointerToRawData as usize + i..section.PointerToRawData as usize + i + 8]
				.iter().all(|&b| b == 0)
		}).unwrap();
		let address = 0x7123_4567u32;
		let raw = section.PointerToRawData as usize + offset;
		memory[raw..raw + 4].copy_from_slice(&address.to_le_bytes());
		let mut map = ImportMap::new();
		map.insert(address as u64, alloc::vec![ResolvedImport {
			dll: b"example.dll".to_vec(), symbol: Symbol::Ordinal(17),
		}]);
		let runs = find_runs(&memory, &map).unwrap();
		assert!(runs.iter().any(|run| run.first_thunk == section.VirtualAddress + offset as u32));
		let pe = crate::pe32::PeFile::from_bytes(&memory).unwrap();
		let old = pe.imports().unwrap();
		let built = build(section_rva(&memory).unwrap(), 4, old.image(), &runs).unwrap();
		install(&mut memory, built).unwrap();
		let rebuilt = crate::pe32::PeFile::from_bytes(&memory).unwrap();
		let last = rebuilt.imports().unwrap().iter().next_back().unwrap();
		assert_eq!(last.dll_name().unwrap().as_ref(), b"example.dll");
		assert!(matches!(last.int().unwrap().next().unwrap().unwrap(), crate::Import::ByOrdinal { ord: 17 }));
		assert_ne!(*last.iat().unwrap().next().unwrap(), address);
	}

	#[test]
	fn rebuilds_a_resolved_run() {
		let path = concat!(env!("CARGO_MANIFEST_DIR"), "/demo/Demo64.dll");
		let mut memory = PeMemory::open(path).unwrap();
		let section = {
			let pe = crate::pe64::PeFile::from_bytes(&memory).unwrap();
			**pe.section_headers().by_name(".data").unwrap()
		};
		let size = cmp::min(section.VirtualSize, section.SizeOfRawData) as usize;
		let offset = (0..size - 16).step_by(8).find(|&i| {
			memory[section.PointerToRawData as usize + i..section.PointerToRawData as usize + i + 16]
				.iter().all(|&b| b == 0)
		}).unwrap();
		let address = 0x7fff_1234_5678_9000u64;
		let raw = section.PointerToRawData as usize + offset;
		memory[raw..raw + 8].copy_from_slice(&address.to_le_bytes());
		let mut map = ImportMap::new();
		map.insert(address, alloc::vec![ResolvedImport {
			dll: b"example.dll".to_vec(), symbol: Symbol::Name(b"SomeFunction".to_vec()),
		}]);
		let runs = find_runs(&memory, &map).unwrap();
		assert!(runs.iter().any(|run| run.first_thunk == section.VirtualAddress + offset as u32));
		let pe = crate::pe64::PeFile::from_bytes(&memory).unwrap();
		let old = pe.imports().unwrap();
		let built = build(section_rva(&memory).unwrap(), 8, old.image(), &runs).unwrap();
		let old_count = old.image().len();
		install(&mut memory, built).unwrap();
		let rebuilt = crate::pe64::PeFile::from_bytes(&memory).unwrap();
		let imports = rebuilt.imports().unwrap();
		assert_eq!(imports.image().len(), old_count + runs.len());
		let last = imports.iter().next_back().unwrap();
		assert_eq!(last.dll_name().unwrap().as_ref(), b"example.dll");
		let first_import = last.int().unwrap().next().unwrap().unwrap();
		match first_import {
			crate::Import::ByName { name, .. } => assert_eq!(name.as_ref(), b"SomeFunction"),
			_ => panic!("expected import by name"),
		}
		assert_ne!(*last.iat().unwrap().next().unwrap(), address);
	}
}
