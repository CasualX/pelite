use super::*;
mod syntax;
use syntax::{Class, Text};

pub fn command() -> clap::Command {
	clap::Command::new("text-view")
		.about("Render fact-backed code and data in address order")
		.after_help(include_str!("docs/text-view.md"))
		.arg(clap::Arg::new("file").value_name("FILE").value_parser(clap::value_parser!(PathBuf)).required(true))
		.arg(clap::Arg::new("address").value_name("ADDRESS").value_parser(Address::parse).required(true))
		.arg(clap::Arg::new("length").value_name("BYTES").value_parser(value_parser::parse_u32).required(true))
		.arg(symbols::arg())
}

fn alignment_end(align: &AlignFact) -> Option<u32> {
	let boundary = 1u32.checked_shl(u32::from(align.align))?;
	align.rva.checked_add(boundary - 1).map(|end| end & !(boundary - 1))
}

/// Select relevant facts in loading order. Only annotations with replacement or
/// removal semantics need to revisit earlier selections; byte interpretations coexist.
fn select(facts: &[Fact], start: u32, end: u32) -> Vec<Fact> {
	let mut selected: Vec<Fact> = Vec::new();
	if start >= end {
		return selected;
	}
	for fact in facts {
		let (rva, extent) = match fact {
			Fact::Symbol(symbol) => (symbol.rva, if matches!(symbol.name, SymbolName::Undef) { None } else { symbol.ty.size().ok() }),
			Fact::Decode(decode) => {
				if decode.bytes == 0 {
					continue;
				}
				(decode.rva, Some(decode.bytes))
			},
			Fact::Align(align) => (align.rva, alignment_end(align).map(|end| end - align.rva)),
			Fact::Comment(comment) => (comment.rva, Some(0)),
			Fact::Function(function) => (function.rva, Some(0)),
			Fact::Ref(reference) => (reference.rva, Some(0)),
		};
		if rva >= end || (rva < start && extent.is_some_and(|size| u64::from(rva) + u64::from(size) <= u64::from(start))) {
			continue;
		}
		match fact {
			Fact::Symbol(symbol) => {
				if matches!(symbol.name, SymbolName::Undef) {
					selected.retain(|fact| !matches!(fact, Fact::Symbol(previous) if previous.rva == rva));
					continue;
				}
			},
			Fact::Comment(_) => selected.retain(|fact| !matches!(fact, Fact::Comment(previous) if previous.rva == rva)),
			Fact::Function(function) => {
				if let Some(previous) = selected.iter_mut().find_map(|fact| match fact {
					Fact::Function(previous) if previous.rva == rva => Some(previous),
					_ => None,
				}) {
					previous.merge_content(function.content.clone());
					continue;
				}
			},
			_ => {},
		}
		selected.push(fact.clone());
	}
	selected
}

/// Reference targets may lie outside the displayed range. Resolve their names
/// on demand rather than indexing every symbol in the input.
fn target_names(facts: &[Fact], rva: u32) -> String {
	let mut names = Vec::new();
	for fact in facts {
		let Fact::Symbol(symbol) = fact else { continue };
		if symbol.rva != rva { continue; }
		match symbol.name {
			SymbolName::Undef => names.clear(),
			_ => names.push(symbol),
		}
	}
	if names.is_empty() { String::new() }
	else { format!(" {}", names.into_iter().map(symbol_name).collect::<Vec<_>>().join(", ")) }
}

fn symbol_name(symbol: &SymbolFact) -> String {
	let prefix = match &symbol.name {
		SymbolName::Named(name) => return name.to_string(),
		SymbolName::Data => "data",
		SymbolName::RData => "rdata",
		SymbolName::Code => "code",
		SymbolName::Fn => "fn",
		SymbolName::Thunk => "thunk",
		SymbolName::Undef => unreachable!("undefined symbols are removed when loading facts"),
	};
	format!("{prefix}_{:x}", symbol.rva)
}

/// A zero length denotes an annotation, not a byte interpretation.
#[derive(Debug, PartialEq, Eq, serde::Serialize)]
struct Row {
	rva: u32,
	length: u32,
	kind: &'static str,
	text: String,
	syntax: Vec<syntax::Range>,
	#[serde(skip_serializing_if = "Option::is_none")]
	overlaps: Option<u32>,
}

struct View {
	start: u32,
	end: u32,
	rows: Vec<Row>,
}

