Demonstration
=============

Examples of tools that can be build using pelite.

32-bit RTTI dump
----------------

`cargo run --bin msrtti -- "demo/Demo.dll" > demo/demo-rtti.txt`

Dumps the runtime type information, each associated vtable and class hierarchy for every type found.

The result can be seen [here](demo-rtti.txt).

64-bit PE dump
--------------

`cargo run --bin pedump -- "demo/Demo64.dll" -dnsiertxg > demo/demo64-pe.txt`

Dumps all the PE related headers and then some.

The result can be seen [here](demo64-pe.txt).
