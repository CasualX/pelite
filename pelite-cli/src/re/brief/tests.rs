use super::*;
use std::mem;

fn image(image_base: u64) -> pelite::PeMemory {
	use pelite::pe64::image;
	let mut dos: image::IMAGE_DOS_HEADER = dataview::zeroed();
	dos.e_magic = image::IMAGE_DOS_SIGNATURE;
	dos.e_lfanew = mem::size_of::<image::IMAGE_DOS_HEADER>() as u32;
	let mut nt: image::IMAGE_NT_HEADERS = dataview::zeroed();
	nt.Signature = image::IMAGE_NT_HEADERS_SIGNATURE;
	nt.FileHeader.NumberOfSections = 1;
	nt.FileHeader.SizeOfOptionalHeader = mem::size_of::<image::IMAGE_OPTIONAL_HEADER>() as u16;
	nt.OptionalHeader.Magic = image::IMAGE_NT_OPTIONAL_HDR_MAGIC;
	nt.OptionalHeader.ImageBase = image_base.into();
	nt.OptionalHeader.SectionAlignment = 0x1000;
	nt.OptionalHeader.FileAlignment = 0x200;
	nt.OptionalHeader.SizeOfHeaders = 0x200;
	nt.OptionalHeader.SizeOfImage = 0x3000;
	let mut section: image::IMAGE_SECTION_HEADER = dataview::zeroed();
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
		string_preview_length: read::DEFAULT_STRING_PREVIEW_LENGTH.parse().unwrap(),
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
		assert_eq!(serde_json::to_value(report).unwrap(), serde_json::json!({"references": [-1], "values": {}, "instructions": {"encodings": ["Legacy"], "cpuid_features": [], "segment_overrides": [], "register_classes": ["GPR"], "privileged": false}, "control_flow": {"direct_jumps": 0, "conditional_branches": 0, "indirect_branches": 0, "internal_targets": 0, "external_branches": 0, "leaders": 1, "returns": 0, "return_pop": []}, "constants": {"immediates": [], "comparisons": [], "displacements": []}}));
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
	assert_eq!(report.control_flow.indirect_branches, 7);
	assert_eq!(report.references[0], Reference::Symbol("__imp_Test".into()));
	assert_eq!(report.constants.displacements, [8]);
	// Count by instruction start, even when the final call crosses the boundary.
	assert_eq!(collect_report(&bytes, 1, Arch::X86_64, 0x1000, 0, &symbols).unwrap().control_flow.indirect_branches, 1);
	assert_eq!(collect_report(&bytes, 3, Arch::X86_64, 0x1000, 0, &symbols).unwrap().control_flow.indirect_branches, 2);
}

#[test]
fn counts_stack_indirect_jump_without_reporting_its_displacement() {
	let bytes = [0xff, 0x64, 0x24, 0xc0]; // jmp [rsp-0x40]
	let report = collect_report(&bytes, bytes.len(), Arch::X86_64, 0x1000, 0, &HashMap::new()).unwrap();
	assert_eq!(report.control_flow.indirect_branches, 1);
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
				assert_eq!(report.values[name]["$rva"], serde_json::json!(0x2010));
			}
		}
	}
}

#[test]
fn summarizes_control_flow_with_unique_targets_and_leaders() {
	let bytes = [
		0xeb, 0xfe, // 0: jmp entry
		0x75, 0xfc, // 2: jne entry; fallthrough 4
		0xe8, 8, 0, 0, 0, // 4: call 17 (excluded from targets and leaders)
		0xeb, 0x7f, // 9: external jump
		0x74, 0x7f, // 11: external conditional; fallthrough 13
		0x75, 0, // 13: jne 15 (same target and fallthrough)
		0xff, 0xe0, // 15: jmp rax
		0xc3, // 17: ret; scanning continues
		0xc2, 12, 0, // 18: ret 12
	];
	for arch in [Arch::X86_32, Arch::X86_64] {
		let report = collect_report(&bytes, bytes.len(), arch, 0x1000, 0, &HashMap::new()).unwrap();
		assert_eq!(serde_json::to_value(report.control_flow).unwrap(), serde_json::json!({
			"direct_jumps": 5, "conditional_branches": 3, "indirect_branches": 1,
			"internal_targets": 2, "external_branches": 2, "leaders": 4,
			"returns": 2, "return_pop": [0, 12],
		}));
	}
}

