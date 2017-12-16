use operators::Operator;
use state::InternalState;
use Value;
use Location;

enum Error {
	PointerCellContainsChar,
	NoValue{cell: Location},
	BumpChar{value: char}
}

struct BumpOp {}

impl BumpOp {
	fn explain_error(error: Error) -> String {
		match error {
			Error::NoValue{cell: Location::Cell(_cell)} => format!("There is no value at cell {:?}", _cell),
			Error::NoValue{cell: Location::Address(_cell)} => format!("There is no value at cell {:?}", _cell),
			Error::BumpChar{value: _char} => format!("Cannot bump char {:?}", _char),
			Error::PointerCellContainsChar => String::from("The selected cell should contain a number, not a char")
		}
	}
}

pub struct BumpPlusOp {
	cell: Location
}

impl Operator for BumpPlusOp {
	fn changes_instruction_counter(&self) -> bool { false }

	fn apply_to(&self, s: &mut InternalState) -> Result<(), String> {
		let memory_position = match self.cell {
			Location::Cell(mempos) => Ok(mempos),
			Location::Address(mempos) => {
				let value_from_memory = s.memory[mempos].clone();
				match value_from_memory {
					Some(Value::Number{value: pointed_cell}) => Ok(pointed_cell as usize),
					Some(Value::Character{value: _}) => Err(BumpOp::explain_error(Error::PointerCellContainsChar)),
					None => Err(BumpOp::explain_error(Error::NoValue{cell: Location::Cell(mempos)}))
				}
			}
		};
		if let Err(error_message) = memory_position {
			return Err(error_message);
		}

		let mempos = memory_position.unwrap();
		let data = s.memory[mempos];
		match data {
			None => Err(BumpOp::explain_error(Error::NoValue{cell: Location::Cell(mempos)})),
			Some(Value::Number{value: _num}) => {
				s.memory[mempos] = Some(Value::Number{value: _num+1});
				s.register = s.memory[mempos];
				Ok(())
			},
			Some(Value::Character{value: _char}) =>
				Err(BumpOp::explain_error(Error::BumpChar{value: _char}))
		}

	}
}

#[cfg(test)]
mod test {
	use Value;
	use Location;
	use state::InternalState;
	use operators::Operator;
	use operators::bump::BumpPlusOp;

	#[test]
	fn bumpplus_number() {
		let mut state = InternalState::new(None, 0);
		state.memory = vec!(Some(Value::Number{value: 0}));

		let result = BumpPlusOp{cell: Location::Cell(0)}.apply_to(&mut state);

		assert!(result.is_ok());
		assert!(match state.memory[0] {
			Some(Value::Number{value: 1}) => true,
			_ => false
		});
		assert!(match state.register {
			Some(Value::Number{value: 1}) => true,
			_ => false
		});
	}
	
	#[test]
	fn bumpplus_char() {
		let mut state = InternalState::new(None, 0);
		state.memory = vec!(Some(Value::Character{value: 'a'}));

		let result = BumpPlusOp{cell: Location::Cell(0)}.apply_to(&mut state);

		assert!(result.is_err());
		assert!(match state.register {
			None => true,
			_ => false
		});
	}

	#[test]
	fn bumpplus_none() {
		let mut state = InternalState::new(None, 0);
		state.memory = vec!(None);

		let result = BumpPlusOp{cell: Location::Cell(0)}.apply_to(&mut state);

		assert!(result.is_err());
		assert!(match state.register {
			None => true,
			_ => false
		});
	}
}
