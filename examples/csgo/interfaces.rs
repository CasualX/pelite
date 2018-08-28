/*!
Source Engine is managed through "interface" singletons.

These singletons are initialized when the DLL is loaded and add themselves to a global linked list called [`s_pInterfaceRegs`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/tier1/interface.cpp#L46).

To access such a singleton you call the appropriate DLL's exported function called [`CreateInterface`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/tier1/interface.cpp#L91) with the name of the interface (which includes a version number).
The name is misleading as it implies and instance is created, instead a pointer is returned to the global singleton which implements this interface.

This only allows you to query interfaces which you know by name and version, this code extracts the entire list of interfaces exposed by a DLL by walking this list manually.
*/

use pelite;
use pelite::pattern as pat;
use pelite::pe32::{Rva, Va, Ptr, Pe, PeFile};
use pelite::pe32::exports::Export;
use pelite::util::{CStr, Pod};

//----------------------------------------------------------------

pub fn print(file: PeFile) {
	for reg in interfaces(file).unwrap() {
		println!("{}!{:08X} {}", reg.dll_name, reg.offset, reg.name);
	}
}

//----------------------------------------------------------------

/// The interface registration as defined in the Valve Source SDK.
///
/// [`class InterfaceReg`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/public/tier1/interface.h#L72)
#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
pub struct InterfaceReg {
	create_fn: Va,
	name: Ptr<CStr>,
	next: Ptr<InterfaceReg>,
}
unsafe impl Pod for InterfaceReg {}

//----------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
pub struct Interface<'a> {
	pub dll_name: &'a str,
	pub name: &'a str,
	pub offset: Rva,
}

pub fn interfaces<'a>(file: PeFile<'a>) -> pelite::Result<Vec<Interface<'a>>> {
	let mut save = [0; 8];

	let exports = file.exports()?.by()?;
	let dll_name = exports.dll_name()?.to_str().unwrap();

	// Grab the CreateInterface export
	let create_interface_export = exports.name("CreateInterface")?;
	let create_interface_fn = match create_interface_export {
		Export::Symbol(&rva) => rva,
		_ => return Err(pelite::Error::Null),
	};

	// Grab the linked list of interface registrations
	#[allow(non_snake_case)]
	let s_pInterfaceRegs = {
		// push    ebp
		// mov     ebp, esp
		// pop     ebp
		// jmp     CreateInterfaceInternal
		// ... ... ...
		// push    ebp
		// mov     ebp, esp
		// push    esi
		// mov     esi, s_pInterfaceRegs
		let pat = pat::parse("55 8BEC 5D E9$ 55 8BEC 56 8B35*{'}").unwrap();
		file.scanner().exec(create_interface_fn, &pat, &mut save);
		save[1]
	};

	// Of course, this linked list isn't yet initialized!
	// Search for the code which constructs this linked list to extract their information
	let mut list = Vec::new();

	// Inlined InterfaceReg constructor for this interface
	// ```
	// mov     eax, s_pInterfaceRegs
	// mov     s_InterfaceReg.next, eax
	// mov     s_pInterfaceRegs, offset s_InterfaceReg
	// retn
	// ```

	// Create_fn returns the global singleton
	// ```
	// mov     eax, offset s_Interface
	// retn
	// ```
	let pat = pat::parse("A1*{'} A3???? C705*{'}*{*{B8*'} *'} C3").unwrap();
	let mut matches = file.scanner().matches_code(&pat);
	while matches.next(&mut save) {
		// Reject false positive matches for the signature
		if save[1] != s_pInterfaceRegs || save[2] != s_pInterfaceRegs {
			continue;
		}

		// Extract the interface information
		let offset = save[3];
		let name = file.derva_c_str(save[4]).unwrap().to_str().unwrap();
		list.push(Interface { dll_name, name, offset });
	}

	Ok(list)
}
