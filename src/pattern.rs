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

use std::{error, fmt, mem};

/// Max recursion depth.
pub const STACK_SIZE: usize = 4;
/// Special skip value to indicate to use platform pointer size instead.
pub(crate) const PTR_SKIP: i8 = -128;

//----------------------------------------------------------------

/// Pattern parsing error.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ParsePatError(PatError);

impl fmt::Display for ParsePatError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		error::Error::description(self).fmt(f)
	}
}

impl error::Error for ParsePatError {
	fn description(&self) -> &str {
		match self.0 {
			PatError::UnpairedHexDigit => "unpaired hex digit",
			PatError::UnknownChar => "unknown char",
			PatError::SkipOverflow => "skip overflow",
			PatError::ManyBounds => "lower bound is greater than the upper bound",
			PatError::SaveOverflow => "save overflow",
			PatError::StackError => "stack error",
			PatError::SyntaxError => "invalid syntax",
			PatError::UnclosedQuote => "unclosed quote",
		}
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum PatError {
	UnpairedHexDigit,
	UnknownChar,
	SkipOverflow,
	ManyBounds,
	SaveOverflow,
	StackError,
	SyntaxError,
	UnclosedQuote,
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
	Push(i8),
	/// Pops the cursor from the stack and continues matching.
	Pop,
	/// Sets a mask to apply on next byte match.
	Fuzzy(u8),
	/// Skips a fixed number of bytes.
	Skip(i8),
	/// Extends the skip range by `argument * 128`.
	SkipExt(i8),
	/// Looks for the next pattern at most a certain number of bytes ahead.
	Many(u8),
	/// Extends the search range of many by `argument * 256`.
	ManyExt(u8),
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
}

/// Patterns are a vector of [`Atom`](enum.Atom.html)s.
pub type Pattern = Vec<Atom>;

/// Pattern parser.
///
/// # Remarks
///
/// Following are examples of the pattern syntax.
/// The syntax takes inspiration from [YARA hexadecimal strings](https://yara.readthedocs.io/en/v3.7.0/writingrules.html#hexadecimal-strings).
///
/// ```text
/// 55 89 e5 83 ec ?
/// ```
///
/// Case insensitive hexadecimal characters match the exact byte pattern and question marks serve as placeholders for unknown bytes.
///
/// Note that a single question mark matches a whole byte. The syntax to mask part of a byte is not yet available.
///
/// Spaces (code point 32) are completely optional and carry no semantic meaning, their purpose is to visually group things together.
///
/// ```text
/// b9 '37 13 00 00
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
/// b8 [16-50] 50
/// ```
///
/// Pairs of numbers separated by a hypen in square brackets indicate the lower and upper bound of number of bytes to skip.
/// The scanner is non greedy and considers the first match while skipping as little as possible.
///
/// The second number must be larger than or equal to the lower bound, if they're equal the upper bound and hypen can be left out to indicate a large fixed skip eg. `[16]`.
/// This is semantically equivalent to writing that many question marks, but it's more convenient to write it like this.
///
/// ```text
/// 31 c0 74 % 'c3
/// e8 $ '31 c0 c3
/// 68 * '31 c0 c3
/// ```
///
/// These symbols are used to follow; a signed 1 byte relative jump (`%`), a signed 4 byte relative jump (`$`) and an absolute pointer (`*`).
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
/// Escape sequences are not supported, switch back to matching with hex digits instead.
/// For UTF-16 support, you are welcome to send a PR.
///
/// ```text
/// e8 $ { ' } 83 f0 5c c3
/// ```
///
/// Curly braces indicate a subpattern.
///
/// Subpatterns allow to follow jumps and return matching before the jump was taken.
/// The bytes used to follow the jump are already skipped to allow to seamlessly continue matching.
///
/// A subpattern is only valid after a jump indicated by any of `%$*`.
pub fn parse(pat: &str) -> Result<Pattern, ParsePatError> {
	let mut result = Vec::with_capacity(pat.len() / 2);
	result.push(Atom::Save(0));
	match parse_helper(pat, &mut result) {
		Ok(()) => {},
		Err(err) => return Err(ParsePatError(err)),
	};

	// Remove redundant atoms
	while match result.last() {
		Some(Atom::Skip(_)) | Some(Atom::Pop) | Some(Atom::Many(_)) => true,
		_ => false,
	} {
		result.pop();
	}

	Ok(result)
}
fn parse_helper(pat: &str, result: &mut Vec<Atom>) -> Result<(), PatError> {
	let mut iter = pat.as_bytes().iter().cloned();
	let mut save = 1;
	let mut depth = 0;
	while let Some(chr) = iter.next() {
		match chr {
			// Follow signed 1 byte jump
			b'%' => {
				result.push(Atom::Jump1);
			},
			// Follow signed 4 byte jump
			b'$' => {
				result.push(Atom::Jump4);
			},
			// Follow pointer
			b'*' => {
				result.push(Atom::Ptr);
			},
			// Start recursive operator
			b'{' => {
				depth += 1;
				// Must follow a jump operator and insert push before the jump
				let atom = match result.last_mut() {
					Some(atom @ Atom::Jump1) => mem::replace(atom, Atom::Push(1)),
					Some(atom @ Atom::Jump4) => mem::replace(atom, Atom::Push(4)),
					Some(atom @ Atom::Ptr) => mem::replace(atom, Atom::Push(PTR_SKIP)),
					_ => return Err(PatError::SyntaxError),
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
			// Skip many operator
			b'[' => {
				let mut chr;
				// Parse the lower bound
				let mut lower_bound = 0u32;
				loop {
					chr = iter.next();
					match chr {
						Some(b'-') | Some(b']') => break,
						Some(chr @ b'0'...b'9') => {
							lower_bound = lower_bound * 10 + (chr - b'0') as u32;
							if lower_bound > 16384 {
								return Err(PatError::SkipOverflow);
							}
						},
						Some(_) => return Err(PatError::SyntaxError),
						None => return Err(PatError::SyntaxError),
					}
				}
				// Turn the lower bound into skip ops
				if lower_bound >= 128 {
					result.push(Atom::SkipExt((lower_bound >> 7) as i8));
				}
				result.push(Atom::Skip((lower_bound & 0x7f) as i8));
				// Second many part is optional
				if chr == Some(b']') {
					continue;
				}
				// Parse the upper bound
				let mut upper_bound = 0u32;
				loop {
					chr = iter.next();
					match chr {
						Some(b']') => break,
						Some(chr @ b'0'...b'9') => {
							upper_bound = upper_bound * 10 + (chr - b'0') as u32;
							if upper_bound > 16384 {
								return Err(PatError::SkipOverflow);
							}
						},
						Some(_) => return Err(PatError::SyntaxError),
						None => return Err(PatError::SyntaxError),
					}
				}
				if lower_bound == 0 {
					result.push(Atom::Many(0));
				}
				else if lower_bound < upper_bound {
					let many_skip = upper_bound - lower_bound;
					if many_skip >= 128 {
						result.push(Atom::ManyExt((many_skip >> 8) as u8));
					}
					result.push(Atom::Many((many_skip & 0xff) as u8));
				}
				else if lower_bound != upper_bound {
					return Err(PatError::ManyBounds);
				}
			},
			// Match a byte
			b'0'...b'9' | b'A'...b'F' | b'a'...b'f' => {
				if let Some(chr2) = iter.next() {
					// High nibble of the byte
					let hi = if chr >= b'a' { chr - b'a' + 0xA }
						else if chr >= b'A' { chr - b'A' + 0xA }
						else { chr - b'0' };
					// Low nibble of the byte
					let lo = if chr2 >= b'a' && chr2 <= b'f' { chr2 - b'a' + 0xA }
						else if chr2 >= b'A' && chr2 <= b'F' { chr2 - b'A' + 0xA }
						else if chr2 >= b'0' && chr2 <= b'9' { chr2 - b'0' }
						else { return Err(PatError::UnpairedHexDigit); };
					// Add byte to the pattern
					result.push(Atom::Byte((hi << 4) + lo));
				}
				else {
					return Err(PatError::UnpairedHexDigit);
				}
			},
			// Match raw bytes
			b'"' => {
				loop {
					if let Some(chr) = iter.next() {
						if chr != b'"' {
							result.push(Atom::Byte(chr));
						}
						else {
							break;
						}
					}
					else {
						return Err(PatError::UnclosedQuote);
					}
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
					if *skip != PTR_SKIP && *skip < 127i8 {
						*skip += 1;
						continue;
					}
				}
				result.push(Atom::Skip(1));
			},
			// Allow spaces as padding
			b' ' => {},
			// Everything else is illegal
			_ => {
				return Err(PatError::UnknownChar);
			},
		}
	}
	// Check balanced stack operators
	if depth != 0 {
		return Err(PatError::StackError);
	}

	Ok(())
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sizes() {
		assert_size_of!(2, Atom);
	}

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

		assert_eq!(parse("*{FF D8 42}"), Ok(vec![
			Save(0), Push(PTR_SKIP), Ptr, Byte(0xFF), Byte(0xD8), Byte(0x42)
		]));
		assert_eq!(parse("*{\"hello\"00}"), Ok(vec![
			Save(0), Push(PTR_SKIP), Ptr, Byte(104), Byte(101), Byte(108), Byte(108), Byte(111), Byte(0)
		]));
	}

	#[test]
	fn errors() {
		use self::PatError::*;
		// assert_eq!(Err(ParsePatError(StackOverflow)), parse("${%{${%{${%{"));
		assert_eq!(Err(ParsePatError(StackError)), parse("${"));
		assert_eq!(Err(ParsePatError(StackError)), parse("}}"));
		assert_eq!(Err(ParsePatError(SyntaxError)), parse("AB {}"));
		assert_eq!(Err(ParsePatError(UnpairedHexDigit)), parse("123"));
		assert_eq!(Err(ParsePatError(UnpairedHexDigit)), parse("EE BZ"));
		assert_eq!(Err(ParsePatError(UnpairedHexDigit)), parse("A?"));
		assert_eq!(Err(ParsePatError(UnknownChar)), parse("@"));
		assert_eq!(Err(ParsePatError(UnclosedQuote)), parse("\"unbalanced"));
	}
}
