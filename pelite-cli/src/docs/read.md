Read typed data from a PE image

Usage:

    pelite-cli read FILE ADDRESS TYPE [--max-string-bytes N] [--max-dynamic-array-length N] [--format=text|json|json-pretty|nul]

ADDRESS is `kind:number` and accepts decimal or hex: `rva:4096`, `va:0x180001000`, or `fo:1024`.

TYPE describes the bytes at ADDRESS:

    u8 u16 u32 u64       unsigned integers
    i8 i16 i32 i64       signed integers
    f32 f64              IEEE 754 floating-point numbers
    ptr                  VirtualAddress converted to its target RVA
    *T                   VirtualAddress, then read T at its target RVA
    [T; N]               N consecutive values of T (decimal N)
    struct { a: T, ... } fields laid out with natural C alignment
    union { a: T, ... }  all fields read from the same address

Dynamically sized types (DSTs) have no fixed size:

    cstr                 NUL-terminated C string at this address
    utf16lez             NUL-terminated UTF-16LE string (Windows wchar_t)
    code                 Code with no declared layout
    unk                  Data with no declared layout
    [T; field]           length read from a named unsigned struct field
    struct { ..., tail: DST }  struct ending in a DST field

`cstr` reads through the first NUL, within the --max-string-bytes limit
(default: 256); increase the limit for longer strings. The opaque DSTs
`code` and `unk` read like `ptr` when used directly. Pointers to opaque DSTs
give the target RVA without reading its contents.

Field names must be unique within each struct or union. Use `_` for a field
whose value should be discarded; it can appear more than once. Its type is
parsed normally and still contributes to the size and alignment.
Union fields can also be unnamed, as in `union{u32,u64}`. `read` includes
these fields under their zero-based position keys (`"0"`, `"1"`, and so on).

At read time, a dynamic array looks up its length field in the containing
struct. The field can appear before or after a pointer to the array and must
have type `u8`, `u16`, `u32`, or `u64`. A missing field or a field of another
type produces an error for that array.

When a struct has a DST field, it must be last. DSTs cannot be union fields or
array elements. A DST struct can be read directly or through a pointer, but
cannot be embedded in another struct. --max-dynamic-array-length limits
field-length arrays; values above the limit produce a read error.

Integers and floats are little-endian. `ptr` and `*T` use 4 bytes in PE32
and 8 bytes in PE32+. A zero pointer produces JSON null; a nonzero pointer
must convert to a valid RVA. `ptr` gives an RVA number, while `*T` reads the
value at that RVA unless T is an opaque DST.

To see both the target RVA and the dereferenced value of a pointer, read its
slot as `union { p: ptr, v: T }`, where T is a typed pointer.

Types can be nested: `*[u32; 4]`, `[struct { id: u16, value: u32 }; 3]`,
or `struct Header { count: u32, names: *[*cstr; 2] }`. Names on structs and
unions are optional and do not change the output. Struct fields are aligned
to their own alignment; the total size is padded to the largest alignment,
which also sets the stride of an array of structs. Union size is the largest
field size, rounded to its largest alignment. Use a pointer to a DST for a
nonfinal struct field or array element. Quote compound TYPEs in
the shell, especially those containing spaces, semicolons, or `*`.

Examples:

    pelite-cli read sample.dll rva:0x2000 u32
    pelite-cli read sample.dll fo:1024 'struct { flags: u16, count: u32, name: *cstr }' --format=json-pretty
    pelite-cli read sample.dll rva:0x3000 '*[f32; 3]' --format=json
    pelite-cli read sample.dll va:0x180004000 '[union { raw: u32, target: ptr }; 4]'
    pelite-cli read sample.dll rva:0x4000 'struct { count: u16, values: [u32; count] }' --max-dynamic-array-length 1024

Results follow TYPE: scalars become JSON numbers, `cstr` and `utf16lez` become strings,
arrays become arrays, and structs/unions become objects keyed by field name.
A failed read appears in place as an object with `$error` and `$address`
(the attempted RVA, or null if conversion failed). Other fields
or elements can still succeed, but any `$error` makes the command exit with a
failure status. Type syntax errors fail before any value is printed.

Example JSON output for `struct { opcode: u8, immediate: [u8; 4] }`:

```json
{
  "opcode": 184,
  "immediate": [1, 0, 0, 0]
}
```

When to use:

Interpret structured data at a known PE address as a scalar, pointer, string, array,
struct, or union.
