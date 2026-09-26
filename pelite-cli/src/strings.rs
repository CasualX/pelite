use super::*;
use pelite::image;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Encoding {
	Ascii,
	Utf8,
	Utf16Le,
}

impl Encoding {
	fn label(self) -> &'static str {
		match self {
			Self::Ascii => "ascii",
			Self::Utf8 => "utf8",
			Self::Utf16Le => "utf16le",
		}
	}

	fn step(self) -> usize {
		if self == Self::Utf16Le { 2 } else { 1 }
	}
}

const MIN_LENGTH: usize = 4;
const MIN_LENGTH_NUL: usize = 3;

#[derive(Debug, Eq, PartialEq, serde::Serialize)]
struct FoundString<'a> {
	section: &'a str,
	address: u32,
	encoding: &'static str,
	confidence: i32,
	value: String,
}

#[derive(Debug)]
struct Candidate {
	start: usize,
	end: usize,
	encoding: Encoding,
	confidence: i32,
	value: String,
}

pub fn command() -> clap::Command {
	clap::Command::new("strings")
		.about("Find ASCII, UTF-8, and UTF-16LE strings in PE sections")
		.arg(clap::Arg::new("file").value_name("FILE").value_parser(clap::value_parser!(PathBuf)).required(true))
		.arg(clap::Arg::new("min-confidence")
			.long("min-confidence")
			.value_name("SCORE")
			.value_parser(clap::value_parser!(i32).range(0..))
			.default_value("40")
			.help("Minimum heuristic score (not a probability)"))
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let min_confidence = *matches.get_one("min-confidence").expect("defaulted by clap");
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	analyze(path, min_confidence, format)
}

fn analyze(path: &Path, min_confidence: i32, format: OutputFormat) -> Result {
	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let mut found = Vec::new();
	for section in pe.section_headers() {
		let Ok(bytes) = pe.get_section_bytes(section) else {
			continue;
		};
		let section_name = section.name().unwrap_or("<invalid>");
		for item in find_strings(bytes, min_confidence, section.Characteristics) {
			let Some(address) = u32::try_from(item.start).ok().and_then(|offset| section.VirtualAddress.checked_add(offset)) else {
				continue;
			};
			found.push(FoundString {
				section: section_name,
				address,
				encoding: item.encoding.label(),
				confidence: item.confidence,
				value: item.value,
			});
		}
	}
	print("Strings", &found, format)
}

fn section_score(flags: u32) -> i32 {
	if flags & (image::IMAGE_SCN_CNT_CODE | image::IMAGE_SCN_MEM_EXECUTE) != 0 {
		-10
	}
	else if flags & (image::IMAGE_SCN_CNT_INITIALIZED_DATA | image::IMAGE_SCN_MEM_READ) == (image::IMAGE_SCN_CNT_INITIALIZED_DATA | image::IMAGE_SCN_MEM_READ)
		&& flags & image::IMAGE_SCN_MEM_WRITE == 0 {
		10
	}
	else {
		0
	}
}

fn find_strings(bytes: &[u8], min_confidence: i32, section_flags: u32) -> Vec<Candidate> {
	let mut found = Vec::new();
	scan(bytes, Encoding::Utf8, 0, &mut found);
	scan(bytes, Encoding::Utf16Le, 0, &mut found);
	scan(bytes, Encoding::Utf16Le, 1, &mut found);
	let bias = section_score(section_flags);
	for item in &mut found {
		if item.confidence > 0 {
			item.confidence = (item.confidence + bias).max(0);
		}
	}
	// A contiguous byte string is stronger evidence than a wide interpretation
	// of those same printable bytes (for example, an embedded XML manifest).
	let byte_ranges: BTreeMap<usize, usize> = found.iter()
		.filter(|item| item.encoding != Encoding::Utf16Le)
		.map(|item| (item.start, item.end))
		.collect();
	// The two UTF-16 alignments can decode the same bytes into different text.
	// Keep the stronger overlapping interpretation, preferring the aligned one on ties.
	found.sort_by_key(|item| (std::cmp::Reverse(item.confidence), item.start));
	let mut chosen: Vec<Candidate> = Vec::new();
	let mut wide_ranges = BTreeMap::new();
	for item in found {
		if item.encoding == Encoding::Utf16Le {
			let overlaps_byte_before = byte_ranges.range(..=item.start).next_back().is_some_and(|(_, &end)| end > item.start);
			let overlaps_byte_after = byte_ranges.range(item.start..item.end).next().is_some();
			if overlaps_byte_before || overlaps_byte_after {
				continue;
			}
			let overlaps_previous = wide_ranges.range(..=item.start).next_back().is_some_and(|(_, &end)| end > item.start);
			let overlaps_next = wide_ranges.range(item.start..item.end).next().is_some();
			if overlaps_previous || overlaps_next {
				continue;
			}
			wide_ranges.insert(item.start, item.end);
		}
		chosen.push(item);
	}
	chosen.retain(|item| item.confidence >= min_confidence);
	chosen.sort_by_key(|item| (item.start, item.encoding != Encoding::Utf8));
	chosen
}

