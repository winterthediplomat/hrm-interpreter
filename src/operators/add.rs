use operators::Operator;
use Value;
use Location;
use state;
use memory;
// --
use std::char;

enum Error {
	NoValue{cell: Location},
	NoEmployeeValue,
	SumOfChars,
	SumOverflow{character: char, number: u32}
}

pub struct AddOp {
	pub cell: Location
}

impl AddOp {
	fn add_number_and_char(num: u32, c: char) -> Result<char, u32> {
		const ALPHABET_RADIX: u32 = 36;
		const SMALL_ASCII_A: u32 = 97;
		const HEX_A_IN_DEC: u32 = 10;

		let c_as_number = c.to_digit(ALPHABET_RADIX).unwrap();
		let new_number = c_as_number + num;
		let fixed_for_char: u32 = SMALL_ASCII_A + (new_number - HEX_A_IN_DEC);

		if new_number >= 36 {
			Err(new_number)
		}
		else {
			Ok(char::from_u32(fixed_for_char).unwrap())
		}
	}

	fn explain_error(e: Error) -> String {
		match e {
			Error::NoValue{cell: Location::Cell(_cell)} => format!("There is no value at cell {:?}", _cell),
			Error::NoValue{cell: Location::Address(_cell)} => format!("There is no value at cell {:?}", _cell),
			Error::NoEmployeeValue => String::from("the Employee register holds no value. Cannot add."),
			Error::SumOfChars => String::from("cannot sum two characters!"),
			Error::SumOverflow{character: _char, number: _num} =>
				format!("value overflowed! {} + {} is not representable as a letter!",
								_char, _num)
		}
	}
}

impl Operator for AddOp {
  fn changes_instruction_counter(&self) -> bool {
		false
	}

  fn apply_to(&self,  s: &mut state::InternalState) -> Result<(), String> {
		let memory_position = memory::extract_memory_position(self.cell, &s);
		if let Err(error) = memory_position {
			return Err(memory::explain(error));
		}

		let value_from_memory = s.memory[memory_position.unwrap()].clone();
		let res = match value_from_memory {
			Some(ref v) => {
				match s.register {
					Some(old_register) => {
						let new_register_value: Result<Value, String> =  match (v, old_register) {
							(&Value::Number{value: _v}, Value::Number{value: _old}) => {
								Ok(Value::Number{value: _v + _old})
							},
							(&Value::Number{value: _v}, Value::Character{value: _old}) => {
								if let Ok(new_char) = AddOp::add_number_and_char(_v, _old) {
									Ok(Value::Character{value: new_char})
								}
								else {
									Err(AddOp::explain_error(Error::SumOverflow{character: _old, number: _v}))
								}
							},
							(&Value::Character{value: _v}, Value::Number{value: _old}) => {
								if let Ok(new_char) = AddOp::add_number_and_char(_old, _v) {
									Ok(Value::Character{value: new_char})
								}
								else {
									Err(AddOp::explain_error(Error::SumOverflow{character: _v, number: _old}))
								}
							},
							_ => Err(AddOp::explain_error(Error::SumOfChars))
						};
						
						match new_register_value {
							Ok(value) => {
								s.register = Some(value);
								Ok(())
							},
							Err(reason) => Err(reason)
						}
					}
					_ => {
						Err(AddOp::explain_error(Error::NoEmployeeValue))
					}
				}
			}
			_ => {
				Err(AddOp::explain_error(Error::NoValue{cell: self.cell}))
			}
		};

		res
  }
}

#[cfg(test)]
mod test {
	use state;
	use Value;
	use Location;
	use Operation;
	use operators::Operator;
	use operators::add::AddOp;

	#[test]
	fn add_two_numbers(){
		let mut state = state::InternalState {
			register: Some(Value::Number{value: 5}),
			input_tape: vec!(),
			output_tape: vec!(),
			memory: vec!(Some(Value::Number{value: 4})),
			instruction_counter: 0
		};
		let operation = AddOp{cell: Location::Cell(0)};

		let _ = operation.apply_to(&mut state).unwrap();

		assert!(match state.register {
			Some(Value::Number{value: 9}) => true,
			_ => false
		});
	}

	#[test]
	fn add_two_numbers_address() {
		let mut state = state::InternalState::new(Some(Value::Number{value: 5}), 0);
		state.memory = vec!(Some(Value::Number{value: 1}), Some(Value::Number{value: 4}));
		let operation = AddOp{cell: Location::Address(0)};

		let _ = operation.apply_to(&mut state).unwrap();

		assert!(match state.register {
			Some(Value::Number{value: 9}) => true,
			_ => false
		});
	}

	#[test]
	fn add_number_to_empty_cell(){
		let mut state = state::InternalState {
			register: Some(Value::Number{value: 5}),
			input_tape: vec!(),
			output_tape: vec!(),
			memory: vec!(None),
			instruction_counter: 0
		};
		let operation = AddOp{cell: Location::Cell(0)};

		let result = operation.apply_to(& mut state);

		assert!(result.is_err());
	}

