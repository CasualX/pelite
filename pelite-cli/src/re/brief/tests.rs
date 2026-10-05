use super::*;
use std::mem;

fn image(image_base: u64) -> pelite::PeMemory {
	use pelite::pe64::image::*;
	let mut dos: IMAGE_DOS_HEADER = dataview::zeroed();
	dos.e_magic = IMAGE_DOS_SIGNATURE;
	dos.e_lfanew = mem::size_of::<IMAGE_DOS_HEADER>() as u32;
	let mut nt: IMAGE_NT_HEADERS = dataview::zeroed();
	nt.Signature = IMAGE_NT_HEADERS_SIGNATURE;
	nt.FileHeader.NumberOfSections = 1;
	nt.FileHeader.SizeOfOptionalHeader = mem::size_of::<IMAGE_OPTIONAL_HEADER>() as u16;
	nt.OptionalHeader.Magic = IMAGE_NT_OPTIONAL_HDR_MAGIC;
	nt.OptionalHeader.ImageBase = image_base.into();
	nt.OptionalHeader.SectionAlignment = 0x1000;
	nt.OptionalHeader.FileAlignment = 0x200;
	nt.OptionalHeader.SizeOfHeaders = 0x200;
	nt.OptionalHeader.SizeOfImage = 0x3000;
	let mut section: IMAGE_SECTION_HEADER = dataview::zeroed();
	section.VirtualAddress = 0x2000;
	section.VirtualSize = 0x20;
	section.SizeOfRawData = 0x10;
	section.PointerToRawData = 0x200;
	let mut bytes = pelite::PeMemory::zeroed(0x210);
	let mut offset = 0;
	for header in [dataview::bytes(&dos), dataview::bytes(&nt), dataview::bytes(&section)] {
		bytes[offset..offset + header.len()].copy_from_slice(header);
		offset += header.len();
	}
	bytes[0x200..0x204].copy_from_slice(&42u32.to_le_bytes());
	bytes[0x208..0x210].copy_from_slice(&image_base.wrapping_add(0x2010).to_le_bytes());
	bytes
}

fn read_options(zerofill: bool) -> read::ReadOptions {
	read::ReadOptions {
		zerofill,
		max_string_bytes: read::DEFAULT_MAX_STRING_BYTES.parse().unwrap(),
		max_dynamic_array_length: read::DEFAULT_MAX_DYNAMIC_ARRAY_LENGTH.parse().unwrap(),
	}
}

fn collect_report(bytes: &[u8], length: usize, arch: Arch, rva: u32, image_base: u64, symbols: &HashMap<u64, symbols::IndexedSymbol>) -> Result<Brief> {
	let mut state = CollectionState::default();
	collect(&mut state, bytes, length, arch, rva, image_base)?;
	let image = image(image_base);
	let pe = pelite::PeFile::from_bytes(&image)?;
	let facts = symbols::IndexedFacts { symbols: symbols.clone(), ..Default::default() };
	Ok(state.finalize(pe, &facts, &read_options(false)))
}

#[test]
fn lists_symbols_constants_and_displacements_once_in_first_use_order() {
	let base = 0x180000000;
	let facts = factmap::FactMap::parse("#factmap\nSx1000 code \"start\"\nSx2000 u32 D\nSx3000 code _\n", ty::PointerWidth::Bits64).unwrap();
	let mut indexed = symbols::IndexedFacts::default();
	indexed.extend(facts.facts, base);
	let bytes = [
		0xe8, 0xfb, 0xff, 0xff, 0xff, // call start
		0x8b, 0x05, 0xf5, 0x0f, 0, 0, // mov eax,[rip+0xff5]: data_2000
		0x48, 0x8b, 0x44, 0x8b, 0xe0, // mov rax,[rbx+rcx*4-0x20]
		0x48, 0xb8, 0, 0x20, 0, 0x80, 1, 0, 0, 0, // immediate stays a constant even when matching a symbol
		0x48, 0xb8, 0, 0x30, 0, 0x80, 1, 0, 0, 0,
		0xb8, 42, 0, 0, 0, // ordinary immediate
		0x48, 0x8b, 0x03, // no displacement
		0xe8, 0xcf, 0xff, 0xff, 0xff, // call start again
	];
	let report = collect_report(&bytes, bytes.len(), Arch::X86_64, 0x1000, base, &indexed.symbols).unwrap();
	assert_eq!(report.references, [Reference::Symbol("start".into()), Reference::Symbol("data_2000".into())]);
	assert_eq!(report.constants.immediates, [0x180002000, 0x180003000, 42]);
	assert_eq!(report.constants.displacements, [-32]);
}

