use operators::Operator;
use state::InternalState;

pub struct CopyFromOp {
	pub cell: usize
}
impl Operator for CopyFromOp {
	fn changes_instruction_counter(&self) -> bool { false }

	fn apply_to(&self, s: &mut InternalState) -> Result<(), String> {
		if let Some(_) = s.memory[self.cell] {
			s.register = s.memory[self.cell];
			Ok(())
		}
		else {
			Err(format!("cell {} holds no value. could not copy a none value to the register", self.cell))
		}
	}
}

#[cfg(test)]
mod test {
	use state::InternalState;
  use Value;
	use operators::Operator;
	use operators::copyfrom::CopyFromOp;

	#[test]
	fn copyfrom_non_empty_cell(){
		let mut state = InternalState{
			register: None,
			memory: vec!(Some(Value::Number{value: 5})),
			input_tape: vec!(),
			output_tape: vec!(),
			instruction_counter: 0
		};
		let operation = CopyFromOp{cell: 0};

		let result = operation.apply_to(&mut state);
		assert!(result.is_ok())
	}

	#[test]
	fn copyfrom_empty_cell() {
		let mut state = InternalState{
			register: None,
			memory: vec!(None),
			input_tape: vec!(),
			output_tape: vec!(),
			instruction_counter: 0
		};
		let operation = CopyFromOp{cell: 0};

		let result = operation.apply_to(&mut state);

		assert!(result.is_err());
	}

	#[test]
	#[should_panic]
	fn copyfrom_non_existent_cell() {
		let mut state = InternalState{
			register: None,
			memory: vec!(None),
			input_tape: vec!(),
			output_tape: vec!(),
			instruction_counter: 0
		};
		let operation = CopyFromOp{cell: 9};

		// #[should_panic]
		let _ = operation.apply_to(&mut state);
	}
}
