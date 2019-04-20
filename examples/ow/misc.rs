use pelite;
use pelite::pe64::*;
use pelite::pattern as pat;

pub fn print(bin: PeFile) {
	println!("# Miscellaneous\n\n```");
	println!("TimeDateStamp: {:#x}", bin.file_header().TimeDateStamp);
	println!("CheckSum: {:#x}", bin.optional_header().CheckSum);
	ignore_mouse_detection(bin);
	local_player_transform(bin);
	view_matrix(bin);
	println!("```\n");
}

fn ignore_mouse_detection(bin: PeFile) {
	// Find access near
	// mov dword ptr [ptr], 0xFF1300C8
	let mut save = [0; 4];
	if bin.scanner().finds_code(pat!("488B05$ {'} 4532C0 4C8BCA C702???? 488B88u4 0FB641u1"), &mut save) {
		println!("[[Overwatch.exe!{:#x}] + {:#x}] + {:#x} IgnoreMouseDetection", save[1], save[2], save[3]);
	}
	else {
		eprintln!("Unable to find `IgnoreMouseDetection` signature.");
	}
}

fn local_player_transform(bin: PeFile) {
	// Accessed in exported ?GetListenerPosition@Query@SoundEngine@AK@@YA?AW4AKRESULT@@_KAEAVAkTransform@@@Z
	// https://www.audiokinetic.com/library/edge/?source=SDK&id=class_ak_transform.html
	let mut save = [0; 4];
	if bin.scanner().finds_code(pat!("0F1005${'} 4C8B7C2420 BB01000000"), &mut save) {
		println!("Overwatch.exe!{:#x} LocalPlayerTransform", save[1]);
	}
	else {
		eprintln!("Unable to find `LocalPlayerTransform` signature.");
	}
}

fn view_matrix(bin: PeFile) {
	let mut save = [0; 4];
	if bin.scanner().finds_code(pat!("74 ? 48 8B 0D ${'} B3 01 E8"), &mut save) {
		println!("[[Overwatch.exe!{:#x}] + {:#x}] + {:#x}", save[1], 0x8, 0x80);
	}
	else {
		eprintln!("Unable to find `view matrix` signatures.");
	}
}
