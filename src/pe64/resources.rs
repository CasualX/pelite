/*!
Resource Directory.

See [here](../../resources/index.html) for the API docs.

# Examples

```
# #![allow(unused_variables)]
use pelite::pe64::{Pe, PeFile};
use pelite::resources::FindError;

# #[allow(dead_code)]
fn example<'a>(file: PeFile<'a>) -> Result<&'a [u8], FindError> {
	// Access the resources
	let resources = file.resources()?;

	// Find the desired resource by its path
	let entry = resources.find_data("/Manifest/2/1033")?;
	let manifest = entry.data()?;

	Ok(manifest)
}
```
*/
