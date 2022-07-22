/*!
Counter-Strike: Global Offensive
================================
*/

macro_rules! tprint {
	($($tt:tt)*) => {
		print!("{}", format_xml::template!{$($tt)*});
	};
}

mod classes;
mod cvars;
mod datamaps;
mod interfaces;
mod kbutton;
mod recvtables;
mod weapondata;

//----------------------------------------------------------------

use std::path::Path;
use std::{env, io};

use pelite::pe32::PeFile;

fn open(base_path: &Path, dll_file: &str) -> io::Result<pelite::FileMap> {
    pelite::FileMap::open(&base_path.join(dll_file))
}

fn main() {
    // Get the csgo folder
    let csgo_path_buffer;
    let csgo_path = {
        let mut args = env::args_os();
        args.next();
        csgo_path_buffer = args.next();
        csgo_path_buffer
            .as_ref()
            .map(Path::new)
            .unwrap_or(Path::new(
                r"C:\Program Files (x86)\Steam\steamapps\common\Counter-Strike Global Offensive",
            ))
    };

    // Start by opening relevant csgo binaries
    let engine_dll = open(csgo_path, "bin/engine.dll").unwrap();
    let inputsystem_dll = open(csgo_path, "bin/inputsystem.dll").unwrap();
    let materialsystem_dll = open(csgo_path, "bin/materialsystem.dll").unwrap();
    let shaderapidx9_dll = open(csgo_path, "bin/shaderapidx9.dll").unwrap();
    let vgui2_dll = open(csgo_path, "bin/vgui2.dll").unwrap();
    let vguimatsurface_dll = open(csgo_path, "bin/vguimatsurface.dll").unwrap();
    let vphysics_dll = open(csgo_path, "bin/vphysics.dll").unwrap();
    let vstdlib_dll = open(csgo_path, "bin/vstdlib.dll").unwrap();
    let client_dll = open(csgo_path, "csgo/bin/client.dll").unwrap();
    let matchmaking_dll = open(csgo_path, "csgo/bin/matchmaking.dll").unwrap();
    let server_dll = open(csgo_path, "csgo/bin/server.dll").unwrap();

    // Interpret them as 32-bit PeFiles
    let engine_file = PeFile::from_bytes(&engine_dll).unwrap();
    let inputsystem_file = PeFile::from_bytes(&inputsystem_dll).unwrap();
    let materialsystem_file = PeFile::from_bytes(&materialsystem_dll).unwrap();
    let shaderapidx9_file = PeFile::from_bytes(&shaderapidx9_dll).unwrap();
    let vgui2_file = PeFile::from_bytes(&vgui2_dll).unwrap();
    let vguimatsurface_file = PeFile::from_bytes(&vguimatsurface_dll).unwrap();
    let vphysics_file = PeFile::from_bytes(&vphysics_dll).unwrap();
    let vstdlib_file = PeFile::from_bytes(&vstdlib_dll).unwrap();
    let client_file = PeFile::from_bytes(&client_dll).unwrap();
    let matchmaking_file = PeFile::from_bytes(&matchmaking_dll).unwrap();
    let server_file = PeFile::from_bytes(&server_dll).unwrap();

    println!("# Counter-Strike Global Offensive\n");
    println!("Demonstrates static analysis by dumping Counter-Strike Global Offensive offsets.\n");

    println!("## Engine.dll\n");
    interfaces::print(engine_file, "engine.dll");
    cvars::print(engine_file, "engine.dll");

    println!("## Client.dll\n");
    interfaces::print(client_file, "client.dll");
    cvars::print(client_file, "client.dll");
    kbutton::print(client_file, "client.dll");
    classes::print(client_file, "client.dll");
    datamaps::print(client_file);
    recvtables::print(client_file);
    weapondata::print(client_file);

    println!("## Server.dll\n");
    interfaces::print(server_file, "server.dll");
    cvars::print(server_file, "server.dll");

    println!("## Other\n");
    interfaces::print(inputsystem_file, "inputsystem.dll");
    interfaces::print(materialsystem_file, "materialsystemd.dll");
    interfaces::print(shaderapidx9_file, "shaderapidx9.dll");
    interfaces::print(vgui2_file, "vgui2.dll");
    interfaces::print(vguimatsurface_file, "vguimatsurface.dll");
    interfaces::print(vphysics_file, "vphysics.dll");
    interfaces::print(vstdlib_file, "vstdlib.dll");
    interfaces::print(matchmaking_file, "matchmaking.dll");
}
