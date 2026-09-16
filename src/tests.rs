/*!
Run tests on a variety of cute binaries.
 */

use crate::{Error, PeFile, PeView, Wrap, pe32, pe64};

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
	($image:expr, $test:ident) => {
		match PeFile::from_bytes(&$image) {
			Ok(Wrap::T32(pe)) => pe32::$test(pe),
			Ok(Wrap::T64(pe)) => pe64::$test(pe),
			Err(err) => Err(err),
		}
	};
}

macro_rules! test_pe64 {
	($image:expr, $test:ident) => {
		match PeFile::from_bytes(&$image) {
			Ok(Wrap::T32(_)) => Err(Error::Invalid),
			Ok(Wrap::T64(pe)) => pe64::$test(pe),
			Err(err) => Err(err),
		}
	};
}

#[test]
fn pocs() {
	for (name, image) in pocs::iter() {
		println!("\n{}", name);

		println!("  base_relocs...    {:?}", test!(image, test_base_relocs));
		println!("  rich_structure... {:?}", test!(image, test_rich_structure));
		println!("  exception_x64...  {:?}", test_pe64!(image, test_exception_x64));
		println!("  exports...        {:?}", test!(image, test_exports));
		println!("  imports...        {:?}", test!(image, test_imports));
		println!("  debug...          {:?}", test!(image, test_debug));
		println!("  load_config...    {:?}", test!(image, test_load_config));
		println!("  security...       {:?}", test!(image, test_security));
		println!("  tls...            {:?}", test!(image, test_tls));
		println!("  resources...      {:?}", test!(image, test_resources));
		println!("  scanner...        {:?}", test!(image, test_scanner));
	}
}

#[cfg(feature = "serde")]
#[test]
fn serialize_pocs() {
	fn check_load_config(config: impl serde::Serialize) {
		let value = serde_json::to_value(config).unwrap();
		let fields = value.as_object().unwrap();
		assert_eq!(fields.len(), 3);
		assert!(fields.contains_key("security_cookie"));
		assert!(fields.contains_key("se_handler_table"));
		let image = fields["image"].as_object().unwrap();
		assert_eq!(image.len(), 49);
		assert!(image.contains_key("GlobalFlagsClear"));
		assert!(image.contains_key("SecurityCookie"));
		assert!(image.contains_key("CodeIntegrity"));
		assert!(image.contains_key("UmaFunctionPointers"));
	}

	let mut load_configs = 0;
	for (name, image) in pocs::iter() {
		if let Ok(pe) = PeFile::from_bytes(&image) {
			serde_json::to_vec(&pe).unwrap_or_else(|err| panic!("failed to serialize {name}: {err}"));
			match pe {
				Wrap::T32(pe) => if let Ok(config) = pe32::Pe::load_config(pe) {
					check_load_config(config);
					load_configs += 1;
				},
				Wrap::T64(pe) => if let Ok(config) = pe64::Pe::load_config(pe) {
					check_load_config(config);
					load_configs += 1;
				},
			}
		}
	}
	assert!(load_configs > 0);
}
