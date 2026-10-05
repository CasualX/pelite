use super::*;

impl Analysis<'_> {
	/// Discover direct exports, classifying executable targets as code and applying names.
	pub fn seed_exports(&mut self) {
		if let Ok(exports) = self.pe.exports().and_then(|exports| exports.by()) {
			for export in exports.iter() {
				if let Some(rva) = export.ok().and_then(|export| export.symbol()) {
					let hint = self.executable(rva).then_some(ty::Type::Code);
					self.add(rva, hint);
				}
			}
			for (name, export) in exports.iter_names() {
				let Some(rva) = export.ok().and_then(|export| export.symbol()) else { continue };
				let Some(name) = name.ok().and_then(|name| name.to_str().ok()) else { continue };
				if let Some(symbol) = self.symbols.get_mut(&rva) {
					symbol.name = factmap::SymbolName::Named(name.to_owned());
				}
			}
		}
	}
}
