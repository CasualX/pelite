use std::env;
use std::path::PathBuf;

use pelite::{FileMap, PeFile};

const HELP_TEXT: &str = "\
EXTRACT-ICONS <BINARY> <DEST>

  BINARY -- Path to the binary to extract icons from.
  DEST   -- Destination folder to write the icons to.
";

fn main() {
    let mut args = env::args_os();
    let _ = args.next();

    match (args.next(), args.next(), args.next()) {
        (Some(bin), Some(dest), None) => {
            let map = FileMap::open(&bin).expect("Error opening the binary");
            let file = PeFile::from_bytes(&map).expect("Error parsing the binary");
            let dest = PathBuf::from(dest);
            let resources = file
                .resources()
                .expect("Error binary does not have resources");
            for (name, group) in resources.icons().filter_map(Result::ok) {
                // Write the ICO file
                let mut contents = Vec::new();
                group.write(&mut contents).unwrap();
                let path = dest.join(&format!("{}.ico", name));
                println!("{}", path.display());
                let _ = std::fs::write(&path, &contents);
            }
        }
        (None, _, _) => {
            eprintln!("{}", HELP_TEXT);
        }
        _ => {
            eprintln!("Expecting arguments `binary.dll` `destination`");
        }
    }
}
