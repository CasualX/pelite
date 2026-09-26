use iced_x86::{Decoder, DecoderOptions, MemorySize, Mnemonic, OpKind, Register};
use pelite::{image, Import, PeFile, Wrap};

use super::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum ReferenceKind {
	Branch,
	Call,
	EntryPoint,
	Export,
	Immediate,
	Memory,
	Relocation,
	SectionStart,
}

impl fmt::Display for ReferenceKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(match self {
			ReferenceKind::Branch => "branch",
			ReferenceKind::Call => "call",
			ReferenceKind::EntryPoint => "entry_point",
			ReferenceKind::Export => "export",
			ReferenceKind::Immediate => "immediate",
			ReferenceKind::Memory => "memory",
			ReferenceKind::Relocation => "relocation",
			ReferenceKind::SectionStart => "section_start",
		})
	}
}

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
#[serde(rename_all = "snake_case")]
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

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, serde::Serialize)]
struct Reference {
	/// Instruction or pointer location; absent for metadata seeds.
	source_rva: Option<u32>,
	kind: ReferenceKind,
}

#[derive(serde::Serialize)]
struct Symbol {
	rva: u32,
	label: String,
	/// None when unknown; Multiple preserves conflicting evidence.
	interpretations: Option<Interpretation>,
	references: BTreeSet<Reference>,
}

struct Analysis<'a> {
	pe: PeFile<'a>,
	size: u32,
	headers_size: u32,
	symbols: BTreeMap<u32, Symbol>,
}

pub fn command() -> clap::Command {
	clap::Command::new("symbols")
		.about("Discover candidate symbols using disassembly and base relocations")
		.after_help("Linearly decodes executable sections of i386/AMD64 images. Labels entry points, exports, branch targets, static memory references, address-like immediates, and relocated pointers. Results are heuristic: embedded data may decode as instructions, and register-based targets cannot be resolved. All addresses are RVAs.")
		.arg(summary::file_arg().required(true))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let map = pelite::FileMap::open(path)?;
	let symbols = analyze(PeFile::from_bytes(&map)?)?;
	match format {
		OutputFormat::Json => print_json(&symbols, false),
		OutputFormat::JsonPretty => print_json(&symbols, true),
		OutputFormat::Text => {
			let mut output = io::stdout().lock();
			writeln!(output, "RVA       Label          Interpretations  References (source RVA:kind)")?;
			for symbol in symbols {
				let types = symbol.interpretations.as_ref().map(ToString::to_string).unwrap_or_default();
				write!(output, "{:#08x}  {:<14} {:<16}", symbol.rva, symbol.label, types)?;
				for reference in symbol.references {
					match reference.source_rva {
						Some(rva) => write!(output, " {rva:#08x}:{}", reference.kind)?,
						None => write!(output, " {}", reference.kind)?,
					}
				}
				writeln!(output)?;
			}
			Ok(())
		},
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
		analysis.add(entry, Some(DataKind::Code), None, ReferenceKind::EntryPoint);
	}
	if let Ok(exports) = pe.exports().and_then(|exports| exports.by()) {
		for export in exports.iter() {
			if let Some(rva) = export.ok().and_then(|export| export.symbol()) {
				analysis.add(rva, None, None, ReferenceKind::Export);
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
			analysis.add(section.VirtualAddress, Some(DataKind::Code), None, ReferenceKind::SectionStart);
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
			analysis.add_va(va, None, rva, ReferenceKind::Relocation);
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

	fn add(&mut self, rva: u32, interpretation: Option<DataKind>, source_rva: Option<u32>, kind: ReferenceKind) {
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
			label: format!("{}_{rva:08x}", if executable { "code" } else { "data" }),
			interpretations: None,
			references: BTreeSet::new(),
		});
		if let Some(interpretation) = interpretation {
			Interpretation::insert(&mut symbol.interpretations, interpretation);
			if interpretation == DataKind::Code {
				symbol.label = format!("code_{rva:08x}");
			}
		}
		symbol.references.insert(Reference { source_rva, kind });
	}

	fn add_va(&mut self, va: u64, interpretation: Option<DataKind>, source: u32, kind: ReferenceKind) {
		if let Some(rva) = va.checked_sub(self.pe.image_base()).and_then(|rva| u32::try_from(rva).ok()) {
			self.add(rva, interpretation, Some(source), kind);
		}
	}

	fn disassemble(&mut self, bitness: u32, bytes: &[u8], rva: u32) {
		let Some(ip) = self.pe.image_base().checked_add(rva as u64) else { return };
		let mut decoder = Decoder::with_ip(bitness, bytes, ip, DecoderOptions::NONE);
		while decoder.can_decode() {
			let source = rva + decoder.position() as u32;
			let instruction = decoder.decode();
			if instruction.is_invalid() {
				continue;
			}
			for operand in instruction.op_kinds() {
				match operand {
					OpKind::NearBranch16 | OpKind::NearBranch32 | OpKind::NearBranch64 => {
						let kind = if instruction.mnemonic() == Mnemonic::Call { ReferenceKind::Call } else { ReferenceKind::Branch };
						self.add_va(instruction.near_branch_target(), Some(DataKind::Code), source, kind);
					},
					OpKind::Memory => {
						let Some(va) = static_memory_address(&instruction) else { continue };
						let ty = if instruction.mnemonic() == Mnemonic::Lea { None } else { interpretation(instruction.memory_size()) };
						self.add_va(va, ty, source, ReferenceKind::Memory);
					},
					OpKind::Immediate32 => self.add_va(instruction.immediate32() as u64, None, source, ReferenceKind::Immediate),
					OpKind::Immediate64 => self.add_va(instruction.immediate64(), None, source, ReferenceKind::Immediate),
					OpKind::Immediate32to64 => self.add_va(instruction.immediate32to64() as u64, None, source, ReferenceKind::Immediate),
					_ => {},
				}
			}
		}
	}
}

/// Resolve only memory operands whose address does not depend on runtime state.
fn static_memory_address(instruction: &iced_x86::Instruction) -> Option<u64> {
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
		let Some(dll) = descriptor.dll_name().ok().and_then(|name| name.to_str().ok()) else { continue };
		let Ok(imports) = descriptor.int() else { continue };
		for (index, import) in imports.enumerate() {
			let Some(rva) = u32::try_from(index).ok()
				.and_then(|index| index.checked_mul(bitness / 8))
				.and_then(|offset| descriptor.image().FirstThunk.get().checked_add(offset)) else { continue };
			let name = match import {
				Ok(Import::ByName { name, .. }) => {
					let Ok(name) = name.to_str() else { continue };
					format!("{dll}!{name}")
				},
				Ok(Import::ByOrdinal { ord }) => format!("{dll}!#{ord}"),
				Err(_) => continue,
			};
			names.insert(rva, name);
		}
	}
	names
}

