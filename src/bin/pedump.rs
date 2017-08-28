/*!
Dumps all PE related headers.
*/

extern crate pelite;

use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process;

use pelite::FileMap;

//----------------------------------------------------------------

const HELP_TEXT: &str = "
NAME:
  pedump - Inspect PE binary files.

SYNOPSIS:
  pedump FILEPATH
         [-h | --help]
         [-d | --dos]
         [-n | --headers]
         [-s | --sections]
         [-i | --imports]
         [-e | --exports]
         [-r | --relocs]
         [-x | --resources]
         [-g | --debug-info]

DESCRIPTION:
  Inspect and dump the contents of windows executable files.

OPTIONS:
  -d, --dos
      Print the DOS header and stub.

  -n, --headers
      Print the NT headers.

  -s, --sections
      Print the section headers.

  -i, --imports
      Print the imported symbols.

  -e, --exports
      Print the exported symbols.

  -r, --relocs
      Print the relocation table.

  -t, --tls
      Print the TLS directory.

  -x, --resources
      Print the embedded resource filesystem.

  -g, --debug-info
      Print debug information.
";

const SEPARATOR: &'static str = "----------------------------------------------------------------\n";

//----------------------------------------------------------------

const NO_INPUT_VAL: &'static str = "missing path to input binary";
const INVALID_ARG: &'static str = "invalid argument was given";

#[derive(Debug)]
struct Parameters {
	path: PathBuf,
	hex: bool,
	dos: bool,
	nt: bool,
	sections: bool,
	imports: bool,
	exports: bool,
	relocs: bool,
	tls: bool,
	resources: bool,
	debug_info: bool,
}

impl Default for Parameters {
	fn default() -> Parameters {
		// Initialize the default arguments of the program
		let mut vars = Parameters {
			path: PathBuf::new(),
			hex: false,
			dos: false,
			nt: false,
			sections: false,
			imports: false,
			exports: false,
			relocs: false,
			tls: false,
			resources: false,
			debug_info: false,
		};
		
		// Get args and print help text
		let mut args = env::args_os();
		let (_, mut args) = (args.next(), args.peekable());
		
		if args.peek().is_none() {
			print!("{}", HELP_TEXT);
			process::exit(0);
		}

		// Get the input binary path
		vars.path = args.next()
			.map(|path| PathBuf::from(path))
			.map_or(None, |path| if path.starts_with("-") { None } else { Some(path) })
			.unwrap_or_else(|| abort(NO_INPUT_VAL));

		// Parse the options for the program
		while let Some(arg) = args.next() {
			let arg = arg.into_string().unwrap();
			if arg.starts_with("--") {
				match arg.as_str() {
					"--dos" => vars.dos = true,
					"--nt" => vars.nt = true,
					"--sections" => vars.sections = true,
					"--imports" => vars.imports = true,
					"--exports" => vars.exports = true,
					"--relocs" => vars.relocs = true,
					"--tls" => vars.tls = true,
					"--resources" => vars.resources = true,
					"--debug-info" => vars.debug_info = true,
					_ => abort(INVALID_ARG),
				}
			}
			else if arg.starts_with("-") {
				let mut it = arg.chars(); it.next();
				while let Some(opt) = it.next() {
					match opt {
						'd' => vars.dos = true,
						'n' => vars.nt = true,
						's' => vars.sections = true,
						'i' => vars.imports = true,
						'e' => vars.exports = true,
						'r' => vars.relocs = true,
						't' => vars.tls = true,
						'x' => vars.resources = true,
						'g' => vars.debug_info = true,
						_ => abort(INVALID_ARG),
					}
				}
			}
			else {
				abort(INVALID_ARG);
			}
		}
		
		vars
	}
}

//----------------------------------------------------------------

// fn print_dbg(view: &pe::PeView) {
// 	print!("{}", SEPARATOR);
// 	if let Some(dbg) = view.debug_info() {
// 		print!("{:?}", dbg);
// 	}
// 	else {
// 		println!("No Debug Directory found.");
// 	}
// }

//----------------------------------------------------------------

fn abort(message: &str) -> ! {
	{
		let stderr = io::stderr();
		let mut stderr = stderr.lock();
		let _ = stderr.write(b"pedump: ");
		let _ = stderr.write(message.as_bytes());
		let _ = stderr.write(b".\n");
		let _ = stderr.flush();
	}
	process::exit(1);
}

