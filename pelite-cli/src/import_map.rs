//! Text address map for the import fixer.
//!
//! Each tab-separated line is:
//! - `0xBASE<TAB>DLL`
//! - `0xADDRESS<TAB>DLL<TAB>NAME` (with `#ORDINAL` in place of NAME).

use super::*;
use pelite::{PeFile, PeMemory};
use pelite::import_rebuild::{ImportMap, ResolvedImport, Symbol};

fn dll_name(path: &str) -> Result<Vec<u8>> {
	let name = path.rsplit(['/', '\\']).next().unwrap_or("");
	if name.is_empty() || !name.is_ascii() || name.as_bytes().contains(&0) { return Err(err("invalid DLL filename")); }
	Ok(name.to_ascii_lowercase().into_bytes())
}

fn add(map: &mut ImportMap, address: u64, dll: &[u8], symbol: Symbol) {
	let item = ResolvedImport { dll: dll.to_vec(), symbol };
	let entry = map.entry(address).or_default();
	if !entry.contains(&item) { entry.push(item); }
}

fn find_dll(map_path: &Path, name: &str, dirs: &[PathBuf]) -> Result<PathBuf> {
	let path = Path::new(name);
	let local = map_path.parent().unwrap_or(Path::new(".")).join(path);
	if local.is_file() { return Ok(local); }
	let filename = name.rsplit(['/', '\\']).next().unwrap_or(name);
	for dir in dirs {
		let candidate = dir.join(filename);
		if candidate.is_file() { return Ok(candidate); }
	}
	Err(err(format!("DLL not found: {name}")))
}

fn expand(map: &mut ImportMap, map_path: &Path, dirs: &[PathBuf], base: u64, name: &str) -> Result {
	let path = find_dll(map_path, name, dirs)?;
	let bytes = PeMemory::open(&path)?;
	let pe = PeFile::from_bytes(&bytes)?;
	let by = pe.exports()?.by()?;
	let dll = dll_name(name)?;
	let mut names: BTreeMap<usize, Vec<Vec<u8>>> = BTreeMap::new();
	for (index, &function_index) in by.name_indices().iter().enumerate() {
		let name_rva = by.names()[index];
		let symbol = pe.derva_c_str(name_rva)?.as_ref().to_vec();
		names.entry(function_index as usize).or_default().push(symbol);
	}
	let ordinal_base = by.image().Base;
	for (index, export) in by.iter().enumerate() {
		let Some(rva) = export.ok().and_then(|item| item.symbol()) else { continue; }; // forwarders and holes have no direct VA
		let address = base.checked_add(rva as u64).ok_or_else(|| err("export address overflow"))?;
		if let Some(aliases) = names.get(&index) {
			for symbol in aliases { add(map, address, &dll, Symbol::Name(symbol.clone())); }
		} else if let Some(ordinal) = ordinal_base.checked_add(index as u32).and_then(|n| u16::try_from(n).ok()) {
			add(map, address, &dll, Symbol::Ordinal(ordinal));
		}
	}
	Ok(())
}

pub fn read(path: &Path, dirs: &[PathBuf]) -> Result<ImportMap> {
	let input = fs::read_to_string(path)?;
	let mut modules = Vec::new();
	let mut explicit = Vec::new();
	for (index, line) in input.lines().enumerate() {
		let line = line.trim_end_matches('\r');
		if line.is_empty() || line.starts_with('#') { continue; }
		let fields: Vec<_> = line.split('\t').collect();
		if !(2..=3).contains(&fields.len()) { return Err(err(format!("{}:{}: expected 2 or 3 tab-separated fields", path.display(), index + 1))); }
		let address = fields[0].strip_prefix("0x").and_then(|s| u64::from_str_radix(s, 16).ok())
			.ok_or_else(|| err(format!("{}:{}: invalid hex address", path.display(), index + 1)))?;
		let dll = dll_name(fields[1])?;
		if fields.len() == 2 {
			modules.push((address, fields[1].to_owned()));
		} else {
			let symbol = if let Some(ord) = fields[2].strip_prefix('#') {
				Symbol::Ordinal(ord.parse::<u16>().map_err(|_| err(format!("{}:{}: invalid ordinal", path.display(), index + 1)))?)
			} else if !fields[2].is_empty() && fields[2].is_ascii() {
				Symbol::Name(fields[2].as_bytes().to_vec())
			} else { return Err(err(format!("{}:{}: invalid symbol", path.display(), index + 1))); };
			explicit.push((address, dll, symbol));
		}
	}
	let mut map = ImportMap::new();
	for (base, name) in modules { expand(&mut map, path, dirs, base, &name)?; }
	let mut overridden = BTreeSet::new();
	for (address, dll, symbol) in explicit {
		// Explicit entries replace inferred aliases, while preserving multiple explicit names.
		if overridden.insert((address, dll.clone())) {
			map.entry(address).or_default().retain(|item| item.dll != dll);
		}
		add(&mut map, address, &dll, symbol);
	}
	Ok(map)
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn reads_module_and_explicit_rows() {
		let path = std::env::temp_dir().join(format!("pelite-import-map-{}.tsv", std::process::id()));
		let dll = Path::new(env!("CARGO_MANIFEST_DIR")).join("../demo/Demo64.dll");
		fs::write(&path, format!("# address map\n0x10000000\t{}\n0x1234\tDemo64.dll\t#17\n", dll.display())).unwrap();
		let map = read(&path, &[]).unwrap();
		fs::remove_file(&path).unwrap();
		assert!(map.len() > 1);
		assert_eq!(map[&0x1234][0].symbol, Symbol::Ordinal(17));
		assert!(map.contains_key(&0x10001230));
	}
}
