/*!
Weapon Data.
*/

#![allow(bad_style)]

use pelite;
use pelite::pattern as pat;
use pelite::pe32::*;

use iced_x86::{Code, Decoder, DecoderOptions, OpKind, Register};

//----------------------------------------------------------------

pub fn print(client: PeFile) {
	let list = weapondata(client).unwrap();

	format_xml::print! {
		"### WeaponData\n\n"
		"```\n"
		"class WeaponInfo {{\n"
		for member in (&list[1].members) {
			"\t"{member.name}": "{member.ty}",\n"
		}
		"}}\n"
		"class CSWeaponInfo extends WeaponInfo {{\n"
		for member in (&list[0].members) {
			"\t"{member.name}": "{member.ty}",\n"
		}
		"}}\n"
		"```\n\n"
		"#### Offsets\n\n"
		"```\n"
		for member in (&list[1].members) {
			"WeaponInfo!"{member.offset;#06x}" "{member.name}"\n"
		}
		for member in (&list[0].members) {
			"CSWeaponInfo!"{member.offset;#06x}" "{member.name}"\n"
		}
		"```\n\n"
	}
}

//----------------------------------------------------------------

pub struct Member<'a> {
	name: &'a str,
	ty: &'a str,
	offset: i32,
}
pub fn Member<'a>(name: &'a str, ty: &'a str, offset: i32) -> Member<'a> {
	Member { name, ty, offset }
}
pub struct WeaponInfo<'a> {
	members: Vec<Member<'a>>,
}

pub fn weapondata<'a>(client: PeFile<'a>) -> pelite::Result<Vec<WeaponInfo<'a>>> {
	// Find the initialize function for CSWeaponInfo
	let mut init_save = [0; 2];
	const INIT_PAT: &[pat::Atom] = pat!("55 8BEC 83E4F8 83EC14 53 56 8BF1 57 8B7E? 8B87???? 85C0 0F84???? 83B8????? 0F84???? E8$'");
	client.scanner().code().find(INIT_PAT, &mut init_save).expect("weapondata initialization pattern must be unique");

	let cs_weapon_info = analyse(client, init_save[0])?;
	let weapon_info = analyse(client, init_save[1])?;
	Ok(vec![cs_weapon_info, weapon_info])
}

fn analyse<'a>(client: PeFile<'a>, code_rva: Rva) -> pelite::Result<WeaponInfo<'a>> {
	let mut members = Vec::new();

	// Grab the code function bytes
	let code_va = client.rva_to_va(code_rva)?;
	let code = client.read_bytes(code_va)?;

	// Run through the initializing code
	const GET_PAT: &[pat::Atom] = pat!("E8$ A1???? A801 75? 83C801 C705????*'");
	let mut get_name = None;

	let mut decoder = Decoder::with_ip(32, code, code_va.into(), DecoderOptions::NONE);
	while decoder.can_decode() {
		let instruction = decoder.decode();
		if instruction.is_invalid() {
			break;
		}

		// Find functions which call `CEconItemSchema__GetAttributeDefinition`
		if instruction.code() == Code::Call_rel32_32 {
			let mut get_m = [0; 4];
			if client.scanner().exec(client.va_to_rva(instruction.ip() as Va).unwrap(), GET_PAT, &mut get_m) {
				let name = client.derva_c_str(get_m[1])?.to_str().unwrap();
				if let Some(previous_name) = get_name {
					eprintln!("missing offset \"{}\"", previous_name);
				}
				get_name = Some(name);
			}
		}
		// movss dword ptr [esi + dword offset], xmm0
		else if instruction.code() == Code::Movss_xmmm32_xmm
			&& instruction.op0_kind() == OpKind::Memory
			&& instruction.memory_base() == Register::ESI
			&& instruction.memory_index() == Register::None
			&& instruction.memory_displ_size() == 4
			&& instruction.op1_register() == Register::XMM0
		{
			let offset = instruction.memory_displacement32() as i32;
			if let Some(name) = get_name {
				members.push(Member(name, "Float", offset));
				get_name = None;
			}
			else {
				eprintln!("missing float {:#X}", offset);
			}
		}
		// mov dword ptr [esi + dword offset], reg
		else if instruction.code() == Code::Mov_rm32_r32
			&& instruction.op0_kind() == OpKind::Memory
			&& instruction.memory_base() == Register::ESI
			&& instruction.memory_index() == Register::None
			&& instruction.memory_displ_size() == 4
		{
			let offset = instruction.memory_displacement32() as i32;
			if let Some(name) = get_name {
				members.push(Member(name, "Int", offset));
				get_name = None;
			}
			else {
				eprintln!("missing int {:#X}", offset);
			}
		}
		// mov byte ptr [esi + dword offset], al
		else if instruction.code() == Code::Mov_rm8_r8
			&& instruction.op0_kind() == OpKind::Memory
			&& instruction.memory_base() == Register::ESI
			&& instruction.memory_index() == Register::None
			&& instruction.memory_displ_size() == 4
			&& instruction.op1_register() == Register::AL
		{
			let offset = instruction.memory_displacement32() as i32;
			if let Some(name) = get_name {
				members.push(Member(name, "Bool", offset));
				get_name = None;
			}
			else {
				eprintln!("missing bool {:#X}", offset);
			}
		}
		// mov dword ptr [esi + byte offset], eax
		else if instruction.code() == Code::Mov_rm32_r32
			&& instruction.op0_kind() == OpKind::Memory
			&& instruction.memory_base() == Register::ESI
			&& instruction.memory_index() == Register::None
			&& instruction.memory_displ_size() == 1
			&& instruction.op1_register() == Register::EAX
		{
			let offset = instruction.memory_displacement32() as i8;
			if let Some(name) = get_name {
				members.push(Member(name, "Int", offset as i32));
				get_name = None;
			}
			else {
				eprintln!("missing int {:#X}", offset);
			}
		}
		// End of the function
		else if instruction.code() == Code::Retnd {
			break;
		}
	}
	members.sort_unstable_by_key(|member| member.offset);

	Ok(WeaponInfo { members })
}
