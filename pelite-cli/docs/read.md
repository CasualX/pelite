Read typed data from a PE image

Usage:

    pelite-cli read FILE ADDRESS TYPE [--max-string-bytes N] [--format=json|json-pretty]

ADDRESS is hexadecimal and needs a prefix: rva:1000, va:0x180001000, or fo:400.
An optional 0x prefix or h suffix is accepted. VA and file offset
addresses are converted to an RVA before reading. Use `pelite-cli addr` to
convert addresses separately. The command reads the mapped PE image, so a
file offset must correspond to a mapped RVA.

TYPE describes the bytes at ADDRESS:

    u8 u16 u32 u64       unsigned integers
    i8 i16 i32 i64       signed integers
    f32 f64              IEEE 754 floating-point numbers
    cstr                 NUL-terminated C string at this address
    ptr                  VirtualAddress converted to its target RVA
    *T                   VirtualAddress, then read T at its target RVA
    [T; N]               N consecutive values of T (decimal N)
    struct { a: T, ... } fields laid out with natural C alignment
    union { a: T, ... }  all fields read from the same address

Integers and floats are little-endian. `ptr` and `*T` use 4 bytes in PE32
and 8 bytes in PE32+. A zero pointer produces JSON null; a nonzero pointer
must convert to a valid RVA. `ptr` gives an RVA number, while `*T` reads the
value at that RVA. `cstr` reads bytes through the first NUL, within the
--max-string-bytes limit (default: 256); increase the limit for longer strings.

To see both the target RVA and the dereferenced value of a pointer, read its
slot as `union { p: ptr, v: T }`, where T is a typed pointer.

Types can be nested: `*[u32; 4]`, `[struct { id: u16, value: u32 }; 3]`,
or `struct Header { count: u32, names: *[*cstr; 2] }`. Names on structs and
unions are optional and do not change the output. Struct fields are aligned
to their own alignment; the total size is padded to the largest alignment,
which also sets the stride of an array of structs. Union size is the largest
field size, rounded to its largest alignment. A bare `cstr` has no fixed size,
so use `*cstr` for a string field or array element. Quote compound TYPEs in
the shell, especially those containing spaces, semicolons, or `*`.

Examples (replace addresses with locations from your PE file):

    pelite-cli read sample.dll rva:2000 u32
    pelite-cli read sample.dll fo:400 'struct { flags: u16, count: u32, name: *cstr }' --format=json-pretty
    pelite-cli read sample.dll rva:3000 '*[f32; 3]' --format=json
    pelite-cli read sample.dll va:0x180004000 '[union { raw: u32, target: ptr }; 4]'

Results follow TYPE: scalars become JSON numbers, `cstr` becomes a string,
arrays become arrays, and structs/unions become objects keyed by field name.
A failed read appears in place as an object with `$error` and `$address`
(the attempted RVA, or null if conversion failed). Other fields
or elements can still succeed, but any `$error` makes the command exit with a
failure status. Type syntax errors fail before any value is printed.