impl View {
	fn add(&mut self, rva: u32, length: u32, kind: &'static str, text: Text) {
		if rva < self.end && (rva >= self.start || u64::from(rva) + u64::from(length) > u64::from(self.start)) {
			self.rows.push(Row { rva, length, kind, text: text.text, syntax: text.syntax, overlaps: None });
		}
	}

	fn error(&mut self, rva: u32, text: impl fmt::Display) {
		self.add(rva, 0, "error", Text::token(format!("; error: {text}"), Class::Error));
	}

	fn finish(&mut self, pe: pelite::PeFile<'_>) {
		// Coverage is independent of overlap detection; annotations consume no bytes.
		let mut covered = vec![false; (self.end - self.start) as usize];
		let mut boundaries = BTreeSet::from([self.end]);
		for row in &self.rows {
			boundaries.insert(row.rva);
			let start = row.rva.max(self.start);
			let end = (u64::from(row.rva) + u64::from(row.length)).min(u64::from(self.end)) as u32;
			if start < end {
				covered[(start - self.start) as usize..(end - self.start) as usize].fill(true);
			}
		}
		let mut offset = 0;
		let mut gap_end = 0;
		while offset < covered.len() {
			if covered[offset] { offset += 1; continue; }
			let rva = self.start + offset as u32;
			let boundary = *boundaries.range(rva + 1..).next().unwrap();
			if offset >= gap_end {
				gap_end = offset + covered[offset..].iter().take_while(|&&bit| !bit).count();
			}
			let mut count = (gap_end - offset).min((boundary - rva) as usize);
			let (kind, text) = match pe.slice_bytes(rva) {
				Ok(bytes) if !bytes.is_empty() => {
					count = count.min(bytes.len()).min(16);
					let mut text = Text::token(".bytes", Class::Directive);
					for byte in &bytes[..count] { text = text.plain(" ").then(format!("{byte:02x}"), Class::Number); }
					("bytes", text)
				},
				_ => {
					// Stop at the next section so a hole cannot hide file-backed bytes.
					if let Some(next) = pe.section_headers().iter().map(|s| s.VirtualAddress).filter(|&va| va > rva).min() {
						count = count.min((next - rva) as usize);
					}
					count = count.min(16);
					let mut text = Text::token(".bytes", Class::Directive);
					for _ in 0..count { text = text.plain(" ").then("??", Class::Unknown); }
					("unavailable", text)
				},
			};
			self.add(rva, count as u32, kind, text);
			offset += count;
		}
		self.rows.sort_by_key(|row| (row.rva, priority(row.kind)));
		let mut seen = HashSet::new();
		self.rows.retain(|row| seen.insert((row.rva, row.length, row.kind, row.text.clone())));
		// The earlier interval with the furthest end is enough to explain an overlap.
		let mut previous: Option<(u32, u64)> = None;
		for row in &mut self.rows {
			if row.length == 0 { continue; }
			if let Some((rva, end)) = previous && u64::from(row.rva) < end {
				row.overlaps = Some(rva);
			}
			let end = u64::from(row.rva) + u64::from(row.length);
			if previous.is_none_or(|(_, old_end)| end > old_end) {
				previous = Some((row.rva, end));
			}
		}
	}
}

