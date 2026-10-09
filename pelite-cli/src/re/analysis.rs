use pelite::{image, PeFile, Wrap};
use sha2::{Digest, Sha256};

use super::*;

mod disassembly;
mod forward_feeder;
mod labels;
mod metadata;
mod strings;

#[cfg(test)]
mod tests;

pub fn command() -> clap::Command {
	clap::Command::new("analysis")
		.about("Discover candidate symbols using disassembly and base relocations")
		.after_help(include_str!("docs/analysis.md"))
		.arg(summary::file_arg().required(true))
		.arg(clap::Arg::new("timings")
			.long("timings")
			.action(clap::ArgAction::SetTrue)
			.help("Log total and per-pass automatic analysis timings to stderr"))
		.arg(clap::Arg::new("output")
			.short('o').long("output")
			.value_name("FACTS.txt")
			.value_parser(clap::value_parser!(PathBuf))
			.help("Write discovered symbols as a new factmap database"))
		.arg(clap::Arg::new("measure-load-time")
			.long("measure-load-time")
			.action(clap::ArgAction::SetTrue)
			.requires("output")
			.help("Read and parse the output database after writing it, logging the load time to stderr"))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let map = pelite::FileMap::open(path)?;
	let pe = PeFile::from_bytes(&map)?;
	let facts = analyze(pe, matches.get_flag("timings"))?;
	if let Some(output_path) = matches.get_one::<PathBuf>("output") {
		let file = fs::OpenOptions::new().write(true).create_new(true).open(output_path)?;
		let mut file = io::BufWriter::new(file);
		let filename = path.file_name().unwrap_or(path.as_os_str()).to_string_lossy();
		let filename = serde_json::to_string(&filename)?;
		let hash = basenc::LowerHex.encode(Sha256::digest(map.as_ref()).as_ref());
		let comment = format!("File: {filename}, SHA-256: {hash}");
		facts.write(&mut file, &comment)?;
		file.flush()?;
		drop(file);
		if matches.get_flag("measure-load-time") {
			let label = format!("reading and parsing {}", output_path.display());
			let _loaded = time(true, &label, || symbols::load_file(output_path, ty::PointerWidth::from(pe)))?;
		}
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
				factmap::Fact::Function(function) => serde_json::json!({ "rva": function.rva, "content": function.content }),
				factmap::Fact::Ref(reference) => serde_json::json!({ "rva": reference.rva, "target_rva": reference.target_rva }),
			}).collect::<Vec<_>>();
			print_json(&report, matches!(format, OutputFormat::JsonPretty))
		},
		OutputFormat::Text => {
			let mut output = io::stdout().lock();
			writeln!(output, "RVA       Name           Type")?;
			for fact in &facts.facts {
				writeln!(output, "{}", fact)?;
			}
			Ok(())
		},
	}
}

/// Run a closure and optionally log its elapsed time to stderr.
fn time<T>(timings: bool, label: &str, f: impl FnOnce() -> T) -> T {
	let start =
		if !timings { None }
		else { Some(time::Instant::now()) };

	let result = f();

	if let Some(start) = start {
		eprintln!("pelite-cli: {label} took {:.3?}", start.elapsed());
	}
	result
}

/// Run the complete automatic analysis pipeline in memory.
pub fn analyze(pe: PeFile<'_>, timings: bool) -> Result<factmap::FactMap> {
	time(timings, "automatic analysis (total)", || {
		let input = AnalysisInput::new(pe)?;
		let mut output = AnalysisOutput::default();
		// Run heuristics first, then increasingly authoritative metadata.
		time(timings, "scan_code", || disassembly::scan_code(&input, &mut output));
		time(timings, "scan_relocations", || metadata::scan_relocations(&input, &mut output));
		time(timings, "refine_labels", || labels::refine_labels(&input, &mut output));
		time(timings, "seed_exports", || metadata::seed_exports(&input, &mut output));
		time(timings, "label_strings", || strings::label_strings(&input, &mut output));
		time(timings, "label_imports", || metadata::label_imports(&input, &mut output));
		time(timings, "scan_exceptions", || metadata::scan_exceptions(&input, &mut output));
		time(timings, "seed_entry_points", || metadata::seed_entry_points(&input, &mut output));
		time(timings, "forward_feed", || forward_feeder::forward_feed(&input, &mut output));
		Ok(time(timings, "into_factmap", || output.into_factmap(&input)))
	})
}

