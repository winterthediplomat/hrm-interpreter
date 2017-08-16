use operators::Operator;
use Value;
use state;
// --
use std::char;

pub struct AddOp {
	pub cell: usize
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
}

impl Operator for AddOp {
  fn changes_instruction_counter(&self) -> bool {
		false
	}

  fn apply_to(&self, mut s: state::InternalState) -> state::InternalState {
		let value_from_memory = s.memory[self.cell].clone();
		match value_from_memory {
			Some(ref v) => {
				match s.register {
					Some(old_register) => {
						let new_register_value =  match (v, old_register) {
							(&Value::Number{value: _v}, Value::Number{value: _old}) => {
								Value::Number{value: _v + _old}
							},
							(&Value::Number{value: _v}, Value::Character{value: _old}) => {
								if let Ok(new_char) = AddOp::add_number_and_char(_v, _old) {
									Value::Character{value: new_char}
								}
								else {
									panic!("value overflowed! {} + {} is not representable as a letter!", _old, _v);
								}
							},
							(&Value::Character{value: _v}, Value::Number{value: _old}) => {
								if let Ok(new_char) = AddOp::add_number_and_char(_old, _v) {
									Value::Character{value: new_char}
								}
								else {
									panic!("value overflowed! {} + {} is not representable as a letter!", _old, _v);
								}
							},
							_ => panic!("cannot sum two characters!")
						};
						s.register = Some(new_register_value);
					}
					_ => {
						panic!("No value in register Employee, cannot add.");
					}
				}
			}
			_ => {
				panic!("No value at cell {}", self.cell);
			}
		}

		return s;
  }
}

#[cfg(test)]
mod test {
	use state;
	use Value;
	use Operation;

	#[test]
	fn add_two_numbers(){
		let mut state = state::InternalState {
			register: Some(Value::Number{value: 5}),
			input_tape: vec!(),
			output_tape: vec!(),
			memory: vec!(Some(Value::Number{value: 4})),
			instruction_counter: 0
		};
		
		state = state.apply(Operation::Add{cell: 0});

		assert!(match state.register {
			Some(Value::Number{value: 9}) => true,
			_ => false
		});
	}

	#[test]
	#[should_panic]
	fn add_number_to_empty_cell(){
		let state = state::InternalState {
			register: Some(Value::Number{value: 5}),
			input_tape: vec!(),
			output_tape: vec!(),
			memory: vec!(None),
			instruction_counter: 0
		};
		
		state.apply(Operation::Add{cell: 0});
	}

	#[test]
	#[should_panic]
	fn add_number_to_empty_register(){
		let state = state::InternalState {
			register: None,
			input_tape: vec!(),
			output_tape: vec!(),
			memory: vec!(Some(Value::Number{value: 5})),
			instruction_counter: 0
		};
		
		state.apply(Operation::Add{cell: 0});
	}

	#[test]
	#[should_panic]
	fn add_char_to_char(){
		let state = state::InternalState {
			register: Some(Value::Character{value: 'a'}),
			input_tape: vec!(),
			output_tape: vec!(),
			memory: vec!(Some(Value::Character{value: 'a'})),
			instruction_counter: 0
		};

		state.apply(Operation::Add{cell: 0});
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

		state = state.apply(Operation::Add{cell: 0});

		assert!(match state.register {
			Some(Value::Character{value: 'f'}) => true,
			_ => false
		});
	}

	#[test]
	#[should_panic]
	fn add_char_to_number_overflow(){
		let state = state::InternalState {
			register: Some(Value::Character{value: 'z'}),
			input_tape: vec!(),
			output_tape: vec!(),
			memory: vec!(Some(Value::Number{value: 5})),
			instruction_counter: 0
		};

		state.apply(Operation::Add{cell: 0});
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

		state = state.apply(Operation::Add{cell: 0});

		assert!(match state.register {
			Some(Value::Character{value: 'f'}) => true,
			_ => false
		});
	}

	#[test]
	#[should_panic]
	fn add_number_to_char_overflow(){
		let state = state::InternalState {
			register: Some(Value::Number{value: 5}),
			input_tape: vec!(),
			output_tape: vec!(),
			memory: vec!(Some(Value::Character{value: 'z'})),
			instruction_counter: 0
		};

		state.apply(Operation::Add{cell: 0});
	}
}
