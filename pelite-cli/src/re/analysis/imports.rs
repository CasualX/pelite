use super::*;
use pelite::Import;

impl Analysis<'_> {
	/// Label IAT slots as pointer-sized integers, replacing weaker guesses.
	/// Their values may refer to other modules rather than this PE image.
	pub fn label_imports(&mut self) {
		let width = ty::PointerWidth::from(self.pe);
		for (rva, name) in import_names(self.pe, self.bitness) {
			if self.mapped(rva) {
				self.symbols.insert(rva, factmap::SymbolFact::new(
					rva, width.unsigned(), factmap::SymbolName::Named(format!("__imp_{name}")),
				));
			}
		}
	}
}

pub(super) fn import_names(pe: PeFile<'_>, bitness: u32) -> HashMap<u32, String> {
	let mut names = HashMap::new();
	let Ok(imports) = pe.imports() else { return names };
	for descriptor in imports {
		let dll = descriptor.dll_name().ok().and_then(|name| name.to_str().ok());
		if descriptor.image().FirstThunk.get() == 0 { continue; }
		let Ok(imports) = descriptor.int() else { continue };
		for (index, import) in imports.enumerate() {
			let Some(rva) = u32::try_from(index).ok()
				.and_then(|index| index.checked_mul(bitness / 8))
				.and_then(|offset| descriptor.image().FirstThunk.get().checked_add(offset)) else { continue };
			let name = match import {
				Ok(Import::ByName { name, .. }) => {
					let Ok(name) = name.to_str() else { continue };
					name.to_owned()
				},
				Ok(Import::ByOrdinal { ord }) => {
					let Some(dll) = dll else { continue };
					format!("{dll}!#{ord}")
				},
				Err(_) => continue,
			};
			names.insert(rva, name);
		}
	}
	names
}
