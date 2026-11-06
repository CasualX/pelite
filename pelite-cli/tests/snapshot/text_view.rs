use super::*;
use std::sync::atomic::{AtomicU32, Ordering};

struct Sample(std::path::PathBuf);

impl Sample {
	fn new() -> Self {
		use pelite::pe64::image::*;
		use std::mem::size_of;
		static NEXT: AtomicU32 = AtomicU32::new(0);
		let dir = std::env::temp_dir().join(format!("pelite-text-view-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
		fs::create_dir_all(&dir).unwrap();
		let mut dos: IMAGE_DOS_HEADER = dataview::zeroed();
		dos.e_magic = IMAGE_DOS_SIGNATURE;
		dos.e_lfanew = size_of::<IMAGE_DOS_HEADER>() as u32;
		let mut nt: IMAGE_NT_HEADERS = dataview::zeroed();
		nt.Signature = IMAGE_NT_HEADERS_SIGNATURE;
		nt.FileHeader.Machine = IMAGE_FILE_MACHINE_AMD64;
		nt.FileHeader.NumberOfSections = 1;
		nt.FileHeader.SizeOfOptionalHeader = size_of::<IMAGE_OPTIONAL_HEADER>() as u16;
		nt.OptionalHeader.Magic = IMAGE_NT_OPTIONAL_HDR_MAGIC;
		nt.OptionalHeader.ImageBase = 0x180000000u64.into();
		nt.OptionalHeader.SectionAlignment = 0x1000;
		nt.OptionalHeader.FileAlignment = 0x200;
		nt.OptionalHeader.SizeOfHeaders = 0x200;
		nt.OptionalHeader.SizeOfImage = 0x2000;
		let mut section: IMAGE_SECTION_HEADER = dataview::zeroed();
		section.Name = *b".text\0\0\0";
		section.VirtualAddress = 0x1000;
		section.VirtualSize = 0x100;
		section.SizeOfRawData = 0x80;
		section.PointerToRawData = 0x200;
		let mut bytes = vec![0u8; 0x280];
		let mut offset = 0;
		for header in [dataview::bytes(&dos), dataview::bytes(&nt), dataview::bytes(&section)] {
			bytes[offset..offset + header.len()].copy_from_slice(header);
			offset += header.len();
		}
		bytes[0x200..0x208].copy_from_slice(&[0xeb, 0xff, 0xf0, 0x58, 0xcc, 0xcc, 0xcc, 0xcc]);
		bytes[0x210] = 1;
		bytes[0x214..0x218].copy_from_slice(&42u32.to_le_bytes());
		bytes[0x218..0x220].copy_from_slice(&0x180001030u64.to_le_bytes());
		bytes[0x230..0x236].copy_from_slice(b"hello\0");
		bytes[0x27f] = 0x0f; // Truncated instruction / unterminated string.
		fs::write(dir.join("sample.dll"), bytes).unwrap();
		Self(dir)
	}

	fn run(&self, address: &str, length: &str, layers: &[&str], format: &str) -> std::process::Output {
		let mut command = Command::new(env!("CARGO_BIN_EXE_pelite-cli"));
		command.args(["re", "text-view"]).arg(self.0.join("sample.dll")).args([address, length, "--format", format]);
		for (index, facts) in layers.iter().enumerate() {
			let path = self.0.join(format!("{index}.facts.txt"));
			fs::write(&path, format!("#factmap\n{facts}\n")).unwrap();
			command.arg("--facts").arg(path);
		}
		command.output().unwrap()
	}

	fn rows(&self, address: &str, length: &str, layers: &[&str]) -> Vec<serde_json::Value> {
		let output = self.run(address, length, layers, "json");
		assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
		let rows: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout).unwrap();
		for row in &rows {
			let text = row["text"].as_str().unwrap();
			let mut previous_end = 0;
			for range in row["syntax"].as_array().expect("every row has syntax ranges") {
				let start = range["start"].as_u64().unwrap() as usize;
				let end = range["end"].as_u64().unwrap() as usize;
				assert!(previous_end <= start && start < end && end <= text.len(), "invalid range: {row}");
				assert!(text.is_char_boundary(start) && text.is_char_boundary(end), "range splits UTF-8: {row}");
				assert!(range["class"].is_string());
				previous_end = end;
			}
		}
		rows
	}
}

fn tokens(row: &serde_json::Value) -> Vec<(&str, &str)> {
	let text = row["text"].as_str().unwrap();
	row["syntax"].as_array().unwrap().iter().map(|range| {
		(&text[range["start"].as_u64().unwrap() as usize..range["end"].as_u64().unwrap() as usize], range["class"].as_str().unwrap())
	}).collect()
}

#[test]
fn syntax_ranges_preserve_semantics_and_utf8_byte_offsets() {
	let sample = Sample::new();
	let rows = sample.rows("rva:0x1000", "0x38", &[MIXED]);
	assert_eq!(tokens(&rows[0]), [("fn", "keyword"), ("Entry", "symbol"), (":", "punctuation")]);
	let jump = rows.iter().find(|row| row["kind"] == "instruction").unwrap();
	assert_eq!(tokens(jump), [("jmp", "mnemonic"), ("short", "keyword"), ("0x1001", "address")]);
	let push = rows.iter().find(|row| row["text"] == "push rax").unwrap();
	assert_eq!(tokens(push), [("push", "mnemonic"), ("rax", "register")]);
	let ptr = rows.iter().find(|row| row["text"] == ".handler: *cstr = &0x1030").unwrap();
	assert_eq!(tokens(ptr), [(".handler", "field"), (":", "punctuation"), ("*cstr", "type"), ("=", "punctuation"), ("&", "punctuation"), ("0x1030", "address")]);
	let string = rows.iter().find(|row| row["text"] == "Greeting: cstr = \"hello\"").unwrap();
	assert_eq!(tokens(string).last(), Some(&("\"hello\"", "string")));
	let rows = sample.rows("rva:0x1014", "4", &["Sx1014 u32 \"café: 🦀\"\nCx1014 \"漢字; comment\"\nRx1014 0x1030\nSx1030 cstr \"挨拶\""]);
	assert_eq!(tokens(&rows[0]), [("; 漢字; comment", "comment")]);
	assert_eq!(tokens(&rows[1]), [("café: 🦀", "symbol"), (":", "punctuation"), ("u32", "type"), ("=", "punctuation"), ("42", "number")]);
	assert_eq!(rows[1]["syntax"][0]["end"], "café: 🦀".len());
	assert_eq!(tokens(&rows[2]), [("; -> ", "comment"), ("0x1030", "address"), ("挨拶", "symbol")]);
	let text = sample.run("rva:0x1014", "4", &["Sx1014 u32 \"café: 🦀\""], "text");
	assert!(!text.stdout.contains(&0x1b), "redirected text must remain plain");
}

#[test]
fn instruction_syntax_distinguishes_addresses_from_immediates() {
	let sample = Sample::new();
	let path = sample.0.join("sample.dll");
	let mut bytes = fs::read(&path).unwrap();
	// mov rax,[rip-0x17]; mov eax,42; jmp rax
	bytes[0x240..0x24e].copy_from_slice(&[0x48, 0x8b, 0x05, 0xe9, 0xff, 0xff, 0xff, 0xb8, 42, 0, 0, 0, 0xff, 0xe0]);
	fs::write(path, bytes).unwrap();
	let rows = sample.rows("rva:0x1040", "14", &["Dx1040 x86_64 14"]);
	assert!(tokens(&rows[0]).contains(&("0x1030", "address")));
	assert!(tokens(&rows[1]).contains(&("0x2a", "number")));
	assert_eq!(tokens(&rows[2]), [("jmp", "mnemonic"), ("rax", "register")]);
}

impl Drop for Sample {
	fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); }
}

