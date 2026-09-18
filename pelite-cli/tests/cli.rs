use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

fn demo(name: &str) -> PathBuf {
	PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../demo").join(name)
}

fn json(args: &[&str]) -> Value {
	let output = Command::new(env!("CARGO_BIN_EXE_pelite-cli")).args(args).output().expect("run pelite-cli");
	assert!(output.status.success(), "pelite-cli failed: {}", String::from_utf8_lossy(&output.stderr));
	serde_json::from_slice(&output.stdout).expect("valid JSON output")
}

#[test]
fn inspect_detects_both_pe_formats() {
	for (file, format) in [("Demo.dll", "PE32"), ("Demo64.dll", "PE32+")] {
		let path = demo(file);
		let value = json(&["--format=json", "inspect", path.to_str().unwrap(), "all"]);
		assert_eq!(value["format"], format);
		assert!(value["headers"].is_object());
		assert!(value.get("resources").is_none());
	}
}

#[test]
fn inspect_sections_uses_a_text_table() {
	let pe32 = demo("Demo.dll");
	let path = pe32.to_str().unwrap();
	let output = Command::new(env!("CARGO_BIN_EXE_pelite-cli"))
		.args(["inspect", path, "sections"])
		.output()
		.expect("run pelite-cli");
	assert!(output.status.success(), "pelite-cli failed: {}", String::from_utf8_lossy(&output.stderr));
	let stdout = String::from_utf8(output.stdout).unwrap();
	assert!(stdout.starts_with("PE image\n========\nformat : PE32\n\nsections:\n"));
	assert!(stdout.contains("Name"));
	assert!(stdout.contains("VirtualAddress"));
	assert!(stdout.contains(".text"));
	assert!(!stdout.contains("Relocations"));
	assert!(!stdout.contains("Linenumbers"));

	let value = json(&["inspect", path, "sections", "--format=json"]);
	let section = &value["sections"][0];
	assert!(section.get("PointerToRelocations").is_none());
	assert!(section.get("PointerToLinenumbers").is_none());
	assert!(section.get("NumberOfRelocations").is_none());
	assert!(section.get("NumberOfLinenumbers").is_none());
}

#[test]
fn inspect_combines_requested_topics() {
	let pe32 = demo("Demo.dll");
	let path = pe32.to_str().unwrap();
	let selected = json(&["inspect", path, "imports", "exports", "--format=json"]);
	assert!(selected["imports"].is_array());
	assert!(selected["exports"].is_object());
	assert_eq!(selected["format"], "PE32");
	assert!(selected.get("headers").is_none());

	let all = json(&["inspect", path, "imports", "exports", "all", "--format=json"]);
	assert!(all["headers"].is_object());
	assert!(all["sections"].is_array());
}

#[test]
fn resources_are_browsable_and_binary_safe() {
	let pe32 = demo("Demo.dll");
	let path = pe32.to_str().unwrap();
	let resource = "/#MANIFEST/#2/#1033";

	let tree = json(&["resources", "tree", path, "--format=json"]);
	assert_eq!(tree["files"], 7);
	assert!(tree["entries"].as_array().unwrap().iter().any(|entry| entry["path"] == resource));
	assert!(tree["entries"].as_array().unwrap().iter().all(|entry| entry.get("data").is_none()));

	let raw = Command::new(env!("CARGO_BIN_EXE_pelite-cli"))
		.args(["resources", "cat", path, resource])
		.output()
		.expect("run pelite-cli");
	assert!(raw.status.success(), "pelite-cli failed: {}", String::from_utf8_lossy(&raw.stderr));
	assert!(raw.stdout.starts_with(b"<?xml"));

	let encoded = json(&["resources", "cat", path, resource, "--format=json"]);
	assert_eq!(encoded["encoding"], "base64");
	assert_eq!(encoded["size"], raw.stdout.len());
	assert_eq!(basenc::Base64Std.decode(encoded["data"].as_str().unwrap()).unwrap(), raw.stdout);

	let report = json(&["resources", "fsck", path, "--format=json"]);
	assert_eq!(report["valid"], true);
	assert_eq!(report["files"], tree["files"]);
}