fn priority(kind: &str) -> u8 {
	match kind { "symbol" => 0, "comment" | "function" => 1, "reference" => 3, _ => 2 }
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let mut facts = Vec::new();
	for path in matches.get_many::<PathBuf>("facts").into_iter().flatten() {
		let map = symbols::load_map(path, pointer_width(pe), Some(pe))?;
		facts.extend(map.facts);
	}
	let address = matches.get_one::<Address>("address").expect("required by clap");
	let mut indexed = symbols::IndexedFacts::default();
	if matches!(address, Address::Sym(_)) {
		indexed.extend(facts.iter().filter(|fact| matches!(fact, Fact::Symbol(_))).cloned(), pe.image_base());
	}
	let start = address.to_rva(pe, &indexed)?;
	let length = *matches.get_one::<u32>("length").expect("required by clap");
	let end = start.checked_add(length).ok_or_else(|| err("RVA plus length overflows"))?;
	let mut view = View { start, end, rows: Vec::new() };
	for fact in select(&facts, start, end) {
		match fact {
			Fact::Symbol(symbol) => {
				let name = symbol_name(&symbol);
				let aggregate = matches!(symbol.ty, ty::Type::Struct(_) | ty::Type::Array(_));
				let header = match &symbol.ty {
					ty::Type::Struct(structure) => {
						let mut text = Text::token(if structure.is_union { "union" } else { "struct" }, Class::Keyword);
						if let Some(name) = &structure.name { text = text.plain(" ").then(name, Class::Type); }
						text.plain(" ").then(&name, Class::Symbol).then(":", Class::Punctuation)
					},
					ty::Type::Code => Text::token(&name, Class::Symbol).then(":", Class::Punctuation),
					ty::Type::Fn | ty::Type::Unknown => Text::token(symbol.ty.to_string(), Class::Keyword)
						.plain(" ").then(&name, Class::Symbol).then(":", Class::Punctuation),
					_ => Text::token(&name, Class::Symbol).then(":", Class::Punctuation).plain(" ").then(symbol.ty.to_string(), Class::Type),
				};
				let separate_header = aggregate || symbol.ty.is_opaque();
				if separate_header {
					view.add(symbol.rva, 0, "symbol", header.clone());
				}
				if let Err(error) = typed(&mut view, pe, symbol.rva, &symbol.ty, if separate_header { "" } else { &name }) {
					if !separate_header { view.add(symbol.rva, 0, "symbol", header); }
					view.error(symbol.rva, error);
				}
			},
			Fact::Comment(comment) => {
				for line in comment.comment.lines() {
					view.add(comment.rva, 0, "comment", Text::token(format!("; {line}"), Class::Comment));
				}
			},
			Fact::Function(function) => view.add(function.rva, 0, "function", Text::token(format!("; function {}", function.content), Class::Comment)),
			Fact::Ref(reference) => {
				let mut text = Text::token("; -> ", Class::Comment).then(format!("{:#x}", reference.target_rva), Class::Address);
				let names = target_names(&facts, reference.target_rva);
				if !names.is_empty() { text = text.plain(" ").then(&names[1..], Class::Symbol); }
				view.add(reference.rva, 0, "reference", text);
			},
			Fact::Align(align) => match alignment_end(&align) {
				Some(end) if end > align.rva => view.add(align.rva, end - align.rva, "alignment", Text::directive(".align", 1u32 << align.align)),
				Some(_) => {},
				None => view.error(align.rva, "alignment exceeds RVA range"),
			},
			Fact::Decode(decode) => {
				if let Err(error) = instructions(&mut view, pe, &decode) {
					view.error(decode.rva.max(start), error);
				}
			},
		}
	}
	view.finish(pe);
	match format {
		OutputFormat::Nul => Ok(()),
		OutputFormat::Json | OutputFormat::JsonPretty => print_json(&view.rows, format == OutputFormat::JsonPretty),
		OutputFormat::Text => {
			let mut output = io::stdout().lock();
			let color = output.is_terminal();
			for row in &view.rows {
				write!(output, "{}:{:#x} {}", get_section_name_by_rva(pe, row.rva), row.rva,
					if row.kind == "symbol" { "" } else if row.overlaps.is_some() { "  ~ " } else { "    " })?;
				syntax::write(&mut output, &row.text, &row.syntax, color)?;
				if let Some(rva) = row.overlaps { write!(output, " ; overlaps @{rva:#x}")?; }
				writeln!(output)?;
			}
			Ok(())
		},
	}
}

fn instructions(view: &mut View, pe: pelite::PeFile<'_>, fact: &DecodeFact) -> Result {
	let bytes = pe.slice_bytes(fact.rva)?;
	let end = fact.rva.checked_add(fact.bytes).ok_or_else(|| err("decode region overflows RVA"))?;
	let bytes = &bytes[..bytes.len().min(fact.bytes as usize)];
	let mut decoder = iced_x86::Decoder::with_ip(fact.arch.bitness(), bytes, u64::from(fact.rva), iced_x86::DecoderOptions::NONE);
	let mut formatter = iced::Formatter::new(false, None);
	while decoder.can_decode() && decoder.ip() < u64::from(view.end) {
		let instruction = decoder.decode();
		let rva = instruction.ip() as u32;
		if instruction.is_invalid() {
			view.error(rva.max(view.start), format!("invalid or truncated instruction at {rva:#x}"));
			return Ok(());
		}
		let mut text = Text::default();
		formatter.format_to(&instruction, decoder.get_constant_offsets(&instruction), &[], 0, &mut text);
		view.add(rva, instruction.len() as u32, "instruction", text);
	}
	if decoder.ip() < u64::from(end.min(view.end)) {
		view.error((decoder.ip() as u32).max(view.start), "decode region exceeds file-backed bytes");
	}
	Ok(())
}

