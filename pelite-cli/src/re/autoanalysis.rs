use pelite::{image, Import, PeFile, Wrap};
use sha2::{Digest, Sha256};

use super::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, serde::Serialize)]
#[serde(rename_all = "lowercase")]
enum DataKind {
	Code,
	U8,
	U16,
	U32,
	U64,
	I8,
	I16,
	I32,
	I64,
	F32,
	F64,
}

impl fmt::Display for DataKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(match self {
			DataKind::Code => "code",
			DataKind::U8 => "u8",
			DataKind::U16 => "u16",
			DataKind::U32 => "u32",
			DataKind::U64 => "u64",
			DataKind::I8 => "i8",
			DataKind::I16 => "i16",
			DataKind::I32 => "i32",
			DataKind::I64 => "i64",
			DataKind::F32 => "f32",
			DataKind::F64 => "f64",
		})
	}
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(untagged)]
enum Interpretation {
	Single(DataKind),
	Multiple(BTreeSet<DataKind>),
}

impl Interpretation {
	fn insert(into: &mut Option<Interpretation>, kind: DataKind) {
		match into {
			None => *into = Some(Interpretation::Single(kind)),
			Some(Interpretation::Single(previous)) if *previous != kind => {
				*into = Some(Interpretation::Multiple(BTreeSet::from([*previous, kind])));
			},
			Some(Interpretation::Single(_)) => {},
			Some(Interpretation::Multiple(kinds)) => { kinds.insert(kind); },
		}
	}
}

impl fmt::Display for Interpretation {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Interpretation::Single(kind) => write!(f, "{kind}"),
			Interpretation::Multiple(kinds) => {
				for (index, kind) in kinds.iter().enumerate() {
					if index != 0 { f.write_str(",")?; }
					write!(f, "{kind}")?;
				}
				Ok(())
			},
		}
	}
}

#[derive(serde::Serialize)]
struct Symbol {
	rva: u32,
	label: String,
	/// None when unknown; Multiple preserves conflicting evidence.
	interpretations: Option<Interpretation>,
}

struct Analysis<'a> {
	pe: PeFile<'a>,
	size: u32,
	headers_size: u32,
	symbols: BTreeMap<u32, Symbol>,
}

pub fn command() -> clap::Command {
	clap::Command::new("autoanalysis")
		.about("Discover candidate symbols using disassembly and base relocations")
		.after_help(include_str!("../docs/re-autoanalysis.md"))
		.arg(summary::file_arg().required(true))
		.arg(clap::Arg::new("output")
			.short('o').long("output").value_name("FACTS.txt")
			.value_parser(clap::value_parser!(PathBuf))
			.help("Write discovered symbols as a new factmap database"))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let map = pelite::FileMap::open(path)?;
	let symbols = analyze(PeFile::from_bytes(&map)?)?;
	if let Some(output_path) = matches.get_one::<PathBuf>("output") {
		let file = fs::OpenOptions::new().write(true).create_new(true).open(output_path)?;
		let mut file = io::BufWriter::new(file);
		let filename = path.file_name().unwrap_or(path.as_os_str()).to_string_lossy();
		let filename = serde_json::to_string(&filename)?;
		let hash = basenc::LowerHex.encode(Sha256::digest(map.as_ref()).as_ref());
		let comment = format!("File: {filename}, SHA-256: {hash}");
		symbol_factmap(&symbols).write(&mut file, &comment)?;
		file.flush()?;
	}
	match format {
		OutputFormat::Nul => Ok(()),
		OutputFormat::Json => print_json(&symbols, false),
		OutputFormat::JsonPretty => print_json(&symbols, true),
		OutputFormat::Text => {
			let mut output = io::stdout().lock();
			writeln!(output, "RVA       Label          Interpretations")?;
			for symbol in symbols {
				let types = symbol.interpretations.as_ref().map(ToString::to_string).unwrap_or_default();
				writeln!(output, "{:#08x}  {:<14} {types}", symbol.rva, symbol.label)?;
			}
			Ok(())
		},
	}
}