#[test]
fn every_command_supports_json() {
	let pe32 = demo("Demo.dll");
	let path = pe32.to_str().unwrap();
	let pe64 = demo("Demo64.dll");
	let path64 = pe64.to_str().unwrap();

	assert!(json(&["inspect", path, "headers", "--format=json"])["headers"].is_object());
	assert!(json(&["resources", "tree", path, "--format=json"])["entries"].is_array());
	assert!(json(&["strings", path, "--format=json"]).is_array());
	assert!(json(&["disasm", path, "1000..1010", "--format=json"]).is_array());
	assert!(json(&["hexdump", path, "1000..1010", "--format=json"]).is_array());
	assert!(json(&["findsig", path, "55 8B EC", "--format=json"]).is_array());
	assert!(json(&["imphash", path, "--format=json"]).is_array());
	assert!(json(&["markov", "8", path, "--seed=1", "--format=json"])["bytes"].is_array());
	assert!(json(&["module-def", path, "--format=json"])["exports"].is_array());
	assert!(json(&["msrtti", path, "--format=json"]).is_array());
	assert!(json(&["rust-format-args", path64, "--format=json"]).is_array());
	assert!(json(&["rust-panic-strings", path64, "--format=json"]).is_array());
	assert!(json(&["version-info", path, "--format=json"])["info"].is_object());
}

#[test]
fn imphash_is_stable_and_standard_length() {
	let pe32 = demo("Demo.dll");
	let value = json(&["imphash", pe32.to_str().unwrap(), "--format=json"]);
	assert_eq!(value[0]["hash"], "7e330da5c7f05d0fd1f12eef6d7bd950");
}

#[test]
fn disasm_resolves_pe_symbols() {
	let pe32 = demo("Demo.dll");
	let value = json(&["disasm", pe32.to_str().unwrap(), "1240..1260", "--format=json"]);
	assert_eq!(value[0]["address"], 0x10001240_u64);
	assert!(value[0].get("rva").is_none());
	assert!(value.as_array().unwrap().iter().any(|instruction| instruction["instruction"] == "call dword ptr [MSVCR120.dll!_strdup]"));

	let pe64 = demo("Demo64.dll");
	let value = json(&["disasm", pe64.to_str().unwrap(), "13a0..13b8", "--format=json"]);
	assert_eq!(value[0]["address"], 0x1800013a0_u64);
	assert!(value.as_array().unwrap().iter().any(|instruction| instruction["instruction"] == "mov [?nPasswds@@3HA],eax"));
}

#[test]
fn disasm_prints_export_labels() {
	let pe32 = demo("Demo.dll");
	let output = Command::new(env!("CARGO_BIN_EXE_pelite-cli"))
		.args(["disasm", pe32.to_str().unwrap(), "1200..1230"])
		.output()
		.expect("run pelite-cli");
	assert!(output.status.success(), "pelite-cli failed: {}", String::from_utf8_lossy(&output.stderr));
	let stdout = String::from_utf8(output.stdout).unwrap();
	assert!(stdout.contains("\n\n?fnPasswdsBypass@@YAHXZ:\n.text:0x10001220"), "unexpected disassembly:\n{stdout}");
}

#[test]
fn hexdump_aligns_partial_rows() {
	let pe32 = demo("Demo.dll");
	let output = Command::new(env!("CARGO_BIN_EXE_pelite-cli"))
		.args(["hexdump", pe32.to_str().unwrap(), "1005..1013"])
		.output()
		.expect("run pelite-cli");
	assert!(output.status.success(), "pelite-cli failed: {}", String::from_utf8_lossy(&output.stderr));
	let stdout = String::from_utf8(output.stdout).unwrap();
	let lines = stdout.lines().collect::<Vec<_>>();
	assert_eq!(lines.len(), 2);
	assert!(lines[0].starts_with("0x10001000"));
	assert!(lines[0].ends_with("|     ...........|"));
	assert!(lines[1].starts_with("0x10001010"));
	assert!(lines[1].ends_with("|.A.             |"));

	let value = json(&["hexdump", pe32.to_str().unwrap(), "1000..1005", "--format=json"]);
	assert_eq!(value, serde_json::json!([0xb8, 1, 0, 0, 0]));
}

