/*!
Scanner Patterns.

# What are patterns?

A pattern is a sequence of atoms. An atom describes either a specific byte or a control instruction such as skipping the next X bytes.

In this regard a pattern looks a lot like a simple [regular expression](https://en.wikipedia.org/wiki/Regular_expression).
But whereas regular expressions are designed to work with text, patterns are designed to work with executable code and binary data.

Patterns can encode more than just _match exact byte_ and _skip X bytes_ such as _follow this 1 byte signed jump_, _follow this 4 byte signed jump_
and _follow this pointer_ including the ability to continue matching after returning from following a relative jump or pointer.

# Why use patterns?

Reverse engineering is hard. When you've found an interesting address (such as a function or global variable)
you don't want to spend all that effort again when the program is updated.

Luckily when programs update they usually don't change all that much, some functions and data are changed but the rest has remained the same.
However this means that the unchanged bits may be shuffled around to a different address.

Patterns let you track interesting parts of a program even as it is updated.

To find all matches of a pattern, eg. find all locations which call a function or reference some data, automating analysis.

# How to use patterns?

Patterns can be created statically from its [atoms](enum.Atom.html) or [parsed](fn.parse.html) from a string.

Create a scanner instance for [PE32](../pe32/trait.Pe.html#method.scanner) or [PE32+](../pe64/trait.Pe.html#method.scanner) and follow their documentation.

# How to create patterns?

This requires knowledge with reverse engineering programs.

Here's a resource to learn more about signature scanning: [wiki.alliedmods.net](https://wiki.alliedmods.net/Signature_scanning).
*/

#![allow(ellipsis_inclusive_range_patterns)]

use std::prelude::v1::*;

use std::{cmp, fmt, mem, str};

#[cfg(feature = "std")]
use std::error;

/// Max recursion depth.
pub const STACK_SIZE: usize = 4;
/// Special skip value to indicate to use platform pointer size instead.
pub(crate) const PTR_SKIP: u8 = 0;

//----------------------------------------------------------------

/// Pattern parsing error.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ParsePatError {
	kind: PatError,
	position: usize,
}
impl fmt::Display for ParsePatError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Syntax Error @{}: {}.", self.position, self.kind.to_str())
	}
}
#[cfg(feature = "std")]
impl error::Error for ParsePatError {
	fn description(&self) -> &str {
		self.kind.to_str()
	}
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum PatError {
	UnpairedHexDigit,
	UnknownChar,
	ManyOverflow,
	ManyRange,
	ManyInvalid,
	SaveOverflow,
	StackError,
	StackInvalid,
	UnclosedQuote,
	AlignedOperand,
	ReadOperand,
	SubPattern,
	SubOverflow,
}
impl PatError {
	fn to_str(self) -> &'static str {
		match self {
			PatError::UnpairedHexDigit => "unpaired hex digit",
			PatError::UnknownChar => "unknown character",
			PatError::ManyOverflow => "many range exceeded",
			PatError::ManyRange => "many bounds nonsensical",
			PatError::ManyInvalid => "many invalid syntax",
			PatError::SaveOverflow => "save store overflow",
			PatError::StackError => "stack unbalanced",
			PatError::StackInvalid => "stack must follow jump",
			PatError::UnclosedQuote => "string missing end quote",
			PatError::AlignedOperand => "aligned operand error",
			PatError::ReadOperand => "read operand error",
			PatError::SubPattern => "sub pattern error",
			PatError::SubOverflow => "sub pattern too large",
		}
	}
}

//----------------------------------------------------------------