#[test]
fn counts_loop_and_counter_zero_branches_in_all_modes() {
	let bytes = [
		0xe0, 0xfe, // loopne 0
		0xe1, 0xfc, // loope 0
		0xe2, 0xfa, // loop 0
		0xe3, 0xf8, // jcxz/jecxz/jrcxz 0; fallthrough at range end excluded
	];
	for arch in [Arch::X86_16, Arch::X86_32, Arch::X86_64] {
		let report = collect_report(&bytes, bytes.len(), arch, 0x1000, 0, &HashMap::new()).unwrap();
		assert_eq!(serde_json::to_value(report.control_flow).unwrap(), serde_json::json!({
			"direct_jumps": 4, "conditional_branches": 4, "indirect_branches": 0,
			"internal_targets": 1, "external_branches": 0, "leaders": 4,
			"returns": 0, "return_pop": [],
		}));
	}
}

#[test]
fn control_flow_respects_range_boundaries_and_address_wraparound() {
	let bytes = [0x75, 0, 0xc3]; // jne offset 2; ret
	for base in [0, 0x180000000, u64::MAX - 0x1000] {
		for length in [1, 2, 3] {
			let report = collect_report(&bytes, length, Arch::X86_64, 0x1000, base, &HashMap::new()).unwrap();
			let internal = usize::from(length == 3);
			assert_eq!(report.control_flow.direct_jumps, 1);
			assert_eq!(report.control_flow.internal_targets, internal);
			assert_eq!(report.control_flow.external_branches, 1 - internal);
			assert_eq!(report.control_flow.leaders, 1 + internal);
			assert_eq!(report.control_flow.returns, internal);
		}
	}
	let report = collect_report(&bytes, 0, Arch::X86_64, 0x1000, 0, &HashMap::new()).unwrap();
	assert_eq!(report.control_flow.leaders, 0);
	assert_eq!(report.control_flow.direct_jumps, 0);
}

#[test]
fn serializes_return_pop_values_as_sorted_unique_array_or_scalar() {
	for arch in [Arch::X86_16, Arch::X86_32, Arch::X86_64] {
		for (bytes, expected, returns) in [
			(&[][..], serde_json::json!([]), 0),
			(&[0xc3][..], serde_json::json!(0), 1),
			(&[0xc3, 0xc3][..], serde_json::json!(0), 2),
			(&[0xc3, 0xc2, 0, 0][..], serde_json::json!(0), 2),
			(&[0xc3, 0xc2, 12, 0][..], serde_json::json!([0, 12]), 2),
			(&[0xc2, 12, 0, 0xc2, 12, 0][..], serde_json::json!(12), 2),
			(&[0xc2, 12, 0, 0xc2, 8, 0, 0xc2, 12, 0][..], serde_json::json!([8, 12]), 3),
			(&[0xc2, 0, 0, 0xc2, 0xff, 0xff][..], serde_json::json!([0, 65535]), 2),
		] {
			let report = collect_report(bytes, bytes.len(), arch, 0x1000, 0, &HashMap::new()).unwrap();
			assert_eq!(report.control_flow.returns, returns);
			assert_eq!(serde_json::to_value(report.control_flow).unwrap()["return_pop"], expected);
		}
	}
}

#[test]
fn counts_far_jumps_as_external_but_excludes_far_calls() {
	let bytes = [
		0xea, 0, 0x10, 0, 0, 0x23, 0, // jmp far 0x23:0x1000
		0x9a, 0, 0x10, 0, 0, 0x23, 0, // call far 0x23:0x1000
		0xcb, // retf
	];
	let report = collect_report(&bytes, bytes.len(), Arch::X86_32, 0x1000, 0, &HashMap::new()).unwrap();
	assert_eq!(report.control_flow.direct_jumps, 1);
	assert_eq!(report.control_flow.external_branches, 1);
	assert_eq!(report.control_flow.internal_targets, 0);
	assert_eq!(report.control_flow.leaders, 1);
	assert_eq!(report.control_flow.returns, 1);
}

