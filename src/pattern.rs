#![doc = include_str!("pattern/syntax.md")]

use alloc::vec::Vec;
use core::fmt;

//----------------------------------------------------------------

mod parser;

pub use self::parser::parse;
#[doc(hidden)]
pub use self::parser::{parse_const, parse_len};

mod analysis;

#[cfg(test)]
mod tests;

//----------------------------------------------------------------

/// Options controlling pattern parsing.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct ParseOptions {
	/// Accept legacy `[n]` and `[a-b]` gap syntax.
	pub legacy_gap: bool,
}
impl ParseOptions {
	#[doc(hidden)]
	pub const DEFAULT: Self = Self { legacy_gap: false };
}
impl Default for ParseOptions {
	fn default() -> Self {
		Self::DEFAULT
	}
}

//----------------------------------------------------------------

/// Pattern syntax or validation error.
///
/// Positions reported by parsers are byte offsets.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PatternError {
	kind: ErrorKind,
	position: usize,
}
impl PatternError {
	/// Returns the byte offset in the pattern source where the error was detected.
	pub const fn position(&self) -> usize {
		self.position
	}
}
impl fmt::Display for PatternError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Pattern Error @{}: {}.", self.position, self.kind.to_str())
	}
}
impl core::error::Error for PatternError {
	fn description(&self) -> &str {
		self.kind.to_str()
	}
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
enum ErrorKind {
	UnpairedHexDigit,
	UnknownChar,
	SaveOverflow,
	StackError,
	UnclosedQuote,
	AlignedOperand,
	ReadOperand,
	SubPattern,
	Operand,
	SlotOperand,
	SlotConflict,
	NestingOverflow,
	UninitializedSlot,
	RetryClobber,
}
impl ErrorKind {
	const fn to_str(self) -> &'static str {
		match self {
			ErrorKind::UnpairedHexDigit => "unpaired hex digit",
			ErrorKind::UnknownChar => "unknown character",
			ErrorKind::SaveOverflow => "save store overflow",
			ErrorKind::StackError => "stack unbalanced",
			ErrorKind::UnclosedQuote => "string missing end quote",
			ErrorKind::AlignedOperand => "aligned operand error",
			ErrorKind::ReadOperand => "read operand error",
			ErrorKind::SubPattern => "sub pattern error",
			ErrorKind::Operand => "invalid or oversized movement operand",
			ErrorKind::SlotOperand => "invalid save slot operand",
			ErrorKind::SlotConflict => "save slot used both automatically and explicitly",
			ErrorKind::NestingOverflow => "pattern nesting limit exceeded",
			ErrorKind::UninitializedSlot => "save slot may be read before initialization",
			ErrorKind::RetryClobber => "save slot may have been overwritten by a failed retry",
		}
	}
}

//----------------------------------------------------------------

