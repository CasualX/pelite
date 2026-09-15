//! Pattern language parser.
//!
//! All entry points run the same const-capable compiler. User slots are `0..=127`
//! (the nonnegative half of the VM's signed operands); automatic slots start at 1.
//! Reference returns use distinct negative slots. There are at most 128 reference
//! bodies and 128 simultaneously nested groups/bodies. Operands are 32-bit values
//! encoded by up to three `Extend` prefixes and a final instruction byte.
//!
//! Compiled patterns are checked for uninitialized slot reads and values clobbered
//! by failed retries. Caller-initialized slots are not assumed. Undecided shorthands
//! are rejected; exported-output coverage and value equivalence are not checked.
//! No trailing instructions are discarded: even `scan()` checks readability.

use super::*;

// Const evaluation cannot use `?` yet.
macro_rules! attempt {
	($expr:expr) => {
		match $expr {
			Ok(value) => value,
			Err(error) => return Err(error),
		}
	};
}

#[derive(Copy, Clone)]
struct Atoms {
	items: [Atom; 4],
	len: usize,
}
impl Atoms {
	const fn one(atom: Atom) -> Atoms {
		Atoms { items: [atom, Atom::Nop, Atom::Nop, Atom::Nop], len: 1 }
	}
	const fn pair(first: Atom, second: Atom) -> Atoms {
		Atoms { items: [first, second, Atom::Nop, Atom::Nop], len: 2 }
	}
	const fn extended(value: u32, atom: Atom) -> Atoms {
		let mut items = [Atom::Nop; 4];
		let mut len = 0;
		if value > 0x00ff_ffff {
			items[len] = Atom::Extend((value >> 24) as u8);
			len += 1;
		}
		if value > 0x0000_ffff {
			items[len] = Atom::Extend((value >> 16) as u8);
			len += 1;
		}
		if value > 0x0000_00ff {
			items[len] = Atom::Extend((value >> 8) as u8);
			len += 1;
		}
		items[len] = atom;
		Atoms { items, len: len + 1 }
	}
}

enum Token {
	Atoms(Atoms),
	Reference(Atom, u8),
	Group,
	Alternative,
	CloseGroup,
	CloseReference,
	End,
}

/// A counting sink is an empty slice. Emission and patching otherwise use the
/// exact same code, so the sizing pass also validates nesting and branch spans.
struct Output<'a> {
	atoms: &'a mut [Atom],
	len: usize,
	locate: Option<usize>,
	position: usize,
}
impl Output<'_> {
	const fn write(&mut self, index: usize, atom: Atom) {
		if !self.atoms.is_empty() {
			self.atoms[index] = atom;
		}
	}
	const fn emit(&mut self, atoms: Atoms, position: usize) {
		if let Some(index) = self.locate {
			if index >= self.len && index - self.len < atoms.len {
				self.position = position;
			}
		}
		let mut i = 0;
		while i < atoms.len {
			self.write(self.len, atoms.items[i]);
			self.len += 1;
			i += 1;
		}
	}
	/// Branches reserve four atoms so patching never moves compiled code.
	const fn branch(&mut self, index: usize, offset: usize, fork: bool) {
		let offset = offset as u32;
		self.write(index, if offset > 0x00ff_ffff { Atom::Extend((offset >> 24) as u8) } else { Atom::Nop });
		self.write(index + 1, if offset > 0x0000_ffff { Atom::Extend((offset >> 16) as u8) } else { Atom::Nop });
		self.write(index + 2, if offset > 0x0000_00ff { Atom::Extend((offset >> 8) as u8) } else { Atom::Nop });
		self.write(index + 3, if fork { Atom::Fork(offset as u8) } else { Atom::Goto(offset as u8) });
	}
}

const fn operand(atoms: &[Atom], pc: usize, low: u8) -> usize {
	let mut first = pc;
	while first > 0 && matches!(atoms[first - 1], Atom::Extend(_)) {
		first -= 1;
	}
	let mut value = 0;
	while first < pc {
		if let Atom::Extend(byte) = atoms[first] {
			value = (value << 8) + byte as usize;
		}
		first += 1;
	}
	(value << 8) + low as usize
}

