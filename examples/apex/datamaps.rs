use std::mem;

use pelite;
use pelite::pattern as pat;
use pelite::{Pod, util::CStr};
use pelite::pe64::*;

pub fn print(bin: PeFile<'_>, _dll_name: &str) {
	let datamaps = datamaps(bin);

	println!("## Datamaps\n");
	for datamap in &datamaps {
		println!("<details>");
		print!("<summary><code>class {}", datamap.name);
		if let Some(base) = datamap.base {
			print!(" extends {}", base);
		}
		println!("</code></summary>\n\n```\n{{");
		for field in &datamap.fields {
			println!("\t{}: {},", field.name, field.ty);
		}
		println!("}}\n```\n\n### Offsets\n\n```");
		for field in &datamap.fields {
			println!("{}!{:#06x} {}", datamap.name, field.offset, field.name);
		}
		println!("```\n</details>");
	}
	println!();
}

#[allow(non_snake_case)]
#[derive(Copy, Clone, Pod)]
#[repr(C)]
struct typedescription_t {
	fieldType: i32,
	fieldName: Ptr<CStr>,
	fieldOffset: [u32; 2],
	externalName: Ptr<CStr>,
	_unk0: [u64; 5], // seen: [0, 0, 0, 0x7FF6FFFFFFFF, 0xFFFFFFFF]
	td: Ptr<datamap_t>,
	fieldSizeInBytes: i32,
	_unk_bool: u8,
	_unk_four: u32, // seen: 4
	_unk1: [u64; 3], // seen: [0, 0, fieldOffset[0]]
	_unk_word: u16,
} // sizeof = 0x80
#[allow(dead_code)]
const SIZE_OF_TYPE_DESCRIPTION: [(); mem::size_of::<typedescription_t>()] = [(); 0x80];

#[allow(non_snake_case)]
#[derive(Copy, Clone, Pod)]
#[repr(C)]
struct datamap_t {
	dataDesc: Ptr<[typedescription_t]>,
	dataNumFields: i32,
	dataClassName: Ptr<CStr>,
	_unk0: u64,
	_unk1: u64, // chains_validated ?
	baseMap: Ptr<datamap_t>,
}

struct Field<'a> {
	ty: &'a str,
	offset: u32,
	name: &'a str,
}
static FIELD_TYPES: [&str; 32] = [
	"Void", "Float", "String", "Vector", "Quaternion",
	"Int", "Bool", "Short", "Char", "Color32",
	"Embedded", "Custom", "ClassPtr", "EHANDLE", "edict",
	"PositionVector", "Time", "Tick", "ModelName", "SoundName",
	"20", "21", "22", "23", "24",
	"25","26", "27", "28", "29",
	"30", "Outer",
];

struct DataMap<'a> {
	name: &'a str,
	base: Option<&'a str>,
	fields: Vec<Field<'a>>,
}

fn datamaps<'a>(bin: PeFile<'a>) -> Vec<DataMap<'a>> {
	let mut save = [0; 4];
	let mut list = Vec::new();
	let mut addresses = Vec::new();

	// Ugh this is kinda shitty
	// Every entity class has a virtual method to return the address of its datamap
	// So try to find those and filter out all false positives?
	let mut matches = bin.scanner().matches_code(pat!("48 8D 05 $ {'} C3 CC CC CC CC CC CC CC CC"));
	while matches.next(&mut save) {
		addresses.push(save[1]);
	}

	// Analyze any datamaps found and add any extras
	let mut i = 0;
	while i < addresses.len() {
		if let Ok(dm) = datamap(bin, addresses[i], &mut addresses) {
			list.push(dm);
		}
		i += 1;
	}

	list.sort_unstable_by_key(|dm| dm.name);
	return list;
}
fn datamap<'a>(bin: PeFile<'a>, address: u32, addresses: &mut Vec<u32>) -> pelite::Result<DataMap<'a>> {
	let datamap = bin.derva::<datamap_t>(address)?;
	let datadesc = bin.deref_slice(datamap.dataDesc, datamap.dataNumFields as usize)?;
	let base = if datamap.baseMap.is_null() {
		None
	}
	else {
		let basemap = bin.deref(datamap.baseMap)?;
		let name = bin.deref_c_str(basemap.dataClassName)?.to_str().or(Err(pelite::Error::Encoding))?;
		Some(name)
	};
	let name = bin.deref_c_str(datamap.dataClassName)?.to_str().or(Err(pelite::Error::Encoding))?;
	let mut fields = Vec::new();
	for desc in datadesc {
		let ty = desc.fieldType;
		if ty as usize >= FIELD_TYPES.len() {
			return Err(pelite::Error::Invalid);
		}
		let ty = if desc.td.is_null() {
			FIELD_TYPES[ty as usize]
		}
		else {
			if !desc.td.is_null() {
				let td_rva = bin.va_to_rva(desc.td.into())?;
				if !addresses.iter().any(|&td| td == td_rva) {
					addresses.push(td_rva);
				}
			}
			let td = bin.deref(desc.td)?;
			let name = bin.deref_c_str(td.dataClassName)?.to_str().or(Err(pelite::Error::Encoding))?;
			name
		};
		let offset = desc.fieldOffset[0];
		let name = bin.deref_c_str(desc.fieldName)?.to_str().or(Err(pelite::Error::Encoding))?;
		fields.push(Field { ty, offset, name });
	}
	fields.sort_by_key(|field| field.offset);
	Ok(DataMap { name, base, fields })
}
