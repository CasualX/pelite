#!/usr/bin/env sh

set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)

cargo build --manifest-path "$script_dir/Cargo.toml" --target wasm32-unknown-unknown --release
cp "$script_dir/../target/wasm32-unknown-unknown/release/pelite_wasm.wasm" "$script_dir/html/pelite.wasm"
