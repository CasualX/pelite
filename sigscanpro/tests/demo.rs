
#[test]
fn demo64() {
	static EXPECTED: &[u32] = &[
		0x14B5,
		0x153A,
		0x16A0,
		0x1772,
		0x1779,
		0x18A0,
		0x18E4,
		0x1A24,
		0x1B99,
		0x1ED7,
		0x1F8E,
		0x1FB9,
	];

	let file = std::fs::read("../demo/Demo64.dll").unwrap();
	let file = file.as_slice();

	unsafe {
		let pefile = sigscanpro::pe64::PeliteFile64(file.as_ptr(), file.len());

		let pat = sigscanpro::PelitePatternParse("E9 ? ? ? ?\0".as_ptr() as *const i8);

		let mut matches = sigscanpro::pe64::PeliteFile64Matches(&pefile, &pat, ".text\0".as_ptr() as *const i8);

		let mut save = [0u32; 4];
		let mut i = 0;
		while sigscanpro::pe64::PeliteFile64MatchesNext(&mut matches, save.as_mut_ptr() as *mut u32, save.len()) {
			assert_eq!(EXPECTED[i], save[0]);
			i += 1;
		}
		assert_eq!(i, EXPECTED.len());

		sigscanpro::PelitePatternFree(pat);
	}
}

#[test]
fn demo32() {
	static EXPECTED: &[u32] = &[
		0x14AF,
		0x15C0,
		0x168B,
		0x1692,
		0x1EE0,
	];

	let file = std::fs::read("../demo/Demo.dll").unwrap();
	let file = file.as_slice();

	unsafe {
		let pefile = sigscanpro::pe32::PeliteFile32(file.as_ptr(), file.len());

		let pat = sigscanpro::PelitePatternParse("E9 ? ? ? ?\0".as_ptr() as *const i8);

		let mut matches = sigscanpro::pe32::PeliteFile32Matches(&pefile, &pat, ".text\0".as_ptr() as *const i8);

		let mut save = [0u32; 4];
		let mut i = 0;
		while sigscanpro::pe32::PeliteFile32MatchesNext(&mut matches, save.as_mut_ptr() as *mut u32, save.len()) {
			assert_eq!(EXPECTED[i], save[0]);
			i += 1;
		}
		assert_eq!(i, EXPECTED.len());

		sigscanpro::PelitePatternFree(pat);
	}
}
