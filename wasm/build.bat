@echo off
cargo build --target=wasm32-unknown-unknown --release
copy /B /Y "%~dp0\..\target\wasm32-unknown-unknown\release\pelite_wasm.wasm" "%~dp0\pkg\pelite.wasm"
