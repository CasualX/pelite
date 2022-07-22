/*!
The ClientClass links client and server entities.
*/

use pelite;
use pelite::pattern as pat;
use pelite::pe32::{Pe, PeFile, Ptr, Va};
use pelite::{util::CStr, Pod};

//----------------------------------------------------------------

pub fn print(client: PeFile) {
    for class in &classes(client).unwrap() {
        println!("{:?}", class);
    }
}

//----------------------------------------------------------------

#[allow(non_snake_case)]
#[derive(Pod, Debug)]
#[repr(C)]
struct ClientClass {
    pCreateFn: Ptr,
    pCreateEventFn: Ptr,
    pNetworkName: Ptr<CStr>,
    pRecvTable: Va,
    pNext: Ptr<ClientClass>,
    ClassID: i32,
}

//----------------------------------------------------------------

#[derive(Debug)]
pub struct Class<'a> {
    pub network_name: &'a str,
    pub class_id: i32,
    pub size_of: u32,
}

pub fn classes<'a>(client: PeFile<'a>) -> pelite::Result<Vec<Class<'a>>> {
    let mut save = [0; 8];
    let mut list = Vec::new();

    // The ClientClasses aren't fully constructed yet, find these constructors
    // ```
    // mov     eax, g_pClientClassHead
    // mov     s_ClientClass.pNext, eax
    // mov     g_pClientClassHead, offset s_ClientClass
    // retn
    // ```
    let pat = pat!("A1*{'} A3*{'} C705*{'}*{'???? ???? *{'}} C3");
    let mut matches = client.scanner().matches_code(pat);
    while matches.next(&mut save) {
        // Remove false positives
        if save[1] != save[3] || save[2] != save[4] + 0x10 {
            continue;
        }
        // Now dealing with a ClientClass
        let client_class: &ClientClass = client.derva(save[4]).unwrap();
        let network_name = client
            .deref_c_str(client_class.pNetworkName)
            .unwrap()
            .to_str()
            .unwrap();
        // Figure out the size of the entity type:
        // The CreateFn is a function to create instances of this entity type
        // It allocates memory and thus necessarily must reference its size
        let size_of = match client.deref_copy(client_class.pCreateFn.offset(4)) {
            Ok(0x68_u8) => client
                .deref_copy::<u32>(client_class.pCreateFn.offset(5))
                .unwrap_or(0),
            Ok(0x6A_u8) => client
                .deref_copy::<u8>(client_class.pCreateFn.offset(5))
                .unwrap_or(0) as u32,
            _ => 0,
        };
        // Class ids are initialized somewhere else...
        let class_id = 0;
        list.push(Class {
            network_name,
            class_id,
            size_of,
        })
    }

    Ok(list)
}
