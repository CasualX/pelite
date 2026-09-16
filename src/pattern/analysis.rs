//! Scratch-flow checks for compiled pattern bytecode.
//!
//! The compiler emits forward branches and puts each Extend chain directly before its consumer.
//! These invariants let us use two linear passes rather than a graph worklist.
//! This is deliberately private: arbitrary hand-written VM programs may carry extensions across instructions or branch into modifier pairs.

use super::*;

/// Front and back operands occupy distinct bits, assuming `save_len` storage.
#[derive(Clone, Copy)]
struct Slots([u64; 4]);
impl Slots {
	const EMPTY: Slots = Slots([0; 4]);
	const fn contains(self, slot: i8) -> bool {
		let index = slot as u8 as usize;
		self.0[index / 64] & (1 << (index % 64)) != 0
	}
	const fn insert(&mut self, slot: i8) {
		let index = slot as u8 as usize;
		self.0[index / 64] |= 1 << (index % 64);
	}
	const fn remove(&mut self, slot: i8) {
		let index = slot as u8 as usize;
		self.0[index / 64] &= !(1 << (index % 64));
	}
	const fn union(self, other: Slots) -> Slots {
		Slots([self.0[0] | other.0[0], self.0[1] | other.0[1], self.0[2] | other.0[2], self.0[3] | other.0[3]])
	}
	const fn intersection(self, other: Slots) -> Slots {
		Slots([self.0[0] & other.0[0], self.0[1] & other.0[1], self.0[2] & other.0[2], self.0[3] & other.0[3]])
	}
}

#[derive(Clone, Copy)]
struct Failure {
	possible: bool,
	writes: Slots,
}
impl Failure {
	const NONE: Failure = Failure {
		possible: false,
		writes: Slots::EMPTY,
	};
}

/// Caller-provided workspace keeps the analysis identical in const and runtime
/// contexts. Each entry describes the state immediately before its instruction.
#[derive(Clone, Copy)]
pub struct State {
	initialized: Slots,
	clobbered: Slots,
	reachable: bool,
	failure: Failure,
}
impl State {
	pub const EMPTY: State = State {
		initialized: Slots::EMPTY,
		clobbered: Slots::EMPTY,
		reachable: false,
		failure: Failure::NONE,
	};
}

const fn operand(atoms: &[Atom], pc: usize, low: u8) -> usize {
	let mut first = pc;
	while first > 0 && matches!(atoms[first - 1], Atom::Extend(_)) {
		first -= 1;
	}
	let mut value = 0;
	while first < pc {
		if let Atom::Extend(byte) = atoms[first] {
			value = (value << 8) + byte as usize;
		}
		first += 1;
	}
	(value << 8) + low as usize
}

const fn failure(states: &[State], pc: usize) -> Failure {
	if pc == states.len() { Failure::NONE } else { states[pc].failure }
}

const fn merge(states: &mut [State], pc: usize, incoming: State) {
	if pc == states.len() {
		return;
	}
	if states[pc].reachable {
		states[pc].initialized = states[pc].initialized.intersection(incoming.initialized);
		states[pc].clobbered = states[pc].clobbered.union(incoming.clobbered);
	}
	else {
		states[pc].initialized = incoming.initialized;
		states[pc].clobbered = incoming.clobbered;
		states[pc].reachable = true;
	}
}

/// On error, `position` is an atom index. The parser maps it back to source bytes.
/// No caller-initialized slots or exported outputs are assumed. All reads must
/// have a local definition on every path; unused, branch-local writes are legal.
/// Retry checks are conservative and do not prove equal values or infeasible paths.
pub const fn analyze(atoms: &[Atom], states: &mut [State]) -> Result<(), PatternError> {
	assert!(atoms.len() == states.len());
	let mut pc = atoms.len();
	while pc > 0 {
		pc -= 1;
		states[pc] = State::EMPTY;
		let atom = atoms[pc];
		let mut summary = failure(states, pc + 1);
		match atom {
			Atom::Goto(low) => {
				summary = failure(states, pc + 1 + operand(atoms, pc, low));
			}
			Atom::Fork(low) => {
				let alternative = failure(states, pc + 1 + operand(atoms, pc, low));
				// A fork fails only when both continuations fail. In that case,
				// either continuation may have written scratch before failing.
				summary.possible = summary.possible && alternative.possible;
				summary.writes = if summary.possible { summary.writes.union(alternative.writes) } else { Slots::EMPTY };
			}
			_ => {
				// Reads store only on success, so their own failure cannot write.
				if summary.possible {
					if let Some(slot) = atom.output_slot() {
						summary.writes.insert(slot);
					}
				}
				summary.possible = summary.possible || atom.can_fail();
			}
		}
		states[pc].failure = summary;
	}

	if atoms.is_empty() {
		return Ok(());
	}
	states[0].reachable = true;
	pc = 0;
	while pc < atoms.len() {
		let mut state = states[pc];
		let atom = atoms[pc];
		if state.reachable {
			if let Some(slot) = atom.input_slot() {
				let kind = if !state.initialized.contains(slot) {
					Some(ErrorKind::UninitializedSlot)
				}
				else if state.clobbered.contains(slot) {
					Some(ErrorKind::RetryClobber)
				}
				else {
					None
				};
				if let Some(kind) = kind {
					return Err(PatternError { kind, position: pc });
				}
			}
			if let Some(slot) = atom.output_slot() {
				state.initialized.insert(slot);
				state.clobbered.remove(slot);
			}
			match atom {
				Atom::Goto(low) => {
					merge(states, pc + 1 + operand(atoms, pc, low), state);
				}
				Atom::Fork(low) => {
					merge(states, pc + 1, state);
					let failed = failure(states, pc + 1);
					if failed.possible {
						state.clobbered = state.clobbered.union(failed.writes.intersection(state.initialized));
						merge(states, pc + 1 + operand(atoms, pc, low), state);
					}
				}
				Atom::Scan(_) => {
					let failed = failure(states, pc + 1);
					state.clobbered = state.clobbered.union(failed.writes.intersection(state.initialized));
					merge(states, pc + 1, state);
				}
				_ => {
					merge(states, pc + 1, state);
				}
			}
		}
		pc += 1;
	}
	Ok(())
}