/// Pattern atoms.
///
/// The scanner will silently ignore nonsensical arguments.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Atom {
	/// Matches a single byte.
	Byte(u8),
	/// Captures the cursor in the save array at the specified index.
	Save(u8),
	/// After a Pop later continue matching at the current cursor plus the argument.
	Push(u8),
	/// Pops the cursor from the stack and continues matching.
	Pop,
	/// Sets a mask to apply on next byte match.
	Fuzzy(u8),
	/// Skips a fixed number of bytes.
	Skip(u8),
	/// Rewinds the cursor a fixed number of bytes.
	Back(u8),
	/// Extends the push, skip, back and many range by `argument * 256`.
	Rangext(u8),
	/// Looks for the next pattern at most a certain number of bytes ahead.
	Many(u8),
	/// Follows a signed 1 byte jump.
	///
	/// Reads the byte under the cursor, sign extends it, adds it plus 1 to the cursor and continues matching.
	Jump1,
	/// Follows a signed 4 byte jump.
	///
	/// Reads the dword under the cursor and adds it plus 4 to the cursor and continues matching.
	Jump4,
	/// Follows an absolute pointer.
	///
	/// Reads the pointer under the cursor, translates it to an RVA, assigns it to the cursor and continues matching.
	///
	/// Matching fails immediately when translation to an RVA fails.
	Ptr,
	/// Follows a position independent reference.
	///
	/// Reads the dword under the cursor and adds it to the saved cursor for the given slot and continues matching.
	Pir(u8),
	/// Given a cursor pointing to the vtable follows it to its type name.
	VTypeName,
	/// Compares the cursor with the value in the given save slot and fails if they're not equal.
	Check(u8),
	/// Checks if the cursor is aligned to `(1 << value)`.
	Aligned(u8),
	/// Reads and sign-extends the byte under the cursor, writes to the given slot and advances the cursor by 1.
	ReadI8(u8),
	/// Reads and zero-extends the byte under the cursor, writes to the given slot and advances the cursor by 1.
	ReadU8(u8),
	/// Reads and sign-extends the word under the cursor, writes to the given slot and advances the cursor by 2.
	ReadI16(u8),
	/// Reads and zero-extends the word under the cursor, writes to the given slot and advances the cursor by 2.
	ReadU16(u8),
	/// Reads the dword under the cursor, writes to the given slot and advances the cursor by 4.
	ReadI32(u8),
	/// Reads the dword under the cursor, writes to the given slot and advances the cursor by 4.
	ReadU32(u8),
	/// Writes zero to the given save slot.
	Zero(u8),
	/// Sets a retry point when matching fails.
	///
	/// When matching fails the cursor is restored and matching begins again skipping _N_ atoms.
	Case(u8),
	/// Continues matching after a case atom, skipping the next _N_ atoms.
	Break(u8),
	/// Null instruction, used to make the parser easier to write.
	Nop,
}

/// Patterns are a vector of [`Atom`](enum.Atom.html)s.
pub type Pattern = Vec<Atom>;

/// Returns the length of the save array needed for this signature.
pub fn save_len(pat: &[Atom]) -> usize {
	pat.iter()
		.filter_map(|&atom| match atom {
			Atom::Save(slot)
			| Atom::Pir(slot)
			| Atom::Check(slot)
			| Atom::Zero(slot)
			| Atom::ReadI8(slot)
			| Atom::ReadI16(slot)
			| Atom::ReadI32(slot)
			| Atom::ReadU8(slot)
			| Atom::ReadU16(slot)
			| Atom::ReadU32(slot) => Some(slot as usize + 1),
			_ => None,
		})
		.max()
		.unwrap_or(0)
}

