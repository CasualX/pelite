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
fn dump_detects_both_pe_formats() {
	for (file, format) in [("Demo.dll", "PE32"), ("Demo64.dll", "PE32+")] {
		let path = demo(file);
		let value = json(&["--format=json", "dump", path.to_str().unwrap(), "--headers"]);
		assert_eq!(value["format"], format);
		assert!(value["headers"].is_object());
	}
}

#[test]
fn every_command_supports_json() {
	let pe32 = demo("Demo.dll");
	let path = pe32.to_str().unwrap();
	let pe64 = demo("Demo64.dll");
	let path64 = pe64.to_str().unwrap();

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
	assert!(stdout.contains("\n\n?fnPasswdsBypass@@YAHXZ:\n0x10001220"), "unexpected disassembly:\n{stdout}");
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
fn extract_icons_writes_a_manifest() {
	let pe32 = demo("Demo.dll");
	let destination = std::env::temp_dir().join(format!(
		"pelite-cli-icons-{}-{}",
		std::process::id(),
		std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
	));
	let value = json(&[
		"extract-icons",
		pe32.to_str().unwrap(),
		destination.to_str().unwrap(),
		"--format=json",
	]);
	assert!(!value.as_array().unwrap().is_empty());
	for entry in value.as_array().unwrap() {
		assert!(PathBuf::from(entry["path"].as_str().unwrap()).is_file());
	}
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
		.args(["dump", "input.exe", "--format=yaml"])
		.output()
		.expect("run pelite-cli");
	assert!(!invalid.status.success());
	assert!(String::from_utf8_lossy(&invalid.stderr).contains("possible values: text, json, json-pretty"));
}