#[test]
fn icon_groups_can_be_listed_and_extracted() {
	let pe32 = demo("Demo.dll");
	let path = pe32.to_str().unwrap();
	let listed = json(&["resources", "icons", "list", path, "--format=json"]);
	assert_eq!(listed, serde_json::json!([{
		"kind": "icon",
		"name": "#103",
		"images": 1,
		"bytes": 9662,
	}]));
	assert_eq!(json(&["resources", "cursors", "list", path, "--format=json"]), serde_json::json!([]));

	let destination = std::env::temp_dir().join(format!(
		"pelite-cli-icons-{}-{}",
		std::process::id(),
		std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
	));
	let value = json(&[
		"resources",
		"icons",
		"extract",
		path,
		destination.to_str().unwrap(),
		"--format=json",
	]);
	assert_eq!(value.as_array().unwrap().len(), 1);
	let icon_path = PathBuf::from(value[0]["path"].as_str().unwrap());
	assert_eq!(icon_path.file_name().unwrap(), "103.ico");
	assert_eq!(&std::fs::read(&icon_path).unwrap()[..4], &[0, 0, 1, 0]);

	let collision = Command::new(env!("CARGO_BIN_EXE_pelite-cli"))
		.args(["resources", "icons", "extract", path, destination.to_str().unwrap(), "#103"])
		.output()
		.expect("run pelite-cli");
	assert!(!collision.status.success());
	assert!(String::from_utf8_lossy(&collision.stderr).contains("--force"));
	std::fs::remove_dir_all(destination).unwrap();
}

#[test]
fn json_can_be_compact_or_pretty() {
	let pe32 = demo("Demo.dll");
	let path = pe32.to_str().unwrap();
	let run = |format: &str| {
		let output = Command::new(env!("CARGO_BIN_EXE_pelite-cli")).args(["module-def", path, format]).output().expect("run pelite-cli");
		assert!(output.status.success());
		output.stdout
	};

	let compact = run("--format=json");
	let pretty = run("--format=json-pretty");
	assert_eq!(compact.iter().filter(|&&byte| byte == b'\n').count(), 1);
	assert!(pretty.iter().filter(|&&byte| byte == b'\n').count() > 1);
	assert_eq!(serde_json::from_slice::<Value>(&compact).unwrap(), serde_json::from_slice::<Value>(&pretty).unwrap());
}

#[test]
fn msrtti_rejects_pe32_plus() {
	let pe64 = demo("Demo64.dll");
	let output = Command::new(env!("CARGO_BIN_EXE_pelite-cli"))
		.args(["msrtti", pe64.to_str().unwrap()])
		.output()
		.expect("run pelite-cli");
	assert!(!output.status.success());
	assert!(String::from_utf8_lossy(&output.stderr).contains("not a PE32 image"));
}

#[test]
fn clap_validates_arguments_and_prints_help() {
	let help = Command::new(env!("CARGO_BIN_EXE_pelite-cli")).arg("--help").output().expect("run pelite-cli");
	assert!(help.status.success());
	assert!(String::from_utf8_lossy(&help.stdout).contains("Commands:"));

	let invalid = Command::new(env!("CARGO_BIN_EXE_pelite-cli"))
		.args(["inspect", "input.exe", "headers", "--format=yaml"])
		.output()
		.expect("run pelite-cli");
	assert!(!invalid.status.success());
	assert!(String::from_utf8_lossy(&invalid.stderr).contains("possible values: text, json, json-pretty"));
}
