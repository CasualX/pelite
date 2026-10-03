use super::*;

impl Analysis<'_> {
	/// Discover exported symbol addresses without guessing their types.
	pub fn seed_exports(&mut self) {
		if let Ok(exports) = self.pe.exports().and_then(|exports| exports.by()) {
			for export in exports.iter() {
				if let Some(rva) = export.ok().and_then(|export| export.symbol()) {
					self.add(rva, None);
				}
			}
		}
	}
}
