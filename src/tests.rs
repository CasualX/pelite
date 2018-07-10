/*!
Run tests on a variety of cute binaries.
 */

use crate::{pe32, pe64, PeFile};

#[path = "../tests/pocs/pocs.rs"]
mod pocs;

macro_rules! test {
	($image:expr, $module:ident) => {
		match PeFile::from_bytes(&$image) {
			Ok(PeFile::Pe32(pe)) => pe32::$module::test(pe),
			Ok(PeFile::Pe64(pe)) => pe64::$module::test(pe),
			Err(err) => Err(err),
		}
	};
}

#[test]
fn pocs() {
	for (file_name, image) in pocs::iter() {
		println!("\n{}", file_name);

		let image = image.to_vec(); // Fix alignment
		println!("  base_relocs...    {:?}", test!(image, base_relocs));
		println!("  rich_structure... {:?}", test!(image, rich_structure));
		println!("  exception...      {:?}", test!(image, exception));
		println!("  exports...        {:?}", test!(image, exports));
		println!("  imports...        {:?}", test!(image, imports));
		println!("  debug...          {:?}", test!(image, debug));
		println!("  load_config...    {:?}", test!(image, load_config));
		println!("  security...       {:?}", test!(image, security));
		println!("  tls...            {:?}", test!(image, tls));
		println!("  resources...      {:?}", test!(image, resources));
		println!("  scanner...        {:?}", test!(image, scanner));
	}
}
