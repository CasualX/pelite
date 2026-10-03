use super::*;

impl Analysis<'_> {
	/// Discover addresses stored in base relocation fields.
	pub fn scan_relocations(&mut self) {
		match self.pe.base_relocs() {
			Ok(relocs) => relocs.for_each(|rva, ty| {
				let width = match (self.bitness, ty) {
					(32, image::IMAGE_REL_BASED_HIGHLOW) => 4,
					(64, image::IMAGE_REL_BASED_DIR64) => 8,
					_ => return,
				};
				// Relocation fields inside instructions need not be naturally aligned.
				let Ok(bytes) = self.pe.slice(rva, width, 1) else { return };
				let va = if width == 4 {
					u32::from_le_bytes(bytes[..4].try_into().unwrap()) as u64
				}
				else {
					u64::from_le_bytes(bytes[..8].try_into().unwrap())
				};
				self.add_va(va, None);
			}),
			Err(pelite::Error::Null) => {},
			Err(error) => eprintln!("analysis: relocations: {error}"),
		}
	}
}
