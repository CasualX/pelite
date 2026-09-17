use super::*;

const SKIP_VA: u32 = mem::size_of::<Va>() as u32;

pub trait Scan<'a>: Copy {
	fn read<T: Copy + Pod>(self, rva: Rva) -> Option<T>;
	fn pointer(self, va: Va) -> Option<Rva>;
	fn slice(self, rva: Rva) -> Option<&'a [u8]>;
}

impl<'a, P: Pe<'a>> Scan<'a> for P {
	fn read<T: Copy + Pod>(self, rva: Rva) -> Option<T> {
		self.derva_copy(rva).ok()
	}
	fn pointer(self, va: Va) -> Option<Rva> {
		self.va_to_rva(va).ok()
	}
	fn slice(self, rva: Rva) -> Option<&'a [u8]> {
		if self.layout() == PeLayout::File {
			return self.slice_bytes(rva).ok();
		}
		let section = self.section_headers().iter().find(|section| {
			section.VirtualAddress <= rva && section.VirtualAddress.checked_add(section.VirtualSize).is_some_and(|end| rva < end)
		})?;
		self.get_section_bytes(section).ok()?.get((rva - section.VirtualAddress) as usize..)
	}
}

impl<'a> Scan<'a> for &'a [u8] {
	fn read<T: Copy + Pod>(self, rva: Rva) -> Option<T> {
		let bytes = self.get(rva as usize..(rva as usize + mem::size_of::<T>()))?;
		let ptr = bytes.as_ptr() as *const T;
		Some(unsafe { ptr::read_unaligned(ptr) })
	}
	fn pointer(self, va: Va) -> Option<Rva> {
		Some(va as Rva)
	}
	fn slice(self, rva: Rva) -> Option<&'a [u8]> {
		self.get(rva as usize..)
	}
}

/// Resolve a signed save operand. Negative underflow wraps out of bounds, so the
/// caller's slice lookup retains the usual behavior for unavailable slots.
pub fn save_index(slot: i8, len: usize) -> usize {
	if slot < 0 {
		len.wrapping_add_signed(slot as isize)
	}
	else {
		slot as usize
	}
}

/// Read a signed save slot, returning zero when it is unavailable.
pub fn save_read(save: &[Rva], slot: i8) -> Rva {
	save.get(save_index(slot, save.len())).copied().unwrap_or(0)
}

/// Write a signed save slot, ignoring unavailable slots.
pub fn save_write(save: &mut [Rva], slot: i8, value: Rva) {
	if let Some(slot) = save.get_mut(save_index(slot, save.len())) {
		*slot = value;
	}
}

