# C++ inheritance fixture

This standalone CMake project builds binaries for RTTI and machine-code analysis.
`inheritance.cpp` covers polymorphic roots, single/multilevel inheritance,
multiple inheritance, nonvirtual and virtual diamonds, a single virtual base,
mixed virtual/nonvirtual inheritance, private/protected bases, abstract
interfaces, a nonpolymorphic base, and a template specialization. Runtime checks
exercise cross-casts, downcasts, access restrictions, and shared/repeated bases.

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
```

Each executable in `bin/` has a JSON manifest with its compiler, target,
configuration, and source/binary SHA-256 hashes. Builds retain RTTI, exceptions,
base relocations, and distinct method bodies. Optimization and identical COMDAT
folding are disabled; the Windows CRT is static. `/Brepro` requests reproducible
linker output, but compiler/SDK versions can change bytes.

## Analysis

The Windows executables can be analyzed on any supported host:

```sh
cargo run -p pelite-cli -- re msvc rtti fixtures/bin/inheritance-x64.exe
cargo run -p pelite-cli -- re msvc rtti fixtures/bin/inheritance-x86.exe --format=json-pretty
```

The `fixture` namespace distinguishes test classes from CRT RTTI. `Diamond`
contains two `Root` subobjects, `VirtualDiamond` shares one virtual `Root`, and
`Mixed` contains both kinds. `Plain` has no vtable but appears in `WithPlain`'s
hierarchy. Abstract `Interface` can have a construction/destruction vtable.
The CLI reports `for_type: null` for virtual-base vtables whose runtime offsets
cannot be matched to fixed RTTI offsets; the hierarchy still identifies virtual
bases with `virtual_base: true` and `offset: null`.