fn main() {
	let args = Parameters::default();
	let map = FileMap::open(&args.path).unwrap_or_else(|e| {
		abort(&format!("{:?}", e));
	});
	// Try to print as PE32+
	if let Err(_) = pelite::pe64::PeFile::from_bytes(&map).map(|file| dump_pe64(&args, &file)) {
		// If that fails try PE32
		pelite::pe32::PeFile::from_bytes(&map).map(|file| dump_pe32(&args, &file)).unwrap_or_else(|e| {
			abort(&format!("{:?}", e));
		});
	}
}

fn dump_pe64(args: &Parameters, file: &pelite::pe64::PeFile) {
	use pelite::pe64::Pe;
	if args.dos {
		let dos = file.dos_header();
		print!("{}{:?}", SEPARATOR, dos);
	}
	if args.nt {
		let nt = file.nt_headers();
		print!("{}{:?}", SEPARATOR, nt);
	}
	if args.sections {
		print!("{}", SEPARATOR);
		for sec in file.section_headers() {
			print!("{:?}", sec);
		}
	}
	if args.exports {
		print!("{}", SEPARATOR);
		if let Ok(exports) = file.exports() {
			print!("{:?}", exports);
		}
		else {
			println!("No Export Directory found.");
		}
	}
	if args.imports {
		print!("{}", SEPARATOR);
		if let Ok(imports) = file.imports() {
			print!("{:?}", imports);
		}
		else {
			println!("No Import Directory found.");
		}
	}
	if args.resources {
		print!("{}", SEPARATOR);
		if let Ok(res) = file.resources() {
			print!("{:?}", res);
		}
		else {
			println!("No Resources Directory found.");
		}
	}
	if args.relocs {
		print!("{}", SEPARATOR);
		if let Ok(base_relocs) = file.base_relocs() {
			print!("{:?}", base_relocs);
		}
		else {
			println!("No BaseRelocation Directory found.");
		}
	}
	if args.tls {
		print!("{}", SEPARATOR);
		if let Ok(tls) = file.tls() {
			print!("{:?}", tls);
		}
		else {
			println!("No TLS Directory found.");
		}
	}
	if args.debug_info {
		print!("{}", SEPARATOR);
		if let Ok(debug) = file.debug() {
			print!("{:?}", debug);
		}
		else {
			println!("No Debug Directory found.");
		}
	}
}

fn dump_pe32(args: &Parameters, file: &pelite::pe32::PeFile) {
	use pelite::pe32::Pe;
	if args.dos {
		let dos = file.dos_header();
		print!("{}{:?}", SEPARATOR, dos);
	}
	if args.nt {
		let nt = file.nt_headers();
		print!("{}{:?}", SEPARATOR, nt);
	}
	if args.sections {
		print!("{}", SEPARATOR);
		for sec in file.section_headers() {
			print!("{:?}", sec);
		}
	}
	if args.exports {
		print!("{}", SEPARATOR);
		if let Ok(exports) = file.exports() {
			print!("{:?}", exports);
		}
		else {
			println!("No Export Directory found.");
		}
	}
	if args.imports {
		print!("{}", SEPARATOR);
		if let Ok(imports) = file.imports() {
			print!("{:?}", imports);
		}
		else {
			println!("No Import Directory found.");
		}
	}
	if args.resources {
		print!("{}", SEPARATOR);
		if let Ok(res) = file.resources() {
			print!("{:?}", res);
		}
		else {
			println!("No Resources Directory found.");
		}
	}
	if args.relocs {
		print!("{}", SEPARATOR);
		if let Ok(base_relocs) = file.base_relocs() {
			print!("{:?}", base_relocs);
		}
		else {
			println!("No BaseRelocation Directory found.");
		}
	}
	if args.tls {
		print!("{}", SEPARATOR);
		if let Ok(tls) = file.tls() {
			print!("{:?}", tls);
		}
		else {
			println!("No TLS Directory found.");
		}
	}
	if args.debug_info {
		print!("{}", SEPARATOR);
		if let Ok(debug) = file.debug() {
			print!("{:?}", debug);
		}
		else {
			println!("No Debug Directory found.");
		}
	}
}
