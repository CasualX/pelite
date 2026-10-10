import { spawnSync } from "node:child_process";
import { copyFileSync, mkdirSync, rmSync } from "node:fs";
import { fileURLToPath } from "node:url";

const PACKAGE_ROOT = fileURLToPath(new URL("../", import.meta.url));
const DIST = new URL("../dist/", import.meta.url);
const HTML = new URL("../html/", import.meta.url);
const ARTIFACT_PATH = new URL("../../target/wasm32-unknown-unknown/release/pelite_wasm.wasm", import.meta.url);

function run(command, args) {
	const result = spawnSync(command, args, { cwd: PACKAGE_ROOT, stdio: "inherit" });
	if (result.error) throw result.error;
	if (result.status !== 0) process.exit(result.status ?? 1);
}

rmSync(DIST, { recursive: true, force: true });
mkdirSync(DIST, { recursive: true });
run("cargo", ["build", "--manifest-path", "rust/Cargo.toml", "--target", "wasm32-unknown-unknown", "--release"]);
copyFileSync(ARTIFACT_PATH, new URL("pelite.wasm", DIST));
copyFileSync(new URL("../src/pelite.js", import.meta.url), new URL("pelite.js", DIST));
run(process.execPath, ["node_modules/typescript/bin/tsc", "--project", "tsconfig.json"]);

for (const name of ["pelite.js", "pelite.wasm"]) {
	copyFileSync(new URL(name, DIST), new URL(name, HTML));
}
for (const name of ["Demo.dll", "Demo64.dll"]) {
	copyFileSync(new URL(`../../demo/${name}`, import.meta.url), new URL(name, HTML));
}
copyFileSync(new URL("../license.txt", import.meta.url), new URL("license.txt", HTML));
