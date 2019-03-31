/*!

Analyzes CInput::GetButtonBits to extract the global button input state and their values.

From the Source SDK 2013: https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/game/client/in_main.cpp#L1447

 */

use pelite::Pod;
use pelite::pattern as pat;
use pelite::pe32::*;

#[derive(Copy, Clone, Pod, Debug)]
#[repr(C)]
struct kbutton_t {
	down: [i32; 2],
	state: i32,
}

pub fn print(file: PeFile<'_>) {
	for cmd in &cmds(file) {
		println!("client.dll!{:08x} kbutton_t {}", cmd.kbutton, cmd.name);
	}
}

struct Cmd<'a> {
	name: &'a str,
	kbutton: u32,
}
fn cmds<'a>(file: PeFile<'a>) -> Vec<Cmd<'a>> {
	// Match the ConCommand in .data section...
	let mut cmds = Vec::new();
	let pat = pat!("00000000 00000000 *{'\"+\"} *{} 00000000 *55 8BEC [8-80] B9*{'}");
	let section = file.section_headers().iter().find(|sect| &sect.Name == b".data\0\0\0").unwrap();
	let mut save = [0; 4];
	let mut matches = file.scanner().matches_section(pat, section);
	while matches.next(&mut save) {
		let name = file.derva_c_str(save[1]).unwrap().to_str().unwrap();
		let kbutton = save[2];
		cmds.push(Cmd { name, kbutton });
	}
	cmds.sort_unstable_by_key(|cmd| cmd.name);
	cmds
}
