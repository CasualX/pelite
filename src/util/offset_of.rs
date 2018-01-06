
#[allow(unused)]
macro_rules! offset_of {
	($ty:ty, $($field:tt)+) => ({
		use std::mem;
		let inst: $ty = unsafe { mem::uninitialized() };

		let base = &inst as *const _ as usize;
		let member = &inst.$($field)* as *const _ as usize;
		let offset = member - base;

		mem::forget(inst);

		offset
	});
}

#[test]
fn units() {
	use image;
	assert_eq!(0x3C, offset_of!(image::IMAGE_DOS_HEADER,e_lfanew))
}