/// Two-byte instructions for the pattern scanner.
///
/// The cursor and save slots hold 32-bit RVAs or integer values. Pointer-sized operations
/// use the scanned image's pointer width (4 bytes for PE32, 8 for PE32+), not the host's.
/// Reads fail the match when their source bytes cannot be read. Stores outside the save
/// array are ignored, and reads from out-of-bounds save slots return zero.
///
/// Save slot operands are signed: `0..=127` index from the start of the save array,
/// while `-1` selects its last slot, `-2` its second-last, through `-128`.
/// Use [`save_len`] to reserve enough space for front and back slots without overlap.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Atom {
	/// Matches one byte under the cursor and advances by 1.
	///
	/// When immediately preceded by [`Fuzzy`](Atom::Fuzzy), compares `byte & mask == argument & mask`.
	/// A successful byte match resets the mask to `0xff`.
	Byte(u8),
	/// Writes the current cursor to the given save slot without advancing it.
	Save(i8),
	/// Sets the cursor to the value in the given save slot, without reading image bytes.
	Seek(i8),
	/// Sets the bit mask for the immediately following [`Byte`](Atom::Byte).
	///
	/// Set bits must match; clear bits are ignored. The mask applies to that byte only.
	/// Behavior is unspecified if the next instruction is not `Byte`.
	Fuzzy(u8),
	/// Advances the cursor by `argument + extension`, wrapping at 32 bits.
	///
	/// Consumes the pending [`Extend`](Atom::Extend) extension. A combined offset of zero
	/// means the image's pointer width. Does not check whether skipped bytes are readable.
	Skip(u8),
	/// Rewinds the cursor by `argument + extension`, wrapping at 32 bits.
	///
	/// Consumes the pending [`Extend`](Atom::Extend) extension. A combined offset of zero
	/// means the image's pointer width. Does not read or validate the destination.
	Rewind(u8),
	/// Sets the range extension for the next [`Skip`](Atom::Skip),
	/// [`Rewind`](Atom::Rewind), [`Scan`](Atom::Scan), [`Fork`](Atom::Fork) or
	/// [`Goto`](Atom::Goto) in the current interpreter frame.
	///
	/// Appends `argument` as the next high byte of that instruction's operand, then is consumed.
	/// Consecutive extensions accumulate, allowing up to three prefixes for a 32-bit operand.
	/// For example, `Extend(1), Extend(2), Skip(3)` advances `0x010203` bytes.
	Extend(u8),
	/// Searches forward for a match of the following atoms, trying the nearest cursor first.
	///
	/// Consumes [`Extend`](Atom::Extend). The combined operand is the maximum distance to skip:
	/// offsets `0..=limit`, restricted to the current section's remaining readable bytes.
	/// Zero searches the entire remainder of that section. Each attempt starts with fresh modifiers;
	/// save-slot contents after a failed attempt are unspecified. Succeeds on the first successful continuation.
	Scan(u8),
	/// Follows a signed 1-byte relative reference.
	///
	/// Reads an `i8` under the cursor and sets `cursor = cursor + 1 + displacement`,
	/// wrapping at 32 bits. The cursor must point at the displacement, not the opcode.
	Jump1,
	/// Follows a signed 4-byte relative reference.
	///
	/// Reads an `i32` under the cursor and sets `cursor = cursor + 4 + displacement`,
	/// wrapping at 32 bits. The base is the end of the displacement, not necessarily the
	/// end of the instruction; account separately for any following immediate bytes.
	Jump4,
	/// Follows an absolute pointer of the scanned image's pointer width.
	///
	/// Reads the VA under the cursor, translates it to an RVA and assigns it to the cursor.
	/// Matching fails when the pointer cannot be read or translated to an RVA.
	Ptr,
	/// Follows a 4-byte relative reference based on the given save slot.
	///
	/// Reads an `i32` under the cursor and sets `cursor = save[slot] + displacement`, wrapping at 32 bits.
	/// Unlike [`Jump4`](Atom::Jump4), does not add the displacement's width to the base.
	Pir(i8),
	/// Fails unless the cursor equals the value in the given save slot.
	///
	/// Does not advance the cursor.
	Check(i8),
	/// Fails unless the cursor is aligned to `1 << argument` bytes; does not advance it.
	///
	/// The argument must be less than 32. For example, 4 requires 16-byte alignment.
	IsAlign(u8),
	/// Reads an `i8`, sign-extends it to 32 bits, stores it in the given slot and advances by 1.
	ReadI8(i8),
	/// Reads a `u8`, zero-extends it to 32 bits, stores it in the given slot and advances by 1.
	ReadU8(i8),
	/// Reads an `i16`, sign-extends it to 32 bits, stores it in the given slot and advances by 2.
	ReadI16(i8),
	/// Reads a `u16`, zero-extends it to 32 bits, stores it in the given slot and advances by 2.
	ReadU16(i8),
	/// Reads 32 bits into the given save slot and advances the cursor by 4.
	///
	/// Signed and unsigned values have the same representation; both `i4` and `u4` syntax
	/// compile to this atom. Two consecutive reads can capture a 64-bit value in two slots
	/// for the caller to reassemble.
	ReadU32(i8),
	/// Reads an `i8`, sign-extends it to 32 bits and compares it with the given save slot.
	///
	/// Fails on a mismatching value or unreadable source. On success, advances by 1 without changing the slot.
	TestI8(i8),
	/// Reads a `u8`, zero-extends it to 32 bits and compares it with the given save slot.
	///
	/// Fails on a mismatching value or unreadable source. On success, advances by 1 without changing the slot.
	TestU8(i8),
	/// Reads an `i16`, sign-extends it to 32 bits and compares it with the given save slot.
	///
	/// Fails on a mismatching value or unreadable source. On success, advances by 2 without changing the slot.
	TestI16(i8),
	/// Reads a `u16`, zero-extends it to 32 bits and compares it with the given save slot.
	///
	/// Fails on a mismatching value or unreadable source. On success, advances by 2 without changing the slot.
	TestU16(i8),
	/// Reads a `u32` and compares it with the given save slot.
	///
	/// Fails on a mismatching value or unreadable source. On success, advances by 4 without changing the slot.
	TestU32(i8),
	/// Writes zero to the given save slot without advancing the cursor.
	Zero(i8),
	/// Records an alternative at `pc + 1 + argument + extension`, then continues with the next atom.
	///
	/// Consumes [`Extend`](Atom::Extend); the offset counts atoms, not bytes. On later failure,
	/// retries the most recent alternative with its saved cursor.
	/// The extension used by this fork is consumed before recording that state.
	/// Alternatives remain available across [`Goto`](Atom::Goto) and [`Seek`](Atom::Seek).
	/// The first alternative succeeds only if the entire remaining pattern matches.
	/// Save-slot contents after a failed alternative are unspecified; initialize slots before relying on their values.
	Fork(u8),
	/// Skips the next `argument + extension` atoms without moving the byte cursor.
	///
	/// Consumes [`Extend`](Atom::Extend); zero skips no atoms. Does not return from the current
	/// interpreter frame, modify scratch slots or commit an alternative.
	Goto(u8),
	/// Does nothing; used as a placeholder when compiling patterns.
	Nop,
}

