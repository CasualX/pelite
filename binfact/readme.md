BinFact
=======

Shared facts and type descriptions for binary analysis tools. `binfact` provides
a text format for address annotations and a type parser that computes field
offsets, sizes, and alignment for a chosen target pointer width.

Documentation
-------------

- [Fact maps](docs/factmap.md): text storage for facts about a binary, including
  symbols, comments, references, function metadata, and decode regions.
- [Type system](docs/ty.md): annotate facts and describe structured data using
  scalars, pointers, arrays, structs, unions, and dynamically sized types.

Using the library
-----------------

```rust
let width = binfact::ty::PointerWidth::Bits64;
let facts = binfact::FactMap::parse("#factmap\nSx2000 *u32 \"counter\"\n", width).unwrap();
let header = binfact::ty::Type::parse("struct { flags: u16, value: u32 }", width).unwrap();
assert_eq!(header.layout().unwrap(), (8, 4));
```

`FactMap::write` emits the text format in stored order. Parse errors report a
fact map line number or a type syntax byte offset.

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