/// Pattern parser.
///
/// # Remarks
///
/// Following are examples of the pattern syntax.
/// The syntax takes inspiration from [YARA hexadecimal strings](https://yara.readthedocs.io/en/v3.7.0/writingrules.html#hexadecimal-strings).
///
/// ```text
/// 55 89 e5 83 ? ec
/// ```
///
/// Case insensitive hexadecimal characters match the exact byte pattern and question marks serve as placeholders for unknown bytes.
///
/// Note that a single question mark matches a whole byte. The syntax to mask part of a byte is not yet available.
///
/// Spaces (code point 32) are completely optional and carry no semantic meaning, their purpose is to visually group things together.
///
/// ```text
/// b9 ' 37 13 00 00
/// ```
///
/// Single quotes are used as a bookmarks, to save the current cursor rva in the save array passed to the scanner.
///
/// It is no longer necessary to do tedious address calculations to read information out of the byte stream after a match was found.
/// This power really comes to life with the capability to follow relative and absolute references.
///
/// The first entry in the save array is reserved for the rva where the pattern was matched.
/// The rest of the save array is filled in order of appearance of the quotes. Here the rva of the quote can be found in `save[1]`.
///
/// ```text
/// b8 [16] 50 [13-42] ff
/// ```
///
/// Pairs of decimal numbers separated by a hypen in square brackets indicate the lower and upper bound of number of bytes to skip.
/// The scanner is non greedy and considers the first match while skipping as little as possible.
///
/// A single decimal number in square brackets without hypens is a fixed size jump, equivalent to writing that number of consecutive question marks.
///
/// ```text
/// 31 c0 74 % ' c3
/// e8 $ ' 31 c0 c3
/// 68 * ' 31 c0 c3
/// ```
///
/// These symbols are used to follow; a signed 1 byte relative jump: `%`, a signed 4 byte relative jump: `$` and an absolute pointer: `*`.
///
/// They are designed to be able to have the scanner follow short jumps, calls and longer jumps, and absolute pointers.
///
/// Composes really well with bookmarks to find the addresses of referenced functions and other data without tedious address calculations.
///
/// ```text
/// b8 * "STRING" 00
/// ```
///
/// String literals appear in double quotes and will be matched as UTF-8.
///
/// Escape sequences are not supported, switch back to matching with hex digits as needed.
/// For UTF-16 support, you are welcome to send a PR.
///
/// ```text
/// e8 $ { ' } 83 f0 5c c3
/// ```
///
/// Curly braces must follow a jump symbol (see above).
///
/// The sub pattern enclosed within the curly braces is matched at the destination after following the jump.
/// After the pattern successfully matched, the cursor returns to before the jump was followed.
/// The bytes defining the jump are skipped and matching continues again from here.
///
/// ```text
/// e8 $ @4
/// ```
///
/// Checks that the cursor is aligned at this point in the scan.
/// The align value is `(1 << arg)`, in this example the cursor is checked to be aligned to 16.
///
/// ```text
/// e8 i1 a0 u4
/// ```
///
/// An `i` or `u` indicates memory read operations followed by the size of the operand to read.
///
/// The read values are stored in the save array alongside the bookmarked addresses (single quotes).
/// This means the values are sign- or zero- extended respectively before being stored.
/// Operand sizes are 1 (byte), 2 (word) or 4 (dword).
///
/// The cursor is advanced by the size of the operand.
///
/// ```text
/// 83 c0 2a ( 6a ? | 68 ? ? ? ? ) e8
/// ```
///
/// Parentheses indicate alternate subpatterns separated by a pipe character.
///
/// The scanner attempts to match the alternate subpatterns from left to right and fails if none of them match.
pub fn parse(pat: &str) -> Result<Pattern, ParsePatError> {
	let mut result = Vec::with_capacity(pat.len() / 2);
	let mut pat_end = pat;
	match parse_helper(&mut pat_end, &mut result) {
		Ok(()) => Ok(result),
		Err(kind) => {
			let position = pat_end.as_ptr() as usize - pat.as_ptr() as usize;
			Err(ParsePatError { kind, position })
		},
	}
}
// This is preferable but currently limited by macro rules...
// pub use crate::pattern as parse;
fn parse_helper(pat: &mut &str, result: &mut Vec<Atom>) -> Result<(), PatError> {
	result.push(Atom::Save(0));
	let mut iter = pat.as_bytes().iter();
	let mut save = 1;
	let mut depth = 0;
	#[derive(Default)]
	struct SubPattern {
		case: usize,
		brks: Vec<usize>,
		save: u8,
		save_next: u8,
		depth: u8,
	}
	let mut subs = Vec::<SubPattern>::new();
	while let Some(mut chr) = iter.next().cloned() {
		match chr {
			// Follow signed 1 byte jump
			b'%' => result.push(Atom::Jump1),
			// Follow signed 4 byte jump
			b'$' => result.push(Atom::Jump4),
			// Follow pointer
			b'*' => result.push(Atom::Ptr),
			// Start recursive operator
			b'{' => {
				depth += 1;
				// Must follow a jump operator and insert push before the jump
				let atom = match result.last_mut() {
					Some(atom @ Atom::Jump1) => mem::replace(atom, Atom::Push(1)),
					Some(atom @ Atom::Jump4) => mem::replace(atom, Atom::Push(4)),
					Some(atom @ Atom::Ptr) => mem::replace(atom, Atom::Push(PTR_SKIP)),
					_ => return Err(PatError::StackInvalid),
				};
				result.push(atom);
			},
			// End recursive operator
			b'}' => {
				// Unbalanced recursion
				if depth <= 0 {
					return Err(PatError::StackError);
				}
				depth -= 1;
				result.push(Atom::Pop);
			},
			// Start subpattern
			b'(' => {
				subs.push(SubPattern::default());
				let sub = subs.last_mut().unwrap();
				// Keep the save and depth state
				sub.save = save;
				sub.depth = depth;
				// Add a new case, update the case offset later
				sub.case = result.len();
				result.push(Atom::Case(0));
			},
			// Case subpattern
			b'|' => {
				// Should already have started a subpattern
				let sub = subs.last_mut().ok_or(PatError::SubPattern)?;
				// Update the save state
				sub.save_next = cmp::max(sub.save_next, save);
				save = sub.save;
				depth = sub.depth;
				// Add a break of the previous subpattern
				sub.brks.push(result.len());
				result.push(Atom::Break(0));
				// Add a new case of the next subpattern
				let case_offset = result.len() - sub.case - 1;
				if case_offset >= 256 {
					return Err(PatError::SubOverflow);
				}
				result[sub.case] = Atom::Case(case_offset as u8);
				sub.case = result.len();
				result.push(Atom::Case(0));
			},
			// End subpattern
			b')' => {
				// Should already have started a subpattern
				let sub = subs.pop().ok_or(PatError::SubPattern)?;
				// Prepare for the next save
				save = cmp::max(sub.save_next, save);
				depth = sub.depth;
				// Neutralize the last case, since there are no more
				result[sub.case] = Atom::Nop;
				// Fill in the breaks
				for &brk in &sub.brks {
					let brk_offset = result.len() - brk - 1;
					if brk_offset >= 256 {
						return Err(PatError::SubOverflow);
					}
					result[brk] = Atom::Break(brk_offset as u8);
				}
			},
			// Skip many operator
			b'[' => {
				// Parse the lower bound
				let mut lower_bound = 0u32;
				let mut at_least_one_char = false;
				loop {
					chr = iter.next().cloned().ok_or(PatError::ManyInvalid)?;
					match chr {
						b'-' | b']' => break,
						chr @ b'0'...b'9' => {
							at_least_one_char = true;
							lower_bound = lower_bound * 10 + (chr - b'0') as u32;
							if lower_bound >= 16384 {
								return Err(PatError::ManyOverflow);
							}
						},
						_ => return Err(PatError::ManyInvalid),
					}
				}
				if !at_least_one_char {
					return Err(PatError::ManyInvalid);
				}
				// Turn the lower bound into skip ops
				if lower_bound > 0 {
					if lower_bound >= 256 {
						result.push(Atom::Rangext((lower_bound >> 8) as u8));
					}
					result.push(Atom::Skip((lower_bound & 0xff) as u8));
				}
				// Second many part is optional
				if chr == b']' {
					continue;
				}
				// Parse the upper bound
				let mut upper_bound = 0u32;
				loop {
					chr = iter.next().cloned().ok_or(PatError::ManyInvalid)?;
					match chr {
						b']' => break,
						chr @ b'0'...b'9' => {
							upper_bound = upper_bound * 10 + (chr - b'0') as u32;
							if upper_bound >= 16384 {
								return Err(PatError::ManyOverflow);
							}
						},
						_ => return Err(PatError::ManyInvalid),
					}
				}
				// Lower bound should be strictly less than the upper bound
				if lower_bound < upper_bound {
					let many_skip = upper_bound - lower_bound;
					if many_skip >= 256 {
						result.push(Atom::Rangext((many_skip >> 8) as u8));
					}
					result.push(Atom::Many((many_skip & 0xff) as u8));
				}
				else {
					return Err(PatError::ManyRange);
				}
			},
			// Match a byte
			b'0'...b'9' | b'A'...b'F' | b'a'...b'f' => {
				// High nibble of the byte
				let hi = if chr >= b'a' {
					chr - b'a' + 10
				}
				else if chr >= b'A' {
					chr - b'A' + 10
				}
				else {
					chr - b'0'
				};
				chr = iter.next().cloned().ok_or(PatError::UnpairedHexDigit)?;
				// Low nibble of the byte
				let lo = if chr >= b'a' && chr <= b'f' {
					chr - b'a' + 10
				}
				else if chr >= b'A' && chr <= b'F' {
					chr - b'A' + 10
				}
				else if chr >= b'0' && chr <= b'9' {
					chr - b'0'
				}
				else {
					return Err(PatError::UnpairedHexDigit);
				};
				// Add byte to the pattern
				result.push(Atom::Byte((hi << 4) + lo));
			},
			// Match raw bytes
			b'"' => loop {
				if let Some(chr) = iter.next().cloned() {
					if chr == b'"' {
						// Handle escaped quotes `""`
						if iter.as_slice().first() == Some(&b'"') {
							iter.next();
							result.push(Atom::Byte(b'"'));
							continue;
						}
						break;
					}
					result.push(Atom::Byte(chr));
				}
				else {
					return Err(PatError::UnclosedQuote);
				}
			},
			// Save the cursor
			b'\'' => {
				// 'Limited' save space
				if save >= u8::max_value() {
					return Err(PatError::SaveOverflow);
				}
				result.push(Atom::Save(save));
				save += 1;
			},
			// Skip bytes
			b'?' => {
				// match result.last_mut() {
				// 	Some(Atom::Skip(skip)) if *skip != PTR_SKIP && *skip < 127i8 => *skip += 1,
				// 	_ => result.push(Atom::Skip(1)),
				// };
				// Coalescence skips together
				if let Some(Atom::Skip(skip)) = result.last_mut() {
					if *skip != PTR_SKIP && *skip < 255u8 {
						*skip += 1;
						continue;
					}
				}
				result.push(Atom::Skip(1));
			},
			b'@' => {
				let op = iter.next().cloned().ok_or(PatError::AlignedOperand)?;
				let atom = if op >= b'0' && op <= b'9' {
					Atom::Aligned(op - b'0')
				}
				else if op >= b'A' && op <= b'Z' {
					Atom::Aligned(10 + (op - b'A'))
				}
				else if op >= b'a' && op <= b'z' {
					Atom::Aligned(10 + (op - b'a'))
				}
				else {
					return Err(PatError::AlignedOperand);
				};
				result.push(atom);
			},
			b'i' => {
				let atom = match iter.next().cloned() {
					Some(b'1') => Atom::ReadI8(save),
					Some(b'2') => Atom::ReadI16(save),
					Some(b'4') => Atom::ReadI32(save),
					_ => return Err(PatError::ReadOperand),
				};
				if save >= u8::max_value() {
					return Err(PatError::SaveOverflow);
				}
				save += 1;
				result.push(atom);
			},
			b'u' => {
				let atom = match iter.next().cloned() {
					Some(b'1') => Atom::ReadU8(save),
					Some(b'2') => Atom::ReadU16(save),
					Some(b'4') => Atom::ReadU32(save),
					_ => return Err(PatError::ReadOperand),
				};
				if save >= u8::max_value() {
					return Err(PatError::SaveOverflow);
				}
				save += 1;
				result.push(atom);
			},
			b'z' => {
				if save >= u8::max_value() {
					return Err(PatError::SaveOverflow);
				}
				result.push(Atom::Zero(save));
				save += 1;
			},
			// Allow spaces as padding
			b' ' | b'\n' | b'\r' | b'\t' => {},
			// Everything else is illegal
			_ => {
				return Err(PatError::UnknownChar);
			},
		}
		// Converted from str originally, should be safe
		*pat = unsafe { str::from_utf8_unchecked(iter.as_slice()) };
	}
	// Check balanced stack operators
	if depth != 0 {
		return Err(PatError::StackError);
	}
	// Check if sub patterns are balanced
	if subs.len() != 0 {
		return Err(PatError::SubPattern);
	}

	// Remove redundant atoms at the end
	fn is_redundant(atom: &Atom) -> bool {
		return match atom {
			Atom::Skip(_) | Atom::Rangext(_) | Atom::Pop | Atom::Many(_) => true,
			_ => false,
		};
	}
	while result.last().map(is_redundant).unwrap_or(false) {
		result.pop();
	}

	Ok(())
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;

	const _: [(); 2] = [(); std::mem::size_of::<Atom>()];

	#[rustfmt::skip]
	#[test]
	fn patterns() {
		use self::Atom::*;

		assert_eq!(parse("12 34 56 ? ?"), Ok(vec![Save(0), Byte(0x12), Byte(0x34), Byte(0x56)]));

		assert_eq!(parse("B9'?? 68???? E8${'} 8B"), Ok(vec![
			Save(0), Byte(0xB9), Save(1), Skip(2), Byte(0x68), Skip(4), Byte(0xE8), Push(4), Jump4, Save(2), Pop, Byte(0x8B)
		]));

		assert_eq!(parse("${%{${%{}}}}"), Ok(vec![
			Save(0), Push(4), Jump4, Push(1), Jump1, Push(4), Jump4, Push(1), Jump1
		]));

		assert_eq!(parse("24 5A9e D0 AFBea3 fCdd"), Ok(vec![
			Save(0), Byte(0x24), Byte(0x5A), Byte(0x9E), Byte(0xD0), Byte(0xAF), Byte(0xBE), Byte(0xA3), Byte(0xFC), Byte(0xDD)
		]));

		assert_eq!(parse("\"string\""), Ok(vec![
			Save(0), Byte(115), Byte(116), Byte(114), Byte(105), Byte(110), Byte(103)
		]));

		assert_eq!(parse("\"string\"\"quote\"\"string\""), Ok(vec![
			Save(0), Byte(115), Byte(116), Byte(114), Byte(105), Byte(110), Byte(103),
			Byte(34), Byte(113), Byte(117), Byte(111), Byte(116), Byte(101), Byte(34),
			Byte(115), Byte(116), Byte(114), Byte(105), Byte(110), Byte(103)
		]));

		assert_eq!(parse("*{FF D8 42}"), Ok(vec![
			Save(0), Push(PTR_SKIP), Ptr, Byte(0xFF), Byte(0xD8), Byte(0x42)
		]));
		assert_eq!(parse("*{\"hello\"00}"), Ok(vec![
			Save(0), Push(PTR_SKIP), Ptr, Byte(104), Byte(101), Byte(108), Byte(108), Byte(111), Byte(0)
		]));

		assert_eq!(parse("b8 [16] 50 [13-42] ff"), Ok(vec![
			Save(0), Byte(0xb8), Skip(16), Byte(0x50), Skip(13), Many(29), Byte(0xff)
		]));

		assert_eq!(parse("e9 $ @4"), Ok(vec![
			Save(0), Byte(0xe9), Jump4, Aligned(4)
		]));

		assert_eq!(parse("83 c0 2a ( 6a ? | 68 ? ? ? ? ) e8"), Ok(vec![
			Save(0), Byte(0x83), Byte(0xc0), Byte(0x2a),
			Case(3), Byte(0x6a), Skip(1), Break(3),
			Nop, Byte(0x68), Skip(4), Byte(0xe8),
		]));
	}

	#[test]
	fn errors() {
		use self::PatError::*;
		assert_eq!(Err(ParsePatError { kind: StackError, position: 2 }), parse("${"));
		assert_eq!(Err(ParsePatError { kind: StackError, position: 0 }), parse("}}"));
		assert_eq!(Err(ParsePatError { kind: StackInvalid, position: 3 }), parse("AB {}"));
		assert_eq!(Err(ParsePatError { kind: UnpairedHexDigit, position: 2 }), parse("123"));
		assert_eq!(Err(ParsePatError { kind: UnpairedHexDigit, position: 3 }), parse("EE BZ"));
		assert_eq!(Err(ParsePatError { kind: UnpairedHexDigit, position: 0 }), parse("A?"));
		assert_eq!(Err(ParsePatError { kind: UnknownChar, position: 0 }), parse("é"));
		assert_eq!(Err(ParsePatError { kind: AlignedOperand, position: 0 }), parse("@"));
		assert_eq!(Err(ParsePatError { kind: UnclosedQuote, position: 0 }), parse("\"unbalanced"));
		assert_eq!(Err(ParsePatError { kind: ManyInvalid, position: 0 }), parse("[-2]"));
		assert_eq!(Err(ParsePatError { kind: ManyInvalid, position: 0 }), parse("[-]"));
		assert_eq!(Err(ParsePatError { kind: ManyInvalid, position: 0 }), parse("[]"));
		assert_eq!(Err(ParsePatError { kind: ManyRange, position: 0 }), parse("[0-]"));
		assert_eq!(Err(ParsePatError { kind: ManyRange, position: 0 }), parse("[0-0]"));
		assert_eq!(Err(ParsePatError { kind: ManyRange, position: 0 }), parse("[20-1]"));
		assert_eq!(Err(ParsePatError { kind: ManyOverflow, position: 0 }), parse("[20000-40000]"));
	}
}