const MIXED: &str = r#"Sx1000 fn "Entry"
Cx1001 "Inside the jump\nz first\na second"
Dx1000 x86_64 2
Dx1001 x86_64 3
Dx1001 x86_64 3
Rx1000 0x1030
Rx1000 0x1001
Sx1002 u8 "Embedded"
Sx1010 "struct { enabled: u8, count: u32, handler: *cstr, entries: [struct { id: u16, active: u8 }; 2] }" "Record"
Sx1030 cstr "Greeting"
Ax1036 4"#;

#[test]
fn mixed_snapshot() {
	let sample = Sample::new();
	let output = sample.run("rva:0x1000", "0x44", &[MIXED], "text");
	assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
	assert_snapshot(&Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/snapshot/text-view.txt"), &String::from_utf8(output.stdout).unwrap());
}

#[test]
fn overlapping_streams_and_window_boundaries() {
	let sample = Sample::new();
	let rows = sample.rows("rva:0x1001", "2", &[MIXED]);
	let instructions: Vec<_> = rows.iter().filter(|r| r["kind"] == "instruction").collect();
	assert_eq!(instructions.len(), 2);
	assert_eq!(instructions[0]["rva"], 0x1000);
	assert_eq!(instructions[0]["length"], 2);
	assert_eq!(instructions[1]["rva"], 0x1001);
	assert_eq!(instructions[1]["text"], "push rax");
	assert_eq!(instructions[1]["overlaps"], 0x1000);
	let comments: Vec<_> = rows.iter().filter(|r| r["kind"] == "comment").map(|r| r["text"].as_str().unwrap()).collect();
	assert_eq!(comments, ["; Inside the jump", "; z first", "; a second"]);
	assert!(!rows.iter().any(|r| r["kind"] == "bytes" || r["kind"] == "error"));
}

#[test]
fn gaps_annotations_and_unavailable_bytes() {
	let sample = Sample::new();
	let rows = sample.rows("rva:0x1004", "8", &["Cx1006 \"middle\""]);
	assert_eq!(rows.iter().map(|r| r["rva"].as_u64().unwrap()).collect::<Vec<_>>(), [0x1004, 0x1006, 0x1006]);
	assert_eq!(rows[0]["length"], 2);
	assert_eq!(rows[2]["kind"], "bytes");
	assert_eq!(rows[2]["text"], ".bytes cc cc 00 00 00 00");
	let rows = sample.rows("rva:0x107e", "6", &[]);
	assert_eq!(rows.last().unwrap()["kind"], "unavailable");
	assert_eq!(rows.last().unwrap()["length"], 4);
	assert_eq!(rows.last().unwrap()["text"], ".bytes ?? ?? ?? ??");
	let rows = sample.rows("rva:0x1080", "20", &[]);
	assert_eq!(rows.iter().map(|row| row["length"].as_u64().unwrap()).collect::<Vec<_>>(), [16, 4]);
	assert!(sample.rows("rva:0x1000", "0", &[MIXED]).is_empty());
}

#[test]
fn fields_unions_and_pointer_slots() {
	let sample = Sample::new();
	let rows = sample.rows("rva:0x1015", "0x23", &[MIXED, "Sx1036 \"union { a:u16, b:u32 }\" \"Union\""]);
	assert!(rows.iter().any(|r| r["rva"] == 0x1014 && r["text"].as_str().unwrap().starts_with(".count: u32 = 42")));
	let pointer = rows.iter().find(|r| r["rva"] == 0x1018).unwrap();
	assert_eq!(pointer["length"], 8);
	assert_eq!(pointer["text"], ".handler: *cstr = &0x1030");
	assert!(rows.iter().any(|r| r["text"].as_str().unwrap().starts_with(".entries[1].id")));
	assert!(rows.iter().any(|r| r["rva"] == 0x1036 && !r["overlaps"].is_null()));
	let rows = sample.rows("rva:0x1018", "0x20", &["Sx1018 *cstr \"Ptr\""]);
	assert!(!rows.iter().any(|r| r["kind"] == "data" && r["rva"] == 0x1030));
}

#[test]
fn overlays_and_failed_interpretations() {
	let sample = Sample::new();
	let layers = ["Sx1014 u32 \"Old\"\nCx1014 \"old\"", "Sx1014 u8 \"New\"\nCx1014 \"new\""];
	let rows = sample.rows("sym:New", "4", &layers);
	assert_eq!(rows[0]["text"], "; new");
	assert_eq!(rows[1]["text"], "Old: u32 = 42");
	assert_eq!(rows[2]["text"], "New: u8 = 42");
	assert_eq!(rows[1]["length"], 4);
	assert_eq!(rows[2]["length"], 1);
	assert_eq!(rows[2]["overlaps"], 0x1014);
	assert!(!sample.run("sym:Old", "1", &layers, "json").status.success());
	let rows = sample.rows("rva:0x1014", "4", &[layers[0], layers[1], "Sx1014 unk undef"]);
	assert!(!rows.iter().any(|r| r["kind"] == "data" || r["kind"] == "symbol"));
	let rows = sample.rows("rva:0x1015", "1", &[layers[0], "Sx1014 u8 undef"]);
	assert!(!rows.iter().any(|r| r["kind"] == "data"));
	let rows = sample.rows("rva:0x1080", "1", &["Dx1080 x86_64 0"]);
	assert_eq!(rows.len(), 1);
	assert_eq!(rows[0]["kind"], "unavailable");
	for facts in ["Dx107f x86_64 1", "Sx107f cstr \"Bad\"", "Ax107f 255", "Sx107f \"[u8; count]\" \"Bad\""] {
		let rows = sample.rows("rva:0x107f", "1", &[facts]);
		assert!(rows.iter().any(|r| r["kind"] == "error"), "{facts}");
		assert!(rows.iter().any(|r| r["kind"] == "bytes"), "{facts}");
	}
	assert!(!sample.run("rva:0xffffffff", "1", &[], "json").status.success());
	assert!(sample.run("rva:0x1000", "4", &[MIXED], "nul").stdout.is_empty());
}

#[test]
fn unavailable_typed_fields_preserve_layout_and_coverage() {
	let sample = Sample::new();
	let facts = "Sx1078 \"struct { present: u32, partial: u64, ptr: *unk, items: [u16; 2] }\" \"Record\"";
	let rows = sample.rows("rva:0x1078", "0x20", &[facts]);
	let fields: Vec<_> = rows.iter().filter(|row| row["kind"] == "data").collect();
	assert_eq!(fields.iter().map(|row| row["text"].as_str().unwrap()).collect::<Vec<_>>(), [
		".present: u32 = 0", ".partial: u64 = ?", ".ptr: *unk = ?", ".items[0]: u16 = ?", ".items[1]: u16 = ?",
	]);
	assert_eq!(fields[1]["rva"], 0x1080);
	assert_eq!(fields[1]["length"], 8);
	assert!(!rows.iter().any(|row| matches!(row["kind"].as_str(), Some("error" | "bytes" | "unavailable"))));
	// A field straddling file-backed bytes and zero-fill has no complete value.
	let rows = sample.rows("rva:0x107f", "1", &["Sx107e u32 \"Partial\""]);
	assert_eq!(rows.last().unwrap()["rva"], 0x107e);
	assert_eq!(rows.last().unwrap()["length"], 4);
	assert_eq!(rows.last().unwrap()["text"], "Partial: u32 = ?");
	// An unavailable pointer retains its full slot and does not follow its pointee.
	let rows = sample.rows("rva:0x1080", "8", &["Sx1080 \"struct { ptr: *unk }\" \"Head\""]);
	assert_eq!(rows.len(), 2);
	assert_eq!(rows[1]["text"], ".ptr: *unk = ?");
	assert_eq!(rows[1]["length"], 8);
}

#[test]
fn multiple_symbols_and_shared_name_lookup() {
	let sample = Sample::new();
	let first = r#"Sx1030 u8 "Same"
Sx1014 u32 "Same"
Sx1030 u16 "Alias"
Rx1014 0x1030"#;
	let rows = sample.rows("sym:Alias", "2", &[first]);
	assert_eq!(rows[0]["rva"], 0x1030);
	assert_eq!(rows[0]["text"], "Same: u8 = 104");
	assert_eq!(rows[1]["text"], "Alias: u16 = 25960");
	let rows = sample.rows("sym:Same", "1", &[first]);
	assert_eq!(rows[0]["rva"], 0x1014);
	let rows = sample.rows("rva:0x1014", "1", &[first]);
	assert!(rows.iter().any(|r| r["text"] == "; -> 0x1030 Same, Alias"));
	let rows = sample.rows("sym:Same", "1", &[first, "Sx1030 unk undef"]);
	assert_eq!(rows[0]["rva"], 0x1014);
	let rows = sample.rows("sym:Revived", "2", &[first, "Sx1030 unk undef\nSx1030 u16 \"Revived\""]);
	assert_eq!(rows[0]["text"], "Revived: u16 = 25960");
	assert!(!rows.iter().any(|r| r["text"] == "Same: u8 = 104" || r["text"] == "Alias: u16 = 25960"));
	assert!(!sample.run("sym:Missing", "1", &[first], "json").status.success());
}