struct Parser<'a> {
	bytes: &'a [u8],
	pos: usize,
	start: usize,
	string: Option<usize>,
	next_save: usize,
	next_return: usize,
	automatic_slots: u128,
	explicit_slots: u128,
	depth: usize,
}
impl<'a> Parser<'a> {
	const fn new(source: &'a str) -> Self {
		Self {
			bytes: source.as_bytes(),
			pos: 0,
			start: 0,
			string: None,
			next_save: 1,
			next_return: 0,
			automatic_slots: 0,
			explicit_slots: 0,
			depth: 0,
		}
	}
	const fn error(&self, kind: ErrorKind) -> PatternError {
		PatternError { kind, position: self.start }
	}
	const fn peek(&self) -> Option<u8> {
		if self.pos >= self.bytes.len() {
			return None;
		}
		Some(self.bytes[self.pos])
	}
	const fn take(&mut self, byte: u8) -> bool {
		if let Some(next) = self.peek() {
			if next == byte {
				self.pos += 1;
				return true;
			}
		}
		false
	}
	const fn keyword(&mut self, word: &[u8]) -> bool {
		let mut i = 0;
		while i < word.len() {
			if self.pos + i >= self.bytes.len() || self.bytes[self.pos + i] != word[i] {
				return false;
			}
			i += 1;
		}
		self.pos += word.len();
		true
	}
	const fn trivia(&mut self) {
		loop {
			match self.peek() {
				Some(b' ' | b'\t' | b'\r' | b'\n') => self.pos += 1,
				Some(b'/') if self.pos + 1 < self.bytes.len() && self.bytes[self.pos + 1] == b'/' => {
					while self.pos < self.bytes.len() && self.bytes[self.pos] != b'\n' {
						self.pos += 1;
					}
				},
				_ => return,
			}
		}
	}
	const fn number(&mut self, limit: u32, kind: ErrorKind) -> Result<u32, PatternError> {
		let radix = if self.keyword(b"0x") { 16 } else { 10 };
		let begin = self.pos;
		let mut value = 0u32;
		while let Some(byte) = self.peek() {
			let digit = hex(byte) as u32;
			if digit >= radix {
				break;
			}
			value = match value.checked_mul(radix) {
				Some(value) => value,
				None => return Err(self.error(kind)),
			};
			value = match value.checked_add(digit) {
				Some(value) if value <= limit => value,
				_ => return Err(self.error(kind)),
			};
			self.pos += 1;
		}
		if self.pos == begin {
			return Err(self.error(kind));
		}
		Ok(value)
	}
	const fn automatic(&mut self) -> Result<i8, PatternError> {
		if self.next_save > 127 {
			return Err(self.error(ErrorKind::SaveOverflow));
		}
		let slot = self.next_save;
		let bit = 1u128 << slot;
		if self.explicit_slots & bit != 0 {
			return Err(self.error(ErrorKind::SlotConflict));
		}
		self.automatic_slots |= bit;
		self.next_save += 1;
		Ok(slot as i8)
	}
	const fn slot(&mut self, output: bool) -> Result<i8, PatternError> {
		if !self.take(b'[') {
			return Err(self.error(ErrorKind::SlotOperand));
		}
		let slot = attempt!(self.number(127, ErrorKind::SlotOperand));
		let slot_index = slot as usize;
		if output && self.next_save <= slot_index {
			self.next_save = slot_index + 1;
		}
		if !self.take(b']') {
			return Err(self.error(ErrorKind::SlotOperand));
		}
		let bit = 1u128 << slot;
		if self.automatic_slots & bit != 0 {
			return Err(self.error(ErrorKind::SlotConflict));
		}
		self.explicit_slots |= bit;
		Ok(slot as i8)
	}
	const fn movement(&mut self, scan: bool) -> Result<Atoms, PatternError> {
		if !self.take(b'(') {
			return Err(self.error(ErrorKind::Operand));
		}
		if scan && self.take(b')') {
			return Ok(Atoms::one(Atom::Scan(0)));
		}
		let negative = !scan && self.take(b'-');
		let pointer = !scan && self.keyword(b"ptr");
		let value = if pointer {
			0
		}
		else {
			attempt!(self.number(u32::MAX, ErrorKind::Operand))
		};
		if !self.take(b')') {
			return Err(self.error(ErrorKind::Operand));
		}
		if !pointer && value == 0 {
			return Ok(Atoms { items: [Atom::Nop; 4], len: 0 });
		}
		let atom = if scan {
			Atom::Scan(value as u8)
		}
		else if negative {
			Atom::Rewind(value as u8)
		}
		else {
			Atom::Skip(value as u8)
		};
		Ok(Atoms::extended(value, atom))
	}
	const fn alignment(&mut self) -> Result<Atom, PatternError> {
		if !self.take(b'(') {
			return Err(self.error(ErrorKind::AlignedOperand));
		}
		let exponent = attempt!(self.number(31, ErrorKind::AlignedOperand));
		if !self.take(b')') {
			return Err(self.error(ErrorKind::AlignedOperand));
		}
		Ok(Atom::IsAlign(exponent as u8))
	}
	const fn byte(&mut self) -> Result<u8, PatternError> {
		if self.pos + 1 >= self.bytes.len() {
			return Err(self.error(ErrorKind::UnpairedHexDigit));
		}
		let hi = hex(self.bytes[self.pos]);
		let lo = hex(self.bytes[self.pos + 1]);
		if hi == 255 || lo == 255 {
			return Err(self.error(ErrorKind::UnpairedHexDigit));
		}
		self.pos += 2;
		Ok((hi << 4) | lo)
	}

