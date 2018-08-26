/*!
Find patterns utility.
 */

use crate::{pattern as pat, PeFile};

pub fn main(pefile: PeFile<'_>, pattern: &[pat::Atom]) -> Vec<[u32; 16]> {
	let mut results = Vec::new();
	let mut save = [0; 16];
	match pefile {
		PeFile::Pe32(pe32) => {
			use crate::pe32::*;
			let mut matches = pe32.scanner().matches_code(&pattern);
			while matches.next(&mut save) {
				results.push(save);
			}
		},
		PeFile::Pe64(pe64) => {
			use crate::pe64::*;
			let mut matches = pe64.scanner().matches_code(&pattern);
			while matches.next(&mut save) {
				results.push(save);
			}
		},
	}
	results
}
