/*!
Resource Directory.

# Examples

```
# #![allow(unused_variables)]
use pelite::pe64::{Pe, PeFile};
use pelite::{resources};

# #[allow(dead_code)]
fn example<'a>(file: PeFile<'a>) -> pelite::Result<Option<&'a [u8]>> {
	// Access the resources
	let resources = file.resources()?;

	// Navigate the directory structure
	let root = resources.root()?;

	// The `RT_MANIFEST` directory
	let dir_e = root.entries().find(|e| e.name() == Ok(resources::Name::Id(24)));
	let dir_e = match dir_e { Some(dir_e) => dir_e, _ => return Ok(None) };
	let dir = match dir_e.entry()? { resources::Entry::Directory(dir) => dir, _ => return Ok(None) };

	// Find the `ISOLATION_AWARE_RESOURCE_ID`
	let dir_e = dir.entries().find(|e| e.name() == Ok(resources::Name::Id(2)));
	let dir_e = match dir_e { Some(dir_e) => dir_e, _ => return Ok(None) };
	let dir = match dir_e.entry()? { resources::Entry::Directory(dir) => dir, _ => return Ok(None) };

	// Just grab the first language entry
	let dir_e = dir.entries().next();
	let dir_e = match dir_e { Some(dir_e) => dir_e, _ => return Ok(None) };
	let data_e = match dir_e.entry()? { resources::Entry::DataEntry(dir_e) => dir_e, _ => return Ok(None) };

	// The raw manifest data
	let manifest = data_e.data()?;

	Ok(Some(manifest))
}
```
*/
