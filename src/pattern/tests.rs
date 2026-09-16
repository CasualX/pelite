use super::*;
use Atom::*;

#[test]
fn all_signed_slots_and_read_instructions() {
	for value in 0..=255u8 {
		let slot = value as i8;
		for reader in [Seek, Check, Pir, TestI8, TestU8, TestI16, TestU16, TestU32] {
			assert_eq!(
				analysis::analyze(&[reader(slot)], &mut [analysis::State::EMPTY]),
				Err(PatternError {
					kind: ErrorKind::UninitializedSlot,
					position: 0
				})
			);
			for writer in [Save, Zero, ReadI8, ReadU8, ReadI16, ReadU16, ReadU32] {
				assert_eq!(analysis::analyze(&[writer(slot), reader(slot)], &mut [analysis::State::EMPTY; 2]), Ok(()));
			}
		}
	}
	assert_eq!(
		analysis::analyze(&[Save(127), Seek(-1)], &mut [analysis::State::EMPTY; 2]),
		Err(PatternError {
			kind: ErrorKind::UninitializedSlot,
			position: 1
		})
	);
	assert_eq!(
		analysis::analyze(&[Save(0), Seek(-128)], &mut [analysis::State::EMPTY; 2]),
		Err(PatternError {
			kind: ErrorKind::UninitializedSlot,
			position: 1
		})
	);
}

#[test]
fn unreachable_reads_and_workspace_reuse() {
	let mut states = [analysis::State::EMPTY; 2];
	assert_eq!(analysis::analyze(&[Goto(1), Seek(1)], &mut states), Ok(()));
	assert_eq!(analysis::analyze(&[Save(1), Seek(1)], &mut states), Ok(()));
	assert_eq!(
		analysis::analyze(&[Nop, Seek(1)], &mut states),
		Err(PatternError {
			kind: ErrorKind::UninitializedSlot,
			position: 1
		})
	);
	assert_eq!(analysis::analyze(&[], &mut []), Ok(()));
}

#[test]
#[rustfmt::skip]
fn lowering() {
	assert_eq!(parse("\"string\"\"quote\"\"string\"").unwrap(), [
		Save(0), Byte(b's'), Byte(b't'), Byte(b'r'), Byte(b'i'), Byte(b'n'), Byte(b'g'), Byte(b'"'), Byte(b'q'), Byte(b'u'), Byte(b'o'), Byte(b't'), Byte(b'e'), Byte(b'"'), Byte(b's'), Byte(b't'), Byte(b'r'), Byte(b'i'), Byte(b'n'), Byte(b'g')]);
	assert_eq!(parse("\"\" \"\" \"\"\"\"").unwrap(), [
		Save(0), Byte(b'"')]);
	assert_eq!(parse("80FF? ?? A0/F8 \"é\"00//comment\n@2align(12)").unwrap(), [
		Save(0), Byte(0x80), Byte(0xff), Skip(3), Fuzzy(0xf8), Byte(0xa0), Byte(0xc3), Byte(0xa9), Byte(0), IsAlign(2), IsAlign(12)]);
	assert_eq!(parse("skip(0)skip(-0)scan(0)skip(ptr)skip(-ptr)skip(255)skip(-256)skip(0xffff)skip(0x01020304)scan()scan(1)scan(255)scan(65534)").unwrap(), [
		Save(0), Skip(0), Rewind(0), Skip(255), Extend(1), Rewind(0), Extend(255), Skip(255), Extend(1), Extend(2), Extend(3), Skip(4), Scan(0), Scan(1), Scan(255), Extend(255), Scan(254)]);
	assert_eq!(parse("skip(4294967295)scan(0xffffffff)").unwrap(), [
		Save(0), Extend(255), Extend(255), Extend(255), Skip(255), Extend(255), Extend(255), Extend(255), Scan(255)]);
	assert_eq!(parse("save[1]u4[2]zero[5]'check[1]save[7]AAcheck[0x7]BBseek[2]save[3]").unwrap(), [
		Save(0), Save(1), ReadU32(2), Zero(5), Save(6), Check(1), Save(7), Byte(0xaa), Check(7), Byte(0xbb), Seek(2), Save(3)]);
	assert_eq!(parse("i1[1]u1[2]i2[3]u2[4]i4[5]u4[6]=i1[1]=u1[2]=i2[3]=u2[4]=i4[5]=u4[6]").unwrap(), [
		Save(0), ReadI8(1), ReadU8(2), ReadI16(3), ReadU16(4), ReadU32(5), ReadU32(6), TestI8(1), TestU8(2), TestI16(3), TestU16(4), TestU32(5), TestU32(6)]);
	assert_eq!(parse("$ % * ${%{'}AA} *{}BB").unwrap(), [
		Save(0), Jump4, Jump1, Ptr, Save(-1), Jump4, Save(-2), Jump1, Save(1), Seek(-2), Skip(1), Byte(0xaa), Seek(-1), Skip(4), Save(-3), Ptr, Seek(-3), Skip(0), Byte(0xbb)]);
	assert_eq!(parse("rel32 rel8 ptr rel32{rel8{save}AA} ptr{}BB").unwrap(), [
		Save(0), Jump4, Jump1, Ptr, Save(-1), Jump4, Save(-2), Jump1, Save(1), Seek(-2), Skip(1), Byte(0xaa), Seek(-1), Skip(4), Save(-3), Ptr, Seek(-3), Skip(0), Byte(0xbb)]);
	assert_eq!(parse("save save[3] save").unwrap(), [
		Save(0), Save(1), Save(3), Save(4)]);
	assert_eq!(parse("check[0] save zero[7] save").unwrap(), [
		Save(0), Check(0), Save(1), Zero(7), Save(8)]);
	assert_eq!(parse("(AA|BBCC)DD").unwrap(), [
		Save(0), Nop, Nop, Nop, Fork(5), Byte(0xaa), Nop, Nop, Nop, Goto(6), Nop, Nop, Nop, Nop, Byte(0xbb), Byte(0xcc), Byte(0xdd)]);
}

