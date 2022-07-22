/*!
Strings utility.
 */

use pelite;
use pelite::strings::Config;
use std::env;
use std::path::Path;

const HELP_TEXT: &str = "\
STRINGS [OPTS] <FILE>

  OPTS:
  --help (-h)           - Display this message.
  --min-length={u8}     - Minimum string length to accept as a valid string.
  --min-length-nul={u8} - Minimum string length when there is a nul terminator.
  --strict-nul={bool}   - When true, requires any found string to be terminated by a nul terminator.

  FILE:
  Path to the executable to analyze.
";

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let mut path = None;
    let mut config = Config::default();

    for arg in &args[1..] {
        if arg == "--help" || arg == "-h" {
            print!("{}", HELP_TEXT);
            return;
        } else if arg.starts_with("--min-length=") {
            config.min_length = arg[13..].parse().expect("invalid min-length");
        } else if arg.starts_with("--min-length-nul=") {
            config.min_length_nul = arg[17..].parse().expect("invalid min-length-nul");
        } else if arg.starts_with("--strict-nul=") {
            config.strict_nul = arg[13..].parse().expect("invalid strict-nul");
        } else if arg.starts_with("-") {
            panic!("unknown argument: {}", arg);
        } else {
            path = Some(arg);
        }
    }

    let path = path.expect("invalid path");
    let file_name = Path::new(path).file_name().unwrap().to_str().unwrap();

    let filemap = pelite::FileMap::open(path).unwrap();
    match analyze(config, filemap.as_ref(), file_name) {
        Ok(()) => {}
        Err(err) => {
            eprintln!("error: {}", err);
        }
    }
}

fn analyze(config: Config, image: &[u8], file_name: &str) -> pelite::Result<()> {
    let image = pelite::PeFile::from_bytes(image)?;
    for sect in image.section_headers() {
        if let Ok(bytes) = image.get_section_bytes(sect) {
            for s in config.clone().enumerate(sect.VirtualAddress, bytes) {
                println!(
                    "{}!{:?}:{:#x} {} {:?}",
                    file_name,
                    sect.name(),
                    s.address,
                    if s.has_nul { "!" } else { "?" },
                    std::str::from_utf8(s.string).unwrap()
                );
            }
        }
    }
    Ok(())
}
