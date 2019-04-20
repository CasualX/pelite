use std::collections::HashSet;

use pelite::pe64::*;
use pelite::pattern as pat;

/*
Dump encrypted strings.

Each string is stored in a following structure:

```
	+0x00 xor_key: [u8; 8] ; Simple 8 byte xor key
	+0x08 data_len: u32    ; Low 24 bits store a length, high byte is 1 if not yet decrypted, 0 if decrypted.
	+0x0c data: [u8]       ; Variable length array with length equal to data_len
```

The decryption initializer is a pretty simple xor decrypt loop.
There are a few variants, with a good signature match all of them.
*/

pub fn print(pefile: PeFile, patched: &mut [u8]) {
	let mut save = [0; 4];
	let mut matches = pefile.scanner().matches_code(pat!("8B 05 ${'} 25 FF FF FF 00 3B D0 72"));
	let mut strings = HashSet::new();
	while matches.next(&mut save) {
		let address = save[1] - 8;
		if let Err(err) = dumpstring(pefile, patched, address, &mut strings) {
			eprintln!("Error[{}]: cannot dump string at {:#x}", err, address);
		}
	}
	let mut strings = strings.drain().collect::<Vec<String>>();
	strings.sort_unstable();
	println!("# Encrypted strings\n");
	println!("<details><summary>Click to expand!</summary>\n");
	for s in &strings {
		println!("* `{:?}`", s);
	}
	println!("\n</details>");
}

fn dumpstring(pefile: PeFile, patched: &mut [u8], address: u32, strings: &mut HashSet<String>) -> pelite::Result<()> {
	let xor_key = pefile.derva_copy::<[u8; 8]>(address)?;
	let data_len = pefile.derva_copy::<u32>(address + 0x08)?;
	let data_offset = pefile.rva_to_file_offset(address + 0x0c)?;
	let data = patched.get_mut(data_offset..data_offset + (data_len & 0xffffff) as usize).ok_or(pelite::Error::Bounds)?;
	if data_len & !0xffffff != 0 {
		for i in 0..data.len() {
			let x = xor_key[i & 7];
			data[i] ^= x;
		}
		patched[data_offset - 3] = 0;
	}
	let data = patched.get_mut(data_offset..data_offset + (data_len & 0xffffff) as usize).ok_or(pelite::Error::Bounds)?;
	strings.insert(String::from_utf8_lossy(data).into_owned());
	Ok(())
}