	/// Consume one syntactic unit and lower it immediately where possible. This
	/// also handles slot allocation, string bytes, masks and reference lookahead.
	const fn next(&mut self) -> Result<Token, PatternError> {
		loop {
			if let Some(start) = self.string {
				self.start = start;
				match self.peek() {
					None => {
						return Err(self.error(ErrorKind::UnclosedQuote));
					}
					Some(b'"') => {
						self.pos += 1;
						self.string = None;
					}
					Some(byte) => {
						self.pos += 1;
						return Ok(Token::Atoms(Atoms::one(Atom::Byte(byte))));
					}
				}
			}
			self.trivia();
			self.start = self.pos;
			let byte = match self.peek() {
				Some(byte) => byte,
				None => return Ok(Token::End),
			};
			let atoms = if self.keyword(b"skip") {
				attempt!(self.movement(false))
			}
			else if self.keyword(b"scan") {
				attempt!(self.movement(true))
			}
			else if self.keyword(b"align") {
				Atoms::one(attempt!(self.alignment()))
			}
			else if self.keyword(b"save") {
				let slot = if matches!(self.peek(), Some(b'[')) {
					attempt!(self.slot(true))
				}
				else {
					attempt!(self.automatic())
				};
				Atoms::one(Atom::Save(slot))
			}
			else if self.keyword(b"seek") {
				Atoms::one(Atom::Seek(attempt!(self.slot(false))))
			}
			else if self.keyword(b"check") {
				Atoms::one(Atom::Check(attempt!(self.slot(false))))
			}
			else if self.keyword(b"zero") {
				Atoms::one(Atom::Zero(attempt!(self.slot(true))))
			}
			else if self.keyword(b"rel32") {
				self.trivia();
				if self.take(b'{') {
					return Ok(Token::Reference(Atom::Jump4, 4));
				}
				Atoms::one(Atom::Jump4)
			}
			else if self.keyword(b"rel8") {
				self.trivia();
				if self.take(b'{') {
					return Ok(Token::Reference(Atom::Jump1, 1));
				}
				Atoms::one(Atom::Jump1)
			}
			else if self.keyword(b"ptr") {
				self.trivia();
				if self.take(b'{') {
					return Ok(Token::Reference(Atom::Ptr, 0));
				}
				Atoms::one(Atom::Ptr)
			}
			else {
				self.pos += 1;
				match byte {
					b'"' => {
						self.string = Some(self.start);
						continue;
					}
					b'?' => {
						let mut count = 1u32;
						loop {
							self.trivia();
							if !self.take(b'?') {
								break;
							}
							count += 1;
						}
						Atoms::extended(count, Atom::Skip(count as u8))
					}
					b'\'' => Atoms::one(Atom::Save(attempt!(self.automatic()))),
					b'(' => return Ok(Token::Group),
					b')' => return Ok(Token::CloseGroup),
					b'|' => return Ok(Token::Alternative),
					b'}' => return Ok(Token::CloseReference),
					b'$' | b'%' | b'*' => {
						let (atom, width) = match byte {
							b'$' => (Atom::Jump4, 4),
							b'%' => (Atom::Jump1, 1),
							_ => (Atom::Ptr, 0),
						};
						self.trivia();
						if self.take(b'{') {
							return Ok(Token::Reference(atom, width));
						}
						Atoms::one(atom)
					}
					b'@' => {
						let exponent = match self.peek() {
							Some(b'0'..=b'9') => self.bytes[self.pos] - b'0',
							_ => return Err(self.error(ErrorKind::AlignedOperand)),
						};
						self.pos += 1;
						Atoms::one(Atom::IsAlign(exponent))
					}
					b'i' | b'u' | b'=' => {
						let test = byte == b'=';
						let signed = if test {
							match self.peek() {
								Some(b'i' | b'u') => {
									let signed = self.bytes[self.pos] == b'i';
									self.pos += 1;
									signed
								},
								_ => return Err(self.error(ErrorKind::ReadOperand)),
							}
						}
						else {
							byte == b'i'
						};
						let size = match self.peek() {
							Some(size @ (b'1' | b'2' | b'4')) => {
								self.pos += 1;
								size
							},
							_ => return Err(self.error(ErrorKind::ReadOperand)),
						};
						let slot = if !test && !matches!(self.peek(), Some(b'[')) {
							attempt!(self.automatic())
						}
						else {
							attempt!(self.slot(!test))
						};
						Atoms::one(match (test, signed, size) {
							(false, true, b'1') => Atom::ReadI8(slot),
							(false, false, b'1') => Atom::ReadU8(slot),
							(false, true, b'2') => Atom::ReadI16(slot),
							(false, false, b'2') => Atom::ReadU16(slot),
							(false, _, _) => Atom::ReadU32(slot),
							(true, true, b'1') => Atom::TestI8(slot),
							(true, false, b'1') => Atom::TestU8(slot),
							(true, true, b'2') => Atom::TestI16(slot),
							(true, false, b'2') => Atom::TestU16(slot),
							(true, _, _) => Atom::TestU32(slot),
						})
					}
					_ if hex(byte) != 255 => {
						self.pos -= 1;
						let value = attempt!(self.byte());
						// A comment after a byte is not a mask. Masks attach without whitespace.
						if matches!(self.peek(), Some(b'/')) && !(self.pos + 1 < self.bytes.len() && self.bytes[self.pos + 1] == b'/') {
							self.pos += 1;
							Atoms::pair(Atom::Fuzzy(attempt!(self.byte())), Atom::Byte(value))
						}
						else {
							Atoms::one(Atom::Byte(value))
						}
					}
					_ => {
						return Err(self.error(ErrorKind::UnknownChar));
					}
				}
			};
			return Ok(Token::Atoms(atoms));
		}
	}

