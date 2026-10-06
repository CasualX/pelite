use super::*;

#[derive(Debug, Eq, PartialEq, Hash, serde::Serialize)]
#[serde(untagged)]
enum Reference {
	Symbol(String),
	Rva(i64),
}

#[derive(Debug, Default, serde::Serialize)]
struct Brief {
	references: Vec<Reference>,
	values: serde_json::Map<String, serde_json::Value>,
	control_flow: ControlFlow,
	instructions: Instructions,
	constants: Constants,
}

#[derive(Debug, Default, serde::Serialize)]
struct ControlFlow {
	direct_jumps: usize,
	conditional_branches: usize,
	indirect_branches: usize,
	internal_targets: usize,
	external_branches: usize,
	leaders: usize,
	returns: usize,
	#[serde(serialize_with = "serialize_return_pop")]
	return_pop: Vec<u16>,
}

fn serialize_return_pop<S: serde::Serializer>(values: &[u16], serializer: S) -> std::result::Result<S::Ok, S::Error> {
	use serde::Serialize;
	match values {
		[value] => value.serialize(serializer),
		_ => values.serialize(serializer),
	}
}

#[derive(Debug, Default, serde::Serialize)]
struct Instructions {
	encodings: Vec<String>,
	cpuid_features: Vec<String>,
	segment_overrides: Vec<String>,
	register_classes: Vec<String>,
	privileged: bool,
}

#[derive(Debug, Default, serde::Serialize)]
struct Constants {
	immediates: Vec<i64>,
	comparisons: Vec<i64>,
	displacements: Vec<i64>,
}

pub fn command() -> clap::Command {
	clap::Command::new("brief")
		.about("Summarize references, constants, control flow, and instruction metadata in a byte range")
		.after_help(include_str!("docs/brief.md"))
		.arg(clap::Arg::new("file")
			.value_name("FILE")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))
		.arg(clap::Arg::new("address")
			.value_name("ADDRESS")
			.value_parser(Address::parse)
			.required(true))
		.arg(clap::Arg::new("length")
			.value_name("BYTES")
			.value_parser(value_parser::parse_usize)
			.required(true)
			.help("Number of bytes to decode (decimal or 0xhex)"))
		.arg(clap::Arg::new("arch")
			.long("arch")
			.value_name("ARCH")
			.value_parser(Arch::parse)
			.help("Override the PE machine header (x86_16, x86_32 [alias x86], or x86_64)"))
		.arg(clap::Arg::new("zerofill")
			.long("zerofill")
			.action(clap::ArgAction::SetTrue)
			.help("Allow typed reads from zero-filled section data"))
		.arg(symbols::arg())
}

pub fn run(matches: &clap::ArgMatches, format: OutputFormat) -> Result {
	let path = matches.get_one::<PathBuf>("file").expect("required by clap");
	let address = *matches.get_one::<Address>("address").expect("required by clap");
	let length = *matches.get_one::<usize>("length").expect("required by clap");
	let map = pelite::FileMap::open(path)?;
	let pe = pelite::PeFile::from_bytes(&map)?;
	let rva = address.to_rva(pe)?;
	u32::try_from(length).ok().and_then(|length| rva.checked_add(length))
		.ok_or_else(|| err("RVA plus length overflows"))?;
	let arch = get_arch(matches, pe)?;
	let bytes = pe.slice(rva, length, 1)?;
	let facts = symbols::load(matches, ty::PointerWidth::from(pe), pe.image_base(), Some(pe))?;

	// Collect the brief data
	let mut state = CollectionState::default();
	collect(&mut state, bytes, length, arch, rva, pe.image_base())?;

	// Process the brief data
	let options = read::ReadOptions {
		zerofill: matches.get_flag("zerofill"),
		string_preview_length: read::DEFAULT_STRING_PREVIEW_LENGTH.parse().unwrap(),
		max_dynamic_array_length: read::DEFAULT_MAX_DYNAMIC_ARRAY_LENGTH.parse().unwrap(),
	};
	let brief = state.finalize(pe, &facts, &options);

	print("Instruction values", &brief, format)
}

