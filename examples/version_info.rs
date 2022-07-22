/*!
Demonstrates how to read the version info from a PE file.
 */

use std::env;
use std::path::Path;

fn main() {
    let mut args_os = env::args_os();
    match (
        args_os.next(),
        args_os.next(),
        args_os.next(),
        args_os.next(),
    ) {
        (Some(_), Some(path), Some(lang), None) => {
            let lang = lang.to_str().unwrap().parse().expect("lang parameter");
            print_version_info(Path::new(&path), Some(lang));
        }
        (Some(_), Some(path), None, None) => {
            print_version_info(Path::new(&path), None);
        }
        _ => {
            println!("Pelite example: Print version info\nUsage: <path> [lang]\n");
        }
    }
}

fn print_version_info(path: &Path, lang: Option<u16>) {
    // Map the file into memory
    let file_map = pelite::FileMap::open(path).expect("cannot open the file specified");

    // Interpret as a PE image
    let image = pelite::PeFile::from_bytes(file_map.as_ref()).expect("file is not a PE image");

    // Extract the resources from the image
    let resources = image.resources().expect("resources not found");

    // Extract the version info from the resources
    let version_info = match lang {
        Some(lang) => resources
            .find_resource_ex(&[pelite::resources::Name::VERSION, 1.into(), lang.into()])
            .and_then(|bytes| {
                Ok(pelite::resources::version_info::VersionInfo::try_from(
                    bytes,
                )?)
            }),
        None => resources.version_info(),
    }
    .expect("version info not found");

    // Print the version info strings
    struct Printer;
    impl pelite::resources::version_info::Visit<'_> for Printer {
        fn version_info(
            &mut self,
            _key: &[u16],
            fixed: Option<&pelite::image::VS_FIXEDFILEINFO>,
        ) -> bool {
            if let Some(fixed) = fixed {
                println!("{:<20} {}.{}.{}.{}\n{:<20} {}.{}.{}.{}\n{:<20} {:#x}\n{:<20} {:#x}\n{:<20} {}, {}\n{:<20} {}\n{:<20} {}",
					"FileVersion", fixed.dwFileVersion.Major, fixed.dwFileVersion.Minor, fixed.dwFileVersion.Patch, fixed.dwFileVersion.Build,
					"ProductVersion", fixed.dwProductVersion.Major, fixed.dwProductVersion.Minor, fixed.dwProductVersion.Patch, fixed.dwProductVersion.Build,
					"FileFlagsMask", fixed.dwFileFlagsMask,
					"FileFlags", fixed.dwFileFlags,
					"FileOS", fixed.dwFileOS >> 16, fixed.dwFileOS & 0xffff,
					"FileType", fixed.dwFileType,
					"FileSubtype", fixed.dwFileSubtype);
            }
            true
        }
        fn string_table(&mut self, lang: &[u16]) -> bool {
            let lang = String::from_utf16_lossy(lang);
            println!("\n[{}]", lang);
            true
        }
        fn string(&mut self, key: &[u16], value: &[u16]) {
            let key = String::from_utf16_lossy(key);
            let value = String::from_utf16_lossy(value);
            println!("{:<20} {:?}", key, value);
        }
    }
    version_info.visit(&mut Printer);

    // Render as source code
    let source_code = version_info.source_code();
    println!("\n```\n{}```", source_code);
}
