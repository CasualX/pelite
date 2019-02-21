use pelite;
use pelite::pe64::*;
use pelite::pattern as pat;

pub fn print(bin: PeFile<'_>) {
	println!("[Misc]");
	entity_list(bin);
	local_entity_handle(bin);
}

fn entity_list(bin: PeFile<'_>) {
	// EntityList
	//
	// Find GetEntityByIndex:
	// "Index must be greater than or equal to "
	// "Index must be less than %i.\n"
	//
	// entity_ptr = *(uintptr_t*)(entity_list + index * 32)
	let pat = pat::parse("4883EC28 85C9 79% 81F9u4 7C% 4863C1 488D0D$'").unwrap();

	let mut save = [0; 4];
	if bin.scanner().finds_code(&pat, &mut save) {
		let num_ent_entries = save[1];
		let cl_entitylist = save[2];
		println!("\tNUM_ENT_ENTRIES = {:#x}", num_ent_entries);
		println!("\tcl_entitylist = {:#x}", cl_entitylist);
	}
	else {
		eprintln!("unable to find cl_entitylist!");
	}
}

fn local_entity_handle(bin: PeFile<'_>) {
	let pat = pat::parse("488D15???? 833D${'}FF 741F").unwrap();
	let mut save = [0; 4];
	if bin.scanner().finds_code(&pat, &mut save) {
		let local_entity_handle = save[1];
		println!("\tLocalEntityHandle = {:#x}", local_entity_handle);
	}
	else {
		eprintln!("unable to find LocalEntityHandle!");
	}
}
