use super::*;

#[unsafe(export_name = "demangleSymbol")]
pub unsafe fn demangle_symbol(name: *const [u8]) {
	let bytes = unsafe { &*name };
	// These parsers can recurse on nested types from untrusted symbols.
	if bytes.len() > 4096 {
		return return_null();
	}
	let Ok(name) = str::from_utf8(bytes) else {
		return return_null();
	};
	let demangled = if name.starts_with('?') {
		msvc_demangler::demangle(name, msvc_demangler::DemangleFlags::llvm()).ok()
	}
	else {
		// An x86 COFF symbol may have an extra leading underscore.
		let name = if name.starts_with("__Z") { &name[1..] } else { name };
		cpp_demangle::Symbol::new(name.as_bytes()).ok().and_then(|symbol| symbol.demangle().ok())
	};
	match demangled {
		Some(name) => return_str(&name),
		None => return_null(),
	}
}