/// Inspect the first instruction at each discovered code address, without following
/// jumps or changing symbol identity. The RVA suffix keeps duplicate stubs distinct.
fn refine_labels(pe: PeFile<'_>, bitness: u32, size: u32, symbols: &mut BTreeMap<u32, Symbol>) {
	let imports = import_names(pe, bitness);
	for symbol in symbols.values_mut() {
		if !symbol.label.starts_with("code_") {
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
		let instruction = Decoder::with_ip(bitness, &bytes[..len], ip, DecoderOptions::NONE).decode();
		if instruction.is_invalid() {
			continue;
		}
		let label = match instruction.mnemonic() {
			Mnemonic::Ret => {
				let pop = if instruction.op_count() == 0 { 0 } else { instruction.immediate16() };
				format!("ret{pop}_{rva:08x}")
			},
			Mnemonic::Jmp => match instruction.op0_kind() {
				OpKind::NearBranch16 | OpKind::NearBranch32 | OpKind::NearBranch64 => format!("thunk_{rva:08x}"),
				OpKind::Memory if matches!(instruction.memory_size(), MemorySize::DwordOffset | MemorySize::QwordOffset) => {
					let Some(va) = static_memory_address(&instruction) else { continue };
					let name = va.checked_sub(pe.image_base()).and_then(|rva| u32::try_from(rva).ok())
						.and_then(|rva| imports.get(&rva));
					match name {
						Some(name) => format!("imp_{name}_{rva:08x}"),
						None => format!("indirect_{rva:08x}"),
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

fn interpretation(size: MemorySize) -> Option<DataKind> {
	Some(match size {
		MemorySize::UInt8 => DataKind::U8,
		MemorySize::UInt16 => DataKind::U16,
		MemorySize::UInt32 => DataKind::U32,
		MemorySize::UInt64 => DataKind::U64,
		MemorySize::Int8 => DataKind::I8,
		MemorySize::Int16 => DataKind::I16,
		MemorySize::Int32 => DataKind::I32,
		MemorySize::Int64 => DataKind::I64,
		MemorySize::Float32 => DataKind::F32,
		MemorySize::Float64 => DataKind::F64,
		_ => return None,
	})
}
