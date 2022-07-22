/*!
RecvTables for networking entity data.
*/

#![allow(bad_style)]

use std::mem;

use lde;
use pelite;
use pelite::pattern as pat;
use pelite::pe32::*;
use pelite::{util::CStr, Pod};

//----------------------------------------------------------------

pub fn print(client: PeFile) {
    let classes = recvtables(client).unwrap();

    tprint! {
        "### Recvtables\n\n"
        for cls in (&classes) {
            "<details>\n"
            "<summary><code>class "{cls.name}
            if let Some(base) = (cls.base) {
                " extends "{base}
            }
            "</code></summary>\n\n"
            "```\n"
            "{{\n"
            for prop in (&cls.props) {
                "\t"{prop.name}": "{prop.ty}",\n"
            }
            "}}\n"
            "```\n\n"
            "#### Offsets\n\n"
            "```\n"
            for prop in (&cls.props) {
                {cls.name}"!"{prop.offset;#06x}" "{prop.name}"\n"
            }
            "```\n"
            "</details>\n"
        }
        "\n"
    }
}

//----------------------------------------------------------------

#[derive(Pod, Debug)]
#[repr(C)]
struct RecvTable {
    pProps: Ptr<RecvProp>,
    nProps: i32,
    pDecoder: Va,
    pNetTableName: Ptr<CStr>,
    bInitialized: u8,
    bInMainList: u8,
    _pad: [u8; 2],
}
#[derive(Pod, Debug, Clone)]
#[repr(C)]
struct RecvProp {
    pVarName: Ptr<CStr>,
    RecvType: i32,
    Flags: i32,
    StringBufferSize: i32,
    bInsideArray: u8,
    _pad1: [u8; 3],
    pExtraData: Va,
    pArrayProp: Ptr<RecvProp>,
    ArrayLengthProxy: Va,
    ProxyFn: Va,
    DataTableProxyFn: Va,
    RecvTable: Ptr<RecvTable>,
    Offset: i32,
    ElementStride: i32,
    nElements: i32,
    pParentArrayPropName: Ptr<CStr>,
}

static PROP_TYPES: [&str; 8] = [
    "Int",
    "Float",
    "Vector",
    "VectorXY",
    "String",
    "Array",
    "DataTable",
    "Int64",
];

//----------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Class<'a> {
    pub name: &'a str,
    pub base: Option<&'a str>,
    pub props: Vec<Prop<'a>>,
}
#[derive(Copy, Clone, Debug)]
pub struct Prop<'a> {
    pub ty: &'a str,
    pub name: &'a str,
    pub offset: i32,
}

pub fn recvtables<'a>(client: PeFile<'a>) -> pelite::Result<Vec<Class<'a>>> {
    let mut save = [0; 8];
    let mut classes = Vec::new();

    // This pattern is quite the sight, isn't it?
    // To find one of these constructors, search a typical netvar and xref it.
    // `save[1]`: End of constructor code
    // `save[2]`: Address of first RecvProp of the RecvTable's props
    // `save[3]`: Number of RecvProps
    // `save[4]`: Name of the datatable
    // `save[5]`: Start of constructor code
    let pat1 = pat!("A1???? A801 0F85${'C705????*{'} C705????'???? C705???????? C705????*{'}} 83C801 'C705????00000000 A3");
    let mut matches1 = client.scanner().matches_code(pat1);
    while matches1.next(&mut save) {
        if let Ok(class) = recvtable(client, &save) {
            classes.push(class);
        }
    }

    // Variation of the above for DT_CSPlayer and others
    let pat2 = pat!("55 8BEC A1???? 83EC? A801 0F85${'C705????*{'} B801000000 C705????'???? C705???????? C705????*{'}} 83C801 'B9???? A3");
    let mut matches2 = client.scanner().matches_code(pat2);
    while matches2.next(&mut save) {
        if let Ok(class) = recvtable(client, &save) {
            classes.push(class);
        }
    }

    classes.sort_unstable_by_key(|cls| cls.name);
    Ok(classes)
}

fn recvtable<'a>(client: PeFile<'a>, save: &[Rva; 8]) -> pelite::Result<Class<'a>> {
    let props_rva = save[2];
    let code: &[u8] = client.derva_slice(save[5], (save[1] - save[5]) as usize)?;
    let &n_props: &i32 = client.derva(save[3])?;
    let net_table_name = client.derva_c_str(save[4])?.to_str().unwrap();

    // Allocate memory to initialize the props
    let mut recv_props = vec![unsafe { mem::zeroed::<RecvProp>() }; n_props as usize];
    let props_size = (n_props as usize * mem::size_of::<RecvProp>()) as u32;
    let props_ptr = recv_props.as_mut_ptr() as *mut u8;

    // Run through the code virtually executing only the relevant instructions initializing the RecvTable
    for (opcode, _) in lde::X86.iter(code, save[5]) {
        // mov dword ptr addr, imm32
        if opcode.starts_with(&[0xC7, 0x05]) {
            let rva = client.va_to_rva(opcode.read::<Va>(2)).unwrap();
            let imm = opcode.read::<u32>(6);
            if rva >= props_rva && rva - props_rva < props_size {
                unsafe {
                    *(props_ptr.offset((rva - props_rva) as isize) as *mut u32) = imm;
                }
            }
        }
        // mov byte ptr addr, imm8
        if opcode.starts_with(&[0xC6, 0x05]) {
            let rva = client.va_to_rva(opcode.read::<Va>(2)).unwrap();
            let imm = opcode.read::<u8>(6);
            if rva >= props_rva && rva - props_rva < props_size {
                unsafe {
                    *(props_ptr.offset((rva - props_rva) as isize) as *mut u8) = imm;
                }
            }
        }
    }

    let mut props = Vec::new();
    for recv_prop in &recv_props {
        if let Ok(name) = client
            .deref_c_str(recv_prop.pVarName)
            .and_then(|s| Ok(s.to_str()?))
        {
            let ty = *PROP_TYPES.get(recv_prop.RecvType as usize).unwrap_or(&"?");
            let offset = recv_prop.Offset;
            if name != "baseclass" {
                props.push(Prop { name, ty, offset });
            }
        }
    }
    props.sort_unstable_by_key(|prop| prop.offset);

    Ok(Class {
        base: None,
        name: net_table_name,
        props,
    })
}