#[derive(Clone)]
pub struct Exec<'pat, P> {
	pub pe: P,
	pub pat: &'pat [pat::Atom],
	pub cursor: Rva,
	pub pc: usize,
}
impl<'a, 'pat, P: Scan<'a>> Exec<'pat, P> {
	// Each recursive call attempts the entire remaining pattern, so Goto and Seek
	// cannot commit an alternative. Scratch writes intentionally survive failure.
	pub fn exec(&mut self, save: &mut [Rva]) -> bool {
		let mut mask = 0xff;
		let mut extend = 0u32;
		while let Some(atom) = self.pat.get(self.pc).cloned() {
			self.pc += 1;
			match atom {
				pat::Atom::Byte(pat_byte) => {
					match self.pe.read::<u8>(self.cursor) {
						Some(byte) if byte & mask == pat_byte & mask => (),
						_ => return false,
					}
					mask = 0xff;
					self.cursor += 1;
				},
				pat::Atom::Save(slot) => {
					save_write(save, slot, self.cursor);
				},
				pat::Atom::Seek(slot) => {
					self.cursor = save_read(save, slot);
				},
				pat::Atom::Fuzzy(pat_mask) => {
					mask = pat_mask;
				},
				pat::Atom::Skip(skip) => {
					let skip = (extend << 8) + skip as u32;
					let skip = if skip == 0 { SKIP_VA } else { skip };
					let cursor = self.cursor.wrapping_add(skip);
					extend = 0;
					self.cursor = cursor;
				},
				pat::Atom::Rewind(back) => {
					let rewind = (extend << 8) + back as u32;
					let rewind = if rewind == 0 { SKIP_VA } else { rewind };
					let cursor = self.cursor.wrapping_sub(rewind);
					extend = 0;
					self.cursor = cursor;
				},
				pat::Atom::Extend(ext) => {
					extend = (extend << 8) + ext as u32;
				},
				pat::Atom::Scan(limit) => {
					let limit = (extend << 8) + limit as u32;
					return self.exec_scan(save, limit);
				},
				pat::Atom::Jump1 => {
					if let Some(sbyte) = self.pe.read::<i8>(self.cursor) {
						self.cursor = self.cursor.wrapping_add(sbyte as Rva).wrapping_add(1);
					}
					else {
						return false;
					}
				},
				pat::Atom::Jump4 => {
					if let Some(sdword) = self.pe.read::<i32>(self.cursor) {
						self.cursor = self.cursor.wrapping_add(sdword as Rva).wrapping_add(4);
					}
					else {
						return false;
					}
				},
				pat::Atom::Ptr => {
					if let Some(rva) = self.pe.read::<Va>(self.cursor).and_then(|va| self.pe.pointer(va)) {
						self.cursor = rva;
					}
					else {
						return false;
					}
				},
				pat::Atom::Pir(slot) => {
					if let Some(sdword) = self.pe.read::<i32>(self.cursor) {
						let base = save_read(save, slot);
						self.cursor = base.wrapping_add(sdword as Rva);
					}
					else {
						return false;
					}
				},
				pat::Atom::Check(slot) => {
					if save_read(save, slot) != self.cursor {
						return false;
					}
				},
				pat::Atom::IsAlign(align) => {
					if !self.cursor.aligned_to(1 << align as u32) {
						return false;
					}
				},
				pat::Atom::ReadU8(slot) | pat::Atom::TestU8(slot) => {
					if let Some(byte) = self.pe.read::<u8>(self.cursor) {
						if matches!(atom, pat::Atom::TestU8(_)) {
							if save_read(save, slot) != byte as Rva {
								return false;
							}
						}
						else {
							save_write(save, slot, byte as Rva);
						}
						self.cursor = self.cursor.wrapping_add(1);
					}
					else {
						return false;
					}
				},
				pat::Atom::ReadI8(slot) | pat::Atom::TestI8(slot) => {
					if let Some(sbyte) = self.pe.read::<i8>(self.cursor) {
						if matches!(atom, pat::Atom::TestI8(_)) {
							if save_read(save, slot) != sbyte as Rva {
								return false;
							}
						}
						else {
							save_write(save, slot, sbyte as Rva);
						}
						self.cursor = self.cursor.wrapping_add(1);
					}
					else {
						return false;
					}
				},
				pat::Atom::ReadU16(slot) | pat::Atom::TestU16(slot) => {
					if let Some(word) = self.pe.read::<u16>(self.cursor) {
						if matches!(atom, pat::Atom::TestU16(_)) {
							if save_read(save, slot) != word as Rva {
								return false;
							}
						}
						else {
							save_write(save, slot, word as Rva);
						}
						self.cursor = self.cursor.wrapping_add(2);
					}
					else {
						return false;
					}
				},
				pat::Atom::ReadI16(slot) | pat::Atom::TestI16(slot) => {
					if let Some(sword) = self.pe.read::<i16>(self.cursor) {
						if matches!(atom, pat::Atom::TestI16(_)) {
							if save_read(save, slot) != sword as Rva {
								return false;
							}
						}
						else {
							save_write(save, slot, sword as Rva);
						}
						self.cursor = self.cursor.wrapping_add(2);
					}
					else {
						return false;
					}
				},
				pat::Atom::ReadU32(slot) | pat::Atom::TestU32(slot) => {
					if let Some(dword) = self.pe.read::<Rva>(self.cursor) {
						if matches!(atom, pat::Atom::TestU32(_)) {
							if save_read(save, slot) != dword {
								return false;
							}
						}
						else {
							save_write(save, slot, dword);
						}
						self.cursor = self.cursor.wrapping_add(4);
					}
					else {
						return false;
					}
				},
				pat::Atom::Zero(slot) => {
					save_write(save, slot, 0);
				},
				pat::Atom::Fork(next) => {
					let pc = self.pc + ((extend << 8) + next as u32) as usize;
					let cursor = self.cursor;
					extend = 0;
					if self.exec(save) {
						return true;
					}
					self.pc = pc;
					self.cursor = cursor;
				},
				pat::Atom::Goto(next) => {
					self.pc += ((extend << 8) + next as u32) as usize;
					extend = 0;
				},
				pat::Atom::Nop => {},
			}
		}
		return true;
	}
	pub fn exec_scan(&mut self, save: &mut [Rva], limit: u32) -> bool {
		// Capture the current cursor and PC to restore while trying
		let cursor = self.cursor;
		let pc = self.pc;
		// Slice a section of bytes to limit the scan to
		let bytes = match self.pe.slice(cursor) {
			Some(bytes) if limit == 0 => bytes,
			Some(bytes) => &bytes[..cmp::min(limit.saturating_add(1) as usize, bytes.len())],
			None => return false,
		};
		// Only prefilter an immediate byte: skipping earlier instructions could omit
		// scratch writes that remain observable when an enclosing fork retries.
		if let Some(&pat::Atom::Byte(byte)) = self.pat.get(pc) {
			for i in 0..bytes.len() as u32 {
				if bytes[i as usize] == byte {
					self.cursor = cursor.wrapping_add(i);
					self.pc = pc;
					if self.exec(save) {
						return true;
					}
				}
			}
		}
		// Execute every candidate when the continuation does not start with a byte.
		else {
			for i in 0..bytes.len() as u32 {
				self.cursor = cursor.wrapping_add(i);
				self.pc = pc;
				if self.exec(save) {
					return true;
				}
			}
		}
		// No match found, exec fails
		return false;
	}
}

//----------------------------------------------------------------
