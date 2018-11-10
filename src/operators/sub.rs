use operators::Operator;
use Value;
use Location;
use state;
// --
use std::char;

enum Error {
	NoValue{cell: Location},
	PointerCellContainsChar,
	NoEmployeeValue,
	NumLessChar,
	SubUnderflow{character: char, number: i32},
	SubUnderflowOfChar{character: char, other_character: char}
}

pub struct SubOp {
	pub cell: Location
}

impl SubOp {
	fn sub_char_and_number(c: char, num: i32) -> Result<char, i32> {
		const ALPHABET_RADIX: u32 = 36;
		const SMALL_ASCII_A: i32 = 97;
		const HEX_A_IN_DEC: i32 = 10;

		let c_as_number = c.to_digit(ALPHABET_RADIX).unwrap() as i32;
		if num < c_as_number {
			Err(num - c_as_number)
		}
		else {
			let new_number = num - c_as_number;
			let fixed_for_char: u32 = (SMALL_ASCII_A - (new_number - HEX_A_IN_DEC)) as u32;

			Ok(char::from_u32(fixed_for_char).unwrap())
		}
	}

	fn sub_char_and_char(a: char, b: char) -> Result<i32, i32> {
		const ALPHABET_RADIX: u32 = 36;

		let a_as_number = a.to_digit(ALPHABET_RADIX).unwrap() as i32;
		let b_as_number = b.to_digit(ALPHABET_RADIX).unwrap() as i32;
		let new_number = b_as_number - a_as_number;
		Ok(new_number)
	}

	fn explain_error(e: Error) -> String {
		match e {
			Error::NoValue{cell: Location::Cell(_cell)} => format!("There is no value at cell {:?}", _cell),
			Error::NoValue{cell: Location::Address(_cell)} => format!("There is no value at cell {:?}", _cell),
			Error::PointerCellContainsChar => String::from("The selected cell should contain a number, not a char"),
			Error::NoEmployeeValue => String::from("the Employee register holds no value. Cannot add."),
			Error::NumLessChar => String::from("cannot perform <num> - <char>"),
			Error::SubUnderflow{character: _char, number: _num} =>
				format!("value underflowed! {} - {} is not representable as a letter!",
								_char, _num),
			Error::SubUnderflowOfChar{character: _char, other_character: _num} =>
				format!("value underflowed! {} - {} is not representable as a letter!",
								_char, _num)
		}
	}
}

impl Operator for SubOp {
  fn changes_instruction_counter(&self) -> bool {
		false
	}

