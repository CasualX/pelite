/*!
Run tests on a variety of cute binaries.
 */

use crate::{Error, pe32, pe64, PeFile, PeView, Wrap};

#[path = "../tests/pocs/pocs.rs"]
mod pocs;

#[test]
fn wrap_four_byte_aligned_nt_headers() {
	let (_, pe32) = pocs::iter().find(|(name, _)| *name == "tiny.exe").unwrap();
	assert!(matches!(PeFile::from_bytes(&pe32), Ok(Wrap::T32(_))));
	assert!(matches!(PeView::from_bytes(&pe32), Ok(Wrap::T32(_))));

	let (_, pe64) = pocs::iter().find(|(name, _)| *name == "tinyW7x64.exe").unwrap();
	assert!(matches!(PeFile::from_bytes(&pe64), Ok(Wrap::T64(_))));
	assert!(matches!(PeView::from_bytes(&pe64), Ok(Wrap::T64(_))));
}

macro_rules! test {
	($image:expr, $module:ident) => {
		match PeFile::from_bytes(&$image) {
			Ok(Wrap::T32(pe)) => pe32::$module::test(pe),
			Ok(Wrap::T64(pe)) => pe64::$module::test(pe),
			Err(err) => Err(err),
		}
	};
}

macro_rules! test_pe64 {
	($image:expr, $module:ident) => {
		match PeFile::from_bytes(&$image) {
			Ok(Wrap::T32(_)) => Err(Error::Invalid),
			Ok(Wrap::T64(pe)) => pe64::$module::test(pe),
			Err(err) => Err(err),
		}
	};
}

#[test]
fn pocs() {
	for (name, image) in pocs::iter() {
		println!("\n{}", name);

		println!("  base_relocs...    {:?}", test!(image, base_relocs));
		println!("  rich_structure... {:?}", test!(image, rich_structure));
		println!("  exception_x64...  {:?}", test_pe64!(image, exception_x64));
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