/// PE image and decoding context shared by analysis passes.
pub struct AnalysisInput<'a> {
	pe: PeFile<'a>,
	bitness: u32,
	size: u32,
	alignment: u32,
}

/// Results accumulated by independently selectable analysis passes.
#[derive(Default)]
pub struct AnalysisOutput {
	/// Candidates indexed by RVA, available between passes.
	pub symbols: HashMap<u32, factmap::SymbolFact>,
	/// Direct call targets indexed by RVA, promoted to functions when emitting symbols.
	pub function_candidates: HashSet<u32>,
	/// Comments attached to instruction RVAs by analysis passes.
	pub comments: HashMap<u32, factmap::CommentFact>,
	/// Function metadata indexed by entry RVA; later discoveries replace earlier ones.
	pub functions: HashMap<u32, factmap::FunctionFact>,
	/// References from instruction RVAs to statically discovered symbol RVAs.
	pub references: Vec<factmap::RefFact>,
}

impl<'a> AnalysisInput<'a> {
	/// Validate and prepare an i386 or AMD64 image for analysis.
	pub fn new(pe: PeFile<'a>) -> Result<Self> {
		let bitness = match pe.file_header().Machine {
			image::IMAGE_FILE_MACHINE_I386 => 32,
			image::IMAGE_FILE_MACHINE_AMD64 => 64,
			machine => return Err(err(format!("unsupported machine type {machine:#06x}; expected i386 or AMD64"))),
		};
		let (size, alignment) = match pe.optional_header() {
			Wrap::T32(h) => (h.SizeOfImage, h.SectionAlignment),
			Wrap::T64(h) => (h.SizeOfImage, h.SectionAlignment),
		};
		Ok(AnalysisInput { pe, bitness, size, alignment })
	}
}

impl AnalysisOutput {
	/// Finish the analysis and return facts sorted by RVA.
	pub fn into_factmap(mut self, input: &AnalysisInput<'_>) -> factmap::FactMap {
		for rva in std::mem::take(&mut self.function_candidates) {
			self.add_symbol(input, rva, Some(ty::Type::Fn));
		}
		for symbol in self.symbols.values_mut() {
			if symbol.name == factmap::SymbolName::Code && symbol.ty == ty::Type::Unknown {
				upgrade_type(symbol, ty::Type::Code)
					.expect("code hints always upgrade successfully");
			}
		}
		let capacity = self.symbols.len() + self.comments.len() + self.references.len() + self.functions.len();
		let mut other_facts = Vec::with_capacity(capacity - self.references.len());
		other_facts.extend(self.symbols.into_values().map(factmap::Fact::Symbol));
		other_facts.extend(self.comments.into_values().map(factmap::Fact::Comment));
		other_facts.extend(self.functions.into_values().map(factmap::Fact::Function));
		// These maps have unique RVAs, so their fact sort keys are also unique.
		other_facts.sort_unstable_by_key(factmap::Fact::sort_key);
		// Usually already sorted by the linear scans. Keep equal-RVA references in
		// discovery order, even when executable sections are out of RVA order.
		self.references.sort_by_key(|reference| reference.rva);

		let mut facts = Vec::with_capacity(capacity);
		facts.extend(interleave_facts(
			other_facts.into_iter(),
			self.references.into_iter().map(factmap::Fact::Ref),
		));
		factmap::FactMap { facts }
	}
}

/// Interleave sorted streams, constructing only the next fact from each input.
fn interleave_facts(left: impl Iterator<Item = factmap::Fact>, right: impl Iterator<Item = factmap::Fact>) -> impl Iterator<Item = factmap::Fact> {
	let mut left = left.peekable();
	let mut right = right.peekable();
	std::iter::from_fn(move || match (left.peek(), right.peek()) {
		(Some(a), Some(b)) if a.sort_key() <= b.sort_key() => left.next(),
		(Some(_), Some(_)) => right.next(),
		(Some(_), None) => left.next(),
		(None, _) => right.next(),
	})
}

fn align_up(v: u32, a: u32) -> u32 {
	(v + a - 1) & !(a - 1)
}