	#[test]
	fn add_two_numbers_to_empty_address() {
		let mut state = state::InternalState::new(Some(Value::Number{value: 5}), 0);
		state.memory = vec!(None, Some(Value::Number{value: 4}));
		let operation = AddOp{cell: Location::Address(0)};

		let result = operation.apply_to(&mut state);
		assert!(result.is_err());
	}

	#[test]
	fn add_number_to_empty_addressed_cell(){
		let mut state = state::InternalState::new(Some(Value::Number{value: 5}), 0);
		state.memory = vec!(Some(Value::Number{value: 1}), None, None, None, None);
		let operation = AddOp{cell: Location::Address(0)};

		let result = operation.apply_to(&mut state);
		println!("{:?}", result);
		assert!(result.is_err());
	}

	#[test]
	fn add_number_to_empty_register(){
		let mut state = state::InternalState {
			register: None,
			input_tape: vec!(),
			output_tape: vec!(),
			memory: vec!(Some(Value::Number{value: 5})),
			instruction_counter: 0
		};
		let operation = AddOp{cell: Location::Cell(0)};

		let result = operation.apply_to(&mut state);

		assert!(result.is_err());
	}

	#[test]
	fn add_addressed_number_to_empty_register(){
		let mut state = state::InternalState {
			register: None,
			input_tape: vec!(),
			output_tape: vec!(),
			memory: vec!(Some(Value::Number{value: 1}), Some(Value::Number{value: 5})),
			instruction_counter: 0
		};
		let operation = AddOp{cell: Location::Address(0)};

		let result = operation.apply_to(&mut state);
		assert!(result.is_err());
	}

	#[test]
	fn add_char_to_char(){
		let mut state = state::InternalState {
			register: Some(Value::Character{value: 'a'}),
			input_tape: vec!(),
			output_tape: vec!(),
			memory: vec!(Some(Value::Character{value: 'a'})),
			instruction_counter: 0
		};
		let operator = AddOp{cell: Location::Cell(0)};

		let result = operator.apply_to(&mut state);

		assert!(result.is_err());
	}

	#[test]
	fn add_char_to_addressed_char(){
		let mut state = state::InternalState::new(Some(Value::Character{value: 'a'}), 0);
		state.memory = vec!(Some(Value::Number{value: 1}), Some(Value::Character{value: 'a'}));
		let operator = AddOp{cell: Location::Address(0)};

		let result = operator.apply_to(&mut state);

		assert!(result.is_err());
	}

	#[test]
	fn add_char_to_number(){
		let mut state = state::InternalState {
			register: Some(Value::Character{value: 'a'}),
			input_tape: vec!(),
			output_tape: vec!(),
			memory: vec!(Some(Value::Number{value: 5})),
			instruction_counter: 0
		};

		let _ = state.apply(Operation::Add{cell: Location::Cell(0)});

		assert!(match state.register {
			Some(Value::Character{value: 'f'}) => true,
			_ => false
		});
	}

	#[test]
	fn add_char_to_number_overflow(){
		let mut state = state::InternalState {
			register: Some(Value::Character{value: 'z'}),
			input_tape: vec!(),
			output_tape: vec!(),
			memory: vec!(Some(Value::Number{value: 5})),
			instruction_counter: 0
		};
		let operation = AddOp{cell: Location::Cell(0)};

		let result = operation.apply_to(&mut state);

		assert!(result.is_err());
	}

	#[test]
	fn add_number_to_char(){
		let mut state = state::InternalState {
			register: Some(Value::Number{value: 5}),
			input_tape: vec!(),
			output_tape: vec!(),
			memory: vec!(Some(Value::Character{value: 'a'})),
			instruction_counter: 0
		};

		let _ = state.apply(Operation::Add{cell: Location::Cell(0)}).unwrap();

		assert!(match state.register {
			Some(Value::Character{value: 'f'}) => true,
			_ => false
		});
	}

	#[test]
	fn add_number_to_addressed_char(){
		let mut state = state::InternalState::new(Some(Value::Number{value: 5}), 0);
		state.memory = vec!(Some(Value::Number{value: 1}), Some(Value::Character{value: 'a'}));

		let _ = state.apply(Operation::Add{cell: Location::Address(0)}).unwrap();
		assert!(match state.register {
			Some(Value::Character{value: 'f'}) => true,
			_ => false
		});
	}

	#[test]
	fn add_number_to_char_overflow(){
		let mut state = state::InternalState {
			register: Some(Value::Number{value: 5}),
			input_tape: vec!(),
			output_tape: vec!(),
			memory: vec!(Some(Value::Character{value: 'z'})),
			instruction_counter: 0
		};
		let operation = AddOp{cell: Location::Cell(0)};

		let result = operation.apply_to(&mut state);

		assert!(result.is_err());
	}
}
