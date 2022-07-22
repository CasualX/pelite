/*!

Analyzes CInput::GetButtonBits to extract the global button input state and their values.

From the Source SDK 2013: https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/client/in_main.cpp#L1447

 */

use pelite::pattern as pat;
use pelite::pe32::*;
use pelite::Pod;

pub fn print(bin: PeFile<'_>, dll_name: &str) {
    let btns = buttons(bin);

    tprint! {
        "### Buttons\n\n```\n"
        for btn in (&btns) {
            {dll_name}"!"{btn.kbutton;#010x}" kbutton_t "{btn.name}"\n"
        }
        "```\n\n"
    }
}

#[derive(Copy, Clone, Pod, Debug)]
#[repr(C)]
struct kbutton_t {
    down: [i32; 2],
    state: i32,
}

struct Button<'a> {
    name: &'a str,
    kbutton: u32,
}

fn buttons<'a>(file: PeFile<'a>) -> Vec<Button<'a>> {
    let mut save = [0; 4];
    let mut btns = Vec::new();

    // Match the ConCommand in .data section...
    let section = file
        .section_headers()
        .iter()
        .find(|sect| &sect.Name == b".data\0\0\0")
        .unwrap();
    let mut matches = file.scanner().matches(
        pat!("@2 00000000 00000000 *{'\"+\"} *{} 00000000 *55 8BEC [8-80] B9*{'}"),
        section.virtual_range(),
    );
    while matches.next(&mut save) {
        let name = file.derva_c_str(save[1]).unwrap().to_str().unwrap();
        let kbutton = save[2];
        btns.push(Button { name, kbutton });
    }

    // Sort the list by name for improved diff viewer experience
    btns.sort_unstable_by_key(|cmd| cmd.name);
    return btns;
}