fn symbol_factmap(symbols: &[Symbol]) -> factmap::FactMap {
	let facts = symbols.iter().map(|symbol| {
		let ty = match &symbol.interpretations {
			Some(Interpretation::Single(kind)) => data_type(*kind),
			Some(Interpretation::Multiple(kinds)) if kinds.contains(&DataKind::Code) => ty::Type::Code,
			Some(Interpretation::Multiple(kinds)) => {
				let fields = kinds.iter().map(|kind| data_type(*kind).to_string()).collect::<Vec<_>>();
				ty::Type::parse(&format!("union{{{}}}", fields.join(",")), ty::PointerWidth::Bits64)
					.expect("numeric interpretations form a valid union")
			},
			None if symbol.label == "data" => ty::Type::Unknown,
			None => ty::Type::Code,
		};
		let name = if symbol.label == "thunk" { factmap::SymbolName::Thunk }
			else if symbol.label == "code" { factmap::SymbolName::Code }
			else if symbol.label == "data" { factmap::SymbolName::Data }
			else { factmap::SymbolName::Named(symbol.label.clone()) };
		factmap::Fact::Symbol(factmap::SymbolFact::new(symbol.rva, ty, name))
	}).collect();
	factmap::FactMap { facts }
}

fn data_type(kind: DataKind) -> ty::Type {
	match kind {
		DataKind::Code => ty::Type::Code,
		DataKind::U8 => ty::Type::U8,
		DataKind::U16 => ty::Type::U16,
		DataKind::U32 => ty::Type::U32,
		DataKind::U64 => ty::Type::U64,
		DataKind::I8 => ty::Type::I8,
		DataKind::I16 => ty::Type::I16,
		DataKind::I32 => ty::Type::I32,
		DataKind::I64 => ty::Type::I64,
		DataKind::F32 => ty::Type::F32,
		DataKind::F64 => ty::Type::F64,
	}
}

fn analyze(pe: PeFile<'_>) -> Result<Vec<Symbol>> {
	let bitness = match pe.file_header().Machine {
		image::IMAGE_FILE_MACHINE_I386 => 32,
		image::IMAGE_FILE_MACHINE_AMD64 => 64,
		machine => return Err(err(format!("unsupported machine type {machine:#06x}; expected i386 or AMD64"))),
	};
	let (size, headers_size, entry) = match pe.optional_header() {
		Wrap::T32(h) => (h.SizeOfImage, h.SizeOfHeaders, h.AddressOfEntryPoint),
		Wrap::T64(h) => (h.SizeOfImage, h.SizeOfHeaders, h.AddressOfEntryPoint),
	};
	let mut analysis = Analysis { pe, size, headers_size, symbols: BTreeMap::new() };
	if entry != 0 {
		analysis.add(entry, Some(DataKind::Code));
	}
	if let Ok(exports) = pe.exports().and_then(|exports| exports.by()) {
		for export in exports.iter() {
			if let Some(rva) = export.ok().and_then(|export| export.symbol()) {
				analysis.add(rva, None);
			}
		}
	}
	for section in pe.section_headers() {
		if section.Characteristics & image::IMAGE_SCN_MEM_EXECUTE == 0 {
			continue;
		}
		let bytes = pe.get_section_bytes(section)?;
		// Exclude file alignment padding and bytes beyond the mapped image.
		let virtual_size = if section.VirtualSize == 0 { section.SizeOfRawData } else { section.VirtualSize };
		let len = bytes.len().min(virtual_size as usize).min(size.saturating_sub(section.VirtualAddress) as usize);
		if len != 0 {
			analysis.add(section.VirtualAddress, Some(DataKind::Code));
			analysis.disassemble(bitness, &bytes[..len], section.VirtualAddress);
		}
	}
	match pe.base_relocs() {
		Ok(relocs) => relocs.for_each(|rva, ty| {
			let width = match (bitness, ty) {
				(32, image::IMAGE_REL_BASED_HIGHLOW) => 4,
				(64, image::IMAGE_REL_BASED_DIR64) => 8,
				_ => return,
			};
			// Relocation fields inside instructions need not be naturally aligned.
			let Ok(bytes) = pe.slice(rva, width, 1) else { return };
			let va = if width == 4 {
				u32::from_le_bytes(bytes[..4].try_into().unwrap()) as u64
			}
			else {
				u64::from_le_bytes(bytes[..8].try_into().unwrap())
			};
			analysis.add_va(va, None);
		}),
		Err(pelite::Error::Null) => {},
		Err(error) => return Err(error.into()),
	}
	refine_labels(pe, bitness, size, &mut analysis.symbols);
	Ok(analysis.symbols.into_values().collect())
}

