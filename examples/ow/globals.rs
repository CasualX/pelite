use pelite;
use pelite::pe64::*;
use pelite::pattern as pat;

pub fn print(bin: PeFile) {
	println!("# Globals\n\n```");
	for g in &globals(bin) {
		println!("Overwatch.exe!{:#x} {:?}", g.address, g.name);
	}
	println!("```\n");
}

struct Global<'a> {
	address: u32,
	name: &'a str,
}

fn globals(bin: PeFile<'_>) -> Vec<Global<'_>> {
	let mut globals = Vec::new();

	let mut matches = bin.scanner().matches_code(pat!("488D0D$ {'} E8$ {488BC4 56 57 4883EC58} 488D0D???? 488905$ {'} E8"));
	let mut save = [0; 4];
	while matches.next(&mut save) {
		if let Ok(g) = global(bin, &save) {
			globals.push(g);
		}
	}

	globals.sort_by_key(|g| g.address);
	globals.sort_by_key(|g| g.name);
	return globals;
}

fn global<'a>(bin: PeFile<'a>, save: &[u32; 4]) -> pelite::Result<Global<'a>> {
	let name = bin.derva_c_str(save[1])?.to_str().unwrap();
	let address = save[2];
	Ok(Global { address, name })
}
