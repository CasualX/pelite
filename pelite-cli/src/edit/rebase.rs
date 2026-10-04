use super::*;
use pelite::image;

pub fn run(bytes: &mut pelite::PeMemory, base: u64) -> Result {
	match pelite::PeFile::from_bytes(bytes)? {
		pelite::Wrap::T32(_) => {
			let base = u32::try_from(base).map_err(|_| err("PE32 base address exceeds u32 range"))?;
			rebase32(bytes, base)
		},
		pelite::Wrap::T64(_) => rebase64(bytes, base),
	}
}

fn rebase32(bytes: &mut pelite::PeMemory, base: u32) -> Result {
	let pe = pelite::pe32::PeFile::from_bytes(bytes)?;
	let header = pe.optional_header();
	let base_offset = pe.offset_of(&header.ImageBase);
	let old_base: u32 = header.ImageBase.into();
	let delta = base.wrapping_sub(old_base);
	if delta == 0 {
		return Ok(());
	}

	// TODO: Handle stricter validation of malformed relocation blocks in pelite itself.
	let mut fixups = Vec::new();
	let relocs = pe.base_relocs().map_err(|error| err(format!("cannot rebase without base relocations: {error}")))?;
	relocs.for_each(|rva, ty| {
		if ty != image::IMAGE_REL_BASED_HIGHLOW {
			return;
		}

		match pe.slice(rva, 4, 1) {
			Ok(target) => fixups.push(pe.offset_of(target)),
			Err(error) => eprintln!("invalid relocation target at RVA {rva:#x}: {error}"),
		}
	});

	let view = dataview::DataView::from_mut(bytes.as_mut());
	for &offset in &fixups {
		let old: u32 = view.read(offset);
		let new = old.wrapping_add(delta);
		view.write(offset, &new);
	}

	view.write(base_offset, &base);
	Ok(())
}

fn rebase64(bytes: &mut pelite::PeMemory, base: u64) -> Result {
	let pe = pelite::pe64::PeFile::from_bytes(bytes)?;
	let header = pe.optional_header();
	let base_offset = pe.offset_of(&header.ImageBase);
	let old_base: u64 = header.ImageBase.into();
	let delta = base.wrapping_sub(old_base);
	if delta == 0 {
		return Ok(());
	}

	// TODO: Handle stricter validation of malformed relocation blocks in pelite itself.
	let relocs = pe.base_relocs().map_err(|error| err(format!("cannot rebase without base relocations: {error}")))?;
	let mut fixups = Vec::new();
	relocs.for_each(|rva, ty| {
		if ty != image::IMAGE_REL_BASED_DIR64 {
			return;
		}

		match pe.slice(rva, 8, 1) {
			Ok(target) => fixups.push(pe.offset_of(target)),
			Err(error) => eprintln!("invalid relocation target at RVA {rva:#x}: {error}"),
		}
	});

	let view = dataview::DataView::from_mut(bytes.as_mut());
	for &offset in &fixups {
		let old: u64 = view.read(offset);
		let new = old.wrapping_add(delta);
		view.write(offset, &new);
	}

	view.write(base_offset, &base);
	Ok(())
}
