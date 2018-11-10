use operators::Operator;
use state::InternalState;
use Location;
use memory;

pub struct CopyFromOp {
	pub cell: Location
}
impl Operator for CopyFromOp {
	fn changes_instruction_counter(&self) -> bool { false }

	fn apply_to(&self, s: &mut InternalState) -> Result<(), String> {
		let memory_position = memory::extract_memory_position(self.cell, &s);
		if let Err(error) = memory_position {
			return Err(memory::explain(error));
		}

		let cell = memory_position.unwrap();
		if let Some(_) = s.memory[cell] {
			s.register = s.memory[cell];
			Ok(())
		}
		else {
			Err(format!("cell {} holds no value. could not copy a none value to the register", cell))
		}
	}
}

#[cfg(test)]
mod test {
	use state::InternalState;
  use Value;
	use Location;
	use operators::Operator;
	use operators::copyfrom::CopyFromOp;

	#[test]
	fn copyfrom_non_empty_cell(){
		let mut state = InternalState::new(None, 0)
			.with_memory(vec!(Some(Value::Number{value: 5})));
		let operation = CopyFromOp{cell: Location::Cell(0)};

		let result = operation.apply_to(&mut state);

		assert!(result.is_ok());
		assert!(match state.register {
			Some(Value::Number{value: 5}) => true,
			_ => false
		});
	}

	#[test]
	fn copyfrom_non_empty_addressed_cell(){
		let mut state = InternalState::new(None, 0);
		state.memory = vec!(Some(Value::Number{value: 1}), Some(Value::Number{value:5}));
		let operation = CopyFromOp{cell: Location::Address(0)};

		let result = operation.apply_to(&mut state);
		assert!(result.is_ok());
		assert!(match state.register {
			Some(Value::Number{value: 5}) => true,
			_ => false
		});
	}

	#[test]
	fn copyfrom_empty_cell() {
		let mut state = InternalState::new(None, 0)
			.with_memory(vec!(None));
		let operation = CopyFromOp{cell: Location::Cell(0)};

		let result = operation.apply_to(&mut state);

		assert!(result.is_err());
	}

	#[test]
	fn copyfrom_empty_addressed_cell() {
		let mut state = InternalState::new(None, 0);
		state.memory = vec!(Some(Value::Number{value: 1}), None);
		let operation = CopyFromOp{cell: Location::Address(0)};

		let result = operation.apply_to(&mut state);

		println!("{:?}", result);
		assert!(result.is_err());
	}


	#[test]
	#[should_panic]
	fn copyfrom_non_existent_cell() {
		let mut state = InternalState::new(None, 0)
			.with_memory(vec!(None));
		let operation = CopyFromOp{cell: Location::Cell(9)};

		// #[should_panic]
		let _ = operation.apply_to(&mut state);
	}

	#[test]
	#[should_panic]
	fn copyfrom_non_existent_addressed_cell() {
		let mut state = InternalState::new(None, 0);
		state.memory = vec!(Some(Value::Number{value: 9}));
		let operation = CopyFromOp{cell: Location::Address(0)};

		// #[should_panic]
		let _ = operation.apply_to(&mut state);
	}
}
