use pelite::{image, PeFile, Wrap};
use sha2::{Digest, Sha256};

use super::*;

mod disassembly;
mod entry_point;
mod exceptions;
mod exports;
mod forward_feeder;
mod imports;
mod labels;
mod relocations;
mod strings;

#[cfg(test)]
mod tests;

/// Candidate symbols accumulated by independently selectable analysis passes.
pub struct Analysis<'a> {
	pe: PeFile<'a>,
	bitness: u32,
	size: u32,
	headers_size: u32,
	/// Candidates indexed by RVA, available between passes.
	pub symbols: HashMap<u32, factmap::SymbolFact>,
	/// Comments attached to instruction RVAs by analysis passes.
	pub comments: HashMap<u32, factmap::CommentFact>,
}

impl<'a> Analysis<'a> {
	/// Create an empty analysis for an i386 or AMD64 image.
	pub fn new(pe: PeFile<'a>) -> Result<Self> {
		let bitness = match pe.file_header().Machine {
			image::IMAGE_FILE_MACHINE_I386 => 32,
			image::IMAGE_FILE_MACHINE_AMD64 => 64,
			machine => return Err(err(format!("unsupported machine type {machine:#06x}; expected i386 or AMD64"))),
		};
		let (size, headers_size) = match pe.optional_header() {
			Wrap::T32(h) => (h.SizeOfImage, h.SizeOfHeaders),
			Wrap::T64(h) => (h.SizeOfImage, h.SizeOfHeaders),
		};
		Ok(Analysis { pe, bitness, size, headers_size, symbols: HashMap::new(), comments: HashMap::new() })
	}

	/// Finish the analysis and return candidates sorted by RVA.
	pub fn into_symbols(mut self) -> Vec<factmap::SymbolFact> {
		for symbol in self.symbols.values_mut() {
			if symbol.name == factmap::SymbolName::Code && symbol.ty == ty::Type::Unknown {
				symbol.upgrade_type(ty::Type::Code, ty::PointerWidth::from(self.pe))
					.expect("code hints always upgrade successfully");
			}
		}
		let mut symbols = self.symbols.into_values().collect::<Vec<_>>();
		symbols.sort_unstable_by_key(|symbol| symbol.rva);
		symbols
	}

	/// Finish the analysis and return symbol and comment facts sorted by RVA.
	pub fn into_factmap(mut self) -> factmap::FactMap {
		let comments = std::mem::take(&mut self.comments);
		let mut facts = self.into_symbols().into_iter().map(factmap::Fact::Symbol)
			.chain(comments.into_values().map(factmap::Fact::Comment)).collect::<Vec<_>>();
		facts.sort_unstable_by_key(|fact| match fact {
			factmap::Fact::Symbol(symbol) => (symbol.rva, 0),
			factmap::Fact::Comment(comment) => (comment.rva, 1),
			factmap::Fact::Ref(reference) => (reference.rva, 2),
		});
		factmap::FactMap { facts }
	}
}

pub fn command() -> clap::Command {
	clap::Command::new("analysis")
		.about("Discover candidate symbols using disassembly and base relocations")
		.after_help(include_str!("docs/analysis.md"))
		.arg(summary::file_arg().required(true))
		.arg(clap::Arg::new("output")
			.short('o').long("output").value_name("FACTS.txt")
			.value_parser(clap::value_parser!(PathBuf))
			.help("Write discovered symbols as a new factmap database"))
}

/// Run the complete automatic analysis pipeline in memory.
pub fn analyze(pe: PeFile<'_>) -> Result<factmap::FactMap> {
	let mut analysis = Analysis::new(pe)?;
	// Run heuristics first, then increasingly authoritative metadata.
	analysis.scan_code();
	analysis.scan_relocations();
	analysis.refine_labels();
	analysis.seed_exports();
	analysis.label_strings();
	analysis.label_imports();
	analysis.scan_exceptions();
	analysis.seed_entry_points();
	analysis.forward_feed();
	Ok(analysis.into_factmap())
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let map = pelite::FileMap::open(path)?;
	let facts = analyze(PeFile::from_bytes(&map)?)?;
	if let Some(output_path) = matches.get_one::<PathBuf>("output") {
		let file = fs::OpenOptions::new().write(true).create_new(true).open(output_path)?;
		let mut file = io::BufWriter::new(file);
		let filename = path.file_name().unwrap_or(path.as_os_str()).to_string_lossy();
		let filename = serde_json::to_string(&filename)?;
		let hash = basenc::LowerHex.encode(Sha256::digest(map.as_ref()).as_ref());
		let comment = format!("File: {filename}, SHA-256: {hash}");
		facts.write(&mut file, &comment)?;
		file.flush()?;
	}
	match format {
		OutputFormat::Nul => Ok(()),
		OutputFormat::Json | OutputFormat::JsonPretty => {
			let report = facts.facts.iter().map(|fact| match fact {
				factmap::Fact::Symbol(symbol) => serde_json::json!({
					"rva": symbol.rva,
					"name": symbol.name.to_string(),
					"ty": symbol.ty.to_string(),
				}),
				factmap::Fact::Comment(comment) => serde_json::json!({ "rva": comment.rva, "comment": comment.comment }),
				factmap::Fact::Ref(_) => unreachable!("analysis produces symbols and comments"),
			}).collect::<Vec<_>>();
			print_json(&report, matches!(format, OutputFormat::JsonPretty))
		},
		OutputFormat::Text => {
			let mut output = io::stdout().lock();
			writeln!(output, "RVA       Name           Type")?;
			for fact in facts.facts {
				match fact {
					factmap::Fact::Symbol(symbol) => writeln!(output, "{:#08x}  {:<14} {}", symbol.rva, symbol.name.to_string(), symbol.ty)?,
					factmap::Fact::Comment(comment) => writeln!(output, "{:#08x}  ; {}", comment.rva, comment.comment)?,
					factmap::Fact::Ref(_) => unreachable!("analysis produces symbols and comments"),
				}
			}
			Ok(())
		},
	}
}

