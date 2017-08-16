use operators::Operator;
use state::InternalState;

pub struct CopyFromOp {
	pub cell: usize
}
impl Operator for CopyFromOp {
	fn changes_instruction_counter(&self) -> bool { false }

	fn apply_to(&self, mut s: InternalState) -> InternalState {
		if let Some(_) = s.memory[self.cell] {
			s.register = s.memory[self.cell]
		}
		else {
			panic!("cell {} holds no value. could not copy a None value to the register", self.cell)
		}
		s
	}
}

#[cfg(test)]
mod test {
	use state::InternalState;
  use Value;
	use Operation;

	#[test]
	fn copyfrom_non_empty_cell(){
		let mut state = InternalState{
			register: None,
			memory: vec!(Some(Value::Number{value: 5})),
			input_tape: vec!(),
			output_tape: vec!(),
			instruction_counter: 0
		};

		state = state.apply(Operation::CopyFrom{cell: 0});

		assert!(state.register.is_some())
	}

	#[test]
	#[should_panic]
	fn copyfrom_empty_cell() {
		let state = InternalState{
			register: None,
			memory: vec!(None),
			input_tape: vec!(),
			output_tape: vec!(),
			instruction_counter: 0
		};

		state.apply(Operation::CopyFrom{cell: 0});
	}

	#[test]
	#[should_panic]
	fn copyfrom_non_existent_cell() {
		let state = InternalState{
			register: None,
			memory: vec!(None),
			input_tape: vec!(),
			output_tape: vec!(),
			instruction_counter: 0
		};

		state.apply(Operation::CopyFrom{cell: 5});
	}
}
