# Scanner patterns

A pattern is a compact program for finding machine code or binary data. Exact bytes
describe what must match; the other operations move the cursor, follow references,
capture values, or try alternatives. This makes patterns useful for locating code and
data that move when a binary is rebuilt.

Create a pattern at compile time with [`pattern!`](crate::pattern!) or at runtime with
[`parse`]. Then pass it to a [PE32 scanner](crate::pe32::Pe::scanner) or a
[PE32+ scanner](crate::pe64::Pe::scanner).

```
use pelite::pattern;

const FUNCTION_PROLOGUE: &[pattern::Atom] = pattern!("55 8B EC");

let with_wildcard = pattern::parse("48 8B ? 48 85 C0")?;
# Ok::<(), pattern::PatternError>(())
```

The macro and runtime function use the same language and perform the same validation.
Patterns always begin by saving the candidate's RVA in scratch slot 0.

## Bytes, wildcards, and strings

| Syntax | Meaning |
| --- | --- |
| `48 8B C1` | Match the exact bytes `48 8B C1` |
| `?` | Move forward one byte without reading it |
| `A0/F8` | Match when `byte & F8 == A0 & F8` |
| `"hello"00` | Match the UTF-8 bytes of `hello`, then a zero byte |
| `"say ""hi"""` | Match the UTF-8 bytes of `say "hi"` |
| `// comment` | Ignore input through the end of the line |

Hexadecimal bytes are case-insensitive.
Whitespace is optional between complete atoms, so `488BC1` and `48 8B C1` are equivalent.
Compound forms such as `skip(12)`, `u4[1]`, and `=u4[1]` must remain contiguous.
A mask is two hex digits attached to a byte with `/`; nibble wildcards are not supported.

Each `?` is exactly `skip(1)`. It does not check that the skipped byte is readable.
Consecutive wildcard tokens are compiled into a single skip.

Inside a string, two consecutive double quotes (`""`) match one literal double quote byte.
Backslashes have no special meaning; leave the string and use hex bytes for other special bytes.
Because adjacent quotes are an escape, separate adjacent quoted strings with whitespace (or combine them).

## Movement and searching

| Syntax | Meaning |
| --- | --- |
| `skip(12)` | Move forward 12 bytes |
| `skip(-12)` | Move backward 12 bytes |
| `skip(ptr)` | Move forward by the scanned image's pointer width |
| `skip(-ptr)` | Move backward by the scanned image's pointer width |
| `scan(12)` | Try the continuation at offsets 0 through 12 |
| `scan()` | Try the continuation at every remaining readable offset in the current section |
| `@4` | Require the cursor to be aligned to `2^4` bytes |
| `align(12)` | Require the cursor to be aligned to `2^12` bytes |

Numbers are decimal unless prefixed with `0x`. Movement operands are 32-bit values;
larger values compile to a short chain of `Extend` atoms. `skip(0)` does nothing.
`@n` takes one decimal exponent digit; `align(n)` accepts exponents through 31.

Scans are non-greedy: `scan(12) AA` tries the nearest `AA` first.
The bound is the maximum number of bytes to skip, so `scan(1)` tries offsets 0 and 1.
`scan(0)` is a no-op, just like `skip(0)`. `scan()` has no additional distance bound.
All scans stop at the end of the current section's readable bytes;
References can first move to another section and scan there.
Even an empty continuation requires a readable candidate.

Use a skip followed by a scan to express a bounded gap. This example searches for
`FF` after skipping between 13 and 42 bytes:

```text
50 skip(13) scan(29) FF
```

## Following references

| Syntax | Reference stored at the cursor |
| --- | --- |
| `$` or `rel32` | Signed 4-byte relative displacement |
| `%` or `rel8` | Signed 1-byte relative displacement |
| `*` or `ptr` | Absolute pointer, using the scanned image's pointer width |

The cursor must point at the reference operand, not its opcode. Without braces,
matching continues at the target:

```text
E8 $ 55 8B EC       // Follow a call and match the target's prologue.
74 % 33 C0          // Follow a short branch.
68 * "hello"00      // Follow a pointer to a zero-terminated string.
```

The word aliases are identical to the punctuation forms and also accept reference
bodies, for example `E8 rel32{save} 48 85 C0`.

Braces match at the target and then resume immediately after the original reference
field. This is useful when both the target and the referring instruction matter:

