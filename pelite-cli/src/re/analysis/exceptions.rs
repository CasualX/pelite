use super::*;

impl Analysis<'_> {
	/// Mark x64 runtime function starts as functions, overriding weaker labels.
	pub fn scan_exceptions(&mut self) {
		if let Wrap::T64(file) = self.pe {
			match file.exception_x64() {
				Ok(exceptions) => {
					for function in exceptions.image() {
						let rva = function.BeginAddress;
						self.add(rva, Some(ty::Type::Code));
						if let Some(symbol) = self.symbols.get_mut(&rva) {
							symbol.name = factmap::SymbolName::Fn;
						}
					}
				},
				Err(pelite::Error::Null) => {},
				Err(error) => eprintln!("analysis: x64 exceptions: {error}"),
			}
		}
	}
}
