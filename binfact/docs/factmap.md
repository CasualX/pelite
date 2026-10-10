# Fact maps

A fact map is a database of facts about a binary: named and typed addresses,
comments, references, function metadata, and regions to decode. Its text,
line-based format makes it easy to inspect, edit by hand, diff in version
control, and merge with facts produced by analysis tools.

Addresses are relative virtual addresses (RVAs), measured in bytes from the
image base. Keeping addresses relative lets annotations survive a change in
the binary's load address.

## File structure

The first line must be exactly `#factmap`. Each subsequent line is one of:

- A blank line, ignored by the parser.
- A file comment beginning with `#`, also ignored.
- A fact record, identified by its first letter.

Leading and trailing whitespace on these subsequent lines is ignored. Fact
fields are separated by whitespace; quoted fields use JSON string syntax,
including escapes such as `\"`, `\\`, and `\n`. A record occupies one physical
line. File comments use their own lines rather than trailing a record.

| Record | Fact |
| --- | --- |
| `SxRVA TYPE NAME` | Symbol |
| `CxRVA "COMMENT"` | Address comment |
| `RxRVA 0xTARGET` | Reference |
| `FxRVA CONTENT` | Function metadata |
| `DxRVA BYTES ARCH` | Decode region |

`RVA` is hexadecimal, without a `0x` prefix after the record letter: `Sx1000`
is a symbol at RVA `0x1000`. RVAs must fit in 32 bits.

```text
#factmap
# A function and the table it references
Sx1000 fn "load_entries"
Sx2000 "struct { count: u32, entries: *[u32; count] }" "entry_table"
Cx1000 "Reads entries from the table."
Rx1000 0x2000
Fx1000 {"bytes":48}
Dx1000 48 x86_64
```

## Symbols (`S`)

A symbol associates an address with a [type](ty.md) and a name or marker.
The type describes the contents at that address. Quote types containing
whitespace, and always quote literal names:

```text
Sx2000 u32 "counter"
Sx2010 *cstr "name"
Sx2020 "struct { id: u16, value: u32 }" "record"
```

Unquoted names are reserved markers for consumers to interpret:

| Marker | Intended interpretation |
| --- | --- |
| `C` | Generic code |
| `D` | Generic writable data |
| `R` | Generic read-only data |
| `fn` | Generic function |
| `thunk` | Jump stub |
| `_` | Weak anchor; does not override an existing symbol |
| `undef` | Remove an earlier symbol at this RVA |

For example, `Sx2010 unk undef` withdraws a candidate symbol. Quoting a marker
makes it a literal name: `"fn"` is a name, while `fn` is a marker. The type
`fn` and the name marker `fn` are separate fields: `Sx1000 fn fn` uses both.

## Address comments (`C`)

A comment attaches text to an address. Use it to record observations,
evidence, or uncertainty:

```text
Cx1000 "May be the configuration loader."
Cx2000 "Count is checked before use.\nEntries are 32-bit integers."
```

Unlike `#` file comments, these annotations are stored as facts. The comment
must be a JSON string; escaped newlines remain part of a single record.

## References (`R`)

A reference records a directed relationship from a source RVA to a target RVA:

```text
Rx1000 0x2000
```

This could describe a call, a jump, or access to data. The record stores only
the two addresses; their meaning comes from the analysis that produced it.
Targets are written as `0x`-prefixed hexadecimal values. The parser also
accepts decimal targets, and the writer normalizes them to hexadecimal.

## Function metadata (`F`)

Function facts attach a nonempty payload to a function's entry RVA:

```text
Fx1000 {"bytes":48,"calling_convention":"cdecl"}
```

The payload is arbitrary text, commonly JSON. Parsing stores it without JSON
validation or a required schema, trimming surrounding whitespace.

`FunctionFact::merge_content` recursively merges JSON objects, retaining old
keys absent from the new object. New values replace old values when they are
not both objects. If either payload is invalid JSON, the new text replaces
the old text.

## Decode regions (`D`)

A decode fact describes a region's start RVA, byte count, and instruction mode:

```text
Dx1000 48 x86_64
```

Byte counts fit in 32 bits and are normally decimal; `0x`-prefixed counts are
also accepted. Supported modes are `x86_16`, `x86_32`, and `x86_64`, with
`x86` accepted as an alias for `x86_32`. The writer uses decimal counts and
the canonical mode name. A decode fact describes the region without decoding
its instructions itself.

## Editing and combining facts

Keep generated facts and manual corrections in separate files when useful.
Combine their record lines under a single `#factmap` header, or let a consumer
load each file in order. This makes corrections easy to retain when analysis
is rerun.

`FactMap` preserves source order, including repeated addresses and fact kinds.
Parsing and writing do not sort, deduplicate, or apply overrides. Consumers
choose how to combine records; for example, `pelite-cli` applies later symbol
definitions over earlier ones and honors weak and removal markers.

Parse errors identify the first invalid line. For compatibility, the parser
also accepts the legacy `#symtext` header and `0xRVA TYPE NAME` symbol syntax;
the writer emits the current format.
