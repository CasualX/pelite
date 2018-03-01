/*!
*/

#![allow(unused)]

use pelite;
use pelite::pe32::{Rva, Pe, PeFile};
use pelite::pattern as pat;

//----------------------------------------------------------------

pub fn print(file: PeFile) {
	for cvar in &cvars(file).unwrap() {
		println!("{}!{:08X} {:08X} {}", cvar.dll_name, cvar.offset, cvar.flags, cvar.name);
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
	let mut cvars = Vec::new();

	let get_min_max_value = |save: &[Rva]| {
		let has_max = file.derva_copy::<u8>(save[0] + 1).unwrap();
		let max_value = file.derva_copy::<f32>(save[0] + 6).unwrap();
		let has_min = file.derva_copy::<u8>(save[0] + 11).unwrap();
		let min_value = file.derva_copy::<f32>(save[0] + 16).unwrap();
		(
			if has_min != 0 { Some(min_value) } else { None },
			if has_max != 0 { Some(max_value) } else { None },
		)
	};

	// Match static constructors which call [`ConVar::Create`](https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/public/tier1/convar.h#L383)

	// Variant: ConVar with description
	let pat1 = pat::parse("6A? 51 C704????? 6A? 51 C704????? B9*{'} 6A? 68*{'} 68'???? 68*{'} 68*{'} E8$").unwrap();
	let mut matches1 = file.scanner().matches_code(&pat1);
	while matches1.next_match(&mut save) {
		let (min_value, max_value) = get_min_max_value(&save);
		let offset = save[1];
		let desc = Some(file.derva_str(save[2]).unwrap().to_str().unwrap());
		let flags = file.derva_copy(save[3]).unwrap();
		let default_string = file.derva_str(save[4]).unwrap().to_str().unwrap();
		let name = file.derva_str(save[5]).unwrap().to_str().unwrap();
		cvars.push(ConVar { dll_name, name, desc, default_string, offset, flags, min_value, max_value });
	}

	// Variant: ConVar without description
	let pat2 = pat::parse("6A? 51 C704????? 6A? 51 C704????? B9*{'} 6A? 6A00 68'???? 68*{'} 68*{'} E8$").unwrap();
	let mut matches2 = file.scanner().matches_code(&pat2);
	while matches2.next_match(&mut save) {
		let (min_value, max_value) = get_min_max_value(&save);
		let offset = save[1];
		let desc = None;
		let flags = file.derva_copy(save[2]).unwrap();
		let default_string = file.derva_str(save[3]).unwrap().to_str().unwrap();
		let name = file.derva_str(save[4]).unwrap().to_str().unwrap();
		cvars.push(ConVar { dll_name, name, desc, default_string, offset, flags, min_value, max_value });
	}

	Ok(cvars)
}