	const fn enter(&mut self) -> Result<(), PatternError> {
		if self.depth == 128 {
			return Err(self.error(ErrorKind::NestingOverflow));
		}
		self.depth += 1;
		Ok(())
	}

	/// Parse one concatenation. Delimiters are returned to their owning frame,
	/// which prevents a reference body from crossing an alternative boundary.
	const fn sequence(&mut self, output: &mut Output<'_>) -> Result<Token, PatternError> {
		loop {
			match attempt!(self.next()) {
				Token::Atoms(atoms) => {
					output.emit(atoms, self.start);
				}
				Token::Group => {
					attempt!(self.enter());
					attempt!(self.group(output));
					self.depth -= 1;
				}
				Token::Reference(atom, width) => {
					attempt!(self.enter());
					if self.next_return == 128 {
						return Err(self.error(ErrorKind::SaveOverflow));
					}
					self.next_return += 1;
					let slot = -(self.next_return as i16) as i8;
					output.emit(Atoms::pair(Atom::Save(slot), atom), self.start);
					if !matches!(attempt!(self.sequence(output)), Token::CloseReference) {
						return Err(self.error(ErrorKind::StackError));
					}
					output.emit(Atoms::pair(Atom::Seek(slot), Atom::Skip(width)), self.start);
					self.depth -= 1;
				}
				delimiter => {
					return Ok(delimiter);
				}
			}
		}
	}