#[test]
fn const_runtime_parity() {
	macro_rules! cases {
		($($source:expr),* $(,)?) => { &[$(($source, &parse_const::<{ parse_len($source) }>($source) as &[Atom])),*] };
	}
	const CASES: &[(&str, &[Atom])] = cases![
		"",
		" \t\r\n// EOF",
		"?",
		"skip(0)",
		"scan(0)",
		"scan()",
		"@0@9align(0)align(31)",
		"00aAfF",
		"80FFA0/F8",
		"\"\"",
		"\"double \"\"quote\"\"\"",
		"\"left\" \"right\"",
		"\"é😀//${\"00",
		"AA// comment\nBB",
	];
	for &(source, expected) in CASES {
		assert_eq!(parse(source).unwrap(), expected, "{source}");
		assert_eq!(parse_len(source), expected.len(), "{source}");
	}
	const STRUCTURES: &[(&str, &[Atom])] = cases![
		"E8${'}4885C0",
		"488D0D${\"hello\"00}",
		"C705${skip(4)'}01000000",
		"AA skip(4) scan(12) BB",
		"(6Ai1[1]|68i4[1])=i4[1]",
		"u4(AA|BB)",
		"(AA|BBCC)DD",
		"(AA|AABB)CC",
		"(E8${save[1]}|zero[1]90)C3",
		"(AA|)BB",
		"(|AA|)",
		"((11|22)|(33|44))",
		"(save[3]|(u4|'')|zero[6])'",
		"()",
		"${}",
		"$ // reference body\n { % { ' } }",
		"skip(-0x100) scan(0xff) u4[1] =u4[1]",
	];
	for &(source, expected) in STRUCTURES {
		assert_eq!(parse(source).unwrap(), expected, "{source}");
		assert_eq!(parse_len(source), expected.len(), "{source}");
	}
}

#[test]
fn branch_allocation() {
	let atoms = parse("(save[3]|(u4|'')|zero[6])'").unwrap();
	let stores: Pattern = atoms.into_iter().filter(|atom| matches!(atom, Save(_) | ReadU32(_) | Zero(_))).collect();
	assert_eq!(stores, [Save(0), Save(3), ReadU32(1), Save(1), Save(2), Zero(6), Save(7)]);
	assert!(parse("(save[127]|save[127])check[0]").is_ok());
	assert_eq!(parse("(save[127]|)'").unwrap_err().kind, ErrorKind::SaveOverflow);
}

