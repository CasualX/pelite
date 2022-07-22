/*!
Weapon Data.
*/

#![allow(bad_style)]

use lde;
use pelite;
use pelite::pattern as pat;
use pelite::pe32::*;

//----------------------------------------------------------------

pub fn print(client: PeFile) {
    let list = weapondata(client).unwrap();

    tprint! {
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
    let init_pat = pat!(
        "55 8BEC 83E4F8 83EC14 53 56 8BF1 57 8B7E? 8B87???? 85C0 0F84???? 83B8????? 0F84???? E8$'"
    );
    let mut init_save = [0; 2];
    if !client.scanner().finds_code(init_pat, &mut init_save) {
        panic!("weapondata pattern not found");
    }

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
    let get_pat = pat!("E8$ A1???? A801 75? 83C801 C705????*'");
    let mut get_name = None;

    for (opcode, va) in lde::X86.iter(code, code_va) {
        // Find functions which call `CEconItemSchema__GetAttributeDefinition`
        if opcode.starts_with(&[0xE8]) {
            let mut get_m = [0; 4];
            if client
                .scanner()
                .exec(client.va_to_rva(va).unwrap(), get_pat, &mut get_m)
            {
                let name = client.derva_c_str(get_m[1])?.to_str().unwrap();
                if let Some(previous_name) = get_name {
                    eprintln!("missing offset \"{}\"", previous_name);
                }
                get_name = Some(name);
            }
        }
        // movss dword ptr [esi + dword offset], xmm0
        else if opcode.starts_with(&[0xF3, 0x0F, 0x11, 0x86]) {
            let offset = opcode.read(4);
            if let Some(name) = get_name {
                members.push(Member(name, "Float", offset));
                get_name = None;
            } else {
                eprintln!("missing float {:#X}", offset);
            }
        }
        // mov dword ptr [esi + dword offset], reg
        // where reg is eax, ecx or ebx
        else if opcode.starts_with(&[0x89]) && opcode[1] & 0b11_000_111 == 0b10_000_110 {
            let offset = opcode.read(2);
            if let Some(name) = get_name {
                members.push(Member(name, "Int", offset));
                get_name = None;
            } else {
                eprintln!("missing int {:#X}", offset);
            }
        }
        // mov byte ptr [esi + dword offset], al
        else if opcode.starts_with(&[0x88, 0x86]) {
            let offset = opcode.read(2);
            if let Some(name) = get_name {
                members.push(Member(name, "Bool", offset));
                get_name = None;
            } else {
                eprintln!("missing bool {:#X}", offset);
            }
        }
        // mov dword ptr [esi + byte offset], eax
        else if opcode.starts_with(&[0x89, 0x46]) {
            let offset = opcode.read::<i8>(2);
            if let Some(name) = get_name {
                members.push(Member(name, "Int", offset as i32));
                get_name = None;
            } else {
                eprintln!("missing int {:#X}", offset);
            }
        }
        // End of the function
        else if opcode.as_ref() == &[0xC3] {
            break;
        }
    }
    members.sort_unstable_by_key(|member| member.offset);

    Ok(WeaponInfo { members })
}
