use super::*;

pub struct PeFile {
	image: Box<[u8]>,
}
impl AsRef<[u8]> for PeFile {
	fn as_ref(&self) -> &[u8] {
		self.image.as_ref()
	}
}

#[unsafe(export_name = "pefileNew")]
pub unsafe fn new(data: *mut [u8]) -> *mut PeFile {
	unsafe {
		let mut return_value = ptr::null_mut();
		match take_bytes(data) {
			Some(image) => match pelite::PeFile::from_bytes(&image) {
				Ok(_) => return_value = Box::into_raw(Box::new(PeFile { image })),
				Err(err) => return_error(err),
			},
			None => return_error(pelite::Error::Null),
		}
		return return_value;
	}
}

#[unsafe(export_name = "pefileDrop")]
pub unsafe fn drop(pefile: *mut PeFile) {
	unsafe {
		let _ = Box::from_raw(pefile);
	}
}

#[unsafe(export_name = "pefileDosHeader")]
pub unsafe fn dos_header(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_json(pefile.dos_header());
}

#[unsafe(export_name = "pefileNtHeaders")]
pub unsafe fn nt_headers(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_json(pefile.nt_headers());
}

#[unsafe(export_name = "pefileFileHeader")]
pub unsafe fn file_header(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_json(pefile.file_header());
}

#[unsafe(export_name = "pefileOptionalHeader")]
pub unsafe fn optional_header(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_json(pefile.optional_header());
}

#[unsafe(export_name = "pefileSectionHeaders")]
pub unsafe fn section_headers(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_json(pefile.section_headers());
}

#[unsafe(export_name = "pefileHeaders")]
pub unsafe fn headers(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_json(pefile.headers());
}

#[unsafe(export_name = "pefileVaToRva")]
pub unsafe fn va_to_rva(pefile: *mut PeFile, va: u64) -> u32 {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => { return_error(err); return 0; }
	};

	let result = match pefile {
		pelite::Wrap::T32(pefile) => {
			match u32::try_from(va) {
				Ok(va) => pefile.va_to_rva(va),
				Err(_) => Err(pelite::Error::Overflow),
			}
		},
		pelite::Wrap::T64(pefile) => {
			pefile.va_to_rva(va)
		},
	};

	match result {
		Ok(rva) => { return_null(); rva }
		Err(err) => { return_error(err); 0 }
	}
}

#[unsafe(export_name = "pefileRvaToVa")]
pub unsafe fn rva_to_va(pefile: *mut PeFile, rva: u32) -> u64 {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => { return_error(err); return 0; }
	};

	let result = match pefile {
		pelite::Wrap::T32(pefile) => {
			pefile.rva_to_va(rva).map(|va| va as u64)
		},
		pelite::Wrap::T64(pefile) => {
			pefile.rva_to_va(rva)
		},
	};

	match result {
		Ok(va) => { return_null(); va }
		Err(err) => { return_error(err); 0 }
	}
}

#[derive(serde::Serialize)]
struct DisassembledInstruction<'a> {
	address: String,
	bytes: &'a [u8],
	instruction: String,
}

struct PeSymbolResolver {
	symbols: Arc<HashMap<u64, String>>,
}

impl iced_x86::SymbolResolver for PeSymbolResolver {
	fn symbol(&mut self, _instruction: &iced_x86::Instruction, _operand: u32, _instruction_operand: Option<u32>, address: u64, _address_size: u32) -> Option<iced_x86::SymbolResult<'_>> {
		self.symbols.get(&address).map(|name| iced_x86::SymbolResult::with_str(address, name))
	}
}