#[test]
fn diagnostics() {
	use ErrorKind::*;
	for (source, kind, position) in [
		("00 A", UnpairedHexDigit, 3),
		("\"escaped \"\" quote", UnclosedQuote, 0),
		("A?", UnpairedHexDigit, 0),
		("AA/F", UnpairedHexDigit, 0),
		("AA/*", UnpairedHexDigit, 0),
		("AA /FF", UnknownChar, 3),
		("\"é\" !", UnknownChar, 5),
		("00 \"é", UnclosedQuote, 3),
		("@A", AlignedOperand, 0),
		("00 @", AlignedOperand, 3),
		("align(32)", AlignedOperand, 0),
		("align (4)", AlignedOperand, 0),
		("skip(99999999999999999999999)", Operand, 0),
		("skip(4294967296)", Operand, 0),
		("scan(0x100000000)", Operand, 0),
		("scan(-1)", Operand, 0),
		("scan(ptr)", Operand, 0),
		("skip()", Operand, 0),
		("skip(0x)", Operand, 0),
		("skip(12x)", Operand, 0),
		("skip(1", Operand, 0),
		("skip (1)", Operand, 0),
		("skip( 1)", Operand, 0),
		("skip(- 1)", Operand, 0),
		("[1-2]", UnknownChar, 0),
		("u8[1]", ReadOperand, 0),
		("=save[1]", ReadOperand, 0),
		("=", ReadOperand, 0),
		("u4 [1]", UnknownChar, 3),
		("= u4[1]", ReadOperand, 0),
		("=i1[']", SlotOperand, 0),
		("u4[']", SlotOperand, 0),
		("save[']", SlotOperand, 0),
		("u1 =u1[1]", SlotConflict, 3),
		("' seek[1]", SlotConflict, 2),
		("(u1|save[1])", SlotConflict, 4),
		("(check[1]|save)", SlotConflict, 10),
		("save[ 1]", SlotOperand, 0),
		("save[128]", SlotOperand, 0),
		("seek[-1]", SlotOperand, 0),
		("u1[]", SlotOperand, 0),
		("u4[1", SlotOperand, 0),
		("(AA", SubPattern, 3),
		("AA|BB", SubPattern, 2),
		("AA)", SubPattern, 2),
		("${", StackError, 2),
		("}", StackError, 0),
		("${AA|BB}", StackError, 4),
		("(${AA|BB})", StackError, 5),
		("${(AA})", SubPattern, 5),
	] {
		let expected = Err(PatternError { kind, position });
		assert_eq!(parser::compile(source, &mut []), expected, "{source}");
		let error = parse(source).unwrap_err();
		assert_eq!(error, expected.unwrap_err(), "{source}");
		assert_eq!(error.position(), position, "{source}");
		// The emitting pass must diagnose exactly the same error as sizing.
		assert_eq!(parser::compile(source, &mut [Nop; 64]), expected, "{source}");
	}
}

#[test]
fn limits() {
	let saves = "'".repeat(127);
	assert_eq!(parse(&saves).unwrap().last(), Some(&Save(127)));
	assert_eq!(parse(&(saves + "'")).unwrap_err().kind, ErrorKind::SaveOverflow);
	let returns = "${}".repeat(128);
	assert!(parse(&returns).unwrap().contains(&Save(-128)));
	assert_eq!(parse(&(returns + "${}")).unwrap_err().kind, ErrorKind::SaveOverflow);
	let nested = alloc::format!("{}{}", "(".repeat(128), ")".repeat(128));
	assert_eq!(parse_len(&nested), parse(&nested).unwrap().len());
	assert_eq!(parse(&alloc::format!("({nested})")).unwrap_err().kind, ErrorKind::NestingOverflow);
}

#[test]
#[should_panic(expected = "save store overflow")]
fn sizing_validates_allocation() {
	parse_len("save[127]'");
}

#[test]
#[should_panic(expected = "sub pattern error")]
fn const_validates_nesting() {
	parse_const::<1>("(");
}