	const fn group(&mut self, output: &mut Output<'_>) -> Result<(), PatternError> {
		let save = self.next_save;
		let mut high_water = save;
		let mut fork = output.len;
		let mut last_goto = None;
		output.emit(Atoms { items: [Atom::Nop; 4], len: 4 }, self.start);
		loop {
			let delimiter = attempt!(self.sequence(output));
			if high_water < self.next_save {
				high_water = self.next_save;
			}
			match delimiter {
				Token::Alternative => {
					let goto = output.len;
					// Unpatched gotos form a backwards list inside the output.
					let link = match last_goto {
						Some(previous) => goto - previous,
						None => 0,
					};
					output.emit(Atoms { items: [Atom::Nop; 4], len: 4 }, self.start);
					output.branch(goto, link, false);
					let offset = output.len - fork - 4;
					output.branch(fork, offset, true);
					last_goto = Some(goto);
					fork = output.len;
					output.emit(Atoms { items: [Atom::Nop; 4], len: 4 }, self.start);
					self.next_save = save;
				}
				Token::CloseGroup => {
					break;
				}
				_ => {
					return Err(self.error(ErrorKind::SubPattern));
				}
			}
		}
		self.next_save = high_water;
		if !output.atoms.is_empty() {
			while let Some(goto) = last_goto {
				let link = match output.atoms[goto + 3] {
					Atom::Goto(low) => operand(output.atoms, goto + 3, low),
					_ => panic!("invalid branch patch list"),
				};
				output.branch(goto, output.len - goto - 4, false);
				last_goto = if link == 0 { None } else { Some(goto - link) };
			}
		}
		Ok(())
	}
}

const fn hex(byte: u8) -> u8 {
	match byte {
		b'0'..=b'9' => byte - b'0',
		b'a'..=b'f' => byte - b'a' + 10,
		b'A'..=b'F' => byte - b'A' + 10,
		_ => 255,
	}
}

const fn compile_located(source: &str, atoms: &mut [Atom], locate: Option<usize>) -> Result<(usize, usize), PatternError> {
	let mut parser = Parser::new(source);
	let mut output = Output { atoms, len: 0, locate, position: 0 };
	output.emit(Atoms::one(Atom::Save(0)), 0);
	match attempt!(parser.sequence(&mut output)) {
		Token::End => Ok((output.len, output.position)),
		Token::CloseReference => Err(parser.error(ErrorKind::StackError)),
		_ => Err(parser.error(ErrorKind::SubPattern)),
	}
}

pub const fn compile(source: &str, atoms: &mut [Atom]) -> Result<usize, PatternError> {
	Ok(attempt!(compile_located(source, atoms, None)).0)
}

pub const fn validate(source: &str, atoms: &[Atom], states: &mut [analysis::State]) -> Result<(), PatternError> {
	match analysis::analyze(atoms, states) {
		Ok(()) => Ok(()),
		Err(error) => {
			// Replay emission only on error, avoiding a source-map allocation for
			// every valid pattern. Counting reproduces the exact atom indices.
			let (_, position) = attempt!(compile_located(source, &mut [], Some(error.position)));
			Err(PatternError { kind: error.kind, position })
		},
	}
}

/// Parse and validate a pattern, reporting syntax and scratch-flow errors at byte offsets.
pub fn parse(source: &str) -> Result<Pattern, PatternError> {
	let len = compile(source, &mut [])?;
	let mut atoms = alloc::vec![Atom::Nop; len];
	compile(source, &mut atoms)?;
	validate(source, &atoms, &mut alloc::vec![analysis::State::EMPTY; len])?;
	Ok(atoms)
}

/// Return the exact compiled length.
///
/// Panics on invalid syntax, including slot and nesting overflow.
pub const fn parse_len(source: &str) -> usize {
	match compile(source, &mut []) {
		Ok(len) => len,
		Err(error) => panic!("{}", error.kind.to_str()),
	}
}

/// Compile and validate a pattern into an exactly sized array.
///
/// Panics on syntax or scratch-flow errors, or if `N` differs from [`parse_len`].
///
/// ```
/// use pelite::pattern::{Atom, parse_const, parse_len};
/// const SOURCE: &str = "A0/F8 skip(4) u4";
/// const PATTERN: [Atom; parse_len(SOURCE)] = parse_const(SOURCE);
/// ```
///
/// Scratch-flow errors also fail constant evaluation:
///
/// ```compile_fail
/// use pelite::pattern::{Atom, parse_const, parse_len};
/// const SOURCE: &str = "(AA u1[1] FF | BB) =u1[1]";
/// const PATTERN: [Atom; parse_len(SOURCE)] = parse_const(SOURCE);
/// ```
pub const fn parse_const<const N: usize>(source: &str) -> [Atom; N] {
	if parse_len(source) != N {
		panic!("pattern output length mismatch");
	}
	let mut atoms = [Atom::Nop; N];
	match compile(source, &mut atoms) {
		Ok(_) => match validate(source, &atoms, &mut [analysis::State::EMPTY; N]) {
			Ok(()) => atoms,
			Err(error) => panic!("{}", error.kind.to_str()),
		},
		Err(error) => panic!("{}", error.kind.to_str()),
	}
}
