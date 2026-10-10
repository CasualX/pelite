# Type system

The type system describes data and code in a binary.

## Primitive types

| Types | Meaning | Size and alignment |
| --- | --- | --- |
| `u8`, `i8` | Unsigned or signed 8-bit integer | 1 |
| `u16`, `i16` | Unsigned or signed 16-bit integer | 2 |
| `u32`, `i32` | Unsigned or signed 32-bit integer | 4 |
| `u64`, `i64` | Unsigned or signed 64-bit integer | 8 |
| `f32` | IEEE 754 single-precision float | 4 |
| `f64` | IEEE 754 double-precision float | 8 |

Numeric types describe little-endian values. Type names are case-sensitive.

## Pointers

`*T` describes a virtual address pointing to a value of type `T`:

```text
*u32
**u8
*cstr
*fn
```

The caller selects `PointerWidth::Bits32` or `PointerWidth::Bits64` when
parsing. Every pointer in that type then has size and alignment of four or
eight bytes, independent of the host machine. The parsed type retains this
width; the text spelling `*T` does not encode it.

A pointer always has a fixed size, even if its target does not. For example,
`*cstr` is a pointer slot, while `cstr` describes string bytes at the address
being annotated or read.

## Arrays

`[T; N]` describes `N` consecutive elements, where `N` is a 32-bit decimal count.
Elements must have a fixed layout. Array alignment is the element alignment,
and total size is the element size multiplied by the count.

```text
[u8; 16]
[f32; 3]
[*cstr; 4]
*[u32; 4]
```

`[*cstr; 4]` is an array of four string pointers; `*[u32; 4]` is one pointer
to an array of four integers. A field name can replace the count for a
[dynamic array](#dynamic-arrays).

## Structs and unions

Struct fields follow declaration order and natural C alignment:

```text
struct Header { flags: u16, value: u32 }
```

Here `flags` starts at byte 0 and `value` at byte 4, with two padding bytes
between them. The struct has size 8 and alignment 4. Fixed-size structs have
trailing padding to a multiple of their largest field alignment, so arrays
of structs keep each element aligned.

Union fields all start at byte 0. A union's size is its largest field size,
rounded up to its largest alignment:

```text
union Value { bits: u32, number: f32 }
```

Structs, unions, arrays, and pointers can nest:

```text
struct { id: u16, position: [f32; 3], next: *unk }
[struct { key: u32, value: *cstr }; 8]
union { raw: u64, pointer: *unk }
```

The name after `struct` or `union` is optional and descriptive; it does not
define a reusable type alias. Fields are comma-separated, and a trailing comma
is allowed. Whitespace around tokens is optional. Field and composite names
use ASCII identifiers: a letter or `_`, followed by letters, digits, or `_`.

### Field names

Explicit field names must be unique within their struct or union. Use `_` for
a field whose value should be discarded by a reader; its type still contributes
to layout, and multiple `_` fields are allowed:

```text
struct { _: [u8; 4], value: u32, _: u16 }
```

Unions also allow unnamed fields, such as `union { u32, f32 }`. These are
distinct from discarded fields. A reader can expose them by their zero-based
field indices, as `pelite-cli` does.

## Dynamically sized types (DSTs)

A DST has no complete size known from its type alone. These include strings,
opaque regions, field-length arrays, and structs whose final field is unsized.
They still have a starting alignment.

### Strings

`cstr` describes a NUL-terminated byte string with alignment 1. `utf16lez`
describes a NUL-terminated UTF-16LE string with alignment 2. Their lengths
depend on the binary's contents:

```text
struct { tag: u8, text: cstr }
struct { title: *utf16lez, description: *cstr }
```

The first struct is unsized because its inline string has no fixed length.
The second is fixed-size because it stores two pointers.

### Dynamic arrays

`[T; field]` takes its count from a named unsigned integer field in the
containing struct:

```text
struct { count: u32, values: [u16; count] }
struct { values: *[u16; count], count: u32 }
```

The first stores its elements inline as an unsized tail. The second stores a
pointer to the elements and has a fixed layout. For a pointer to an array,
the count field can appear before or after the pointer.

The consumer resolves the field at read time and requires `u8`, `u16`, `u32`,
or `u64`. Parsing records the field name without checking that it exists or
has a suitable type. Readers can impose limits on element counts.

### Opaque types

Opaque types annotate a region without declaring its contents or extent:

| Type | Meaning |
| --- | --- |
| `code` | Code without a declared layout or confirmed function identity |
| `fn` | Confirmed function without a declared layout |
| `unk` | Data with an unknown layout |

They are unsized and have alignment 1. `fn` describes function identity,
rather than a signature or calling convention. `*fn` is a function pointer;
`*unk` is an opaque data pointer. A reader can report their target addresses
without trying to interpret the target bytes.

### Placement rules

An unsized field must be the last field of a struct, making that struct
unsized too. Unsized types cannot be union fields or array elements. Use
pointers to place them in those positions or before another struct field:

```text
struct { name: *cstr, count: u32 }
[*unk; 8]
union { address: *fn, raw: u64 }
```

An unsized struct can itself be a final struct field; it propagates the
unsized layout to the enclosing struct. Reading support for nested DSTs
depends on the consumer.

## Parsing and layout

`Type::parse(text, pointer_width)` returns a type with computed field offsets
and alignment. `Type::layout()` returns `(size, alignment)` for fixed-size
types and an error for DSTs; `Type::is_dst()` and `Type::alignment()` also
work for unsized types. Parse errors identify a byte offset and report syntax
errors, invalid layouts, duplicate fields, or size overflow.

When placing a type in a fact map symbol, use JSON string quoting if it
contains whitespace:

```text
Sx2000 "struct { count: u32, values: *[u16; count] }" "table"
```
