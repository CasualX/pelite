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
    let data = resources.find_data("/Manifest/2/1033")?;
    let manifest = data.bytes()?;

    Ok(manifest)
}
```
*/

#[cfg(test)]
pub(crate) fn test<'a, P: super::Pe<'a>>(pe: P) -> crate::Result<()> {
    pe.resources().and_then(crate::resources::test)
}