fn scan(bytes: &[u8], encoding: Encoding, alignment: usize, found: &mut Vec<Candidate>) {
	let mut pos = alignment;
	let mut start = pos;
	let mut value = String::new();
	while pos < bytes.len() {
		let decoded = decode(bytes, pos, encoding);
		match decoded {
			Some(('\0', width)) => {
				finish(&mut value, start, pos, true, encoding, found);
				pos += width;
				start = pos;
			},
			Some((ch, width)) if printable(ch) => {
				value.push(ch);
				pos += width;
			},
			_ => {
				finish(&mut value, start, pos, false, encoding, found);
				pos += encoding.step();
				start = pos;
			},
		}
	}
	finish(&mut value, start, pos, false, encoding, found);
}

fn finish(value: &mut String, start: usize, end: usize, nul: bool, encoding: Encoding, found: &mut Vec<Candidate>) {
	let length = value.chars().count();
	let minimum = if nul { MIN_LENGTH_NUL } else { MIN_LENGTH };
	if length >= minimum {
		let confidence = score(value, nul, encoding);
		let detected = if encoding == Encoding::Utf8 && value.is_ascii() { Encoding::Ascii } else { encoding };
		found.push(Candidate { start, end: end + if nul { encoding.step() } else { 0 }, encoding: detected, confidence, value: std::mem::take(value) });
		return;
	}
	value.clear();
}

fn decode(bytes: &[u8], pos: usize, encoding: Encoding) -> Option<(char, usize)> {
	match encoding {
		Encoding::Ascii => bytes.get(pos).copied().filter(|&b| b < 0x80).map(|b| (b as char, 1)),
		Encoding::Utf8 => {
			let first = *bytes.get(pos)?;
			let width = match first {
				0..=0x7f => 1,
				0xc2..=0xdf => 2,
				0xe0..=0xef => 3,
				0xf0..=0xf4 => 4,
				_ => return None,
			};
			let text = std::str::from_utf8(bytes.get(pos..pos + width)?).ok()?;
			Some((text.chars().next()?, width))
		},
		Encoding::Utf16Le => {
			let first = u16::from_le_bytes([*bytes.get(pos)?, *bytes.get(pos + 1)?]);
			if (0xd800..=0xdbff).contains(&first) {
				let second = u16::from_le_bytes([*bytes.get(pos + 2)?, *bytes.get(pos + 3)?]);
				let mut decoded = char::decode_utf16([first, second]);
				Some((decoded.next()?.ok()?, 4))
			}
			else {
				Some((char::from_u32(first as u32)?, 2))
			}
		},
	}
}

fn printable(ch: char) -> bool {
	ch == '\t' || (!ch.is_control() && ch != '\u{fffd}')
}

// A ranking signal, not a calibrated probability. Length and NUL termination
// are the strongest evidence; ordinary text and UTF-16 structure add support.
fn score(value: &str, nul: bool, encoding: Encoding) -> i32 {
	let length = value.chars().count();
	let mut points = 20 + (length.saturating_sub(3).min(7) * 4) as i32;
	if nul { points += 20; }
	if encoding == Encoding::Utf16Le {
		let Some(structure) = wide_structure(value) else { return 0; };
		points += structure;
	}
	if value.chars().any(char::is_alphanumeric) { points += 10; }
	if value.chars().filter(|ch| ch.is_alphabetic()).count() > length - length.div_ceil(4) { points += 10; }
	let first = value.chars().next().unwrap();
	if value.chars().all(|ch| ch == first) { return 0; }
	if value.chars().filter(|&ch| ch == '\t').count() > 1 { points -= 10; }
	points.max(0)
}

// Wide random bytes frequently decode to printable code points. Require either
// the zero-high-byte shape of Windows wide text or a dominant Unicode script.
fn wide_structure(value: &str) -> Option<i32> {
	let length = value.chars().count();
	let ascii = value.chars().filter(char::is_ascii).count();
	if ascii * 4 >= length * 3 {
		return Some(35);
	}
	let mut scripts = [0usize; 10];
	for ch in value.chars() {
		let script = match ch as u32 {
			0x00c0..=0x024f => 1, // Latin extensions
			0x0370..=0x03ff => 2, // Greek
			0x0400..=0x052f => 3, // Cyrillic
			0x0590..=0x05ff => 4, // Hebrew
			0x0600..=0x06ff => 5, // Arabic
			0x0900..=0x097f => 6, // Devanagari
			0x3040..=0x30ff | 0x3400..=0x9fff => 7, // Japanese and Han
			0xac00..=0xd7af => 8, // Hangul
			0x0e00..=0x0e7f => 9, // Thai
			_ => 0,
		};
		scripts[script] += 1;
	}
	let dominant = scripts[1..].iter().copied().max().unwrap_or(0);
	(length >= 8 && dominant * 5 >= length * 4).then_some(15)
}
