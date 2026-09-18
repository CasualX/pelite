Demonstration
=============

Examples of tools that can be build using pelite.

32-bit RTTI dump
----------------

Dumps the RunTime Type Information, each associated vtable and class hierarchy for every type found.

Limited to PE32 (32bit binaries) only. Pull requests are welcome to support PE32+ and/or GNU ABI!

```bat
cargo run -p pelite-cli -- msrtti "demo/Demo.dll" > demo/Demo-rtti.txt
```

The result can be seen [here](Demo-rtti.txt).

64-bit PE inspection
--------------------

Inspects every supported PE header and data directory.

```bat
cargo run -p pelite-cli -- inspect "demo/Demo64.dll" all > demo/Demo64-pe.txt
```

The result can be seen [here](Demo64-pe.txt).

Resource filesystem
-------------------

Browse resources without embedding their payloads in the PE inspection output:

```bat
cargo run -p pelite-cli -- resources tree "demo/Demo.dll"
cargo run -p pelite-cli -- resources fsck "demo/Demo.dll"
cargo run -p pelite-cli -- resources cat "demo/Demo.dll" /#MANIFEST/#2/#1033 > manifest.xml
cargo run -p pelite-cli -- resources icons list "demo/Demo.dll"
cargo run -p pelite-cli -- resources icons extract "demo/Demo.dll" extracted-icons
```

Use `--format=json` with `tree`, `fsck`, `icons`, or `cursors` for structured results. With `cat`, text output is the exact resource payload while JSON wraps it in base64 with its size and code page.

Generate PE Module-Definition file
----------------------------------

Prints a [Module-Defintion](https://msdn.microsoft.com/en-us/library/28d6s79h.aspx) file for the given input DLL.

```bat
cargo run -p pelite-cli -- module-def "demo\Demo64.dll" > "demo\Demo64.DEF"
cargo run -p pelite-cli -- module-def "demo\Demo.dll" > "demo\Demo.DEF"
```

An Import Library can be created from the Module-Definition file.
Note that this needs access the VC build tools.

```bat
vcvarsall x64
lib /def:"demo\Demo64.DEF" /out:"demo\Demo64.LIB" /machine:x64
```

Also works for 32-bit binaries using the 32-bit VC build tools and commands.

```bat
vcvarsall x86
lib /def:"demo\Demo.DEF" /out:"demo\Demo.LIB" /machine:x86
```

The result can be seen [here](Demo64.def) for x64 and [here](Demo.def) for x86.

Find Signatures tool
--------------------

Finds matches of signatures in binaries using a [language designed specifically for executable code](https://docs.rs/pelite/*/pelite/pattern/fn.parse.html).

Play around in interactive mode:

```bat
cargo run -p pelite-cli -- findsig "demo\Demo64.dll"
```

Try out the pattern `E8${B8'???? C3}` to find all `call` instructions to a function which load a constant into `eax` and returns.
In addition save the address of the constant so it can be extracted later.

Find signatures by passing them as command line arguments:

```bat
cargo run -p pelite-cli -- findsig "demo\Demo64.dll" "E8${B8'???? C3}"
```

The result is two matches for this pattern for this particular binary:

```text
Pattern `E8${B8'???? C3}` matches:
  Demo64.dll!00001930  [1/00001001]
  Demo64.dll!0000194c  [1/00001001]
```
