/*!
Team Fortress 2
===============
*/

mod classes;
mod cvars;
mod interfaces;

//----------------------------------------------------------------

use std::path::Path;
use std::{env, io};

use pelite::pe32::PeFile;

fn open(base_path: &Path, dll_file: &str) -> io::Result<pelite::FileMap> {
    pelite::FileMap::open(&base_path.join(dll_file))
}

fn main() {
    // Get the tf2 folder
    let tf2_path_buffer;
    let tf2_path = {
        let mut args = env::args_os();
        args.next();
        tf2_path_buffer = args.next();
        tf2_path_buffer.as_ref().map(Path::new).unwrap_or(Path::new(
            r"C:\Program Files (x86)\Steam\steamapps\common\Team Fortress 2",
        ))
    };

    // Start by opening relevant tf2 binaries
    let engine_dll = open(tf2_path, "bin/engine.dll").unwrap();
    let inputsystem_dll = open(tf2_path, "bin/inputsystem.dll").unwrap();
    let materialsystem_dll = open(tf2_path, "bin/materialsystem.dll").unwrap();
    let shaderapidx9_dll = open(tf2_path, "bin/shaderapidx9.dll").unwrap();
    let vgui2_dll = open(tf2_path, "bin/vgui2.dll").unwrap();
    let vguimatsurface_dll = open(tf2_path, "bin/vguimatsurface.dll").unwrap();
    let vphysics_dll = open(tf2_path, "bin/vphysics.dll").unwrap();
    let vstdlib_dll = open(tf2_path, "bin/vstdlib.dll").unwrap();
    let client_dll = open(tf2_path, "tf/bin/client.dll").unwrap();
    let server_dll = open(tf2_path, "tf/bin/server.dll").unwrap();

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
    let server_file = PeFile::from_bytes(&server_dll).unwrap();

    interfaces::print(engine_file);
    interfaces::print(inputsystem_file);
    interfaces::print(materialsystem_file);
    interfaces::print(shaderapidx9_file);
    interfaces::print(vgui2_file);
    interfaces::print(vguimatsurface_file);
    interfaces::print(vphysics_file);
    interfaces::print(vstdlib_file);
    interfaces::print(client_file);
    interfaces::print(server_file);

    classes::print(client_file);

    cvars::print(engine_file);
    cvars::print(client_file);
    cvars::print(server_file);
}
