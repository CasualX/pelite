/*!
Writes a [Module-Defintion](https://msdn.microsoft.com/en-us/library/28d6s79h.aspx) file for the given binary.

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

use std::fmt;
use crate::util::CStr;
use crate::{PeFile, Result};

pub fn main(pefile: PeFile<'_>, f: &mut fmt::Write) -> Result<()> {
	let dll_name;
	let (by32, by64);
	let (mut names32, mut names64);
	let names: &mut Iterator<Item = Result<&'_ CStr>>;

	match pefile {
		PeFile::Pe32(pe) => {
			use crate::pe32::Pe;
			by32 = pe.exports()?.by()?;
			dll_name = by32.dll_name()?;
			names32 = by32.iter_names().map(move |(name, _)| name);
			names = &mut names32;
		},
		PeFile::Pe64(pe) => {
			use crate::pe64::Pe;
			by64 = pe.exports()?.by()?;
			dll_name = by64.dll_name()?;
			names64 = by64.iter_names().map(move |(name, _)| name);
			names = &mut names64;
		},
	}

	let _ = write!(f, "LIBRARY {}\nEXPORTS\n", dll_name);
	for name in names {
		// It's ok to skip the broken stuff
		if let Ok(name) = name {
			let _ = write!(f, "{}\n", name);
		}
	}

	Ok(())
}
