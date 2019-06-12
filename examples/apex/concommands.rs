use pelite;
use pelite::pe64::*;
use pelite::{util::CStr, Pod};
use pelite::pattern as pat;

pub fn print(bin: PeFile, dll_name: &str) {
	let cmds = concommands(bin);

	println!("## ConCommands\n");
	for cmd in &cmds {
		println!("<details>");
		println!("<summary><code>{}</code></summary>\n", cmd.name);
		if let Some(desc) = cmd.desc {
			println!("{}\n", desc);
		}
		println!("flags: `{:#x}`  ", cmd.flags);
		println!("</details>");
	}
	println!("\n### Addresses\n\n```");
	for cmd in &cmds {
		println!("{}!{:#010x} ConCommand {}", dll_name, cmd.address, cmd.name);
	}
	println!("```\n");
}

#[allow(non_snake_case)]
#[derive(Pod, Debug)]
#[repr(C)]
pub struct RawConCommand {
	// ConCommandBase
	pub vtable: u64,
	pub pNext: Ptr,
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
	pub desc: Option<&'a str>,
	pub flags: u32,
	pub callback: u32,
}

pub fn concommands(bin: PeFile<'_>) -> Vec<ConCommand<'_>> {
	let mut concommands = Vec::new();
	let mut save = [0; 4];
	// Find the ConCommand vtable
	if !bin.scanner().finds_code(pat!("4C 8D 25 ${'} 33 FF 4C 89 25"), &mut save) {
		eprintln!("ERR: unable to find ConCommand vftable");
		return concommands;
	}
	let vftable = save[1];
	// Find all concommands in .data
	let data_section = bin.section_headers().iter()
		.find(|sect| &sect.Name == b".data\0\0\0")
		.expect("unable to find `.data` section");
	// Find matches by scanning for the ConCommand vtable
	let pat = pat!("@3 *{'} (0000000000000000|*{}) 0100000000000000 *{}");
	let mut matches = bin.scanner().matches(pat, data_section.virtual_range());
	while matches.next(&mut save) {
		if save[1] != vftable {
			continue;
		}
		let address = save[0];
		let raw = bin.derva::<RawConCommand>(address).unwrap();
		let name = bin.deref_c_str(raw.pszName).unwrap_or(CStr::empty()).to_str().unwrap();
		let desc = bin.deref_c_str(raw.pszHelpString).ok().map(|desc| desc.to_str().unwrap());
		let flags = raw.fFlags;
		let callback = bin.va_to_rva(raw.fnCommandCallback).unwrap_or(0);
		concommands.push(ConCommand { address, name, desc, flags, callback })
	}
	// Sort to make a nice diff
	concommands.sort_by_key(|concommand| concommand.name);
	concommands
}
