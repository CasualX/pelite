//! Exercise the public API on corkami's PE proof-of-concept files.
//! Run this suite on its own with `cargo test --test pocs`.

use pelite::{Error, PeFile, PeView, Wrap};

mod pocs;

#[test]
fn wrap_four_byte_aligned_nt_headers() {
	let (_, pe32) = pocs::iter().find(|(name, _)| *name == "tiny.exe").unwrap();
	assert!(matches!(PeFile::from_bytes(pe32), Ok(Wrap::T32(_))));
	assert!(matches!(PeView::from_bytes(pe32), Ok(Wrap::T32(_))));

	let (_, pe64) = pocs::iter().find(|(name, _)| *name == "tinyW7x64.exe").unwrap();
	assert!(matches!(PeFile::from_bytes(pe64), Ok(Wrap::T64(_))));
	assert!(matches!(PeView::from_bytes(pe64), Ok(Wrap::T64(_))));
}

macro_rules! check {
	($image:expr, $test:ident) => {
		match PeFile::from_bytes($image) {
			Ok(Wrap::T32(pe)) => checks32::$test(pe),
			Ok(Wrap::T64(pe)) => checks64::$test(pe),
			Err(err) => Err(err),
		}
	};
}

#[test]
fn pocs() {
	for (name, image) in pocs::iter() {
		println!("\n{name}");
		println!("  base_relocs...    {:?}", check!(image, base_relocs));
		println!("  rich_structure... {:?}", check!(image, rich_structure));
		println!("  exception_x64...  {:?}", match PeFile::from_bytes(image) {
			Ok(Wrap::T64(pe)) => checks64::exception_x64(pe),
			Ok(Wrap::T32(_)) => Err(Error::Invalid),
			Err(err) => Err(err),
		});
		println!("  exports...        {:?}", check!(image, exports));
		println!("  imports...        {:?}", check!(image, imports));
		println!("  debug...          {:?}", check!(image, debug));
		println!("  load_config...    {:?}", check!(image, load_config));
		println!("  security...       {:?}", check!(image, security));
		println!("  tls...            {:?}", check!(image, tls));
		println!("  resources...      {:?}", check!(image, resources));
		println!("  scanner...        {:?}", check!(image, scanner));
	}
}

#[cfg(feature = "serde")]
#[test]
fn serialize_pocs() {
	fn check_load_config(config: impl serde::Serialize) {
		let value = serde_json::to_value(config).unwrap();
		let fields = value.as_object().unwrap();
		assert_eq!(fields.len(), 4);
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
		if let Ok(pe) = PeFile::from_bytes(image) {
			serde_json::to_vec(&pe).unwrap_or_else(|err| panic!("failed to serialize {name}: {err}"));
			match pe {
				Wrap::T32(pe) => if let Ok(config) = pelite::pe32::Pe::load_config(pe) {
					check_load_config(config);
					load_configs += 1;
				},
				Wrap::T64(pe) => if let Ok(config) = pelite::pe64::Pe::load_config(pe) {
					check_load_config(config);
					load_configs += 1;
				},
			}
		}
	}
	assert!(load_configs > 0);
}

