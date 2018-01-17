/*!
Markov Chain generator for Binary Code.
*/

extern crate pelite;
extern crate rand;

use std::{env};

fn main() {
	let mut args = env::args_os();
	args.next();

	// Help text
	if args.len() == 1 {
		println!("markovbin <COUNT> <FILES>...\n\nGiven a list of binary <FILES> (.exe, .dll) analyzes their code to generate a markov chain of <COUNT> bytes.");
		return;
	}

	// Get the number of bytes to generate
	let count: usize = match args.next().unwrap().into_string().unwrap().parse() {
		Ok(count) => count,
		Err(e) => {
			eprintln!("first argument should be a number (of bytes to generate): {}", e);
			return;
		}
	};

	// Markov chain data
	// Represents the number of times the byte at index J follows the byte at index I in the outer array
	// Uh, for every possible byte the inner array gives the number of times another byte followed it
	let mut markov_data = [[0u32; 256]; 256];

	// Combine the markov data from all the given input binary files
	for file in args {
		// Load the file into memory
		println!("generating markov data from {:?}...", file);
		match pelite::FileMap::open(&file) {
			Ok(file_map) => {
				// Try to parse as a PE32 file
				if let Ok(pe) = pelite::pe32::PeFile::from_bytes(&file_map) {
					use pelite::pe32::Pe;
					// Find the binary code bytes
					let code_section = pe.section_headers().iter().find(|&s| s.Characteristics & 0xE0000000 == 0x60000000).unwrap();
					let bytes = match pe.derva_slice(code_section.VirtualAddress, code_section.VirtualSize as usize) {
						Ok(bytes) => bytes,
						Err(e) => {
							eprintln!("cannot read code from {:?}: {}", file, e);
							continue;
						},
					};
					// Mix the markov data from this binary
					analyze(bytes, &mut markov_data);
				}
				// Otherwise try to parse as a PE32+ file
				else if let Ok(pe) = pelite::pe64::PeFile::from_bytes(&file_map) {
					use pelite::pe64::Pe;
					// Find the binary code bytes
					let code_section = pe.section_headers().iter().find(|&s| s.Characteristics & 0xE0000000 == 0x60000000).unwrap();
					let bytes = match pe.derva_slice(code_section.VirtualAddress, code_section.VirtualSize as usize) {
						Ok(bytes) => bytes,
						Err(e) => {
							eprintln!("cannot read code from {:?}: {}", file, e);
							continue;
						},
					};
					// Mix the markov data from this binary
					analyze(bytes, &mut markov_data);
				}
			},
			Err(e) => {
				eprintln!("cannot read file {:?}: {}", file, e);
			},
		}
	}

	// With the analyzed markov data, generate some random bytes
	generate(&markov_data, count);
}

fn analyze(bytes: &[u8], buckets: &mut [[u32; 256]; 256]) {
	if bytes.len() == 0 {
		return;
	}

	let mut bucket = bytes[0] as usize;
	for &byte in &bytes[1..] {
		buckets[bucket][byte as usize] += 1;
		bucket = byte as usize;
	}
}

fn generate(buckets: &[[u32; 256]; 256], count: usize) {
	use rand::{random, Rng, thread_rng};
	let mut rng = thread_rng();

	// Start with a random byte (not weighted :/)
	let mut byte = random::<u8>();

	for _ in 0..count {
		// Select the bucket which stores the weights for the next byte
		let bucket = &buckets[byte as usize];

		// Total number of follow bytes
		let total = bucket.iter().cloned().sum::<u32>();
		// Retry a random byte if there are no follow bytes for this one...
		if total == 0 {
			byte = random::<u8>();
			continue;
		}

		// Print the selected byte
		print!("{:02X} ", byte);

		// Pick the next byte based on the weight for this byte
		// FIXME! Use weighted reservoir sampling instead but this is more simple...
		let mut pick = rng.gen_range(0, total);
		for i in 0..256 {
			if pick < bucket[i] {
				byte = i as u8;
				break;
			}
			pick -= bucket[i];
		}
	}

	println!();
}
