use super::*;
use pelite::Import;

/// Name TLS callbacks and the PE entry-point function.
pub fn seed_entry_points(input: &AnalysisInput<'_>, output: &mut AnalysisOutput) {
	let callbacks = match input.pe.tls().and_then(|tls| tls.callbacks()) {
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
			let Ok(rva) = input.pe.va_to_rva(va) else { continue };
			output.add(input, rva, Some(ty::Type::Fn));
			if let Some(symbol) = output.symbols.get_mut(&rva) {
				symbol.name = factmap::SymbolName::Named(format!("TlsCallback_{index}"));
			}
		}
	}
	let entry = match input.pe.optional_header() {
		Wrap::T32(h) => h.AddressOfEntryPoint,
		Wrap::T64(h) => h.AddressOfEntryPoint,
	};
	if entry != 0 {
		output.add(input, entry, Some(ty::Type::Fn));
		if let Some(symbol) = output.symbols.get_mut(&entry) {
			symbol.name = factmap::SymbolName::Named("EntryPoint".into());
		}
	}
}


/// Annotate x64 unwind coverage starts with their runtime function record RVAs.
pub fn scan_exceptions(input: &AnalysisInput<'_>, output: &mut AnalysisOutput) {
	if let Wrap::T64(file) = input.pe {
		match file.exception_x64() {
			Ok(exceptions) => {
				for function in exceptions.image() {
					let rva = function.BeginAddress;
					let Ok(runtime_function) = input.pe.headers().file_offset_to_rva(input.pe.offset_of(function)) else {
						continue
					};
					let comment = format!("RUNTIME_FUNCTION at {runtime_function:#x}");
					output.comments.insert(rva, factmap::CommentFact { rva, comment });
				}
			},
			Err(pelite::Error::Null) => {},
			Err(error) => eprintln!("analysis: x64 exceptions: {error}"),
		}
	}
}

/// Discover direct exports, classifying executable targets as code and applying names.
pub fn seed_exports(input: &AnalysisInput<'_>, output: &mut AnalysisOutput) {
	if let Ok(exports) = input.pe.exports().and_then(|exports| exports.by()) {
		for export in exports.iter() {
			if let Some(rva) = export.ok().and_then(|export| export.symbol()) {
				let hint = input.executable(rva).then_some(ty::Type::Code);
				output.add(input, rva, hint);
			}
		}
		for (name, export) in exports.iter_names() {
			let Some(rva) = export.ok().and_then(|export| export.symbol()) else { continue };
			let Some(name) = name.ok().and_then(|name| name.to_str().ok()) else { continue };
			if let Some(symbol) = output.symbols.get_mut(&rva) {
				symbol.name = factmap::SymbolName::Named(name.to_owned());
			}
		}
	}
}

/// Label IAT slots as pointer-sized integers, replacing weaker guesses.
/// Their values may refer to other modules rather than this PE image.
pub fn label_imports(input: &AnalysisInput<'_>, output: &mut AnalysisOutput) {
	let width = ty::PointerWidth::from(input.pe);
	for (rva, name) in import_names(input.pe, input.bitness) {
		if input.mapped(rva) {
			output.symbols.insert(rva, factmap::SymbolFact::new(
				rva, width.unsigned(), factmap::SymbolName::Named(format!("__imp_{name}")),
			));
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

/// Discover addresses stored in base relocation fields.
pub fn scan_relocations(input: &AnalysisInput<'_>, output: &mut AnalysisOutput) {
	match input.pe.base_relocs() {
		Ok(relocs) => relocs.for_each(|rva, ty| {
			let width = match (input.bitness, ty) {
				(32, image::IMAGE_REL_BASED_HIGHLOW) => 4,
				(64, image::IMAGE_REL_BASED_DIR64) => 8,
				_ => return,
			};
			// Relocation fields inside instructions need not be naturally aligned.
			let Ok(bytes) = input.pe.slice(rva, width, 1) else { return };
			let va = if width == 4 {
				u32::from_le_bytes(bytes[..4].try_into().unwrap()) as u64
			}
			else {
				u64::from_le_bytes(bytes[..8].try_into().unwrap())
			};
			output.add_va(input, va, None);
		}),
		Err(pelite::Error::Null) => {},
		Err(error) => eprintln!("analysis: relocations: {error}"),
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
		let input = AnalysisInput::new(pe).unwrap();
		let mut analysis = AnalysisOutput::default();
		metadata::scan_exceptions(&input, &mut analysis);
		metadata::seed_entry_points(&input, &mut analysis);
		// Later weak passes must not downgrade established names or function types.
		disassembly::scan_code(&input, &mut analysis);
		labels::refine_labels(&input, &mut analysis);
		assert_eq!(analysis.symbols[&entry].name, factmap::SymbolName::Named("EntryPoint".into()));
		assert_eq!(analysis.symbols[&entry].ty, ty::Type::Fn);
		for (index, va) in callbacks.into_iter().enumerate() {
			let rva = pe.va_to_rva(va).unwrap();
			assert_eq!(analysis.symbols[&rva].name, factmap::SymbolName::Named(format!("TlsCallback_{index}")));
			assert_eq!(analysis.symbols[&rva].ty, ty::Type::Fn);
		}
	}
}