#[test]
fn collects_all_immediate_widths_and_signed_extended_values() {
	let bytes = [
		0xb0, 0xff, // imm8
		0x66, 0xb8, 0x34, 0x12, // imm16
		0xb8, 0x78, 0x56, 0x34, 0x12, // imm32
		0x48, 0xb8, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, // imm64
		0x66, 0x83, 0xc0, 0xfe, // imm8 sign-extended to 16 bits
		0x83, 0xc0, 0xfd, // imm8 sign-extended to 32 bits
		0x48, 0x83, 0xc0, 0xfc, // imm8 sign-extended to 64 bits
		0x48, 0xc7, 0xc0, 0xfb, 0xff, 0xff, 0xff, // imm32 sign-extended to 64 bits
		0xc8, 0x20, 0, 3, // enter: two immediates
		0xd1, 0xe0, // shl eax,1: implicit immediate
	];
	let report = collect_report(&bytes, bytes.len(), Arch::X86_64, 0x1000, 0, &HashMap::new()).unwrap();
	assert!(report.references.is_empty());
	assert_eq!(report.constants.immediates, [255, 0x1234, 0x12345678, 0x1122334455667788, -2, -3, -4, -5, 32, 3, 1]);
}

#[test]
fn reports_addresses_segment_displacements_and_explicit_zero_displacements() {
	let bytes = [
		0x8b, 0x05, 0xfa, 0xff, 0xff, 0xff, // [rip-6]
		0x65, 0x48, 0x8b, 0x04, 0x25, 0x60, 0, 0, 0, // gs:[0x60]
		0x48, 0x8b, 0x43, 0, // [rbx+0]
	];
	let report = collect_report(&bytes, bytes.len(), Arch::X86_64, 0x1000, 0x180000000, &HashMap::new()).unwrap();
	assert_eq!(report.references, [Reference::Rva(0x1000)]);
	assert_eq!(report.constants.displacements, [0x60, 0]);
	assert_eq!(collect_report(&[0xa1, 0, 0x20, 0x40, 0], 5, Arch::X86_32, 0x1000, 0x400000, &HashMap::new()).unwrap().references, [Reference::Rva(0x2000)]);
}

#[test]
fn evex_displacements_use_the_effective_scaled_offset() {
	// vmovdqu64 zmm0,[rax-0x40]: encoded disp8=-1, tuple scale=64.
	assert_eq!(collect_report(&[0x62, 0xf1, 0xfe, 0x48, 0x6f, 0x40, 0xff], 7, Arch::X86_64, 0x1000, 0, &HashMap::new()).unwrap().constants.displacements, [-64]);
}

#[test]
fn unresolved_references_are_signed_rvas_even_across_address_wraparound() {
	// [rip-0x1007] at RVA 0x1000 targets RVA -1.
	let bytes = [0x8b, 0x05, 0xf9, 0xef, 0xff, 0xff];
	for base in [0, 0x180000000, 0x8000000000000000, u64::MAX - 0x800] {
		let symbols = HashMap::from([(base.wrapping_sub(1), symbols::IndexedSymbol::Weak(0))]);
		let report = collect_report(&bytes, bytes.len(), Arch::X86_64, 0x1000, base, &symbols).unwrap();
		assert_eq!(report.references, [Reference::Rva(-1)]);
		assert_eq!(serde_json::to_value(report).unwrap(), serde_json::json!({"references": [-1], "values": {}, "indirect_branches": 0, "constants": {"immediates": [], "comparisons": [], "displacements": []}}));
	}
}

#[test]
fn constants_preserve_operand_context_and_use_i64_casts() {
	let bytes = [
		0x48, 0x8b, 0x43, 8, // displacement 8
		0xb8, 8, 0, 0, 0, // same value as an immediate
		0x48, 0xb8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // u64::MAX
	];
	let report = collect_report(&bytes, bytes.len(), Arch::X86_64, 0x1000, 0, &HashMap::new()).unwrap();
	assert_eq!(report.constants.immediates, [8, -1]);
	assert_eq!(report.constants.displacements, [8]);
	assert!(report.references.is_empty());
}

#[test]
fn hides_internal_jumps_but_keeps_calls_external_jumps_and_data_references() {
	let bytes = [
		0xeb, 0xfe, // jump to start: hidden
		0x75, 0xfc, // conditional jump to start: hidden
		0xe8, 0xf7, 0xff, 0xff, 0xff, // call start: retained
		0xeb, 0, // jump to offset 11, excluded range endpoint: retained
		0x8b, 0x05, 0xef, 0xff, 0xff, 0xff, // data at start: retained
	];
	let report = collect_report(&bytes, 11, Arch::X86_64, 0x1000, 0x180000000, &HashMap::new()).unwrap();
	assert_eq!(report.references, [Reference::Rva(0x1000), Reference::Rva(0x100b)]);
	// With the memory instruction included, internal data still contributes a reference.
	let report = collect_report(&bytes, bytes.len(), Arch::X86_64, 0x1000, 0x180000000, &HashMap::new()).unwrap();
	assert_eq!(report.references, [Reference::Rva(0x1000)]);
	let report = collect_report(&bytes[11..], 6, Arch::X86_64, 0x100b, 0x180000000, &HashMap::new()).unwrap();
	assert_eq!(report.references, [Reference::Rva(0x1000)]);
}