impl Analysis<'_> {
	fn mapped(&self, rva: u32) -> bool {
		rva < self.size && (rva < self.headers_size || self.pe.section_headers().iter().any(|section| {
			let len = section.VirtualSize.max(section.SizeOfRawData);
			rva.checked_sub(section.VirtualAddress).is_some_and(|offset| offset < len)
		}))
	}

	fn add(&mut self, rva: u32, interpretation: Option<DataKind>) {
		if !self.mapped(rva) {
			return;
		}
		let executable = self.pe.section_headers().iter().any(|section| {
			section.Characteristics & image::IMAGE_SCN_MEM_EXECUTE != 0
				&& rva.checked_sub(section.VirtualAddress)
					.is_some_and(|offset| offset < section.VirtualSize.max(section.SizeOfRawData))
		});
		let symbol = self.symbols.entry(rva).or_insert_with(|| Symbol {
			rva,
			label: (if executable { "code" } else { "data" }).to_owned(),
			interpretations: None,
		});
		if let Some(interpretation) = interpretation {
			Interpretation::insert(&mut symbol.interpretations, interpretation);
			if interpretation == DataKind::Code {
				symbol.label = "code".to_owned();
			}
		}
	}

	fn add_va(&mut self, va: u64, interpretation: Option<DataKind>) {
		if let Some(rva) = va.checked_sub(self.pe.image_base()).and_then(|rva| u32::try_from(rva).ok()) {
			self.add(rva, interpretation);
		}
	}

	fn disassemble(&mut self, bitness: u32, bytes: &[u8], rva: u32) {
		let Some(ip) = self.pe.image_base().checked_add(rva as u64) else { return };
		let mut decoder = iced_x86::Decoder::with_ip(bitness, bytes, ip, iced_x86::DecoderOptions::NONE);
		while decoder.can_decode() {
			let instruction = decoder.decode();
			if instruction.is_invalid() {
				continue;
			}
			for operand in instruction.op_kinds() {
				use iced_x86::{Mnemonic, OpKind};
				match operand {
					OpKind::NearBranch16 | OpKind::NearBranch32 | OpKind::NearBranch64 => {
						self.add_va(instruction.near_branch_target(), Some(DataKind::Code));
					},
					OpKind::Memory => {
						let Some(va) = static_memory_address(&instruction) else { continue };
						let ty = if instruction.mnemonic() == Mnemonic::Lea { None } else { interpretation(instruction.memory_size()) };
						self.add_va(va, ty);
					},
					OpKind::Immediate32 => self.add_va(instruction.immediate32() as u64, None),
					OpKind::Immediate64 => self.add_va(instruction.immediate64(), None),
					OpKind::Immediate32to64 => self.add_va(instruction.immediate32to64() as u64, None),
					_ => {},
				}
			}
		}
	}
}

/// Resolve only memory operands whose address does not depend on runtime state.
fn static_memory_address(instruction: &iced_x86::Instruction) -> Option<u64> {
	use iced_x86::Register;
	if matches!(instruction.segment_prefix(), Register::FS | Register::GS) {
		return None;
	}
	if instruction.is_ip_rel_memory_operand() {
		Some(instruction.ip_rel_memory_address())
	}
	else if instruction.memory_base() == Register::None && instruction.memory_index() == Register::None {
		Some(instruction.memory_displacement64())
	}
	else {
		None
	}
}

fn import_names(pe: PeFile<'_>, bitness: u32) -> HashMap<u32, String> {
	let mut names = HashMap::new();
	let Ok(imports) = pe.imports() else { return names };
	for descriptor in imports {
		let dll = descriptor.dll_name().ok().and_then(|name| name.to_str().ok());
		let Ok(imports) = descriptor.int() else { continue };
		for (index, import) in imports.enumerate() {
			let Some(rva) = u32::try_from(index).ok()
				.and_then(|index| index.checked_mul(bitness / 8))
				.and_then(|offset| descriptor.image().FirstThunk.get().checked_add(offset)) else { continue };
			let name = match import {
				Ok(Import::ByName { name, .. }) => {
					let Ok(name) = name.to_str() else { continue };
					name.to_owned()
				},
				Ok(Import::ByOrdinal { ord }) => {
					let Some(dll) = dll else { continue };
					format!("{dll}!#{ord}")
				},
				Err(_) => continue,
			};
			names.insert(rva, name);
		}
	}
	names
}