fn collect(state: &mut CollectionState, bytes: &[u8], length: usize, arch: Arch, rva: u32, image_base: u64) -> Result {
	use iced_x86::{FlowControl, Mnemonic, OpKind, Register};

	let ip = image_base.wrapping_add(u64::from(rva));
	let mut decoder = iced_x86::Decoder::with_ip(arch.bitness(), bytes, ip, iced_x86::DecoderOptions::NONE);
	let mut info_factory = iced_x86::InstructionInfoFactory::new();

	// Track stack registers
	let mut rbp_is_frame = false;
	let (frame_register, stack_register) = match arch {
		Arch::X86_16 => (Register::BP, Register::SP),
		Arch::X86_32 => (Register::EBP, Register::ESP),
		Arch::X86_64 => (Register::RBP, Register::RSP),
	};

	if length > 0 {
		state.leaders.insert(ip);
	}

	// Keep the remaining section bytes available to complete the final instruction.
	while decoder.can_decode() && decoder.position() < length {
		let offset = decoder.position();
		let instruction = decoder.decode();
		if instruction.is_invalid() {
			return Err(err(format!("unable to decode instruction at rva:{:#x}", u64::from(rva) + offset as u64)));
		}

		let metadata = &mut state.instructions;
		metadata.encodings.insert(instruction.encoding());
		metadata.cpuid_features.extend(instruction.cpuid_features().iter().copied());
		if instruction.segment_prefix() != Register::None {
			metadata.segment_overrides.insert(instruction.segment_prefix());
		}
		metadata.privileged |= instruction.is_privileged();
		let info = info_factory.info_options(&instruction, iced_x86::InstructionInfoOptions::NO_MEMORY_USAGE);
		for operand in 0..instruction.op_count() {
			if instruction.op_kind(operand) == OpKind::Register && info.op_access(operand) != iced_x86::OpAccess::None {
				metadata.add_register(instruction.op_register(operand));
			}
		}
		for used in info.used_registers() {
			metadata.add_register(used.register());
		}

		let constants = decoder.get_constant_offsets(&instruction);
		let flow = instruction.flow_control();
		let direct_branch = matches!(flow, FlowControl::UnconditionalBranch | FlowControl::ConditionalBranch);
		match flow {
			FlowControl::UnconditionalBranch | FlowControl::ConditionalBranch => {
				state.control_flow.direct_jumps += 1;
				if flow == FlowControl::ConditionalBranch {
					state.control_flow.conditional_branches += 1;
					if decoder.position() < length {
						state.leaders.insert(instruction.next_ip());
					}
				}
			},
			FlowControl::IndirectCall | FlowControl::IndirectBranch => state.control_flow.indirect_branches += 1,
			FlowControl::Return => {
				state.control_flow.returns += 1;
				if instruction.mnemonic() == Mnemonic::Ret {
					let pop = if instruction.op0_kind() == OpKind::Immediate16 { instruction.immediate16() } else { 0 };
					state.control_flow.return_pop.push(pop);
				}
			},
			_ => {},
		}

		// Detect if rbp is being used as stack base register
		let frame_setup = instruction.op0_kind() == OpKind::Register && instruction.op0_register() == frame_register
			&& match instruction.mnemonic() {
				iced_x86::Mnemonic::Mov => instruction.op1_kind() == OpKind::Register && instruction.op1_register() == stack_register,
				iced_x86::Mnemonic::Lea => instruction.op1_kind() == OpKind::Memory && instruction.memory_base() == stack_register
					&& instruction.memory_index() == Register::None,
				_ => false,
			};
		// Keep the frame hint across early epilogues and later blocks in the range.
		rbp_is_frame |= frame_setup;

		let is_stack_register = |register: Register| {
			register.full_register() == Register::RSP || (rbp_is_frame && register.full_register() == Register::RBP)
		};

		// Stack register immediates are bookkeeping; immediates stored on the stack are data.
		let uses_stack_register_operand = instruction.op_kinds().enumerate().any(|(operand, kind)| {
			kind == OpKind::Register && is_stack_register(instruction.op_register(operand as u32))
		});

		let immediate_kind = if matches!(instruction.mnemonic(), iced_x86::Mnemonic::Cmp | iced_x86::Mnemonic::Test) {
			ConstantKind::Comparison
		}
		else {
			ConstantKind::Immediate
		};

		for (operand, kind) in instruction.op_kinds().enumerate() {
			match kind {
				OpKind::NearBranch16 | OpKind::NearBranch32 | OpKind::NearBranch64 => {
					let target = instruction.near_branch_target();
					// Ignore internal targets
					let internal = target.wrapping_sub(ip) < length as u64;
					if direct_branch {
						if internal {
							state.internal_targets.insert(target);
							state.leaders.insert(target);
						}
						else {
							state.control_flow.external_branches += 1;
						}
					}
					// Calls remain useful evidence, including recursion and local helpers.
					if !internal || instruction.flow_control() == iced_x86::FlowControl::Call {
						state.add_reference(target);
					}
				},
				OpKind::FarBranch16 | OpKind::FarBranch32 => {
					// Segmented targets cannot be resolved inside the flat analyzed range.
					if direct_branch {
						state.control_flow.external_branches += 1;
					}
					if uses_stack_register_operand {
						continue;
					}
					// A segmented far pointer is not a flat image VA.
					state.add_constant(instruction.far_branch_selector() as i64, immediate_kind);
					let far_value = if kind == OpKind::FarBranch16 { instruction.far_branch16() as i64 } else { instruction.far_branch32() as i64 };
					state.add_constant(far_value, immediate_kind);
				},
				// Extract displacements and references
				OpKind::Memory => {
					let absolute = instruction.memory_base() == Register::None && instruction.memory_index() == Register::None;
					let static_address = (instruction.is_ip_rel_memory_operand() || absolute)
						&& !matches!(instruction.segment_prefix(), Register::FS | Register::GS);
					if static_address {
						// Report the address used by the instruction, rather than its encoding offset.
						state.add_reference(if instruction.is_ip_rel_memory_operand() { instruction.ip_rel_memory_address() } else { instruction.memory_displacement64() });
					}
					else if constants.has_displacement()
						&& !is_stack_register(instruction.memory_base()) && !is_stack_register(instruction.memory_index()) {
						let value = instruction.memory_displacement64();
						let displacement = match instruction.memory_displ_size() {
							1 => value as i8 as i64,
							2 => value as i16 as i64,
							4 => value as i32 as i64,
							_ => value as i64,
						};
						state.add_constant(displacement, ConstantKind::Displacement);
					}
				},
				// Extract immediates
				OpKind::Immediate8to16 if !uses_stack_register_operand => state.add_constant(instruction.immediate8to16() as i64, immediate_kind),
				OpKind::Immediate8to32 if !uses_stack_register_operand => state.add_constant(instruction.immediate8to32() as i64, immediate_kind),
				OpKind::Immediate8to64 | OpKind::Immediate32to64 | OpKind::Immediate8 | OpKind::Immediate8_2nd | OpKind::Immediate16 | OpKind::Immediate32 | OpKind::Immediate64
					if !uses_stack_register_operand => state.add_constant(instruction.immediate(operand as u32) as i64, immediate_kind),
				// Nothing interesting to say
				_ => {},
			}
		}
	}
	Ok(())
}