#[test]
fn filters_stack_displacements_and_adjustments_but_keeps_stored_immediates() {
	let bytes = [
		0x48, 0x83, 0xec, 0x38, // sub rsp,56
		0x48, 0xc7, 0x44, 0x24, 0x20, 0x80, 0, 0, 0, // mov [rsp+32],128
		0x48, 0x8d, 0x44, 0x24, 0x18, // lea rax,[rsp+24]
		0x83, 0xc4, 0x10, // add esp,16
		0x66, 0x83, 0xc4, 0x10, // add sp,16
		0x68, 0xf5, 0, 0, 0, // push 0xf5: keep its immediate despite implicit rsp use
		0xc2, 8, 0, // ret 8: implicit rsp use
		0xc8, 0x20, 0, 3, // enter: implicit rsp use
		0x48, 0x8b, 0x45, 0xe0, // mov rax,[rbp-32]: keep offset
		0xb8, 0x80, 0, 0, 0, // mov eax,128: keep same value when used elsewhere
		0x48, 0x3b, 0x25, 0, 0, 0, 0, // cmp rsp,[rip]: retain address reference
	];
	let report = collect_report(&bytes, bytes.len(), Arch::X86_64, 0x1000, 0, &HashMap::new()).unwrap();
	assert_eq!(report.constants.immediates, [128, 0xf5, 8, 32, 3]);
	assert_eq!(report.constants.displacements, [-32]);
	assert!(report.constants.comparisons.is_empty());
	assert_eq!(report.references.len(), 1);
}

#[test]
fn annotates_comparisons_and_deduplicates_within_each_category() {
	let bytes = [
		0x83, 0xf8, 0x2a, // cmp eax,42
		0xa9, 0x2a, 0, 0, 0, // test eax,42
		0xb8, 0x2a, 0, 0, 0, // mov eax,42
		0xb9, 0x2a, 0, 0, 0, // mov ecx,42
		0x8b, 0x43, 0x2a, // mov eax,[rbx+42]
		0x8b, 0x4b, 0x2a, // mov ecx,[rbx+42]
	];
	let report = collect_report(&bytes, bytes.len(), Arch::X86_64, 0x1000, 0, &HashMap::new()).unwrap();
	assert_eq!(report.constants.immediates, [42]);
	assert_eq!(report.constants.comparisons, [42]);
	assert_eq!(report.constants.displacements, [42]);
}

#[test]
fn frame_setup_filters_rbp_for_the_rest_of_the_range() {
	let bytes = [
		0x48, 0x8b, 0x45, 0xe0, // [rbp-32] before setup: retain
		0x48, 0x89, 0xe5, // mov rbp,rsp
		0x48, 0xc7, 0x45, 0xf8, 0x80, 0, 0, 0, // [rbp-8]=128: keep immediate
		0x48, 0x83, 0x7d, 0xf0, 0x2a, // cmp [rbp-16],42: keep comparison
		0x68, 0xf5, 0, 0, 0, // push 245: retain
		0xe8, 0, 0, 0, 0, // call: preserve frame hint
		0x48, 0x8b, 0x45, 0xe8, // [rbp-24]: still suppress
		0xbd, 0x34, 0x12, 0, 0, // mov ebp,0x1234: frame hint remains
		0x48, 0x8b, 0x45, 0xf8, // [rbp-8]: still suppress
	];
	let report = collect_report(&bytes, bytes.len(), Arch::X86_64, 0x1000, 0, &HashMap::new()).unwrap();
	assert_eq!(report.constants.immediates, [128, 245]);
	assert_eq!(report.constants.displacements, [-32]);
	assert_eq!(report.constants.comparisons, [42]);
}

#[test]
fn recognizes_native_width_frame_setup_and_stack_relative_lea() {
	for (arch, bytes) in [
		(Arch::X86_64, &[0x48, 0x8b, 0xec, 0x48, 0x8b, 0x45, 0xf8][..]), // mov rbp,rsp
		(Arch::X86_32, &[0x89, 0xe5, 0x8b, 0x45, 0xf8][..]), // mov ebp,esp
		(Arch::X86_16, &[0x89, 0xe5, 0x8b, 0x46, 0xf8][..]), // mov bp,sp
		(Arch::X86_64, &[0x48, 0x8d, 0x6c, 0x24, 0x40, 0x48, 0x8b, 0x45, 0xf8][..]), // lea rbp,[rsp+64]
	] {
		let report = collect_report(bytes, bytes.len(), arch, 0x1000, 0, &HashMap::new()).unwrap();
		assert!(report.constants.displacements.is_empty(), "arch={arch:?}, bytes={bytes:x?}");
	}
	// A truncated stack pointer copy in 64-bit mode does not establish a frame.
	let bytes = [0x89, 0xe5, 0x48, 0x8b, 0x45, 0xf8];
	assert_eq!(collect_report(&bytes, bytes.len(), Arch::X86_64, 0x1000, 0, &HashMap::new()).unwrap().constants.displacements, [-8]);
}

