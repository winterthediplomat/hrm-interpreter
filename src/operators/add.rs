use operators::Operator;
use Value;
use state;

use std::char;

// structure to be modified by `Operator`s
// implement an operator
pub struct AddOp {
	pub cell: usize
}
impl Operator for AddOp {
  fn changes_instruction_counter(&self) -> bool {
		false
	}

  fn apply_to(&self, mut s: state::InternalState) -> state::InternalState {
		let x = s.memory[self.cell].clone();
		match x {
			Some(ref v) => {
				match s.register {
					Some(old_register) => {
						let new_register_value =  match (v, old_register) {
							(&Value::Number{value: _v}, Value::Number{value: _old}) => {
								Value::Number{value: _v + _old}
							},
								(&Value::Number{value: _v}, Value::Character{value: _old})
							| (&Value::Character{value: _old}, Value::Number{value: _v}) => {
								const ALPHABET_RADIX: u32 = 36;
								const SMALL_ASCII_A: u32 = 97;
								const HEX_A_IN_DEC: u32 = 10;
								let number = _old.to_digit(ALPHABET_RADIX).unwrap();
								let new_number = number + _v;
								let fixed_for_char : u32 = SMALL_ASCII_A + (new_number - HEX_A_IN_DEC);
								if new_number >= 36 {
									panic!("value overflowed! {} + {} is not representable as a letter!", _old, _v);
								}
								let new_char = char::from_u32(fixed_for_char).unwrap();
								Value::Character{value: new_char}
							},
							_ => panic!("cannot sum two characters or different Value types!")
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
