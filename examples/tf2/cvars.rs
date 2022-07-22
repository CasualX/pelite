/*!
Console Variables.
*/

#![allow(unused)]

use pelite;
use pelite::pattern as pat;
use pelite::pe32::{Pe, PeFile, Rva};

//----------------------------------------------------------------

pub fn print(file: PeFile) {
    for cvar in &cvars(file).unwrap() {
        println!(
            "{}!{:08X} {:08X} {}",
            cvar.dll_name, cvar.offset, cvar.flags, cvar.name
        );
    }
}

//----------------------------------------------------------------

pub struct ConVar<'a> {
    dll_name: &'a str,
    name: &'a str,
    desc: Option<&'a str>,
    default_string: &'a str,
    offset: Rva,
    flags: u32,
    min_value: Option<f32>,
    max_value: Option<f32>,
}

pub fn cvars<'a>(file: PeFile<'a>) -> pelite::Result<Vec<ConVar<'a>>> {
    let mut save = [0; 8];
    let dll_name = file.exports()?.dll_name()?.to_str().unwrap();
    let scanner = file.scanner();
    let mut cvars = Vec::new();

    // Match static constructors which call [`ConVar::Create`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/public/tier1/convar.h#L383)
    // This method is far from perfect. The linked list is created at runtime through a bunch of variations.

    // Variant: ConVar with description and without min/max values
    let pat3 = pat!("CC 68*{'} 68'???? 68*{'} 68*{'} B9*{'} E8$");
    let mut matches = file.scanner().matches_code(pat3);
    while matches.next(&mut save) {
        let min_value = None;
        let max_value = None;
        let desc = Some(file.derva_c_str(save[1]).unwrap().to_str().unwrap());
        let flags = file.derva_copy(save[2]).unwrap();
        let default_string = file.derva_c_str(save[3]).unwrap().to_str().unwrap();
        let name = file.derva_c_str(save[4]).unwrap().to_str().unwrap();
        let offset = save[5];
        cvars.push(ConVar {
            dll_name,
            name,
            desc,
            default_string,
            offset,
            flags,
            min_value,
            max_value,
        });
    }

    // Variant: ConVar with description and with min/max values
    let pat4 = pat!(
        "D905*{'} 51 D91C24 D905*{'} 6A01 51 D91C24 6A01 68*{'} 68'???? 68*{'} 68*{'} B9*{'} E8$"
    );
    let mut matches = file.scanner().matches_code(pat4);
    while matches.next(&mut save) {
        let max_value = Some(file.derva_copy(save[1]).unwrap());
        let min_value = Some(file.derva_copy(save[2]).unwrap());
        let desc = Some(file.derva_c_str(save[3]).unwrap().to_str().unwrap());
        let flags = file.derva_copy(save[4]).unwrap();
        let default_string = file.derva_c_str(save[5]).unwrap().to_str().unwrap();
        let name = file.derva_c_str(save[6]).unwrap().to_str().unwrap();
        let offset = save[7];
        cvars.push(ConVar {
            dll_name,
            name,
            desc,
            default_string,
            offset,
            flags,
            min_value,
            max_value,
        });
    }

    Ok(cvars)
}