#[test]
fn early_epilogues_and_frame_register_overwrites_preserve_the_hint() {
	for overwrite in [&[0x5d, 0xc3][..], &[0x40, 0xb5, 0][..], &[0x48, 0x0f, 0x44, 0xe8][..]] { // pop rbp; ret; mov bpl,0; cmove rbp,rax
		let mut bytes = vec![0x48, 0x89, 0xe5];
		bytes.extend_from_slice(overwrite);
		bytes.extend_from_slice(&[0x48, 0x8b, 0x45, 0xf8]);
		assert!(collect_report(&bytes, bytes.len(), Arch::X86_64, 0x1000, 0, &HashMap::new()).unwrap().constants.displacements.is_empty());
	}
}

#[test]
fn counts_indirect_branches_through_registers_and_memory() {
	let bytes = [
		0xff, 0xd0, // call rax
		0xff, 0xd0, // call rax again: count both
		0xff, 0x53, 8, // call [rbx+8]
		0xff, 0x15, 0, 0, 0, 0, // call [rip]: named import slot
		0xff, 0x54, 0x24, 8, // call [rsp+8]
		0xe8, 0, 0, 0, 0, // direct call: excluded
		0xff, 0xe0, // jmp rax: indirect tail call or dispatch
		0xff, 0x63, 8, // jmp [rbx+8]
		0xeb, 0, // direct jump: excluded
	];
	let symbols = HashMap::from([(0x100d, symbols::IndexedSymbol::Named("__imp_Test".into()))]);
	let report = collect_report(&bytes, bytes.len(), Arch::X86_64, 0x1000, 0, &symbols).unwrap();
	assert_eq!(report.indirect_branches, 7);
	assert_eq!(report.references[0], Reference::Symbol("__imp_Test".into()));
	assert_eq!(report.constants.displacements, [8]);
	// Count by instruction start, even when the final call crosses the boundary.
	assert_eq!(collect_report(&bytes, 1, Arch::X86_64, 0x1000, 0, &symbols).unwrap().indirect_branches, 1);
	assert_eq!(collect_report(&bytes, 3, Arch::X86_64, 0x1000, 0, &symbols).unwrap().indirect_branches, 2);
}

#[test]
fn counts_stack_indirect_jump_without_reporting_its_displacement() {
	let bytes = [0xff, 0x64, 0x24, 0xc0]; // jmp [rsp-0x40]
	let report = collect_report(&bytes, bytes.len(), Arch::X86_64, 0x1000, 0, &HashMap::new()).unwrap();
	assert_eq!(report.indirect_branches, 1);
	assert!(report.constants.displacements.is_empty());
	assert!(report.references.is_empty());
}

#[test]
fn finalize_reads_typed_symbols_and_respects_zerofill() {
	let base = 0x180000000;
	let image = image(base);
	let pe = pelite::PeFile::from_bytes(&image).unwrap();
	let facts = factmap::FactMap::parse("#factmap\nSx2000 u32 \"scalar\"\nSx2008 *u32 \"pointer\"\nSx2010 u32 \"zero\"\nSx2014 code \"opaque\"\nSx2018 u32 _\n", ty::PointerWidth::Bits64).unwrap();
	let mut indexed = symbols::IndexedFacts::default();
	indexed.extend(facts.facts, base);
	for zerofill in [false, true] {
		let mut state = CollectionState::default();
		for rva in [0x2000, 0x2008, 0x2010, 0x2014, 0x2018, 0x201c] {
			state.add_reference(base + rva);
		}
		let report = state.finalize(pe, &indexed, &read_options(zerofill));
		assert_eq!(report.references, [
			Reference::Symbol("scalar".into()), Reference::Symbol("pointer".into()),
			Reference::Symbol("zero".into()), Reference::Symbol("opaque".into()),
			Reference::Rva(0x2018), Reference::Rva(0x201c),
		]);
		assert_eq!(report.values.len(), 3);
		assert_eq!(report.values["scalar"], serde_json::json!(42));
		for name in ["pointer", "zero"] {
			if zerofill {
				assert_eq!(report.values[name], serde_json::json!(0));
			}
			else {
				assert!(read::contains_read_errors(&report.values[name]));
				assert_eq!(report.values[name]["$address"], serde_json::json!(0x2010));
			}
		}
	}
}
