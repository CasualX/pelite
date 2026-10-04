use super::*;

impl Analysis<'_> {
	/// Name TLS callbacks and the PE entry-point function.
	pub fn seed_entry_points(&mut self) {
		let callbacks = match self.pe.tls().and_then(|tls| tls.callbacks()) {
			Ok(callbacks) => Some(callbacks),
			Err(pelite::Error::Null) => None,
			Err(error) => {
				eprintln!("analysis: TLS callbacks: {error}");
				None
			},
		};
		if let Some(callbacks) = callbacks {
			let callbacks = match callbacks {
				Wrap::T32(callbacks) => Wrap::T32(callbacks.iter().map(|&va| u64::from(va))),
				Wrap::T64(callbacks) => Wrap::T64(callbacks.iter().copied()),
			};
			for (index, va) in callbacks.map(Wrap::into).enumerate() {
				let Ok(rva) = self.pe.va_to_rva(va) else { continue };
				self.add(rva, Some(ty::Type::Fn));
				if let Some(symbol) = self.symbols.get_mut(&rva) {
					symbol.name = factmap::SymbolName::Named(format!("TlsCallback_{index}"));
				}
			}
		}
		let entry = match self.pe.optional_header() {
			Wrap::T32(h) => h.AddressOfEntryPoint,
			Wrap::T64(h) => h.AddressOfEntryPoint,
		};
		if entry != 0 {
			self.add(entry, Some(ty::Type::Fn));
			if let Some(symbol) = self.symbols.get_mut(&entry) {
				symbol.name = factmap::SymbolName::Named("EntryPoint".into());
			}
		}
	}
}

#[test]
fn entry_point_and_tls_callbacks_keep_function_names() {
	for dll in ["Demo.dll", "Demo64.dll"] {
		let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../demo").join(dll);
		let map = pelite::FileMap::open(&path).unwrap();
		let pe = PeFile::from_bytes(&map).unwrap();
		let entry = match pe.optional_header() {
			Wrap::T32(h) => h.AddressOfEntryPoint,
			Wrap::T64(h) => h.AddressOfEntryPoint,
		};
		assert_ne!(entry, 0);
		let callbacks = match pe.tls().unwrap().callbacks().unwrap() {
			Wrap::T32(callbacks) => callbacks.iter().map(|&va| u64::from(va)).collect::<Vec<_>>(),
			Wrap::T64(callbacks) => callbacks.to_vec(),
		};
		assert!(!callbacks.is_empty());
		let mut analysis = Analysis::new(pe).unwrap();
		analysis.scan_exceptions();
		analysis.seed_entry_points();
		// Later weak passes must not downgrade established names or function types.
		analysis.scan_code();
		analysis.refine_labels();
		assert_eq!(analysis.symbols[&entry].name, factmap::SymbolName::Named("EntryPoint".into()));
		assert_eq!(analysis.symbols[&entry].ty, ty::Type::Fn);
		for (index, va) in callbacks.into_iter().enumerate() {
			let rva = pe.va_to_rva(va).unwrap();
			assert_eq!(analysis.symbols[&rva].name, factmap::SymbolName::Named(format!("TlsCallback_{index}")));
			assert_eq!(analysis.symbols[&rva].ty, ty::Type::Fn);
		}
	}
}
