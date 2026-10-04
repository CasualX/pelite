use super::*;

#[derive(Debug, Eq, PartialEq, Hash, serde::Serialize)]
#[serde(untagged)]
enum Reference {
	Symbol(String),
	Rva(i64),
}

#[derive(Debug, Default, serde::Serialize)]
struct References {
	references: Vec<Reference>,
	indirect_calls: usize,
	constants: Constants,
}

#[derive(Debug, Default, serde::Serialize)]
struct Constants {
	immediates: Vec<i64>,
	comparisons: Vec<i64>,
	displacements: Vec<i64>,
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
	indirect_calls: usize,
	constants: Vec<(ConstantKind, i64)>,
	seen_references: HashSet<u64>,
	seen_constants: HashSet<(ConstantKind, i64)>,
}

impl CollectionState {
	fn add_reference(&mut self, va: u64) {
		if self.seen_references.insert(va) { self.references.push(va); }
	}

	fn add_constant(&mut self, value: i64, kind: ConstantKind) {
		if self.seen_constants.insert((kind, value)) { self.constants.push((kind, value)); }
	}

	fn finalize(self, image_base: u64, symbols: &HashMap<u64, symbols::IndexedSymbol>) -> References {
		let references = self.references.into_iter().map(|va| {
			match symbols.get(&va).filter(|symbol| !matches!(symbol, symbols::IndexedSymbol::Weak(_))) {
				Some(symbol) => Reference::Symbol(symbol.to_string()),
				None => Reference::Rva((va as i64).wrapping_add((image_base as i64).wrapping_neg())),
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
		References { references, indirect_calls: self.indirect_calls, constants }
	}
}

pub fn command() -> clap::Command {
	clap::Command::new("refs")
		.about("List address references and constants used by instructions in a byte range")
		.after_help(include_str!("docs/refs.md"))
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
			.help("Number of bytes to decode (decimal or 0x-prefixed hexadecimal)"))
		.arg(clap::Arg::new("arch")
			.long("arch")
			.value_name("ARCH")
			.value_parser(Arch::parse)
			.help("Override the PE machine header (x86_16, x86_32 [alias x86], or x86_64)"))
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
		.ok_or_else(|| err("address plus length overflows RVA"))?;
	let arch = get_arch(matches, pe)?;
	let bytes = pe.slice(rva, length, 1)?;
	let facts = symbols::load(matches, ty::PointerWidth::from(pe), pe.image_base(), Some(pe))?;
	let references = collect(bytes, length, arch, rva, pe.image_base(), &facts.symbols)?;
	print("Instruction values", &references, format)
}

fn collect(bytes: &[u8], length: usize, arch: Arch, rva: u32, image_base: u64, symbols: &HashMap<u64, symbols::IndexedSymbol>) -> Result<References> {
	use iced_x86::{OpKind, Register};

	let ip = image_base.wrapping_add(u64::from(rva));
	let mut decoder = iced_x86::Decoder::with_ip(arch.bitness(), bytes, ip, iced_x86::DecoderOptions::NONE);

	// Track stack registers
	let mut rbp_is_frame = false;
	let (frame_register, stack_register) = match arch {
		Arch::X86_16 => (Register::BP, Register::SP),
		Arch::X86_32 => (Register::EBP, Register::ESP),
		Arch::X86_64 => (Register::RBP, Register::RSP),
	};

	let mut state = CollectionState::default();

	// Keep the remaining section bytes available to complete the final instruction.
	while decoder.can_decode() && decoder.position() < length {
		let offset = decoder.position();
		let instruction = decoder.decode();
		if instruction.is_invalid() {
			return Err(err(format!("unable to decode instruction at rva:{:#x}", u64::from(rva) + offset as u64)));
		}

		let constants = decoder.get_constant_offsets(&instruction);
		if matches!(instruction.flow_control(), iced_x86::FlowControl::IndirectCall | iced_x86::FlowControl::IndirectBranch) {
			state.indirect_calls += 1;
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
					// Calls remain useful evidence, including recursion and local helpers.
					if !internal || instruction.flow_control() == iced_x86::FlowControl::Call {
						state.add_reference(target);
					}
				},
				OpKind::FarBranch16 | OpKind::FarBranch32 => {
					if uses_stack_register_operand { continue; }
					// A segmented far pointer is not a flat image VA.
					state.add_constant(instruction.far_branch_selector() as i64, immediate_kind);
					state.add_constant(if kind == OpKind::FarBranch16 { instruction.far_branch16() as i64 } else { instruction.far_branch32() as i64 }, immediate_kind);
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
	Ok(state.finalize(image_base, symbols))
}

#[cfg(test)]
mod tests;