macro_rules! checks {
	($module:ident, $pe:ident $(, $($extra:item)*)?) => {
		mod $module {
			use pelite::$pe::{self, Pe};
			use pelite::{Result, resources::{ResourceEntry, ResourceName}};

			pub fn base_relocs<'a, P: Copy + Pe<'a>>(pe: P) -> Result<()> {
				let relocs = pe.base_relocs()?;
				let _ = format!("{relocs:?}");
				let mut baseline = relocs.iter_blocks().flat_map(|block| {
					let _ = format!("{block:?}");
					block.words().iter().filter(move |word| block.type_of(word) != pelite::image::IMAGE_REL_BASED_ABSOLUTE).map(move |word| block.rva_of(word))
				});
				relocs.for_each(|rva, _| assert_eq!(baseline.next(), Some(rva)));
				assert_eq!(baseline.next(), None);
				Ok(())
			}

			pub fn rich_structure<'a, P: Copy + Pe<'a>>(pe: P) -> Result<()> {
				let rich = pe.rich_structure()?;
				let _ = rich.checksum();
				let records: Vec<_> = rich.records().collect();
				let mut encoded = vec![0u32; rich.image().len()];
				let _ = rich.encode(&records, &mut encoded);
				Ok(())
			}

			pub fn exports<'a, P: Copy + Pe<'a>>(pe: P) -> Result<()> {
				let by = pe.exports()?.by()?;
				let _ = format!("{by:?}");
				let _ = (by.dll_name(), by.ordinal_base());
				let sorted = by.check_sorted()?;
				let mut occurrences = std::collections::HashMap::<_, i32>::new();
				for (name, _) in by.iter_names() {
					if let Ok(name) = name { *occurrences.entry(name).or_default() += 1; }
				}
				for (hint, (name, export)) in by.iter_names().enumerate() {
					assert_eq!(name, by.name_of_hint(hint));
					assert_eq!(export, by.hint(hint));
					if let Ok(name) = name {
						let unique = occurrences[name] == 1;
						if unique {
							assert_eq!(export, by.name_linear(name));
							if sorted { assert_eq!(export, by.name(name)); }
						}
						assert_eq!(export, by.hint_name(hint, name));
						assert_eq!(export, by.import($pe::Import::ByName { hint, name }));
						if sorted && unique {
							assert_eq!(export, by.hint_name(0, name));
							assert_eq!(export, by.import($pe::Import::ByName { hint: 0, name }));
						}
					}
				}
				Ok(())
			}

			pub fn imports<'a, P: Copy + Pe<'a>>(pe: P) -> Result<()> {
				let imports = pe.imports()?;
				let _ = format!("{imports:?}");
				for desc in imports {
					let _ = format!("{desc:?}");
					let _ = desc.dll_name();
					if let Ok(iat) = desc.iat() { for _ in iat {} }
					if let Ok(int) = desc.int() { for _ in int {} }
				}
				// Check the combined address table against an independent decode.
				fn decode<'a, P: Copy + Pe<'a>>(pe: P, &va: &'a $pe::Va) -> Result<$pe::Import<'a>> {
					if va & $pe::image::IMAGE_ORDINAL_FLAG == 0 {
						let rva = $pe::Rva::try_from(va).map_err(|_| pelite::Error::Overflow)?;
						let hint = pe.derva::<u16>(rva)?;
						let name_rva = rva.checked_add(2).ok_or(pelite::Error::Overflow)?;
						let name = pe.derva_c_str(name_rva)?;
						Ok($pe::Import::ByName { hint: *hint as usize, name })
					} else {
						if va & !($pe::image::IMAGE_ORDINAL_FLAG | u16::MAX as $pe::Va) != 0 {
							return Err(pelite::Error::Invalid);
						}
						Ok($pe::Import::ByOrdinal { ord: va as $pe::image::Ordinal })
					}
				}
				for (va, import) in pe.iat()?.iter() {
					let _ = format!("{import:?}");
					if import.is_ok() { assert_eq!(decode(pe, va), import); }
				}
				Ok(())
			}

			pub fn debug<'a, P: Copy + Pe<'a>>(pe: P) -> Result<()> {
				for dir in pe.debug()? {
					let _ = dir.data();
					match dir.entry() {
						Ok(Some($pe::DebugData::CodeView(cv))) => { let _ = (cv.format(), cv.pdb_file_name()); },
						Ok(Some($pe::DebugData::Pgo(pgo))) => for _ in pgo {},
						_ => (),
					}
				}
				Ok(())
			}

			pub fn load_config<'a, P: Copy + Pe<'a>>(pe: P) -> Result<()> {
				let config = pe.load_config()?;
				let _ = format!("{config:?}");
				let _ = (config.image_copy(), config.size(), config.time_date_stamp(), config.version(), config.security_cookie(), config.se_handler_table());
				Ok(())
			}

			pub fn security<'a, P: Copy + Pe<'a>>(pe: P) -> Result<()> {
				let security = pe.security()?;
				let _ = format!("{security:?}");
				let _ = (security.certificate_type(), security.certificate_data());
				Ok(())
			}

			pub fn tls<'a, P: Copy + Pe<'a>>(pe: P) -> Result<()> {
				let tls = pe.tls()?;
				let _ = format!("{tls:?}");
				let _ = (tls.raw_data(), tls.slot(), tls.callbacks());
				Ok(())
			}

			pub fn resources<'a, P: Copy + Pe<'a>>(pe: P) -> Result<()> {
				fn visit(dir: pelite::resources::ResourceDirectoryTable<'_>) {
					let _ = (format!("{dir}"), format!("{dir:?}"));
					for entry in dir.entries() {
						if let Ok(name) = entry.name() {
							match name {
								ResourceName::Id(_) => assert!(dir.id_entries().any(|e| e.name() == Ok(name))),
								ResourceName::Wide(_) => assert!(dir.named_entries().any(|e| e.name() == Ok(name))),
								ResourceName::Str(_) => unreachable!(),
							}
						}
						match entry.entry() {
							Ok(ResourceEntry::Data(data)) => { assert!(!entry.is_dir()); let _ = (data.size(), data.code_page(), data.bytes()); },
							Ok(ResourceEntry::Directory(dir)) => { assert!(entry.is_dir()); visit(dir); },
							Err(_) => (),
						}
					}
				}
				let resources = pe.resources()?;
				let _ = resources.fsck();
				let _ = format!("{resources}");
				if let Ok(info) = resources.version_info() {
					let _ = (info.fixed(), info.translation(), info.file_info(), info.source_code());
				}
				resources.root().map(visit)
			}

			pub fn scanner<'a, P: Copy + Pe<'a>>(pe: P) -> Result<()> {
				use pelite::pattern::Atom::*;
				let scanner = pe.scanner();
				let mut save = [0; 4];
				let mut matches = scanner.code().matches(&[Save(0), Byte(0xE8), Save(-1), Jump4, Save(1), Seek(-1), Skip(4), Save(2)]);
				while matches.next(&mut save).is_some() { assert_eq!(save[0] + 5, save[2]); }
				let mut matches = scanner.code().matches(&[Jump1, Save(1), Byte(0x0F), Byte(0x0D)]);
				while matches.next(&mut save).is_some() {}
				let _ = scanner.code().find(&[Byte(0x8B), Byte(0x01), Byte(0x8B), Byte(0x10), Byte(0xFF), Byte(0xD2)], &mut save);
				Ok(())
			}
			$($($extra)*)?
		}
	};
}

checks!(checks32, pe32,);
checks!(checks64, pe64,
	pub fn exception_x64<'a, P: Copy + Pe<'a>>(pe: P) -> Result<()> {
		let exception = pe.exception_x64()?;
		let _ = format!("{exception:?}");
		let sorted = exception.check_sorted();
		for (index, function) in exception.functions().enumerate() {
			let _ = (format!("{function:?}"), function.bytes());
			if sorted {
				for pc in function.image().BeginAddress..function.image().EndAddress {
					assert_eq!(exception.index_of(pc), Ok(index));
				}
			}
			if let Ok(info) = function.unwind_info() {
				let _ = (format!("{info:?}"), info.version(), info.flags(), info.size_of_prolog(), info.frame_register(), info.frame_offset(), info.unwind_codes());
			}
		}
		Ok(())
	}
);
