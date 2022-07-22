/*!
The ClientClass links client and server entities.
*/

use pelite;
use pelite::pattern as pat;
use pelite::pe32::{Pe, PeFile, Ptr, Va};
use pelite::{util::CStr, Pod};

//----------------------------------------------------------------

pub fn print(client: PeFile, dll_name: &str) {
    let classes = classes(client);

    tprint! {
        "### ClientClasses\n\n"
        for cls in (&classes) {
            "<details>\n"
            "<summary><code>client_class "{cls.network_name}"</code></summary>\n\n"
            "sizeof: `"{cls.size_of}"`  \n"
            "</details>\n"
        }
        "\n```\n"
        for cls in (&classes) {
            {dll_name}"!"{cls.address;#010x}" ClientClass "{cls.network_name}"\n"
        }
        "```\n\n"
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
    pub address: u32,
    pub class_id: i32,
    pub size_of: u32,
}

pub fn classes<'a>(client: PeFile<'a>) -> Vec<Class<'a>> {
    let mut save = [0; 8];
    let mut list = Vec::new();

    // The ClientClasses aren't fully constructed yet, find these constructors
    // ```
    // mov     eax, g_pClientClassHead
    // mov     s_ClientClass.pNext, eax
    // mov     g_pClientClassHead, offset s_ClientClass
    // retn
    // ```
    let mut matches = client
        .scanner()
        .matches_code(pat!("@4 A1*{'} A3*{'} C705*{'}*{'???? ???? *{'}} C3"));
    while matches.next(&mut save) {
        // Remove false positives
        if save[1] != save[3] || save[2] != save[4] + 0x10 {
            continue;
        }
        // Now dealing with a ClientClass
        let address = save[4];
        let client_class: &ClientClass = client.derva(address).unwrap();
        let network_name = client
            .deref_c_str(client_class.pNetworkName)
            .unwrap()
            .to_str()
            .unwrap();
        // Figure out the size of the entity type:
        // The CreateFn is a function to create instances of this entity type, it allocates memory and thus includes its size
        let size_of = client
            .deref_copy::<u32>(client_class.pCreateFn.offset(39))
            .unwrap_or(0);
        // Class ids are initialized somewhere else...
        let class_id = 0;
        list.push(Class {
            network_name,
            address,
            class_id,
            size_of,
        })
    }

    list.sort_unstable_by_key(|cls| cls.network_name);
    return list;
}
