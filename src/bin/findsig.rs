/*!
Find patterns utility.
 */

use std::ffi::OsStr;
use std::io::{self, Write};
use std::path::Path;
use std::{cmp, env};

use pelite::pattern as pat;

const HELP_TEXT: &'static str = "\
FINDSIG <FILE> [PAT]...

  FILE - Path to the input binary to scan.
  PAT  - Any number of patterns to find.

If no patterns are provided, they are interactively read line by line from stdin.

Find documentation about the pattern syntax here:
https://docs.rs/pelite/*/pelite/pattern/fn.parse.html
";

fn main() {
    // Get args and skip the invoker
    let mut args = env::args_os();
    args.next();

    // Read the input file and/or get json option
    let mut file_path = match args.next() {
        Some(file_path) => file_path,
        None => {
            print!("{}", HELP_TEXT);
            return;
        }
    };
    let mut json = false;
    if file_path == OsStr::new("-json") {
        json = true;
        file_path = match args.next() {
            Some(file_path) => file_path,
            None => {
                eprintln!("Missing required file path argument.");
                return;
            }
        }
    }

    // Get the patterns
    let patterns: Vec<String> = args.filter_map(|arg| arg.into_string().ok()).collect();
    if patterns.len() == 0 {
        json = false;
    }

    // Path and filename of the input file
    let file_path = Path::new(&file_path);
    let file_name = file_path
        .file_name()
        .expect("path does not have a file name")
        .to_str()
        .unwrap();

    // Map the file into memory
    let file_map = pelite::FileMap::open(&file_path).expect("cannot open the input file");

    match pelite::PeFile::from_bytes(&file_map) {
        Ok(file) => {
            process_patterns(&patterns, json, &mut |pattern, save| {
                let mut matches = file
                    .scanner()
                    .matches(pattern, file.headers().image_range());
                let mut first = true;
                while matches.next(save) {
                    if json {
                        if first {
                            first = false;
                        } else {
                            print!(",");
                        }
                        print_json(save);
                    } else {
                        print_match(file_name, save);
                    }
                }
            });
        }
        // Must be a valid PE file
        Err(err) => {
            panic!("Not a valid PE file: {}", err);
        }
    }
}

fn process_patterns(patterns: &[String], json: bool, f: &mut dyn FnMut(&[pat::Atom], &mut [u32])) {
    if patterns.len() > 0 {
        // Read from the command line args if available
        if json {
            print!("[");
        }
        let mut first = true;
        for pat in patterns {
            if first {
                first = false;
            } else {
                print!(",");
            }
            process_pattern(pat, json, f);
        }
        if json {
            println!("]");
        }
    } else {
        // Read from standard input if no patterns were provided on command line
        print!("Interactive REPL, enter any pattern to find its matches.\nPress Ctrl-C to quit.\n");
        loop {
            // Print a nice REPL
            print!(">>> ");
            let _ = io::stdout().flush();
            // Read input from stdin, handle any kind of abort by detecting the lack of newline (?)
            let mut line = String::new();
            if io::stdin().read_line(&mut line).is_err() || line.len() == 0 {
                break;
            }
            // Only process non-empty lines, otherwise ask the user again
            let line = line.trim();
            if line.len() > 0 {
                process_pattern(&line, false, f);
            }
        }
    }
}
fn process_pattern(pattern_str: &str, json: bool, f: &mut dyn FnMut(&[pat::Atom], &mut [u32])) {
    // Parse the pattern
    let pattern = match pat::parse(pattern_str) {
        Ok(pattern) => pattern,
        Err(err) => {
            eprintln!("Pattern `{}` error: {}", pattern_str, err);
            return;
        }
    };
    // Print the header followed by its matches
    if json {
        print!(
            "{{\"pattern\":\"{}\",\"matches\":[",
            json_escape(pattern_str)
        );
    } else {
        println!("Pattern `{}` matches:", pattern_str);
    }
    let mut save = [0; 32];
    let save_len = cmp::min(pat::save_len(&pattern), save.len());
    f(&pattern, &mut save[..save_len]);
    if json {
        print!("]}}");
    }
}
fn print_match(file_name: &str, save: &[u32]) {
    print!("  {}!{:#010x}", file_name, save[0]);
    if save.len() > 1 {
        print!("  [");
        for i in 1..save.len() {
            print!("{}/{:#010x} ", i, save[i]);
        }
        print!("]");
    }
    println!();
}
fn print_json(save: &[u32]) {
    print!("[");
    let mut first = true;
    for value in save {
        if first {
            first = false;
        } else {
            print!(",");
        }
        print!("{}", value);
    }
    print!("]");
}
fn json_escape(string: &str) -> String {
    let mut escaped = String::new();
    for chr in string.chars() {
        match chr {
            '\x08' => escaped.push_str("\\b"),
            '\x0C' => escaped.push_str("\\f"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            '\"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            _ => escaped.push(chr),
        }
    }
    escaped
}
