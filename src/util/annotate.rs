use std::ops::Range;

pub trait Annotate {
	fn a_struct(&mut self, range: Range<u32>, name: &str);
	fn a_ref(&mut self, range: Range<u32>, dest: u32);
}