#[unsafe(export_name = "pefileDisasm")]
pub unsafe fn disasm(pefile: *mut PeFile, start: u32, end: u32) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	let bitness = match pefile.file_header().Machine {
		pelite::image::IMAGE_FILE_MACHINE_I386 => 32,
		pelite::image::IMAGE_FILE_MACHINE_AMD64 => 64,
		machine => return return_error(format!("unsupported machine type {machine:#06x}; expected i386 or AMD64")),
	};
	let len = match end.checked_sub(start).and_then(|len| usize::try_from(len).ok()) {
		Some(len) => len,
		None => return return_error("disassembly range end must not precede start"),
	};
	let bytes = match pefile.slice(start, len, 1) {
		Ok(bytes) => bytes,
		Err(err) => return return_error(err),
	};
	let image_base = pefile.image_base();
	let start_ip = image_base + start as u64;
	let end_ip = image_base + end as u64;

	let symbols = Arc::new(build_symbols(pefile, bitness, image_base));
	let mut decoder = iced_x86::Decoder::with_ip(bitness, bytes, start_ip, iced_x86::DecoderOptions::NONE);
	let mut formatter = iced_x86::IntelFormatter::with_options(Some(Box::new(PeSymbolResolver { symbols })), None);
	let mut instructions = Vec::new();
	while decoder.can_decode() && decoder.ip() < end_ip {
		let instruction = decoder.decode();
		let address = instruction.ip();
		let offset = (address - start_ip) as usize;
		let instruction_bytes = &bytes[offset..offset + instruction.len()];
		let section_name = u32::try_from(address - image_base).ok()
			.and_then(|rva| pefile.section_headers().by_rva(rva))
			.and_then(|section| section.name().ok())
			.unwrap_or("<no section>");
		let mut text = String::new();
		iced_x86::Formatter::format(&mut formatter, &instruction, &mut text);
		instructions.push(DisassembledInstruction {
			address: format!("{section_name}:{address:#x}"),
			bytes: instruction_bytes,
			instruction: text,
		});
	}

	return_json(instructions);
}

fn build_symbols(pefile: pelite::PeFile<'_>, bitness: u32, image_base: u64) -> HashMap<u64, String> {
	let mut symbols = HashMap::new();

	if let Ok(by) = pefile.exports().and_then(|exports| exports.by()) {
		let dll = by.dll_name().ok().and_then(|name| name.to_str().ok()).unwrap_or("<exports>");
		let mut names = HashMap::new();
		for (name, index) in by.iter_name_indices() {
			if let Ok(name) = name.and_then(|name| name.to_str().map_err(|_| pelite::Error::Encoding)) {
				names.insert(index, name.to_owned());
			}
		}
		for (index, export) in by.iter().enumerate() {
			let Some(rva) = export.ok().and_then(|export| export.symbol()) else {
				continue;
			};
			let name = names.remove(&index).unwrap_or_else(|| {
				let ordinal = u32::from(by.ordinal_base()).saturating_add(index as u32);
				format!("{dll}!#{ordinal}")
			});
			insert_symbol(&mut symbols, image_base, rva, name);
		}
	}

	if let Ok(imports) = pefile.imports() {
		let pointer_size = bitness / 8;
		for descriptor in imports {
			let Some(dll) = descriptor.dll_name().ok().and_then(|name| name.to_str().ok()) else {
				continue;
			};
			let first_thunk = descriptor.image().FirstThunk.get();
			let Ok(imports) = descriptor.int() else {
				continue;
			};
			for (index, import) in imports.enumerate() {
				let Ok(import) = import else {
					continue;
				};
				let Some(offset) = u32::try_from(index).ok().and_then(|index| index.checked_mul(pointer_size)) else {
					continue;
				};
				let Some(rva) = first_thunk.checked_add(offset) else {
					continue;
				};
				let name = match import {
					pelite::Import::ByName { name, .. } => {
						let Ok(name) = name.to_str() else {
							continue;
						};
						format!("{dll}!{name}")
					},
					pelite::Import::ByOrdinal { ord } => format!("{dll}!#{ord}"),
				};
				insert_symbol(&mut symbols, image_base, rva, name);
			}
		}
	}

	symbols
}

fn insert_symbol(symbols: &mut HashMap<u64, String>, image_base: u64, rva: u32, name: String) {
	if let Some(va) = image_base.checked_add(u64::from(rva)) {
		symbols.entry(va).or_insert(name);
	}
}

#[unsafe(export_name = "pefileSliceBytes")]
pub unsafe fn slice_bytes(pefile: *mut PeFile, rva: u32, min_size: usize, align_of: usize) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	let bytes = match pefile.slice(rva, min_size, align_of) {
		Ok(bytes) => bytes,
		Err(err) => return return_error(err),
	};

	return_slice(bytes);
}