```text
E8 ${'} 48 85 C0               // Capture the call target, then match the caller.
48 8D 0D ${"hello"00} 48 8B D7 // Validate referenced data, then resume.
C7 05 ${skip(4)'} 01 00 00 00  // Skip an immediate at the target before capturing.
```

`'` in these examples is a capture, described below. Reference bodies can be empty:
`${}` resolves a relative reference and immediately resumes. Nested bodies are
supported. Their return addresses use negative scratch slots, so allocate scratch
space with [`save_len`] rather than counting only your captures.

## Captures and scratch slots

The scanner's scratch array stores 32-bit RVAs and values. Slots `0..=127` can be
named explicitly. Slot 0 initially contains the candidate's starting RVA.

| Syntax | Meaning |
| --- | --- |
| `'` or `save` | Save the cursor in the next automatic slot |
| `save[3]` | Save the cursor in slot 3 |
| `seek[3]` | Set the cursor from slot 3 |
| `check[3]` | Require the cursor to equal slot 3 |
| `zero[3]` | Write zero to slot 3 |
| `u4` | Read four bytes into the next automatic slot |
| `i1[2]` | Read and sign-extend one byte into slot 2 |
| `=u4[3]` | Compare four bytes with slot 3 |

Typed reads use the next automatic slot when brackets are omitted.
Comparisons require an explicit slot. Their sizes are 1, 2, or 4 bytes:

```text
i1[1] u1[2] i2[3] u2[4] i4[5] u4[6]
=i1[1] =u1[2] =i2[3] =u2[4] =i4[5] =u4[6]
```

A particular slot cannot be assigned both automatically and explicitly. Use an explicit
slot on the write when the pattern needs to read it again, for example
`u1[1] skip(4) =u1[1]`. Automatic slots are intended as returned captures. Slot 0
remains explicitly readable as the candidate RVA.

Reads use little-endian values and advance by their width. Signed 1- and 2-byte
reads sign-extend; unsigned reads zero-extend. Four-byte signed and unsigned values
have the same representation. A comparison advances only when it succeeds and never
changes its slot.

Automatic numbering begins at 1. An explicit output slot raises the allocation
high-water mark, while input operations such as `check` do not:

```text
' zero[5] ' check[0] '   // Writes slots 1, 5, 6, and 7.
```

Use [`save_len`] to size the array passed to the scanner:

```
use pelite::pattern;

let pattern = pattern::parse("E8 ${'} 48 85 C0")?;
let mut save = vec![0; pattern::save_len(&pattern)];
# Ok::<(), pattern::PatternError>(())
```

Reference bodies reserve internal slots at the end of that array.
Use [`captures_len`](captures_len) to select the user-facing prefix when returning or displaying results:

```
use pelite::pattern;

let pattern = pattern::parse("E8 ${'} 48 85 C0")?;
let save = vec![0; pattern::save_len(&pattern)];
let captures = &save[..pattern::captures_len(&pattern)];
# Ok::<(), pattern::PatternError>(())
```

## Alternatives

Parentheses group alternatives separated by `|`. The scanner tries them from left
to right, but a later failure can make it retry another branch:

```text
(6A ? | 68 ? ? ? ?) E8    // Two instruction encodings.
(61 | 61 62) 63           // Matches either "ac" or "abc".
(AA |) BB                 // Empty alternatives are allowed.
```

Each branch begins automatic slot allocation at the same number. Allocation after
the group continues above the highest slot used by any branch:

```text
(6A i1[1] | 68 i4[1]) =i4[1]
```

Scratch writes survive failed alternatives and scan attempts. The compiler rejects
patterns that read a slot before every reachable path initializes it, or that rely on
a value which a failed retry may have overwritten. Initialize missing branch outputs
explicitly when they are read later:

```text
// Valid: both branches establish slot 1 before the comparison.
(AA u1[1] FF | BB zero[1]) =u1[1]
```

An overall failed match may leave scratch values changed. Treat them as results only
after the scanner reports success.

## Complete examples

Capture a call target while allowing one unstable instruction byte:

```text
48 8B 41 ? 48 2B C2 E8 ${'} 48 85 C9
```

Find an aligned table whose optional description pointer is represented by either a
real pointer or four zero bytes:

```text
@2 *{'} (*{'} | 00 00 00 00) u4
```

Read a byte once and require a later byte to have the same unsigned value:

```text
u1[1] skip(4) =u1[1]
```

For background on choosing stable signature bytes, see the
[AlliedModders signature scanning guide](https://wiki.alliedmods.net/Signature_scanning).