/// Inspect the first instruction at each discovered code address, without following
/// jumps or changing symbol identity.
fn refine_labels(pe: PeFile<'_>, bitness: u32, size: u32, symbols: &mut BTreeMap<u32, Symbol>) {
	use iced_x86::{Mnemonic, OpKind};
	let imports = import_names(pe, bitness);
	for symbol in symbols.values_mut() {
		if symbol.label != "code" {
			continue;
		}
		let rva = symbol.rva;
		let Some(ip) = pe.image_base().checked_add(rva as u64) else { continue };
		let Ok(bytes) = pe.slice(rva, 1, 1) else { continue };
		let mut len = bytes.len().min(size.saturating_sub(rva) as usize).min(15);
		if let Some(section) = pe.section_headers().iter().find(|section| {
			rva.checked_sub(section.VirtualAddress).is_some_and(|offset| offset < section.VirtualSize.max(section.SizeOfRawData))
		}) {
			let extent = if section.VirtualSize == 0 { section.SizeOfRawData } else { section.VirtualSize };
			len = len.min(extent.saturating_sub(rva - section.VirtualAddress) as usize);
		}
		let instruction = iced_x86::Decoder::with_ip(bitness, &bytes[..len], ip, iced_x86::DecoderOptions::NONE).decode();
		if instruction.is_invalid() {
			continue;
		}
		let label = match instruction.mnemonic() {
			Mnemonic::Ret => {
				let pop = if instruction.op_count() == 0 { 0 } else { instruction.immediate16() };
				format!("ret{pop}")
			},
			Mnemonic::Jmp => match instruction.op0_kind() {
				OpKind::NearBranch16 | OpKind::NearBranch32 | OpKind::NearBranch64 => "thunk".to_owned(),
				OpKind::Memory if matches!(instruction.memory_size(), iced_x86::MemorySize::DwordOffset | iced_x86::MemorySize::QwordOffset) => {
					let Some(va) = static_memory_address(&instruction) else { continue };
					let name = va.checked_sub(pe.image_base()).and_then(|rva| u32::try_from(rva).ok())
						.and_then(|rva| imports.get(&rva));
					match name {
						Some(name) => format!("imp_{name}"),
						None => "indirect".to_owned(),
					}
				},
				_ => continue,
			},
			_ => continue,
		};
		symbol.label = label;
		Interpretation::insert(&mut symbol.interpretations, DataKind::Code);
	}
}

fn interpretation(size: iced_x86::MemorySize) -> Option<DataKind> {
	use iced_x86::MemorySize::*;
	Some(match size {
		UInt8 => DataKind::U8,
		UInt16 => DataKind::U16,
		UInt32 => DataKind::U32,
		UInt64 => DataKind::U64,
		Int8 => DataKind::I8,
		Int16 => DataKind::I16,
		Int32 => DataKind::I32,
		Int64 => DataKind::I64,
		Float32 => DataKind::F32,
		Float64 => DataKind::F64,
		_ => return None,
	})
}

#[test]
fn discovered_symbols_write_parseable_factmap() {
	let symbols = vec![
		Symbol { rva: 0x1000, label: "code".into(), interpretations: None },
		Symbol { rva: 0x1100, label: "thunk".into(), interpretations: Some(Interpretation::Single(DataKind::Code)) },
		Symbol { rva: 0x2000, label: "data".into(), interpretations: Some(Interpretation::Multiple(BTreeSet::from([DataKind::U32, DataKind::U64]))) },
		Symbol { rva: 0x2100, label: "data".into(), interpretations: None },
		Symbol { rva: 0x3000, label: "imp_Sleep".into(), interpretations: Some(Interpretation::Single(DataKind::Code)) },
		Symbol { rva: 0x3100, label: "imp_Sleep".into(), interpretations: Some(Interpretation::Single(DataKind::Code)) },
	];
	let mut output = Vec::new();
	symbol_factmap(&symbols).write(&mut output, "").unwrap();
	let text = std::str::from_utf8(&output).unwrap();
	let map = factmap::FactMap::parse(text, ty::PointerWidth::Bits64).unwrap();
	let entries = map.facts.iter().map(|fact| {
		let factmap::Fact::Symbol(symbol) = fact else { panic!("expected a symbol fact") };
		symbol
	}).collect::<Vec<_>>();
	assert_eq!(entries[0].ty, ty::Type::Code);
	assert_eq!(entries[0].name, factmap::SymbolName::Code);
	assert_eq!(entries[1].name, factmap::SymbolName::Thunk);
	assert_eq!(entries[2].ty.to_string(), "union{u32,u64}");
	assert_eq!(entries[2].name, factmap::SymbolName::Data);
	assert_eq!(entries[3].name, factmap::SymbolName::Data);
	assert_eq!(entries[4].name, entries[5].name);
	assert_eq!(entries[4].name, factmap::SymbolName::Named("imp_Sleep".into()));
}
