Demonstration
=============

Examples of tools that can be build using pelite.

32-bit RTTI dump
----------------

Dumps the RunTime Type Information, each associated vtable and class hierarchy for every type found.

Limited to PE32 (32bit binaries) only. Pull requests are welcome to support PE32+ and/or GNU ABI!

```bat
cargo run --bin msrtti -- "demo/Demo.dll" > demo/Demo-rtti.txt
```

The result can be seen [here](Demo-rtti.txt).

64-bit PE dump
--------------

Dumps the PE headers.

```bat
cargo run --bin pedump -- "demo/Demo64.dll" -dnsiertxg > demo/Demo64-pe.txt
```

The result can be seen [here](Demo64-pe.txt).

Generate PE Module-Definition file
----------------------------------

Writes a [Module-Defintion](https://msdn.microsoft.com/en-us/library/28d6s79h.aspx) file for the given input DLL.

```bat
cargo run --bin module-def -- "demo\Demo64.dll" > "demo\Demo64.def"
cargo run --bin module-def -- "demo\Demo.dll" > "demo\Demo.def"
```

An Import Library can be created from the Module-Definition file.
Note that this needs access the VC build tools.

```bat
vcvarsall x64
lib /def:"demo\Demo64.def" /out:"demo\Demo64.lib" /machine:x64
```

Also works for 32-bit binaries using the 32-bit VC build tools and commands.

```bat
vcvarsall x86
lib /def:"demo\Demo.def" /out:"demo\Demo.lib" /machine:x86
```

The result can be seen [here](Demo64.def) for x64 and [here](Demo.def) for x86.