impl Atom {
	#[inline]
	const fn input_slot(self) -> Option<i8> {
		match self {
			| Atom::Seek(slot)
			| Atom::Check(slot)
			| Atom::Pir(slot)
			| Atom::TestI8(slot)
			| Atom::TestU8(slot)
			| Atom::TestI16(slot)
			| Atom::TestU16(slot)
			| Atom::TestU32(slot) => Some(slot),
			_ => None,
		}
	}

	#[inline]
	const fn output_slot(self) -> Option<i8> {
		match self {
			| Atom::Save(slot)
			| Atom::Zero(slot)
			| Atom::ReadI8(slot)
			| Atom::ReadU8(slot)
			| Atom::ReadI16(slot)
			| Atom::ReadU16(slot)
			| Atom::ReadU32(slot) => Some(slot),
			_ => None,
		}
	}

	#[inline]
	const fn referenced_slot(self) -> Option<i8> {
		match self.input_slot() {
			Some(slot) => Some(slot),
			None => self.output_slot(),
		}
	}

	#[inline]
	const fn can_fail(self) -> bool {
		matches!(self,
			| Atom::Byte(_)
			| Atom::Scan(_)
			| Atom::Jump1
			| Atom::Jump4
			| Atom::Ptr
			| Atom::Pir(_)
			| Atom::Check(_)
			| Atom::IsAlign(_)
			| Atom::ReadI8(_)
			| Atom::ReadU8(_)
			| Atom::ReadI16(_)
			| Atom::ReadU16(_)
			| Atom::ReadU32(_)
			| Atom::TestI8(_)
			| Atom::TestU8(_)
			| Atom::TestI16(_)
			| Atom::TestU16(_)
			| Atom::TestU32(_))
	}
}

assert_sizeof!(2, Atom);

/// Patterns are a vector of [`Atom`].
pub type Pattern = Vec<Atom>;

/// Returns the number of user-facing capture slots in this signature.
///
/// Capture slots are the nonnegative slots referenced by the pattern, including slot 0, which conventionally contains the matching RVA.
pub const fn captures_len(pat: &[Atom]) -> usize {
	let mut captures = 0;
	let mut index = 0;
	while index < pat.len() {
		let atom = pat[index];
		index += 1;
		let slot = match atom.referenced_slot() {
			Some(slot) if slot >= 0 => slot,
			_ => continue,
		};
		let len = slot as usize + 1;
		if captures < len {
			captures = len;
		}
	}
	captures
}

/// Returns the length of the save array needed for this signature.
///
/// Reserves disjoint regions for slots addressed from the start and end. For example,
/// `Save(0)` and `Save(-1)` require two slots. A shorter array may alias these regions.
pub const fn save_len(pat: &[Atom]) -> usize {
	let mut front = 0;
	let mut back = 0;
	let mut index = 0;
	while index < pat.len() {
		let atom = pat[index];
		index += 1;
		let slot = match atom.referenced_slot() {
			Some(slot) => slot,
			None => continue,
		};
		if slot >= 0 {
			let len = slot as usize + 1;
			if front < len {
				front = len;
			}
		}
		else {
			let len = slot.unsigned_abs() as usize;
			if back < len {
				back = len;
			}
		}
	}
	front + back
}

/// Parses a scanner pattern at compile time.
///
/// ```
/// const PATTERN: &[pelite::pattern::Atom] = pelite::pattern!("55 8b ec");
/// ```
#[macro_export]
macro_rules! pattern {
	($pattern:expr) => {
		&const {
			$crate::pattern::parse_const::<{
				$crate::pattern::parse_len($pattern, $crate::pattern::ParseOptions::DEFAULT)
			}>($pattern, $crate::pattern::ParseOptions::DEFAULT)
		}
	};
}

#[test]
fn signed_save_len() {
	use Atom::*;
	assert_eq!(captures_len(&[]), 0);
	assert_eq!(captures_len(&[Save(0), Save(-1)]), 1);
	assert_eq!(captures_len(&[Save(0), Seek(7), ReadU32(3), Zero(-2)]), 8);
	assert_eq!(captures_len(&[Seek(127), Check(-128)]), 128);
	let reference = parse("E8 ${'}", ParseOptions::default()).unwrap();
	assert_eq!(captures_len(&reference), 2);
	assert_eq!(save_len(&reference), 3);
	let internal_reference = parse("E8 ${}", ParseOptions::default()).unwrap();
	assert_eq!(captures_len(&internal_reference), 1);
	assert_eq!(save_len(&internal_reference), 2);

	assert_eq!(save_len(&[]), 0);
	assert_eq!(save_len(&[Save(127)]), 128);
	assert_eq!(save_len(&[Seek(-128)]), 128);
	assert_eq!(save_len(&[Save(0), Save(-1)]), 2);
	assert_eq!(save_len(&[ReadU32(7), TestU32(-3)]), 11);
	assert_eq!(save_len(&[Save(0), Seek(127), Check(-128)]), 256);
	assert_eq!(save_len(&[Save(127), Zero(-128)]), 256);
}