#[unsafe(export_name = "pefileSliceCString")]
pub unsafe fn slice_cstring(pefile: *mut PeFile, rva: u32, utf8: bool) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	let cstr = match pefile.derva_c_str(rva) {
		Ok(cstr) => cstr,
		Err(err) => return return_error(err),
	};

	if utf8 {
		let s = match cstr.to_str() {
			Ok(s) => s,
			Err(err) => return return_error(err),
		};
		return_str(s);
	}
	else {
		return_str(&cstr.to_string());
	}
}

fn va_to_rva_impl(pefile: &pelite::PeFile, va: u64) -> pelite::Result<u32> {
	match pefile {
		pelite::Wrap::T32(pefile) => {
			let va = u32::try_from(va).map_err(|_| pelite::Error::Overflow)?;
			pefile.va_to_rva(va)
		},
		pelite::Wrap::T64(pefile) => {
			pefile.va_to_rva(va)
		},
	}
}

#[unsafe(export_name = "pefileReadBytes")]
pub unsafe fn read_bytes(pefile: *mut PeFile, va: u64, min_size: usize, align_of: usize) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	let bytes = match va_to_rva_impl(&pefile, va).and_then(|rva| pefile.slice(rva, min_size, align_of)) {
		Ok(bytes) => bytes,
		Err(err) => return return_error(err),
	};

	return_slice(bytes);
}

#[unsafe(export_name = "pefileReadCString")]
pub unsafe fn read_cstring(pefile: *mut PeFile, va: u64, utf8: bool) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	let cstr = match va_to_rva_impl(&pefile, va).and_then(|rva| pefile.derva_c_str(rva)) {
		Ok(cstr) => cstr,
		Err(err) => return return_error(err),
	};

	if utf8 {
		let s = match cstr.to_str() {
			Ok(s) => s,
			Err(err) => return return_error(err),
		};
		return_str(s);
	}
	else {
		return_str(&cstr.to_string());
	}
}

#[unsafe(export_name = "pefileRichStructure")]
pub unsafe fn rich_structure(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_pelite_result(pefile.rich_structure());
}

#[unsafe(export_name = "pefileImports")]
pub unsafe fn imports(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_pelite_result(pefile.imports());
}

#[unsafe(export_name = "pefileExports")]
pub unsafe fn exports(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_pelite_result(pefile.exports());
}

#[unsafe(export_name = "pefileBaseRelocations")]
pub unsafe fn base_relocations(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_pelite_result(pefile.base_relocs());
}

#[unsafe(export_name = "pefileLoadConfig")]
pub unsafe fn load_config(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_pelite_result(pefile.load_config());
}

#[unsafe(export_name = "pefileTls")]
pub unsafe fn tls(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_pelite_result(pefile.tls());
}

#[unsafe(export_name = "pefileExceptionsX64")]
pub unsafe fn exceptions_x64(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	match pefile {
		pelite::Wrap::T32(_) => {
			return_null();
		}
		pelite::Wrap::T64(pefile) => {
			return_pelite_result(pefile.exception_x64());
		}
	}
}

#[unsafe(export_name = "pefileDebug")]
pub unsafe fn debug(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_pelite_result(pefile.debug());
}

#[unsafe(export_name = "pefileSecurity")]
pub unsafe fn security(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	return_pelite_result(pefile.security());
}

fn resource_name(name: &str) -> Result<pelite::resources::ResourceName<'_>, &'static str> {
	if let Some(id) = name.strip_prefix('#') {
		return id.parse().map(pelite::resources::ResourceName::Id).map_err(|_| "invalid resource id");
	}
	Ok(pelite::resources::ResourceName::Str(name))
}

#[unsafe(export_name = "pefileResourcesTree")]
pub unsafe fn resources_tree(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};
	return_pelite_result(pefile.resources());
}

