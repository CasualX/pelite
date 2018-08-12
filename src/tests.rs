/*!
Run tests on a variety of cute binaries.
 */

use {pe32, pe64, Error};

#[path = "../tests/pocs/pocs.rs"]
mod pocs;

macro_rules! test {
	($image:expr, $module:ident) => {
		match pe32::PeFile::from_bytes(&$image) {
			Ok(image) => pe32::$module::test(image),
			Err(err) if err == Error::BadMagic => pe64::PeFile::from_bytes(&$image).and_then(pe64::$module::test),
			err => err.map(|_| ()),
		}
	};
}

#[test]
fn pocs() {
	for (file_name, image) in pocs::iter() {
		println!("\n{}", file_name);

		let image = image.to_vec(); // Fix alignment
		println!("  base_relocs... {:?}", test!(image, base_relocs));
		println!("  exception...   {:?}", test!(image, exception));
		println!("  exports...     {:?}", test!(image, exports));
		println!("  imports...     {:?}", test!(image, imports));
		println!("  load_config... {:?}", test!(image, load_config));
		println!("  security...    {:?}", test!(image, security));
		println!("  tls...         {:?}", test!(image, tls));
		println!("  resources...   {:?}", test!(image, resources));
		println!("  scanner...     {:?}", test!(image, scanner));
	}
}