#[test]
#[should_panic(expected = "pattern output length mismatch")]
fn const_requires_exact_size() {
	parse_const::<2>("");
}

#[test]
fn scratch_flow_errors() {
	use ErrorKind::{RetryClobber, UninitializedSlot};
	for (source, token, kind) in [
		("seek[1]", "seek", UninitializedSlot),
		("check[127]", "check", UninitializedSlot),
		("=i1[1]", "=", UninitializedSlot),
		("=u1[1]", "=", UninitializedSlot),
		("=i2[1]", "=", UninitializedSlot),
		("=u2[1]", "=", UninitializedSlot),
		("=i4[1]", "=", UninitializedSlot),
		("=u4[1]", "=", UninitializedSlot),
		("(AA u1[1] FF | BB) =u1[1]", "=", UninitializedSlot),
		("(AA|BB u1[1]) =u1[1]", "=", UninitializedSlot),
		("(AA u1[1]|(BB u1[1]|CC)) =u1[1]", "=", UninitializedSlot),
		("zero[1](AA u1[1] FF|BB)=u1[1]", "=", RetryClobber),
		("zero[1](AA|BB)=u1[1] u1[1] FF", "=", RetryClobber),
		("zero[1]scan(1)=u1[1] u1[1] FF", "=", RetryClobber),
		("zero[1]scan(255)=u1[1] u1[1] FF", "=", RetryClobber),
		("zero[1](scan(1) u1[1] FF|BB)=u1[1]", "=", RetryClobber),
		("zero[1](AA u1[1] FF|((BB|CC)))=u1[1]", "=", RetryClobber),
		("(AA|BB)\"é\" A0/F8 skip(256) ${?} =u4[2]", "=", UninitializedSlot),
	] {
		let expected = PatternError {
			kind,
			position: source.find(token).unwrap(),
		};
		assert_eq!(parse(source).unwrap_err(), expected, "{source}");
		let len = parse_len(source);
		let mut atoms = alloc::vec![Nop; len];
		parser::compile(source, &mut atoms).unwrap();
		assert_eq!(parser::validate(source, &atoms, &mut alloc::vec![analysis::State::EMPTY; len]), Err(expected));
	}
}

#[test]
fn scratch_flow_const_parity() {
	macro_rules! cases {
		($($source:expr),* $(,)?) => { &[$(($source, &parse_const::<{ parse_len($source) }>($source) as &[Atom])),*] };
	}
	const CASES: &[(&str, &[Atom])] = cases![
		"check[0]",
		"save[127]check[127]",
		"(AA u1[1] FF | BB zero[1]) =u1[1]",
		"zero[1](AA u1[1] FF | BB zero[1]) =u1[1]",
		"(AA u1[1]|BB u1[1]) =u1[1]",
		"(AA u1[1]|(BB u1[1]|CC zero[1])) =u1[1]",
		"(AA u1[1]|) zero[1] =u1[1]",
		"zero[1]scan(1) zero[1] =u1[1] u1[1] FF",
		"zero[1] =u1[1] u1[1] FF",
		"zero[1]scan(1) =u1[1] u1[1]",
		"zero[1](AA|BB) =u1[1] u1[1]",
		"zero[1](AA|BB) zero[1] =u1[1] u1[1] FF",
		"%{(AA|AABB)u1[1]}%{AA}=u1[1]",
		"%{scan(1) u1[1]}%{AA}=u1[1]",
		"(u1[1]|) zero[1]",
		"(save[1]|) seek[1]", // The first continuation cannot fail; fallback is unreachable.
	];
	for &(source, expected) in CASES {
		assert_eq!(parse(source).unwrap(), expected, "{source}");
	}
}

#[test]
#[should_panic(expected = "save slot may be read before initialization")]
fn const_rejects_uninitialized_read() {
	const SOURCE: &str = "(AA u1[1] FF|BB)=u1[1]";
	parse_const::<{ parse_len(SOURCE) }>(SOURCE);
}

#[test]
#[should_panic(expected = "save slot may have been overwritten")]
fn const_rejects_retry_clobber() {
	const SOURCE: &str = "zero[1]scan(1)=u1[1]u1[1]FF";
	parse_const::<{ parse_len(SOURCE) }>(SOURCE);
}
