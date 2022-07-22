/*!
Source Engine is managed through "interface" singletons.

These singletons are initialized when the DLL is loaded and add themselves to a global linked list called [`s_pInterfaceRegs`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/tier1/interface.cpp#L46).

To access such a singleton you call the appropriate DLL's exported function called [`CreateInterface`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/tier1/interface.cpp#L91) with the name of the interface (which includes a version number).
The name is misleading as it implies and instance is created, instead a pointer is returned to the global singleton which implements this interface.

This only allows you to query interfaces which you know by name and version, this code extracts the entire list of interfaces exposed by a DLL by walking this list manually.
*/

use pelite;
use pelite::pattern as pat;
use pelite::pe32::*;
use pelite::{util::CStr, Pod};

//----------------------------------------------------------------

pub fn print(file: PeFile, dll_name: &str) {
    let ifaces = interfaces(file);

    tprint! {
        "### Interfaces\n\n"
        "```\n"
        for iface in (&ifaces) {
            {dll_name}"!"{iface.address;#010x}" "{iface.name}"\n"
        }
        "```\n\n"
    }
}

//----------------------------------------------------------------

/// The interface registration as defined in the Valve Source SDK.
///
/// [`class InterfaceReg`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/public/tier1/interface.h#L72)
#[allow(dead_code)]
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
    pub name: &'a str,
    pub address: u32,
}

pub fn interfaces<'a>(file: PeFile<'a>) -> Vec<Interface<'a>> {
    let mut save = [0; 8];

    // Grab the CreateInterface export
    let exports = file.exports().unwrap().by().unwrap();
    let create_interface_fn = exports.name("CreateInterface").unwrap().symbol().unwrap();

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
        file.scanner().exec(
            create_interface_fn,
            pat!("55 8BEC 5D E9$ 55 8BEC 56 8B35*{'}"),
            &mut save,
        );
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
    let mut matches = file
        .scanner()
        .matches_code(pat!("A1*{'} A3???? C705*{'}*{*{B8*'} *'} C3"));
    while matches.next(&mut save) {
        // Reject false positive matches for the signature
        if save[1] != s_pInterfaceRegs || save[2] != s_pInterfaceRegs {
            continue;
        }

        // Extract the interface information
        let name = file.derva_c_str(save[4]).unwrap().to_str().unwrap();
        let address = save[3];
        list.push(Interface { name, address });
    }

    // Sort the list by name for improved diff viewer experience
    list.sort_unstable_by_key(|iface| iface.name);
    return list;
}
