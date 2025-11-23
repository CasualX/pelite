/*!
Dumps all PE related headers.
 */

use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process;

use pelite::image::IMAGE_FILE_MACHINE_ARM64;
use pelite::{FileMap, PeFile, Wrap};

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
         [-t | --tls]
         [-c | --exceptions]
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

  -l, --load-config
      Prints the load config.

  -t, --tls
      Print the TLS directory.

  -c, --exceptions
      Print the exception directory.

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
	_hex: bool,
	dos: bool,
	nt: bool,
	sections: bool,
	imports: bool,
	exports: bool,
	relocs: bool,
	load_config: bool,
	tls: bool,
	exceptions: bool,
	resources: bool,
	debug_info: bool,
}

impl Default for Parameters {
	fn default() -> Parameters {
		// Initialize the default arguments of the program
		let mut vars = Parameters {
			path: PathBuf::new(),
			_hex: false,
			dos: false,
			nt: false,
			sections: false,
			imports: false,
			exports: false,
			relocs: false,
			load_config: false,
			tls: false,
			exceptions: false,
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
		vars.path = args
			.next()
			.map(|path| PathBuf::from(path))
			.map_or(None, |path| {
				if path.starts_with("-") {
					None
				}
				else {
					Some(path)
				}
			})
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
					"--load-config" => vars.load_config = true,
					"--tls" => vars.tls = true,
					"--exceptions" => vars.exceptions = true,
					"--resources" => vars.resources = true,
					"--debug-info" => vars.debug_info = true,
					_ => abort(INVALID_ARG),
				}
			}
			else if arg.starts_with("-") {
				let mut it = arg.chars();
				it.next();
				while let Some(opt) = it.next() {
					match opt {
						'd' => vars.dos = true,
						'n' => vars.nt = true,
						's' => vars.sections = true,
						'i' => vars.imports = true,
						'e' => vars.exports = true,
						'r' => vars.relocs = true,
						'l' => vars.load_config = true,
						't' => vars.tls = true,
						'c' => vars.exceptions = true,
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
	match PeFile::from_bytes(&map) {
		Ok(Wrap::T32(file)) => dump_pe32(&args, file),
		Ok(Wrap::T64(file)) => dump_pe64(&args, file),
		Err(err) => abort(&format!("{}", err)),
	}
}

fn dump_pe64(args: &Parameters, file: pelite::pe64::PeFile) {
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
			print!("{:#?}", exports);
		}
		else {
			println!("No Export Directory found.");
		}
	}
	if args.imports {
		print!("{}", SEPARATOR);
		if let Ok(imports) = file.imports() {
			print!("{:#?}", imports);
		}
		else {
			println!("No Import Directory found.");
		}
	}
	if args.resources {
		print!("{}", SEPARATOR);
		if let Ok(res) = file.resources() {
			print!("{:#?}", res);
		}
		else {
			println!("No Resources Directory found.");
		}
	}
	if args.relocs {
		print!("{}", SEPARATOR);
		if let Ok(base_relocs) = file.base_relocs() {
			print!("{:#?}", base_relocs);
		}
		else {
			println!("No BaseRelocation Directory found.");
		}
	}
	if args.load_config {
		print!("{}", SEPARATOR);
		if let Ok(load_config) = file.load_config() {
			print!("{:#?}", load_config);
		}
		else {
			println!("No Load Config Directory found.");
		}
	}
	if args.tls {
		print!("{}", SEPARATOR);
		if let Ok(tls) = file.tls() {
			print!("{:#?}", tls);
		}
		else {
			println!("No TLS Directory found.");
		}
	}
	if args.exceptions {
		use pelite::pe64::exception_arm64::Arm64ExceptionExt;

		print!("{}", SEPARATOR);
		let machine = file.file_header().Machine;
		if machine == IMAGE_FILE_MACHINE_ARM64 {
			match file.exception_arm64() {
				Ok(exceptions) => print!("{:#?}", exceptions),
				Err(_) => println!("No Exception Directory found."),
			}
		}
		else if let Ok(exceptions) = file.exception() {
			print!("{:#?}", exceptions);
		}
		else {
			println!("No Exception Directory found.");
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

fn dump_pe32(args: &Parameters, file: pelite::pe32::PeFile) {
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
			print!("{:#?}", exports);
		}
		else {
			println!("No Export Directory found.");
		}
	}
	if args.imports {
		print!("{}", SEPARATOR);
		if let Ok(imports) = file.imports() {
			print!("{:#?}", imports);
		}
		else {
			println!("No Import Directory found.");
		}
	}
	if args.resources {
		print!("{}", SEPARATOR);
		if let Ok(res) = file.resources() {
			print!("{:#?}", res);
		}
		else {
			println!("No Resources Directory found.");
		}
	}
	if args.relocs {
		print!("{}", SEPARATOR);
		if let Ok(base_relocs) = file.base_relocs() {
			print!("{:#?}", base_relocs);
		}
		else {
			println!("No BaseRelocation Directory found.");
		}
	}
	if args.load_config {
		print!("{}", SEPARATOR);
		if let Ok(load_config) = file.load_config() {
			print!("{:#?}", load_config);
		}
		else {
			println!("No Load Config Directory found.");
		}
	}
	if args.tls {
		print!("{}", SEPARATOR);
		if let Ok(tls) = file.tls() {
			print!("{:#?}", tls);
		}
		else {
			println!("No TLS Directory found.");
		}
	}
	if args.exceptions {
		print!("{}", SEPARATOR);
		if let Ok(exceptions) = file.exception() {
			println!("Exception Directory - {} entries", exceptions.image().len());
			for (index, func) in exceptions.functions().enumerate() {
				let image = func.image();
				println!(
					"[{:04}] begin=0x{:08x} end=0x{:08x} unwind=0x{:08x}",
					index,
					image.BeginAddress,
					image.EndAddress,
					image.UnwindData,
				);
			}
		}
		else {
			println!("No Exception Directory found.");
		}
	}
	if args.debug_info {
		print!("{}", SEPARATOR);
		if let Ok(debug) = file.debug() {
			print!("{:#?}", debug);
		}
		else {
			println!("No Debug Directory found.");
		}
	}
}