  fn apply_to(&self,  s: &mut state::InternalState) -> Result<(), String> {
		let memory_position = match self.cell {
			Location::Cell(mempos) => Ok(mempos),
			Location::Address(mempos) => {
				let value_from_memory = s.memory[mempos].clone();
				match value_from_memory {
					Some(Value::Number{value: pointed_cell}) => Ok(pointed_cell as usize),
					Some(Value::Character{value: _}) => Err(SubOp::explain_error(Error::PointerCellContainsChar)),
					None => Err(SubOp::explain_error(Error::NoValue{cell: Location::Cell(mempos)}))
				}
			}
		};
		if let Err(error_message) = memory_position {
			return Err(error_message);
		}

		let value_from_memory = s.memory[memory_position.unwrap()].clone();
		let res = match value_from_memory {
			Some(ref v) => {
				match s.register {
					Some(old_register) => {
						let new_register_value: Result<Value, String> =  match (v, old_register) {
							(&Value::Number{value: _v}, Value::Number{value: _old}) => {
								Ok(Value::Number{value: _old - _v})
							},
							(&Value::Character{value: _v}, Value::Character{value: _old}) => {
								if let Ok(new_number) = SubOp::sub_char_and_char(_v, _old) {
									Ok(Value::Number{value: new_number})
								}
								else {
									Err(SubOp::explain_error(Error::SubUnderflowOfChar{character: _old, other_character: _v}))
								}
							},
							(&Value::Character{value: _v}, Value::Number{value: _old}) => {
								if let Ok(new_char) = SubOp::sub_char_and_number(_v, _old) {
									Ok(Value::Character{value: new_char})
								}
								else {
									Err(SubOp::explain_error(Error::SubUnderflow{character: _v, number: _old}))
								}
							},
							(&Value::Number{value: _v}, Value::Character{value: _old}) =>
								Err(SubOp::explain_error(Error::NumLessChar))
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
						Err(SubOp::explain_error(Error::NoEmployeeValue))
					}
				}
			}
			_ => {
				Err(SubOp::explain_error(Error::NoValue{cell: self.cell}))
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
	use operators::sub::SubOp;

	#[test]
	fn sub_two_numbers(){
		let mut state = state::InternalState
			::new(Some(Value::Number{value: 5}), 0)
			.with_memory(vec!(Some(Value::Number{value: 4})));
		let operation = SubOp{cell: Location::Cell(0)};

		let _ = operation.apply_to(&mut state).unwrap();

		assert!(match state.register {
			Some(Value::Number{value: 1}) => true,
			_ => false
		});
	}

	#[test]
	fn sub_two_numbers_negative_result(){
		let mut state = state::InternalState
		::new(Some(Value::Number{value: 4}), 0)
			.with_memory(vec!(Some(Value::Number{value: 5})));
		let operation = SubOp{cell: Location::Cell(0)};

		let _ = operation.apply_to(&mut state).unwrap();

		assert!(match state.register {
			Some(Value::Number{value: -1}) => true,
			_ => false
		});
	}

	#[test]
	fn sub_two_numbers_address() {
		let mut state = state::InternalState::new(Some(Value::Number{value: 5}), 0);
		state.memory = vec!(Some(Value::Number{value: 1}), Some(Value::Number{value: 4}));
		let operation = SubOp{cell: Location::Address(0)};

		let _ = operation.apply_to(&mut state).unwrap();

		assert!(match state.register {
			Some(Value::Number{value: 1}) => true,
			_ => false
		});
	}

	#[test]
	fn sub_two_numbers_address_negative_result() {
		let mut state = state::InternalState::new(Some(Value::Number{value: 4}), 0);
		state.memory = vec!(Some(Value::Number{value: 1}), Some(Value::Number{value: 5}));
		let operation = SubOp{cell: Location::Address(0)};

		let _ = operation.apply_to(&mut state).unwrap();

		assert!(match state.register {
			Some(Value::Number{value: -1}) => true,
			_ => false
		});
	}

	#[test]
	fn sub_number_to_empty_cell(){
		let mut state = state::InternalState
		::new(Some(Value::Number{value: 5}), 0)
			.with_memory(vec!(None));
		let operation = SubOp{cell: Location::Cell(0)};

		let result = operation.apply_to(& mut state);

		assert!(result.is_err());
	}

	#[test]
	fn sub_two_numbers_to_empty_address() {
		let mut state = state::InternalState::new(Some(Value::Number{value: 5}), 0);
		state.memory = vec!(None, Some(Value::Number{value: 4}));
		let operation = SubOp{cell: Location::Address(0)};

		let result = operation.apply_to(&mut state);
		assert!(result.is_err());
	}

	#[test]
	fn sub_number_to_empty_addressed_cell(){
		let mut state = state::InternalState::new(Some(Value::Number{value: 5}), 0);
		state.memory = vec!(Some(Value::Number{value: 1}), None, None, None, None);
		let operation = SubOp{cell: Location::Address(0)};

		let result = operation.apply_to(&mut state);
		assert!(result.is_err());
	}

	#[test]
	fn sub_number_to_empty_register(){
		let mut state = state::InternalState
		::new(None, 0)
			.with_memory(vec!(Some(Value::Number{value: 5})));
		let operation = SubOp{cell: Location::Cell(0)};

		let result = operation.apply_to(&mut state);

		assert!(result.is_err());
	}

	#[test]
	fn sub_addressed_number_to_empty_register(){
		let mut state = state::InternalState
		::new(None, 0)
			.with_memory(vec!(Some(Value::Number{value: 1}), Some(Value::Number{value: 5})));
		let operation = SubOp{cell: Location::Address(0)};

		let result = operation.apply_to(&mut state);
		assert!(result.is_err());
	}

	#[test]
	fn sub_char_to_char(){
		let mut state = state::InternalState
		::new(Some(Value::Character{value: 'a'}), 0)
			.with_memory(vec!(Some(Value::Character{value: 'a'})));
		let operator = SubOp{cell: Location::Cell(0)};

		let result = operator.apply_to(&mut state);

		assert!(result.is_ok());
		assert!(match state.register {
			Some(Value::Number{value: 0}) => true,
			_ => false
		});
	}

	#[test]
	fn sub_char_to_char_negative(){
		let mut state = state::InternalState
		::new(Some(Value::Character{value: 'a'}), 0)
			.with_memory(vec!(Some(Value::Character{value: 'b'})));
		let operator = SubOp{cell: Location::Cell(0)};

		let result = operator.apply_to(&mut state);

		assert!(result.is_ok());
		assert!(match state.register {
			Some(Value::Number{value: -1}) => true,
			_ => false
		});
	}

	#[test]
	fn sub_char_to_addressed_char(){
		let mut state = state::InternalState::new(Some(Value::Character{value: 'a'}), 0);
		state.memory = vec!(Some(Value::Number{value: 1}), Some(Value::Character{value: 'a'}));
		let operator = SubOp{cell: Location::Address(0)};

		let result = operator.apply_to(&mut state);

		assert!(result.is_ok());
		assert!(match state.register {
			Some(Value::Number{value: 0}) => true,
			_ => false
		});
	}

	#[test]
	fn sub_char_to_number(){
		let mut state = state::InternalState
		::new(Some(Value::Character{value: 'a'}), 0)
			.with_memory(vec!(Some(Value::Number{value: 5})));

		let result = state.apply(Operation::Sub{cell: Location::Cell(0)});

		assert!(result.is_err());
	}

	#[test]
	fn sub_char_to_number_overflow(){
		let mut state = state::InternalState
		::new(Some(Value::Character{value: 'z'}), 0)
			.with_memory(vec!(Some(Value::Number{value: 5})));
		let operation = SubOp{cell: Location::Cell(0)};

		let result = operation.apply_to(&mut state);

		assert!(result.is_err());
	}

	#[test]
	fn sub_number_to_char(){
		let mut state = state::InternalState
		::new(Some(Value::Number {value: 5}), 0)
			.with_memory(vec!(Some(Value::Character {value:'a'})));

		let result = state.apply(Operation::Sub{cell: Location::Cell(0)});

		assert!(result.is_err());
	}

	#[test]
	fn sub_number_to_addressed_char(){
		let mut state = state::InternalState::new(Some(Value::Number{value: 5}), 0);
		state.memory = vec!(Some(Value::Number{value: 1}), Some(Value::Character{value: 'a'}));

		let result = state.apply(Operation::Sub{cell: Location::Address(0)});
		assert!(result.is_err());
	}

	#[test]
	fn sub_number_to_char_overflow(){
		let mut state = state::InternalState
		::new(Some(Value::Number {value: 5}), 0)
			.with_memory(vec!(Some(Value::Character{value: 'z'})));
		let operation = SubOp{cell: Location::Cell(0)};

		let result = operation.apply_to(&mut state);

		assert!(result.is_err());
	}
}