#[unsafe(export_name = "pefileResourcesGetResource")]
pub unsafe fn resources_get_resource(pefile: *mut PeFile, path: *const [u8]) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};
	let path = match str::from_utf8(unsafe { &*path }) {
		Ok(path) => path,
		Err(err) => return return_error(err),
	};
	let resources = match pefile.resources() {
		Ok(resources) => resources,
		Err(pelite::Error::Null) => return return_null(),
		Err(err) => return return_error(err),
	};
	let bytes = resources.find_data(path).and_then(|entry| entry.bytes().map_err(Into::into));
	match bytes {
		Ok(bytes) => return_bytes(bytes),
		Err(pelite::resources::ResourceFindError::NotFound | pelite::resources::ResourceFindError::Pe(pelite::Error::Null)) => return_null(),
		Err(err) => return_error(err),
	}
}

fn return_resource_names<'a>(values: impl Iterator<Item = Result<(pelite::resources::ResourceName<'a>, pelite::resources::group::ResourceGroup<'a>), pelite::resources::ResourceFindError>>) {
	let names: Result<Vec<_>, _> = values.map(|value| value.map(|(name, _)| name)).collect();
	return_resource_find_result(names);
}

#[unsafe(export_name = "pefileListIcons")]
pub unsafe fn resources_list_icons(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};
	match pefile.resources() {
		Ok(resources) => return_resource_names(resources.icons()),
		Err(pelite::Error::Null) => return_null(),
		Err(err) => return_error(err),
	}
}

#[unsafe(export_name = "pefileGetIcon")]
pub unsafe fn resources_get_icon(pefile: *mut PeFile, name: *const [u8]) {
	unsafe { resources_get_group(pefile, name, true) }
}

#[unsafe(export_name = "pefileListCursors")]
pub unsafe fn resources_list_cursors(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};
	match pefile.resources() {
		Ok(resources) => return_resource_names(resources.cursors()),
		Err(pelite::Error::Null) => return_null(),
		Err(err) => return_error(err),
	}
}

#[unsafe(export_name = "pefileGetCursor")]
pub unsafe fn resources_get_cursor(pefile: *mut PeFile, name: *const [u8]) {
	unsafe { resources_get_group(pefile, name, false) }
}

unsafe fn resources_get_group(pefile: *mut PeFile, name: *const [u8], icon: bool) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};
	let name = match str::from_utf8(unsafe { &*name }) {
		Ok(name) => name,
		Err(err) => return return_error(err),
	};
	let name = match resource_name(name) {
		Ok(name) => name,
		Err(err) => return return_error(err),
	};
	let resources = match pefile.resources() {
		Ok(resources) => resources,
		Err(pelite::Error::Null) => return return_null(),
		Err(err) => return return_error(err),
	};
	let bytes = if icon {
		resources.find_icon(name).and_then(|group| group.to_vec())
	}
	else {
		resources.find_cursor(name).and_then(|group| group.to_vec())
	};
	match bytes {
		Ok(bytes) => return_bytes(&bytes),
		Err(pelite::resources::ResourceFindError::NotFound | pelite::resources::ResourceFindError::Pe(pelite::Error::Null)) => return_null(),
		Err(err) => return_error(err),
	}
}

#[unsafe(export_name = "pefileResourcesManifest")]
pub unsafe fn resources_manifest(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	let resources = match pefile.resources() {
		Ok(resources) => resources,
		Err(pelite::Error::Null) => return return_null(),
		Err(err) => return return_error(err),
	};

	match resources.manifest() {
		Ok(manifest) => return_str(manifest),
		Err(pelite::resources::ResourceFindError::NotFound | pelite::resources::ResourceFindError::Pe(pelite::Error::Null)) => return_null(),
		Err(err) => return_error(err),
	}
}

#[unsafe(export_name = "pefileResourcesVersionInfo")]
pub unsafe fn resources_version_info(pefile: *mut PeFile) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	let resources = match pefile.resources() {
		Ok(resources) => resources,
		Err(pelite::Error::Null) => return return_null(),
		Err(err) => return return_error(err),
	};

	return_resource_find_result(resources.version_info());
}

