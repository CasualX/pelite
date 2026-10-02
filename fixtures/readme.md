# C++ analysis fixtures

This standalone CMake project builds binaries for RTTI and machine-code analysis.
`inheritance.cpp` covers polymorphic roots, single/multilevel inheritance,
multiple inheritance, nonvirtual and virtual diamonds, a single virtual base,
mixed virtual/nonvirtual inheritance, private/protected bases, abstract
interfaces, a nonpolymorphic base, and a template specialization. Runtime checks
exercise cross-casts, downcasts, access restrictions, and shared/repeated bases.

`throws.cpp` covers primitive and nonpolymorphic object throws, multiple and
virtual base exception conversions, nontrivial copy/destruction helpers,
mutable/const pointer throws, and rethrows. Runtime checks exercise catches by
value, reference, base, and pointer, plus object cleanup. This target uses
`/GR-` to verify exception analysis independently of vtable RTTI.

## Windows: MSVC

Requires CMake 3.26+, Visual Studio C++ Build Tools, and a Windows SDK. CMake's
Visual Studio generator discovers the installed toolchain without a developer
shell. Run from the repository root:

```sh
cmake -S fixtures -B fixtures/build/msvc-x64 -G "Visual Studio 17 2022" -A x64
cmake --build fixtures/build/msvc-x64 --config Release
ctest --test-dir fixtures/build/msvc-x64 -C Release --output-on-failure
```

For PE32, use a separate build directory and `-A Win32`:

```sh
cmake -S fixtures -B fixtures/build/msvc-x86 -G "Visual Studio 17 2022" -A Win32
cmake --build fixtures/build/msvc-x86 --config Release
ctest --test-dir fixtures/build/msvc-x86 -C Release --output-on-failure
```

Builds remain in ignored `build/` directories. To deliberately refresh the
precompiled fixtures used by Rust tests, install into this project:

```sh
cmake --install fixtures/build/msvc-x64 --config Release --prefix fixtures
cmake --install fixtures/build/msvc-x86 --config Release --prefix fixtures
cargo test -p pelite-cli msvc::rtti
cargo test -p pelite-cli msvc::throws
```

Each executable in `bin/` has a JSON manifest with its compiler, target,
configuration, and source/binary SHA-256 hashes. Builds retain exceptions,
base relocations, and distinct method bodies. Optimization and identical COMDAT
folding are disabled; the Windows CRT is static. `/Brepro` requests reproducible
linker output, but compiler/SDK versions can change bytes.

To refresh only the throw fixtures, add `--component throws` to the install
commands. Both targets are built and installed by default. The inheritance
target retains RTTI; the throws target deliberately disables it.

## Analysis

The Windows executables can be analyzed on any supported host:

```sh
cargo run -p pelite-cli -- msvc rtti fixtures/bin/inheritance-x64.exe
cargo run -p pelite-cli -- msvc rtti fixtures/bin/inheritance-x86.exe --format=json-pretty
cargo run -p pelite-cli -- msvc throws fixtures/bin/throws-x64.exe --demangle
cargo run -p pelite-cli -- msvc throws fixtures/bin/throws-x86.exe --format=json-pretty
```

The `fixture` namespace distinguishes test classes from CRT RTTI. `Diamond`
contains two `Root` subobjects, `VirtualDiamond` shares one virtual `Root`, and
`Mixed` contains both kinds. `Plain` has no vtable but appears in `WithPlain`'s
hierarchy. Abstract `Interface` can have a construction/destruction vtable.
The CLI reports `for_type: null` for virtual-base vtables whose runtime offsets
cannot be matched to fixed RTTI offsets; the hierarchy still identifies virtual
bases with `virtual_base: true` and `offset: null`.

Throw analysis reports seven fixture ThrowInfo records plus any linked CRT
exceptions. `DerivedException` is catchable as either nonvirtual base;
`VirtualException` carries a virtual-base displacement recipe and copy helper;
`OwnedException` carries both copy and destruction helpers. The const pointer
throw shares its catchable-type array with the mutable pointer throw, with the
const qualifier in ThrowInfo attributes. A rethrow creates no new record.

The throw command follows candidate instruction references to validated
metadata. It does not enumerate catch clauses or function unwind tables.
Fixture assertions check semantic results on both architectures, including
malformed metadata rejection. CLI snapshots in `bin/throws-*.txt` exercise
demangled text output; refresh them with `UPDATE_SNAPSHOTS=1 cargo test -p
pelite-cli --test snapshot msvc::throws` (set the environment variable using
your shell's syntax).
