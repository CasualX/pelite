/*!
Writes a [Module-Defintion](https://msdn.microsoft.com/en-us/library/28d6s79h.aspx) file for the given input DLL.

```bat
cargo run --bin pemoddef -- "demo\Demo64.dll" > "demo\Demo64.def"
```

Creates an Import Library from the Module-Definition file.
Note that this needs access the VC build tools for `vcvarsall`.

```bat
vcvarsall x64
lib /def:"demo\Demo64.def" /out:"demo\Demo64.LIB" /machine:x64
```
*/

extern crate pelite;

use std::{env};

//----------------------------------------------------------------

const HELP_TEXT: &str = r#"\
PE Module-Definition Generator from DLL.

To create an import library run the following command afterwards:
    lib /def:"MODULE.DEF" /out:"MODULE.LIB" /machine:x86 | x64

Usage:
    pemoddef "MODULE.DLL" > "MODULE.DEF"
"#;

fn main() {
	let mut args = env::args_os();
	if let (Some(_), Some(dll), None) = (args.next(), args.next(), args.next()) {
		match pelite::FileMap::open(&dll) {
			Ok(map) => {
				// Try PE32 and PE32+
				let result = pelite::pe32::PeFile::from_bytes(&map).map(lib_pe32)
					.or_else(|_| pelite::pe64::PeFile::from_bytes(&map).map(lib_pe64));
				// Display errors
				if let Err(err) = result {
					eprintln!("pemoddef: {}", err);
				}
			},
			Err(err) => {
				eprintln!("pemoddef: {}", err);
			},
		};
	}
	else {
		println!("{}", HELP_TEXT);
	}
}

//----------------------------------------------------------------

fn lib_pe32(file: pelite::pe32::PeFile) -> pelite::Result<()> {
	use pelite::pe32::Pe;

	let exp = file.exports()?.by()?;
	let dll_name = exp.dll_name()?;
	let names =
		(0..exp.names().len())
		.map(|hint| exp.hint_name(hint))
		.collect::<pelite::Result<Vec<_>>>()?;

	println!("LIBRARY {}\nEXPORTS", dll_name);
	for name in &names {
		println!("{}", name);
	}

	Ok(())
}

fn lib_pe64(file: pelite::pe64::PeFile) -> pelite::Result<()> {
	use pelite::pe64::Pe;

	let exp = file.exports()?.by()?;
	let dll_name = exp.dll_name()?;
	let names =
		(0..exp.names().len())
		.map(|hint| exp.hint_name(hint))
		.collect::<pelite::Result<Vec<_>>>()?;

	println!("LIBRARY {}\nEXPORTS", dll_name);
	for name in &names {
		println!("{}", name);
	}

	Ok(())
}
