use std::{fs, path::Path, process::Command};

mod diff;

fn assert_snapshot(snapshot_path: &Path, value: &str) {
	if std::env::var_os("UPDATE_SNAPSHOTS").is_some() {
		fs::write(snapshot_path, value).unwrap();
		return;
	}

	let expected = fs::read_to_string(snapshot_path).unwrap_or_else(|err| {
		panic!("Snapshot not found at {}: {err}.\nRun with UPDATE_SNAPSHOTS=1 to create it.", snapshot_path.display())
	});

	if !expected.lines().eq(value.lines()) {
		let diff = diff::render_line_diff(snapshot_path, &expected, value);
		panic!("Snapshot mismatch at {}.\n\n{diff}\nRun with UPDATE_SNAPSHOTS=1 to update the snapshot.", snapshot_path.display());
	}
}

mod msvc {
	use super::*;

	fn rtti(arch: &str) {
		let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures/bin");
		let output = Command::new(env!("CARGO_BIN_EXE_pelite-cli"))
			.args(["re", "msvc", "rtti"])
			.arg(fixtures.join(format!("inheritance-{arch}.exe")))
			.output()
			.expect("failed to run pelite-cli");
		assert!(output.status.success(), "pelite-cli re msvc rtti ({arch}) failed: {}\n{}", output.status, String::from_utf8_lossy(&output.stderr));
		assert!(output.stderr.is_empty(), "unexpected stderr: {}", String::from_utf8_lossy(&output.stderr));
		let text = String::from_utf8(output.stdout).expect("RTTI output is not UTF-8");
		assert!(!text.is_empty(), "RTTI output is empty");
		assert_snapshot(&fixtures.join(format!("inheritance-{arch}.txt")), &text);
	}

	#[test]
	fn rtti_x86() {
		rtti("x86");
	}

	#[test]
	fn rtti_x64() {
		rtti("x64");
	}
}