#[derive(Default)]
struct InstructionMetadata {
	encodings: HashSet<iced_x86::EncodingKind>,
	cpuid_features: HashSet<iced_x86::CpuidFeature>,
	segment_overrides: HashSet<iced_x86::Register>,
	register_classes: HashSet<iced_x86::Register>,
	privileged: bool,
}

impl InstructionMetadata {
	fn add_register(&mut self, register: iced_x86::Register) {
		if register != iced_x86::Register::None {
			self.register_classes.insert(register.base());
		}
	}

	fn finalize(self) -> Instructions {
		fn names<T: std::fmt::Debug>(values: impl IntoIterator<Item = T>) -> Vec<String> {
			let mut names: Vec<_> = values.into_iter().map(|value| format!("{value:?}")).collect();
			names.sort_unstable();
			names
		}
		let mut register_classes: Vec<_> = self.register_classes.into_iter().map(|base| {
			if base.is_gpr() { "GPR".into() }
			else if base.is_segment_register() { "SEG".into() }
			else if base.is_ip() { "IP".into() }
			else {
				// Iced's base identifies the register family: XMM0, ST0, K0, etc.
				format!("{base:?}").trim_end_matches(|c: char| c.is_ascii_digit()).to_owned()
			}
		}).collect();
		register_classes.sort_unstable();
		register_classes.dedup();
		// Keep raw features during collection; suppress baseline noise only in the report.
		// CPU-specific *_ONLY markers and architectural extensions remain informative.
		use iced_x86::CpuidFeature as Feature;
		let cpuid_features = self.cpuid_features.into_iter().filter(|feature| !matches!(feature,
			Feature::INTEL8086 | Feature::INTEL186 | Feature::INTEL286 | Feature::INTEL386 | Feature::INTEL486
			| Feature::X64 | Feature::CMOV | Feature::CX8 | Feature::MULTIBYTENOP
		));
		Instructions {
			encodings: names(self.encodings),
			cpuid_features: names(cpuid_features),
			segment_overrides: names(self.segment_overrides),
			register_classes,
			privileged: self.privileged,
		}
	}
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum ConstantKind {
	Immediate,
	Comparison,
	Displacement,
}

#[derive(Default)]
struct CollectionState {
	references: Vec<u64>,
	control_flow: ControlFlow,
	instructions: InstructionMetadata,
	internal_targets: HashSet<u64>,
	leaders: HashSet<u64>,
	constants: Vec<(ConstantKind, i64)>,
	seen_references: HashSet<u64>,
	seen_constants: HashSet<(ConstantKind, i64)>,
}

impl CollectionState {
	fn add_reference(&mut self, va: u64) {
		if self.seen_references.insert(va) {
			self.references.push(va);
		}
	}

