/*!
Scanner Patterns.

# What are patterns?

A pattern is a sequence of atoms. An atom describes either a specific byte or a control instruction such as skipping the next X bytes.

In this regard a pattern looks a lot like a simple [regular expression](https://en.wikipedia.org/wiki/Regular_expression).
But whereas regular expressions are designed to work with text, patterns are designed to work with executable code and binary data.

Patterns can encode more than just _match exact byte_ and _skip X bytes_ such as _follow this 1 byte signed jump, _follow this 4 byte signed jump
and _follow this pointer_ including the ability to continue matching after returning from following a relative jump or pointer.

# Why use patterns?

Reverse engineering is hard, when you've found an interesting address (such as a function or global variable)
you don't want to spend all that effort again when the program is updated.

Luckily when programs update they usually don't change all that much, some functions and data are changed but the rest has remained the same.
However this means that the unchanged bits may be shuffled around to a different place.

To find all matches of a pattern, eg. find all locations which call a function or reference some data, automating analysis.

Patterns let you track interesting parts of a program even as it is updated.

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
			PatError::SaveOverflow => "save overflow",
			PatError::StackOverflow => "stack overflow",
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
	SaveOverflow,
	StackOverflow,
	StackError,
	SyntaxError,
	UnclosedQuote,
}

//----------------------------------------------------------------

/// Pattern atom.
///
/// During matching invalid and nonsensical atom arguments are ignored.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Atom {
	/// Matches a single byte.
	///
	/// Matching fails immediately on a byte mismatch.
	Byte(u8),
	/// Captures the cursor in the [`Match`](struct.Match.html) struct at the specified tuple index.
	///
	/// When an out of range tuple index is given the capture is ignored.
	Save(u8),
	/// Pushes the cursor with the given offset on the stack.
	///
	/// The stack size is limited to [`STACK_SIZE`](constant.STACK_SIZE.html) entries.
	Push(i8),
	/// Pops the cursor from the stack and continues matching.
	Pop,
	/// Skips a fixed number of bytes.
	Skip(i8),
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
	Ptr,
}

/// Patterns are a vector of [`Atom`](enum.Atom.html)s.
pub type Pattern = Vec<Atom>;

/// Parses a pattern string.
///
/// # Remarks
///
/// The pattern string may contain any of the following characters:
///
/// * `AA`
///
///   Two case-insensitive hexadecimal digits (ie. `[0-9a-fA-F]{2}`).
///
///   Matching fails immediately on byte mismatch.
///
/// * Spaces (codepoint 32) are ignored and can be used for visual grouping.
///
/// * `'`
///
///   Accents save the cursor.
///
///   Not always interested in the address where the start of the pattern is found, a backtick saves the cursor.
///   This is especially helpful when following relative jumps and absolute pointers.
///
///   A common pattern is `${'}` which follows a relative jump and saves the destination address before returning back to continue matching.
///
///   The saved cursors can be accessed through the `Match` struct starting from `Match.1` for the first backtick limited by the size of [`Match`](struct.Match.html).
///   `Match.0` is reserved for the address of the start of the pattern match.
///
/// * `?`
///
///   Skips a single byte.
///
///   The question mark acts as a wildcard for a single byte.
///
/// * `$`
///
///   Follow a signed 4 byte relative jump.
///
///   The scanner reads a signed dword and adds it plus 4 to the current address.
///   This allows the pattern to seamlessly follow jumps and calls.
///
///   Use a backtick to save the destination, see above for more information.
///
/// * `%`
///
///   Follow a signed 1 byte relative jump.
///
///   The scanner reads a signed byte, sign extends and adds it plus 1 to the current address.
///   Same as `$` but for short jumps.
///
/// * `*`
///
///   Follow an absolute pointer.
///
///   The scanner reads a pointer sized integer, converts it to an RVA and continues matching.
///
///   If the pointer cannot be converted to an RVA the matching fails.
///   This allows the pattern to seamlessly follow data accesses.
///
///   A common pattern is eg. `B8*{'STRING'00}` to match against loading constants from `.rdata`.
///
/// * `{}`
///
///   Following a `$`, `%` or `*`; saves the current address before chasing the jump and on the closing `}` matching resumes where it left off.
///
///   Any subpattern goes in between the curly braces. These can be nested up to a depth of [`STACK_SIZE`](constant.STACK_SIZE.html).
///
///   The bytes used to jump (1 for `%`, 4 for `$` and 4 or 8 for `*` depending on bitness) are skipped when returning.
///
/// * `"literal"`
///
///   Match raw bytes in between the quotes. Handy for matching string constants.
///
///   Only supports UTF-8. No escape sequences available, specify them as bytes instead.
///
/// * Any other characters are invalid and not allowed.
///
/// # Examples
///
/// ```
/// use pelite::pattern::{parse, Atom};
///
/// static MY_PATTERN: [Atom; 7] = [
/// 	Atom::Save(0),
/// 	Atom::Byte(0xE9),
/// 	Atom::Push(4),
/// 	Atom::Jump4,
/// 	Atom::Save(1),
/// 	Atom::Pop,
/// 	Atom::Byte(0xC3),
/// ];
///
/// let pat = parse("E9${'} C3").unwrap();
/// assert_eq!(pat, &MY_PATTERN);
/// ```
#[inline]
pub fn parse<P: ?Sized + AsRef<str>>(pat: &P) -> Result<Pattern, ParsePatError> {
	parse_pat(pat.as_ref()).map_err(|e| ParsePatError(e))
}

fn parse_pat(pat: &str) -> Result<Pattern, PatError> {
	let mut result = Vec::with_capacity(pat.len() / 2);
	result.push(Atom::Save(0));
	let mut iter = pat.as_bytes().iter();
	let mut depth = 0;
	let mut save = 1;
	let mut jump = None;
	while let Some(&chr) = iter.next() {
		match chr {
			// Follow signed 1 byte jump
			b'%' => {
				result.push(Atom::Jump1);
				jump = Some(1);
			},
			// Follow signed 4 byte jump
			b'$' => {
				result.push(Atom::Jump4);
				jump = Some(4);
			},
			// Follow pointer
			b'*' => {
				result.push(Atom::Ptr);
				jump = Some(PTR_SKIP);
			},
			// Start recursive operator
			b'{' => {
				// Limited recursive depth
				if depth >= STACK_SIZE {
					return Err(PatError::StackOverflow);
				}
				depth += 1;
				// Must follow a jump operator
				if let Some(skip) = jump {
					// Put the push before the jump operator
					let atom = mem::replace(result.last_mut().unwrap(), Atom::Push(skip));
					result.push(atom);
					jump = None;
				}
				else {
					return Err(PatError::SyntaxError);
				}
			},
			// End recursive operator
			b'}' => {
				// Unbalanced recursion
				if depth <= 0 {
					return Err(PatError::StackError);
				}
				depth -= 1;
				result.push(Atom::Pop);
				jump = None;
			},
			// Match a byte
			b'0'...b'9' | b'A'...b'F' | b'a'...b'f' => {
				if let Some(&chr2) = iter.next() {
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
					jump = None;
				}
				else {
					return Err(PatError::UnpairedHexDigit);
				}
			},
			// Match raw bytes
			b'"' => {
				jump = None;
				loop {
					if let Some(&chr) = iter.next() {
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
				// Limited save space
				if save >= (MAX_SAVE as u8) {
					return Err(PatError::SaveOverflow);
				}
				result.push(Atom::Save(save));
				save += 1;
				jump = None;
			},
			// Skip bytes
			b'?' => {
				jump = None;
				// Coalescence skips together
				if let Some(&mut Atom::Skip(ref mut s)) = result.last_mut() {
					if *s != PTR_SKIP && *s < 127i8 {
						*s += 1;
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
	if depth != 0 {
		return Err(PatError::StackError);
	}
	// Remove redundant atoms
	while match result.last() {
		Some(&Atom::Skip(_)) | Some(&Atom::Pop) => true,
		_ => false,
	} {
		result.pop();
	}
	Ok(result)
}

//----------------------------------------------------------------

/// Max saved cursors.
pub(crate) const MAX_SAVE: usize = 7;

/// Pattern scan result.
///
/// The scanner populates the result with `Atom::Save(i)` where the cursor is saved at the tuple index `i`.
///
/// Each backtick in a pattern writes to the next slot where the first element is the start of the pattern match.
#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
pub struct Match(pub u32, pub u32, pub u32, pub u32, pub u32, pub u32, pub u32);
impl AsRef<[u32; MAX_SAVE]> for Match {
	fn as_ref(&self) -> &[u32; MAX_SAVE] {
		unsafe { mem::transmute(self) }
	}
}
impl AsMut<[u32; MAX_SAVE]> for Match {
	fn as_mut(&mut self) -> &mut [u32; MAX_SAVE] {
		unsafe { mem::transmute(self) }
	}
}
impl fmt::Display for Match {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:08X}", self.0)?;
		if self.1 != 0 {
			write!(f, " [{:08X}", self.1)?;
			for &cursor in self.as_ref()[2..].iter().take_while(|&&cursor| cursor != 0) {
				write!(f, ", {:08X}", cursor)?;
			}
			f.write_str("]")?;
		}
		Ok(())
	}
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sizes() {
		use std::mem;
		// Compile time asserts the sizes of these types.
		let _ = unsafe { mem::forget::<u16>(mem::transmute(mem::uninitialized::<Atom>())) };
		let _ = unsafe { mem::forget::<[u32; MAX_SAVE]>(mem::transmute(mem::uninitialized::<Match>())) };
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
		assert_eq!(Err(ParsePatError(StackOverflow)), parse("${%{${%{${%{"));
		assert_eq!(Err(ParsePatError(StackError)), parse("${"));
		assert_eq!(Err(ParsePatError(StackError)), parse("}}"));
		assert_eq!(Err(ParsePatError(SyntaxError)), parse("AB {}"));
		assert_eq!(Err(ParsePatError(UnpairedHexDigit)), parse("123"));
		assert_eq!(Err(ParsePatError(UnpairedHexDigit)), parse("EE BZ"));
		assert_eq!(Err(ParsePatError(UnpairedHexDigit)), parse("A?"));
		assert_eq!(Err(ParsePatError(SaveOverflow)), parse("'?'?'?'?'?'?'?'?"));
		assert_eq!(Err(ParsePatError(UnknownChar)), parse("@"));
		assert_eq!(Err(ParsePatError(UnclosedQuote)), parse("\"unbalanced"));
	}
}
