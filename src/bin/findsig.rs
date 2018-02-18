extern crate pelite;

use std::env;
use std::path::Path;
use std::io::{self, BufRead};

use pelite::{pe32, pe64};
use pelite::pattern as pat;

const HELP_TEXT: &'static str = "\
Example signature scanner; https://github.com/CasualX/scanner-rs

FINDSIG <FILE> [SIG]...

  FILE - Path to the input binary to scan.
  SIG  - Any number of signatures to find.

If no signatures are provided, they are read line by line from stdin.
";

fn main() {
	// Get args and skip the invoker
	let mut args = env::args_os();
	args.next();

	// Read the input file
	if let Some(file_path) = args.next() {
		// Path and filename of the input file
		let file_path = Path::new(&file_path);
		let file_name = file_path.file_name().unwrap().to_str().unwrap();

		// Map the file into memory
		let file_map = pelite::FileMap::open(&file_path).expect("Cannot open the input file.");

		// Try reading as PE32
		if let Ok(pe) = pe32::PeFile::from_bytes(&file_map) {
			process_patterns(args, &mut |pattern, save| {
				let scanner = pe32::Pe::scanner(pe);
				let mut matches = scanner.matches_code(pattern);
				while matches.next_match(save) {
					print_match(file_name, save);
				}
			});
		}
		// Try reading as PE32+
		else if let Ok(pe) = pe64::PeFile::from_bytes(&file_map) {
			process_patterns(args, &mut |pattern, save| {
				let scanner = pe64::Pe::scanner(pe);
				let mut matches = scanner.matches_code(pattern);
				while matches.next_match(save) {
					print_match(file_name, save);
				}
			});
		}
		// Must be a valid PE binary
		else {
			panic!("File not a valid PE binary.");
		}
	}
	else {
		print!("{}", HELP_TEXT);
	}
}

fn process_patterns<F: FnMut(&[pat::Atom], &mut [u32])>(args: env::ArgsOs, f: &mut F) {
	let mut stdin = true;
	for pattern_string in args.filter_map(|pattern_string| pattern_string.into_string().ok()) {
		process_pattern(&pattern_string, f);
		stdin = false;
	}
	// Read from standard input if no patterns were provided on command line
	if stdin {
		let stdin = io::stdin();
		for pattern_string in stdin.lock().lines().filter_map(|line| line.ok()) {
			process_pattern(&pattern_string, f);
		}
	}
}
fn process_pattern<F: FnMut(&[pat::Atom], &mut [u32])>(pattern_str: &str, f: &mut F) {
	// Parse the pattern
	let pattern = match pat::parse(pattern_str) {
		Ok(pattern) => pattern,
		Err(err) => {
			eprintln!("Pattern `{}` error: {}", pattern_str, err);
			return;
		},
	};
	// Print the header followed by its matches
	println!("Pattern `{}` matches:", pattern_str);
	let mut save = [0; 16];
	f(&pattern, &mut save);
}
fn print_match(file_name: &str, save: &[u32]) {
	print!("  {}!{:08x}", file_name, save[0]);
	if save[1] != 0 {
		print!("  [");
		for i in (1..save.len()).take_while(|&i| save[i] != 0) {
			print!("{}/{:08x} ", i, save[i]);
		}
		print!("]");
	}
	println!();
}