	fn add_constant(&mut self, value: i64, kind: ConstantKind) {
		if self.seen_constants.insert((kind, value)) {
			self.constants.push((kind, value));
		}
	}

	fn finalize(self, pe: pelite::PeFile<'_>, facts: &symbols::IndexedFacts, options: &read::ReadOptions) -> Brief {
		let image_base = pe.image_base();
		let mut values = serde_json::Map::new();
		let references = self.references.into_iter().map(|va| {
			match facts.symbols.get(&va).filter(|symbol| !matches!(symbol, symbols::IndexedSymbol::Weak(_))) {
				Some(symbol) => {
					let name = symbol.to_string();
					if let Some(ty) = facts.types.get(&va).filter(|ty| !ty.is_opaque()) {
						// Indexed fact addresses are the image base plus a valid u32 RVA.
						let rva = va.wrapping_sub(image_base) as u32;
						values.insert(name.clone(), read::read_at(pe, rva, ty, &options));
					}
					Reference::Symbol(name)
				},
				None => Reference::Rva((va as i64).wrapping_sub(image_base as i64)),
			}
		}).collect();

		let mut constants = Constants::default();
		for (kind, value) in self.constants {
			match kind {
				ConstantKind::Immediate => constants.immediates.push(value),
				ConstantKind::Comparison => constants.comparisons.push(value),
				ConstantKind::Displacement => constants.displacements.push(value),
			}
		}

		let mut control_flow = self.control_flow;
		control_flow.internal_targets = self.internal_targets.len();
		control_flow.leaders = self.leaders.len();
		control_flow.return_pop.sort_unstable();
		control_flow.return_pop.dedup();
		let instructions = self.instructions.finalize();
		Brief { references, control_flow, instructions, constants, values }
	}
}

#[cfg(test)]
mod tests;
