extern crate pelite;

use pelite::pe32::{Pe, PeFile};
use pelite::FileMap;

#[test]
fn main() {
	let image = FileMap::open(r"tests\samples\pe-Windows-ARMv7-Thumb2LE-HelloWorld").unwrap();
	let file = PeFile::from_bytes(&image).unwrap();
	assert_eq!(file.file_header().Machine, pelite::image::IMAGE_FILE_MACHINE_ARMNT);
}
