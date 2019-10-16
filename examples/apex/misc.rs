use pelite;
use pelite::pe64::*;
use pelite::pattern as pat;

pub fn print(bin: PeFile<'_>, dll_name: &str) {
	println!("## Miscellaneous\n\n```");
	header(bin);
	game_version(bin);
	entity_list(bin, dll_name);
	local_entity_handle(bin, dll_name);
	global_vars(bin, dll_name);
	player_resource(bin, dll_name);
	view_render(bin, dll_name);
	client_state(bin, dll_name);
	println!("```\n");
}

fn header(bin: PeFile<'_>) {
	// Check if offsets are correct
	println!("TimeDateStamp = {:#x}", bin.file_header().TimeDateStamp);
	println!("CheckSum = {:#x}", bin.optional_header().CheckSum);
}

fn entity_list(bin: PeFile<'_>, dll_name: &str) {
	// EntityList
	//
	// Find GetEntityByIndex:
	// "Index must be less than %i.\n"
	//
	// entity_ptr = *(uintptr_t*)(entity_list + index * 32)
	let mut save = [0; 4];
	if bin.scanner().finds_code(pat!("81F9u4 7C% 85C9 79% 4863C1 488D15$'"), &mut save) {
		let num_ent_entries = save[1];
		let cl_entitylist = save[2];
		println!("NUM_ENT_ENTRIES = {:#x}", num_ent_entries);
		println!("{}!{:#x} cl_entitylist", dll_name, cl_entitylist);
	}
	else {
		eprintln!("unable to find cl_entitylist!");
	}
}

fn local_entity_handle(bin: PeFile<'_>, dll_name: &str) {
	let mut save = [0; 4];
	if bin.scanner().finds_code(pat!("833D${?'}FF 74? 0FB70D${'} 0FB705${'}"), &mut save) {
		let local_entity_handle = save[1];
		println!("{}!{:#x} LocalEntityHandle", dll_name, local_entity_handle);
	}
	else {
		eprintln!("unable to find LocalEntityHandle!");
	}
}

fn global_vars(bin: PeFile<'_>, dll_name: &str) {
	// Right above "Client.dll Init_PostVideo() in library "
	// lea r8, qword_XXX
	let mut save = [0; 4];
	if bin.scanner().finds_code(pat!("488B01 4C8D05${'} [17] $\"Client.dll Init_PostVideo\""), &mut save) {
		let global_vars = save[1];
		println!("{}!{:#x} GlobalVars", dll_name, global_vars);
	}
	else {
		eprintln!("unable to find GlobalVars!");
	}
}

fn player_resource(bin: PeFile<'_>, dll_name: &str) {
	// References "#UNCONNECTED_PLAYER_NAME" and the C_PlayerResource vtable
	// At the very end of the constructor assigns this to global variable
	let mut save = [0; 4];
	if bin.scanner().finds_code(pat!("488B6C24? 488BC3 48891D$'"), &mut save) {
		let player_resource = save[1];
		println!("{}!{:#x} PlayerResource", dll_name, player_resource);
	}
	else {
		eprintln!("unable to find PlayerResource!");
	}
}

fn game_version(bin: PeFile<'_>) {
	// References "gameversion.txt"
	let mut save = [0; 4];
	if bin.scanner().finds_code(pat!("488D1D${'} C605????01 488BD3 488D0D$\"gameversion.txt\"00"), &mut save) {
		let game_version = bin.derva_c_str(save[1]).unwrap().to_str().unwrap();
		println!("GameVersion = {:?}", game_version);
	}
	else {
		eprintln!("unable to find GameVersion!");
	}
}

fn view_render(bin: PeFile<'_>, dll_name: &str) {
	let mut save = [0; 4];
	if bin.scanner().finds_code(pat!("74 34 48 8B 0D ${'} 40 0F B6 D7"), &mut save) {
		let view_render = save[1];
		println!("{}!{:#x} ViewRender", dll_name, view_render);
	}
	else {
		eprintln!("unable to find ViewRender");
	}
}

fn client_state(bin: PeFile<'_>, dll_name: &str) {
	let mut save = [0; 4];
	// Address of global CClientState instance
	if bin.scanner().finds_code(pat!("488D15${\"Missing client class\"} 488D0D$'"), &mut save) {
		let client_state = save[1];
		println!("{}!{:#x} ClientState", dll_name, client_state);
	}
	else {
		eprintln!("unable to find ClientState");
	}
	/*
	// SignonState = ClientState + 0x98 has values 0...8
	if bin.scanner().finds_code(pat!("@3 833D${?'}08 0F94C0 C3")) {
		let signon_state = save[1];
		println!("{}!{:#x} SignonState", dll_name, signon_state);
	}
	else {
		eprintln!("unable to find SignonState");
	}
	*/
	// LevelName and SignonState together, look for string "dedicated" the smaller of the two routines
	// LevelName is [u8; 0x40] (buffer of 0x40 bytes inlined in the struct)
	if bin.scanner().finds_code(pat!("488D05${\"dedicated\"} C3 833D${?'}02 488D05${00} 7C07 488D05${'} C3"), &mut save) {
		let signon_state = save[1];
		let level_name = save[2];
		println!("{}!{:#x} SignonState", dll_name, signon_state);
		println!("{}!{:#x} LevelName", dll_name, level_name);
	}
	else {
		eprintln!("unable to find LevelName");
	}
}
