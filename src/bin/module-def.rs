/*!
Prints a [Module-Defintion](https://msdn.microsoft.com/en-us/library/28d6s79h.aspx) file for the given input DLL.

```bat
cargo run --bin module-def -- "demo\Demo64.dll" > "demo\Demo64.DEF"
cargo run --bin module-def -- "demo\Demo.dll" > "demo\Demo.DEF"
```

An Import Library can be created from the Module-Definition file.
Note that this needs access the VC build tools.

```bat
vcvarsall x64
lib /def:"demo\Demo64.DEF" /out:"demo\Demo64.LIB" /machine:x64
```

Also works for 32-bit binaries using the 32-bit VC build tools and commands.

```bat
vcvarsall x86
lib /def:"demo\Demo.DEF" /out:"demo\Demo.LIB" /machine:x86
```
 */

use std::env;

//----------------------------------------------------------------

const HELP_TEXT: &str = r#"\
PE Module-Definition Generator from DLL.

To create an import library run the following command afterwards:
    lib /def:"MODULE.DEF" /out:"MODULE.LIB" /machine:[x86|x64]

Usage:
    module-def "MODULE.DLL" > "MODULE.DEF"
"#;

fn main() {
    let mut args = env::args_os();
    if let (Some(_), Some(dll), None) = (args.next(), args.next(), args.next()) {
        match pelite::FileMap::open(&dll) {
            Ok(map) => {
                // Try PE32 and PE32+
                let result = match pelite::PeFile::from_bytes(&map) {
                    Ok(pe) => lib(pe),
                    Err(err) => Err(err),
                };
                // Display errors
                if let Err(err) = result {
                    eprintln!("module-def: {}", err);
                }
            }
            Err(err) => {
                eprintln!("module-def: {}", err);
            }
        };
    } else {
        println!("{}", HELP_TEXT);
    }
}

fn lib(pe: pelite::PeFile) -> pelite::Result<()> {
    let exp = pe.exports()?.by()?;
    let dll_name = exp.dll_name()?;
    let names = exp
        .iter_names()
        .map(|(name, _)| name)
        .collect::<pelite::Result<Vec<_>>>()?;

    println!("LIBRARY {}\nEXPORTS", dll_name);
    for name in &names {
        println!("{}", name);
    }

    Ok(())
}
