/*!
Source Engine is managed through "interface" singletons.

These singletons are initialized when the DLL is loaded and add themselves to a global linked list called [`s_pInterfaceRegs`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/tier1/interface.cpp#L46).

To access such a singleton you call the appropriate DLL's exported function called [`CreateInterface`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/tier1/interface.cpp#L91) with the name of the interface (which includes a version number).
The name is misleading as it implies and instance is created, instead a pointer is returned to the global singleton which implements this interface.

This only allows you to query interfaces which you know by name and version, this code extracts the entire list of interfaces exposed by a DLL by walking this list manually.
*/

use pelite;
use pelite::pattern as pat;
use pelite::pe32::{Pe, PeFile, Ptr, Rva, Va};
use pelite::{util::CStr, Pod};

//----------------------------------------------------------------

pub fn print(file: PeFile) {
    for reg in &interfaces(file).unwrap() {
        println!("{}!{:08X} {}", reg.dll_name, reg.offset, reg.name);
    }
}

//----------------------------------------------------------------

/// The interface registration as defined in the Valve Source SDK.
///
/// [`class InterfaceReg`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/public/tier1/interface.h#L72)
#[allow(unused)]
#[derive(Pod, Debug)]
#[repr(C)]
pub struct InterfaceReg {
    create_fn: Va,
    name: Ptr<CStr>,
    next: Ptr<InterfaceReg>,
}

//----------------------------------------------------------------

#[derive(Copy, Clone, Debug)]
pub struct Interface<'a> {
    pub dll_name: &'a str,
    pub name: &'a str,
    pub offset: Rva,
}

pub fn interfaces<'a>(file: PeFile<'a>) -> pelite::Result<Vec<Interface<'a>>> {
    let mut save = [0; 4];

    let exports = file.exports()?.by()?;
    let dll_name = exports.dll_name()?.to_str().unwrap();

    // Of course, this linked list isn't yet initialized!
    // Search for the code which constructs this linked list to extract their information
    let mut list = Vec::new();

    // Find the static initializers which register the interface
    // ```
    // push    offset aInterfaceName
    // push    offset create_fn
    // mov     ecx, offset interface_reg
    // call    InterfaceReg::InterfaceReg
    // retn
    // ```

    // Create fn returns the global singleton
    // ```
    // mov     eax, offset g_Interface
    // retn
    // ```
    let pat = pat!("68*{'} 68*{B8*'} B9???? E8${55 8BEC} C3");
    let mut matches = file.scanner().matches_code(pat);
    while matches.next(&mut save) {
        // Extract the interface information
        let name = file.derva_c_str(save[1]).unwrap().to_str().unwrap();
        let offset = save[2];
        list.push(Interface {
            dll_name,
            name,
            offset,
        });
    }

    Ok(list)
}