#[test]
fn aggregates_architectural_metadata_in_sorted_unique_lists() {
	let bytes = [
		0xc5, 0xfd, 0xfe, 0xc1, // vpaddd ymm0,ymm0,ymm1: AVX2
		0xc5, 0xf8, 0x58, 0xc1, // vaddps xmm0,xmm0,xmm1: AVX
		0xc4, 0xe2, 0xfb, 0xf5, 0xc1, // pdep rax,rax,rcx: BMI2
		0xc5, 0xfd, 0xfe, 0xc1, // repeat
		0x65, 0x8b, 0x00, // mov eax,gs:[rax]
		0x64, 0x8b, 0x00, // mov eax,fs:[rax]
		0x64, 0x8b, 0x00, // repeat FS
	];
	let report = collect_report(&bytes, bytes.len(), Arch::X86_64, 0x1000, 0, &HashMap::new()).unwrap();
	assert_eq!(serde_json::to_value(report.instructions).unwrap(), serde_json::json!({
		"encodings": ["Legacy", "VEX"],
		"cpuid_features": ["AVX", "AVX2", "BMI2"],
		"segment_overrides": ["FS", "GS"],
		"register_classes": ["GPR", "SEG", "XMM", "YMM", "ZMM"],
		"privileged": false,
	}));
}

#[test]
fn records_implicit_registers_and_privileged_instructions() {
	let bytes = [
		0xc3, // ret: implicit stack register; scan continues
		0xd8, 0xc1, // fadd st0,st1
		0x0f, 0x6f, 0xc1, // movq mm0,mm1
		0x0f, 0x20, 0xc0, // mov rax,cr0
		0x0f, 0x21, 0xc0, // mov rax,dr0
	];
	let report = collect_report(&bytes, bytes.len(), Arch::X86_64, 0x1000, 0, &HashMap::new()).unwrap();
	assert_eq!(report.instructions.register_classes, ["CR", "DR", "GPR", "MM", "ST"]);
	assert!(report.instructions.privileged);
	let report = collect_report(&bytes, 1, Arch::X86_64, 0x1000, 0, &HashMap::new()).unwrap();
	assert_eq!(report.instructions.register_classes, ["GPR"]);
	assert!(!report.instructions.privileged);
}

#[test]
fn metadata_includes_crossing_instructions_but_excludes_later_starts() {
	let bytes = [
		0x62, 0xf1, 0xfe, 0x49, 0x6f, 0x00, // vmovdqu64 zmm0{k1},[rax]
		0xf4, // hlt: privileged, outside range
	];
	let report = collect_report(&bytes, 1, Arch::X86_64, 0x1000, 0, &HashMap::new()).unwrap();
	assert_eq!(serde_json::to_value(report.instructions).unwrap(), serde_json::json!({
		"encodings": ["EVEX"], "cpuid_features": ["AVX512F"],
		"segment_overrides": [], "register_classes": ["GPR", "K", "ZMM"],
		"privileged": false,
	}));
	let report = collect_report(&bytes, 0, Arch::X86_64, 0x1000, 0, &HashMap::new()).unwrap();
	assert_eq!(serde_json::to_value(report.instructions).unwrap(), serde_json::json!({
		"encodings": [], "cpuid_features": [], "segment_overrides": [],
		"register_classes": [], "privileged": false,
	}));
}

#[test]
fn distinguishes_explicit_segment_prefixes_from_default_segments() {
	for arch in [Arch::X86_16, Arch::X86_32, Arch::X86_64] {
		let bytes = [0x8b, 0x00]; // ordinary memory operand with no segment prefix
		let report = collect_report(&bytes, bytes.len(), arch, 0x1000, 0, &HashMap::new()).unwrap();
		assert!(report.instructions.segment_overrides.is_empty());
		assert_eq!(report.instructions.register_classes.contains(&"SEG".into()), arch != Arch::X86_64);
	}
	let bytes = [0x2e, 0x90]; // explicit CS prefix, even though NOP uses no segment
	let report = collect_report(&bytes, bytes.len(), Arch::X86_64, 0x1000, 0, &HashMap::new()).unwrap();
	assert_eq!(report.instructions.segment_overrides, ["CS"]);
	assert!(report.instructions.register_classes.is_empty());
}

