/*!
*/

#![allow(unused)]

use pelite;
use pelite::pattern as pat;
use pelite::pe32::*;
use pelite::{util::CStr, Pod};

//----------------------------------------------------------------

pub fn print(bin: PeFile, dll_name: &str) {
    let cvars = convars(bin);
    let cmds = concommands(bin);

    tprint! {
        "### ConVars\n\n"
        for cvar in (&cvars) {
            "<details>\n"
            "<summary><code>"{cvar.name}"</code></summary>\n\n"
            if let Some(desc) = (cvar.desc) {
                {desc}"\n\n"
            }
            "default: `"{cvar.default;?}"`  \n"
            "flags: `"{cvar.flags;#x}"`  \n"
            if let Some(min_value) = (cvar.min_value) {
                "min value: `"{min_value}"`  \n"
            }
            if let Some(max_value) = (cvar.max_value) {
                "max value: `"{max_value}"`  \n"
            }
            "</details>\n"
        }
        "\n#### Addresses\n\n"
        "```\n"
        for cvar in (&cvars) {
            {dll_name}"!"{cvar.address;#010x}" ConVar "{cvar.name}"\n"
        }
        "```\n\n"

        "### ConCommands\n\n"
        for cmd in (&cmds) {
            "<details>\n"
            "<summary><code>"{cmd.name}"</code></summary>\n\n"
            if let Some(desc) = (cmd.desc) {
                {desc}"\n\n"
            }
            "flags: `"{cmd.flags;#x}"`  \n"
            "</details>\n"
        }
        "\n#### Addresses\n\n"
        "```\n"
        for cmd in (&cmds) {
            {dll_name}"!"{cmd.address;#010x}" ConCommand "{cmd.name}"\n"
        }
        "```\n\n"
    }
}

//----------------------------------------------------------------

pub struct ConVar<'a> {
    address: u32,
    name: &'a str,
    desc: Option<&'a str>,
    default: &'a str,
    flags: u32,
    min_value: Option<f32>,
    max_value: Option<f32>,
}

pub fn convars<'a>(file: PeFile<'a>) -> Vec<ConVar<'a>> {
    let mut save = [0; 12];
    let mut cvars = Vec::new();

    // Match static constructors which call [`ConVar::Create`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/public/tier1/convar.h#L383)
    // This signature is quite a beast, let's analyze it in detail:
    // 6A00     push 0                  ; The change callback parameter, assumed to be null and skipped
    // 51       push ecx                ; Allocates space for the max float value
    // C70424u4 mov [esp], max_float    ; Writes the fMax argument
    // 6Au1     push has_max            ; Pushes the bMax argument
    // 51       push ecx                ; Allocates space for the min float value
    // C70424u4 mov [esp], min_float    ; Writes the fMin argument
    // B9*{'}   mov ecx, offset address ; Address of the global ConVar instance
    // 6Au1     push has_min            ; Pushes the bMin argument
    // Here comes one of two cases:
    // 6A00     push nullptr            ; Pushes the pHelpString argument
    // If there's a help string:
    // 68*{'}   push help_string        ; Pushes the pHelpString argument
    // Continue here:
    // 68u4     push flags              ; Pushes the flags argument
    // 68*{'}   push default            ; Pushes the pDefaultValue argument
    // 68*{'}   push name               ; Pushes the pName argument
    // E8$      call create_fn          ; Calls the ConVar::Create factory method
    let pat =
        pat!("6A00 51 C70424u4 6Au1 51 C70424u4 B9*{'} 6Au1 (6A00|68*{'}) 68u4 68*{'} 68*{'} E8$");

    let mut matches = file.scanner().matches_code(pat);
    while matches.next(&mut save) {
        if let Ok(cvar) = convar(file, &save) {
            cvars.push(cvar);
        } else {
            eprintln!("Convar false-positive {:#010x}", save[0]);
        }
        save = [0; 12];
    }

    // Sort the list by name for improved diff viewer experience
    cvars.sort_unstable_by_key(|cvar| cvar.name);
    return cvars;
}
fn convar<'a>(bin: PeFile<'a>, save: &[u32; 12]) -> pelite::Result<ConVar<'a>> {
    let has_max = save[2] != 0;
    let max_float = f32::from_bits(save[1]);
    let max_value = if has_max { Some(max_float) } else { None };
    let has_min = save[5] != 0;
    let min_float = f32::from_bits(save[3]);
    let min_value = if has_min { Some(min_float) } else { None };
    let address = save[4];
    let desc = if save[6] == 0 {
        None
    } else {
        Some(bin.derva_c_str(save[6])?.to_str().unwrap())
    };
    let flags = save[7];
    let default = bin.derva_c_str(save[8])?.to_str().unwrap();
    let name = bin.derva_c_str(save[9])?.to_str().unwrap();
    Ok(ConVar {
        address,
        name,
        desc,
        default,
        flags,
        min_value,
        max_value,
    })
}

//----------------------------------------------------------------

#[allow(non_snake_case)]
#[derive(Pod, Debug)]
#[repr(C)]
pub struct RawConCommand {
    // ConCommandBase
    pub vtable: Ptr<[Ptr]>,
    pub pNext: Ptr,
    pub bRegistered: u8,
    _pad: [u8; 3],
    pub pszName: Ptr<CStr>,
    pub pszHelpString: Ptr<CStr>,
    pub fFlags: u32,
    // ConCommand
    pub fnCommandCallback: Ptr,
    pub fnCompletionCallback: Ptr,
    pub fnCommandType: u32,
}

pub struct ConCommand<'a> {
    pub address: u32,
    pub name: &'a str,
    pub desc: Option<&'a str>,
    pub flags: u32,
    pub callback: u32,
}

pub fn concommands<'a>(bin: PeFile<'a>) -> Vec<ConCommand<'a>> {
    let mut save = [0; 8];
    let mut cmds = Vec::new();

    // The ConCommand constructors are already evaluated by the compiler and the global data structures already filled in
    // Perform a fairly slow signature scan for these instances...
    let data_section = bin
        .section_headers()
        .iter()
        .find(|sect| &sect.Name == b".data\0\0\0")
        .unwrap();
    let mut matches = bin.scanner().matches(
        pat!("@2 *{*{}*{}*{}*} 00000000 00000000 *{'} (*{'}|00000000) u4 *{'}"),
        data_section.virtual_range(),
    );
    while matches.next(&mut save) {
        // Filter false-positives...
        if let Ok(cmd) = concommand(bin, &save) {
            cmds.push(cmd);
        }
        save = [0; 8];
    }

    // Sort the list by name for improved diff viewer experience
    cmds.sort_unstable_by_key(|cmd| cmd.name);
    return cmds;
}
fn concommand<'a>(bin: PeFile<'a>, save: &[u32; 8]) -> pelite::Result<ConCommand<'a>> {
    let address = save[0];
    let name = bin.derva_c_str(save[1])?.to_str()?;
    let desc = if save[2] == 0 {
        None
    } else {
        Some(bin.derva_c_str(save[2])?.to_str()?)
    };
    let flags = save[3];
    let callback = save[4];
    Ok(ConCommand {
        address,
        name,
        desc,
        flags,
        callback,
    })
}