#[unsafe(export_name = "pefileScannerExec")]
pub unsafe fn scanner_exec(pefile: *mut PeFile, rva: u32, pattern: *const [u8]) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	let pattern = unsafe { &*pattern };
	let pattern = match str::from_utf8(pattern) {
		Ok(pattern) => pattern,
		Err(err) => return return_error(err),
	};

	let pattern = match pelite::pattern::parse(pattern, pelite::pattern::ParseOptions::DEFAULT) {
		Ok(pattern) => pattern,
		Err(err) => return return_error(err),
	};

	let mut save = vec![0u32; pelite::pattern::save_len(&pattern)];
	if !pefile.scanner().exec(rva, &pattern, &mut save) {
		return return_null();
	}

	let slots = pelite::pattern::captures_len(&pattern);
	return_json(&save[..slots]);
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum ScannerSection {
	ByName(String),
	ByIndex(usize),
}

#[derive(serde::Deserialize)]
struct ScannerArgs {
	pattern: String,
	#[serde(default)]
	section: Option<ScannerSection>,
	#[serde(default)]
	limit: usize,
}

#[derive(serde::Serialize)]
struct ScannerFindResult {
	rva: u32,
	save: Vec<u32>,
}

#[unsafe(export_name = "pefileScannerFind")]
pub unsafe fn scanner_find(pefile: *mut PeFile, args: *const [u8]) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	let args = unsafe { &*args };
	let args = match str::from_utf8(args) {
		Ok(args) => args,
		Err(err) => return return_error(err),
	};

	let args: ScannerArgs = match serde_json::from_str(args) {
		Ok(args) => args,
		Err(err) => return return_error(err),
	};

	let pattern = match pelite::pattern::parse(&args.pattern, pelite::pattern::ParseOptions::DEFAULT) {
		Ok(pattern) => pattern,
		Err(err) => return return_error(err),
	};

	let mut i = 0;
	let sections = pefile.scanner().sections(|section| {
		match &args.section {
			None => section.Characteristics & pelite::image::IMAGE_SCN_MEM_EXECUTE != 0,
			Some(ScannerSection::ByIndex(index)) => {
				i += 1;
				index + 1 == i
			}
			Some(ScannerSection::ByName(name)) => section.name().map(|n| n == name).unwrap_or(false),
		}
	});

	let mut save = vec![0u32; pelite::pattern::save_len(&pattern)];
	let Some(rva) = sections.find(&pattern, &mut save) else {
		return return_null();
	};

	let save = save[..pelite::pattern::captures_len(&pattern)].to_vec();
	return_json(ScannerFindResult { rva, save });
}

#[unsafe(export_name = "pefileScannerMatches")]
pub unsafe fn scanner_matches(pefile: *mut PeFile, args: *const [u8]) {
	let pefile = unsafe { &mut *pefile };
	let pefile = match pelite::PeFile::from_bytes(pefile.as_ref()) {
		Ok(pefile) => pefile,
		Err(err) => return return_error(err),
	};

	let args = unsafe { &*args };
	let args = match str::from_utf8(args) {
		Ok(args) => args,
		Err(err) => return return_error(err),
	};

	let args: ScannerArgs = match serde_json::from_str(args) {
		Ok(args) => args,
		Err(err) => return return_error(err),
	};

	let pattern = match pelite::pattern::parse(&args.pattern, pelite::pattern::ParseOptions::DEFAULT) {
		Ok(pattern) => pattern,
		Err(err) => return return_error(err),
	};

	let mut i = 0;
	let sections = pefile.scanner().sections(|section| {
		match &args.section {
			None => section.Characteristics & pelite::image::IMAGE_SCN_MEM_EXECUTE != 0,
			Some(ScannerSection::ByIndex(index)) => {
				i += 1;
				index + 1 == i
			}
			Some(ScannerSection::ByName(name)) => section.name().map(|n| n == name).unwrap_or(false),
		}
	});

	let captures_len = pelite::pattern::captures_len(&pattern);
	let mut captures = Vec::new();

	let mut count = 0;

	let mut save = vec![0u32; pelite::pattern::save_len(&pattern)];
	let mut matches = sections.matches(&pattern);
	while let Some(rva) = matches.next(&mut save) {
		let save = save[..captures_len].to_vec();
		captures.push(ScannerFindResult { rva, save });

		if args.limit > 0 {
			count += 1;
			if count >= args.limit {
				break;
			}
		}
	}

	return_json(captures);
}