/// Flatten layout without following pointers. Dynamic arrays deliberately remain raw.
fn typed(view: &mut View, pe: pelite::PeFile<'_>, rva: u32, ty: &ty::Type, path: &str) -> Result {
	if rva >= view.end || ty.is_opaque() { return Ok(()); }
	if let Ok(size) = ty.size() && u64::from(rva) + u64::from(size) <= u64::from(view.start) { return Ok(()); }
	match ty {
		ty::Type::Struct(structure) => {
			let mut offset = 0;
			for (index, field) in structure.fields.iter().enumerate() {
				if !structure.is_union && field.offset > offset {
					view.add(rva.checked_add(offset).ok_or_else(|| err("field RVA overflow"))?, field.offset - offset, "padding", Text::directive(".pad", field.offset - offset));
				}
				let name = match &field.name { ty::FieldName::Named(name) => name.to_string(), _ => index.to_string() };
				let field_rva = rva.checked_add(field.offset).ok_or_else(|| err("field RVA overflow"))?;
				if let Err(error) = typed(view, pe, field_rva, &field.ty, &format!("{path}.{name}")) {
					view.error(field_rva, error);
				}
				offset = offset.max(field.offset + field.ty.size().unwrap_or(0));
			}
			if !structure.is_dst && structure.size > offset {
				view.add(rva.checked_add(offset).ok_or_else(|| err("padding RVA overflow"))?, structure.size - offset, "padding", Text::directive(".pad", structure.size - offset));
			}
		},
		ty::Type::Array(array) => {
			let ty::ArrayLen::Fixed(count) = array.len else { return Err(err("dynamic arrays are not supported in text-view")); };
			let stride = array.ty.size().map_err(err)?;
			if stride == 0 { return Ok(()); }
			let first = view.start.saturating_sub(rva) / stride;
			let last = ((u64::from(view.end - rva) + u64::from(stride) - 1) / u64::from(stride)).min(u64::from(count)) as u32;
			for index in first..last {
				let element = rva.checked_add(index.checked_mul(stride).ok_or_else(|| err("array offset overflow"))?).ok_or_else(|| err("array RVA overflow"))?;
				typed(view, pe, element, &array.ty, &format!("{path}[{index}]"))?;
			}
		},
		_ => {
			// A known layout still covers the field when its value is unavailable.
			// Do not turn missing file bytes into a zero or a typed-read error.
			if let Ok(length) = ty.size() && pe.slice_bytes(rva).map_or(true, |bytes| bytes.len() < length as usize) {
				view.add(rva, length, "data", Text::declaration(path, &ty.to_string(), "?", Class::Unknown));
				return Ok(());
			}
			let options = read::ReadOptions { zerofill: false, string_preview_length: 256, max_dynamic_array_length: 1024 };
			let (length, value) = match ty {
				ty::Type::Ptr32(_) | ty::Type::Ptr64(_) => {
					let va = if matches!(ty, ty::Type::Ptr32(_)) { u64::from(pe.derva_copy::<u32>(rva, false)?) } else { pe.derva_copy::<u64>(rva, false)? };
					let value = match pe.va_to_rva(va) { Ok(target) => format!("&{target:#x}"), Err(_) => format!("va:{va:#x}") };
					(ty.size().map_err(err)?, value)
				},
				_ => {
					let length = match ty {
						ty::Type::CStr | ty::Type::Utf16LEZ => {
							let width = if matches!(ty, ty::Type::CStr) { 1 } else { 2 };
							let bytes = pe.slice_bytes(rva)?;
							let units = bytes.chunks_exact(width).position(|unit| unit.iter().all(|&b| b == 0)).ok_or_else(|| err("unterminated string"))? + 1;
							u32::try_from(units * width).map_err(|_| err("string size overflow"))?
						},
						_ => ty.size().map_err(err)?,
					};
					let value = read::read_at(pe, rva, ty, &options);
					if read::contains_read_errors(&value) { return Err(err(value.to_string())); }
					(length, value.to_string())
				},
			};
			let class = match ty {
				ty::Type::Ptr32(_) | ty::Type::Ptr64(_) => Class::Address,
				ty::Type::CStr | ty::Type::Utf16LEZ => Class::String,
				_ => Class::Number,
			};
			view.add(rva, length, "data", Text::declaration(path, &ty.to_string(), &value, class));
		},
	}
	Ok(())
}