impl AnalysisInput<'_> {
	fn executable(&self, rva: u32) -> bool {
		self.pe.section_headers().iter().any(|section| {
			if !section.is_code() {
				return false;
			}
			let offset = rva.wrapping_sub(section.VirtualAddress);
			let len = if section.VirtualSize == 0 { section.SizeOfRawData } else { section.VirtualSize };
			offset < align_up(len, self.alignment)
		})
	}

	fn read_only_data(&self, rva: u32) -> bool {
		self.pe.section_headers().iter().any(|section| {
			if !section.is_rdata() {
				return false;
			}
			let offset = rva.wrapping_sub(section.VirtualAddress);
			let len = if section.VirtualSize == 0 { section.SizeOfRawData } else { section.VirtualSize };
			offset < align_up(len, self.alignment)
		})
	}
}

impl AnalysisOutput {
	/// Add a symbol hint, returning whether the RVA is within the image.
	fn add_symbol(&mut self, input: &AnalysisInput<'_>, rva: u32, interpretation: Option<ty::Type>) -> bool {
		if rva >= input.size {
			return false;
		}
		let symbol = self.symbols.entry(rva).or_insert_with(|| {
			let name = if input.executable(rva) { factmap::SymbolName::Code }
				else if input.read_only_data(rva) { factmap::SymbolName::RData }
				else { factmap::SymbolName::Data };
			factmap::SymbolFact::new(rva, ty::Type::Unknown, name)
		});
		if let Some(interpretation) = interpretation {
			if interpretation == ty::Type::Fn && matches!(symbol.name, factmap::SymbolName::Data | factmap::SymbolName::RData | factmap::SymbolName::Code) {
				symbol.name = factmap::SymbolName::Fn;
			}
			else if interpretation == ty::Type::Code && matches!(symbol.name, factmap::SymbolName::Data | factmap::SymbolName::RData) {
				symbol.name = factmap::SymbolName::Code;
			}
			upgrade_type(symbol, interpretation)
				.expect("analysis hints are code, functions, or fixed-size numeric types");
		}
		true
	}

	fn add_symbol_va(&mut self, input: &AnalysisInput<'_>, va: u64, interpretation: Option<ty::Type>) {
		if let Some(rva) = va.checked_sub(input.pe.image_base()).and_then(|rva| u32::try_from(rva).ok()) {
			self.add_symbol(input, rva, interpretation);
		}
	}
}

/// Merge a type hint, preserving distinct fixed-size hints in a union.
/// Unknown hints add no evidence; functions take precedence over code, which
/// takes precedence over data hints. Both code and functions are unsized.
/// On a layout error, the previous type is unchanged.
fn upgrade_type(symbol: &mut factmap::SymbolFact, hint: ty::Type) -> result::Result<(), &'static str> {
	if hint == ty::Type::Unknown || symbol.ty == hint {
		return Ok(());
	}
	if symbol.ty == ty::Type::Fn {
		return Ok(());
	}
	if hint == ty::Type::Fn {
		symbol.ty = hint;
		return Ok(());
	}
	if symbol.ty == ty::Type::Unknown || hint == ty::Type::Code {
		symbol.ty = hint;
		return Ok(());
	}
	if symbol.ty == ty::Type::Code {
		return Ok(());
	}
	if let ty::Type::Struct(union) = &symbol.ty {
		if union.is_union && union.fields.iter().any(|field| field.ty == hint) {
			return Ok(());
		}
	}
	let (previous_size, previous_align) = symbol.ty.layout()?;
	let (hint_size, hint_align) = hint.layout()?;
	let align = previous_align.max(hint_align);
	let size = previous_size.max(hint_size).checked_add(align - 1)
		.ok_or("type layout overflow")? & !(align - 1);
	let field = ty::Field { name: ty::FieldName::Unnamed, offset: 0, ty: hint };
	if let ty::Type::Struct(union) = &mut symbol.ty {
		if union.is_union {
			union.fields.push(field);
			union.size = size;
			union.align = align;
			return Ok(());
		}
	}
	let previous = std::mem::replace(&mut symbol.ty, ty::Type::Unknown);
	symbol.ty = ty::Type::Struct(Box::new(ty::StructType {
		name: None,
		fields: vec![ty::Field { name: ty::FieldName::Unnamed, offset: 0, ty: previous }, field],
		is_union: true,
		size,
		align,
		is_dst: false,
	}));
	Ok(())
}
