use pelite;
use pelite::pe64::*;
use pelite::util::{CStr, Pod};
use pelite::pattern as pat;

pub fn print(bin: PeFile) {
	for cvar in convars(bin) {
		println!("[ConVar.{:?}]", cvar.name);
		println!("\taddress = {:#x}", cvar.address);
		if cvar.description.len() > 0 {
			println!("\tdescription = {:?}", cvar.description);
		}
		if cvar.data_type.len() > 0 {
			println!("\tdata_type = {:?}", cvar.data_type);
		}
		println!("\tflags = {:#x}", cvar.flags);
		println!("\tdefault_value = {:?}", cvar.default_value);
		if let Some(min_val) = cvar.min_val {
			println!("\tmin_val: {}", min_val);
		}
		if let Some(max_val) = cvar.max_val {
			println!("\tmax_val: {}", max_val);
		}
	}
	for cmd in concommands(bin) {
		println!("[ConCommand.{:?}]", cmd.name);
		if cmd.description.len() > 0 {
			println!("\tdescription = {:?}", cmd.description);
		}
		println!("\tflags = {:#x}", cmd.flags);
		if cmd.callback > 0 {
			println!("\tcallback = {:#x}", cmd.callback);
		}
	}
}

// Find information in the 'setinfo' command
// References "Custom user info value"
// sizeof(ConVar) == 160
#[allow(non_snake_case)]
#[derive(Pod, Debug)]
#[repr(C)]
pub struct RawConVar {
	// ConCommandBase
	pub vtable: u64,
	pub pNext: Ptr<RawConVar>,
	pub bRegistered: u8,
	pub pszName: Ptr<CStr>,
	pub pszHelpString: Ptr<CStr>,
	pub pszDataType: Ptr<CStr>,
	unk_u64: u64,
	pub fFlags: u32,
	// ConVar
	pub IConVar_vtable: u64,
	pub pParent: Ptr<RawConVar>,
	pub pszDefaultValue: Ptr<CStr>,
	pub pszString: u64, // Allocated
	pub StringLength: i32,
	unk_u32: u32,
	pub fValue: f32,
	pub nValue: i32,
	pub bHasMin: u8,
	pub fMinVal: f32,
	pub bHasMax: u8,
	pub fMaxVal: f32,
	// Callback stuff...
	// callback_stuff: [u64; 4],
}

pub struct ConVar<'a> {
	pub address: u32,
	pub name: &'a str,
	pub description: &'a str,
	pub data_type: &'a str,
	pub default_value: &'a str,
	pub flags: u32,
	pub min_val: Option<f32>,
	pub max_val: Option<f32>,
}

pub fn convars(bin: PeFile<'_>) -> Vec<ConVar<'_>> {
	// Find the main ConVar vtable
	let mut save = [0; 4];
	if !bin.scanner().finds_code(&pat::parse("488BC8 488BD3 E8$ 4053 4883EC60 488BD9 C6411000 33C9 488D05$'").unwrap(), &mut save) {
		eprintln!("ERR: unable to find ConVar vftable");
		return Vec::new();
	}
	// Get the virtual address of the ConVar vtable
	let vftable = bin.optional_header().ImageBase + save[1] as u64;
	// Find the data section
	let data_section = bin.section_headers().iter().find(|section| &section.Name == b".data\0\0\0").unwrap();
	// Scan the data section for pointers to the vtable
	let data_data = bin.derva_slice::<u64>(data_section.VirtualAddress, data_section.SizeOfRawData as usize / 8).unwrap();
	let mut convars = Vec::new();
	for i in data_data.iter().enumerate().filter_map(|(index, &ptr)| if ptr == vftable { Some(index) } else { None }) {
		let address = data_section.VirtualAddress + (i * 8) as u32;
		let raw = bin.derva::<RawConVar>(address).unwrap();
		let name = bin.deref_c_str(raw.pszName).unwrap_or(CStr::empty()).to_str().unwrap();
		let description = bin.deref_c_str(raw.pszHelpString).unwrap_or(CStr::empty()).to_str().unwrap();
		let data_type = bin.deref_c_str(raw.pszDataType).unwrap_or(CStr::empty()).to_str().unwrap();
		let default_value = bin.deref_c_str(raw.pszDefaultValue).unwrap_or(CStr::empty()).to_str().unwrap();
		let flags = raw.fFlags;
		let min_val = if raw.bHasMin != 0 { Some(raw.fMinVal) } else { None };
		let max_val = if raw.bHasMax != 0 { Some(raw.fMaxVal) } else { None };
		convars.push(ConVar { address, name, description, data_type, default_value, flags, min_val, max_val });
	}

	// Sort to make a nice diff
	convars.sort_by_key(|convar| convar.name);
	convars
}

#[allow(non_snake_case)]
#[derive(Pod, Debug)]
#[repr(C)]
pub struct RawConCommand {
	// ConCommandBase
	pub vtable: u64,
	pub pNext: Ptr<RawConVar>,
	pub bRegistered: u8,
	pub pszName: Ptr<CStr>,
	pub pszHelpString: Ptr<CStr>,
	pub pszDataType: Ptr<CStr>, // Some string indicating the data type and min/max range in string form
	unk_u64: u64,
	pub fFlags: u32,
	// ConCommand
	unk_fn: u64,
	unk_zero: u64,
	pub fnCommandCallback: u64,
	pub fnCompletionCallback: u64,
	pub fnCommandType: u32,
}

pub struct ConCommand<'a> {
	pub address: u32,
	pub name: &'a str,
	pub description: &'a str,
	pub flags: u32,
	pub callback: u32,
}

pub fn concommands(bin: PeFile<'_>) -> Vec<ConCommand<'_>> {
	// Find ConCommand constructor thingy
	let mut save = [0; 4];
	let pat = pat::parse("488D05${} 488D0D${'} 488905${'} E9$ 4053 4883EC20").unwrap();
	let mut matches = bin.scanner().matches_code(&pat);
	let mut concommands = Vec::new();
	while matches.next(&mut save) {
		if save[1] != save[2] {
			continue;
		}
		let address = save[1];
		let raw = bin.derva::<RawConCommand>(address).unwrap();
		let name = bin.deref_c_str(raw.pszName).unwrap_or(CStr::empty()).to_str().unwrap();
		let description = bin.deref_c_str(raw.pszHelpString).unwrap_or(CStr::empty()).to_str().unwrap();
		let flags = raw.fFlags;
		let callback = bin.va_to_rva(raw.fnCommandCallback).unwrap_or(0);
		concommands.push(ConCommand { address, name, description, flags, callback })
	}
	concommands.sort_by_key(|concommand| concommand.name);
	concommands
}