#[test]
fn collects_xop_and_tile_metadata_without_semantic_categories() {
	let bytes = [
		0x8f, 0xe8, 0x78, 0xc0, 0xc1, 1, // vprotb xmm0,xmm1,1
		0xc4, 0xe2, 0x7b, 0x49, 0xc0, // tilezero tmm0
	];
	let report = collect_report(&bytes, bytes.len(), Arch::X86_64, 0x1000, 0, &HashMap::new()).unwrap();
	assert_eq!(serde_json::to_value(report.instructions).unwrap(), serde_json::json!({
		"encodings": ["VEX", "XOP"], "cpuid_features": ["AMX_TILE", "XOP"],
		"segment_overrides": [], "register_classes": ["TMM", "XMM", "ZMM"],
		"privileged": false,
	}));
}

#[test]
fn suppresses_baseline_cpuid_features_only_when_producing_the_report() {
	use iced_x86::CpuidFeature as Feature;
	let bytes = [
		0x90, // nop: INTEL8086
		0x66, 0xc8, 0, 0, 0, // enterw 0,0: INTEL186
		0x66, 0x0f, 0x02, 0xc0, // lar ax,ax: INTEL286
		0x8b, 0xc1, // mov eax,ecx: INTEL386
		0x0f, 0xc8, // bswap eax: INTEL486
		0x0f, 0x44, 0xc1, // cmove eax,ecx: CMOV
		0x0f, 0xc7, 0x08, // cmpxchg8b [eax]: CX8
		0x0f, 0x1f, 0, // nop [eax]: MULTIBYTENOP
		0x66, 0x0f, 0xef, 0xc0, // pxor xmm0,xmm0: SSE2 retained
		0x0f, 0xa2, // cpuid: retained
		0x0f, 0x31, // rdtsc: TSC retained
		0xf3, 0x90, // pause: retained
	];
	let mut state = CollectionState::default();
	collect(&mut state, &bytes, bytes.len(), Arch::X86_32, 0x1000, 0).unwrap();
	for feature in [Feature::INTEL8086, Feature::INTEL186, Feature::INTEL286, Feature::INTEL386,
		Feature::INTEL486, Feature::CMOV, Feature::CX8, Feature::MULTIBYTENOP] {
		assert!(state.instructions.cpuid_features.contains(&feature), "raw metadata missing {feature:?}");
	}
	assert_eq!(state.instructions.finalize().cpuid_features, ["CPUID", "PAUSE", "SSE2", "TSC"]);

	let mut state = CollectionState::default();
	let bytes = [0x48, 0x89, 0xd8]; // mov rax,rbx: X64
	collect(&mut state, &bytes, bytes.len(), Arch::X86_64, 0x1000, 0).unwrap();
	assert!(state.instructions.cpuid_features.contains(&Feature::X64));
	assert!(state.instructions.finalize().cpuid_features.is_empty());
}

#[test]
fn preserves_unusual_cpu_specific_and_fpu_features() {
	use iced_x86::CpuidFeature as Feature;
	let metadata = InstructionMetadata {
		cpuid_features: HashSet::from([
			Feature::INTEL8086, Feature::INTEL8086_ONLY, Feature::INTEL286_ONLY,
			Feature::INTEL386_ONLY, Feature::INTEL386_A0_ONLY, Feature::INTEL486_A_ONLY,
			Feature::FPU, Feature::FPU287, Feature::FPU387, Feature::FPU287XL_ONLY,
		]),
		..Default::default()
	};
	assert_eq!(metadata.finalize().cpuid_features, [
		"FPU", "FPU287", "FPU287XL_ONLY", "FPU387", "INTEL286_ONLY", "INTEL386_A0_ONLY",
		"INTEL386_ONLY", "INTEL486_A_ONLY", "INTEL8086_ONLY",
	]);
}
