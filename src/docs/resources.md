Returns the resource directory.

See [`crate::resources`] for its API.

# Errors

* [`Null`][crate::Error::Null]: The image has no resource directory.
* [`Bounds`][crate::Error::Bounds]: The directory entry or its address lies outside the image.
* [`ZeroFill`][crate::Error::ZeroFill] or [`Invalid`][crate::Error::Invalid]: The directory refers to unavailable or invalid raw data in a file image.

# Examples

```rust
use pelite::pe64::{Pe, PeFile};
use pelite::resources::ResourceFindError;

# #[allow(dead_code)]
fn manifest<'a>(file: PeFile<'a>) -> Result<&'a [u8], ResourceFindError> {
	// Access the resource directory
	let resources = file.resources()?;

	// Find the manifest resource and return its bytes
	Ok(resources.find_data("/Manifest/2/1033")?.bytes()?)
}
```