impl Analysis<'_> {
	fn mapped(&self, rva: u32) -> bool {
		rva < self.size && (rva < self.headers_size || self.pe.section_headers().iter().any(|section| {
			let len = section.VirtualSize.max(section.SizeOfRawData);
			rva.checked_sub(section.VirtualAddress).is_some_and(|offset| offset < len)
		}))
	}

	fn add(&mut self, rva: u32, interpretation: Option<ty::Type>) {
		if !self.mapped(rva) {
			return;
		}
		let executable = self.pe.section_headers().iter().any(|section| {
			section.Characteristics & image::IMAGE_SCN_MEM_EXECUTE != 0
				&& rva.checked_sub(section.VirtualAddress)
					.is_some_and(|offset| offset < section.VirtualSize.max(section.SizeOfRawData))
		});
		let symbol = self.symbols.entry(rva).or_insert_with(|| factmap::SymbolFact::new(
			rva,
			ty::Type::Unknown,
			if executable { factmap::SymbolName::Code } else { factmap::SymbolName::Data },
		));
		if let Some(interpretation) = interpretation {
			if interpretation == ty::Type::Code && symbol.name == factmap::SymbolName::Data {
				symbol.name = factmap::SymbolName::Code;
			}
			symbol.upgrade_type(interpretation, ty::PointerWidth::from(self.pe))
				.expect("analysis hints are code or fixed-size numeric types");
		}
	}

	fn add_va(&mut self, va: u64, interpretation: Option<ty::Type>) {
		if let Some(rva) = va.checked_sub(self.pe.image_base()).and_then(|rva| u32::try_from(rva).ok()) {
			self.add(rva, interpretation);
		}
	}
}

impl factmap::SymbolFact {
	/// Merge a type hint, preserving distinct fixed-size hints in a union.
	/// Unknown hints add no evidence; code takes precedence because it is unsized.
	/// On a layout error, the previous type is unchanged.
	fn upgrade_type(&mut self, hint: ty::Type, pointer_width: ty::PointerWidth) -> result::Result<(), &'static str> {
		if hint == ty::Type::Unknown || self.ty == hint {
			return Ok(());
		}
		if self.ty == ty::Type::Unknown || hint == ty::Type::Code {
			self.ty = hint;
			return Ok(());
		}
		if self.ty == ty::Type::Code {
			return Ok(());
		}
		if let ty::Type::Struct(union) = &self.ty {
			if union.is_union && union.fields.iter().any(|field| field.ty == hint) {
				return Ok(());
			}
		}
		let (previous_size, previous_align) = self.ty.layout(pointer_width)?;
		let (hint_size, hint_align) = hint.layout(pointer_width)?;
		let align = previous_align.max(hint_align);
		let size = previous_size.max(hint_size).checked_add(align - 1)
			.ok_or("type layout overflow")? & !(align - 1);
		let field = ty::Field { name: ty::FieldName::Unnamed, offset: 0, ty: hint };
		if let ty::Type::Struct(union) = &mut self.ty {
			if union.is_union {
				union.fields.push(field);
				union.size = size;
				union.align = align;
				return Ok(());
			}
		}
		let previous = std::mem::replace(&mut self.ty, ty::Type::Unknown);
		self.ty = ty::Type::Struct(Box::new(ty::StructType {
			name: None,
			fields: vec![ty::Field { name: ty::FieldName::Unnamed, offset: 0, ty: previous }, field],
			is_union: true,
			size,
			align,
			is_dst: false,
		}));
		Ok(())
	}
}
