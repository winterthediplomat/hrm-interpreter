use operators::Operator;
use Value;
use state;

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
						let value_to_add = match v {
							&Value::Number{value: _v} => _v,
							&Value::Character{value: _} => panic!("argh")
						};
						let old_register_value = match old_register {
							Value::Number{value: _v} => _v,
							Value::Character{value: _} => panic!("argh2")
						};
						s.register = Some(Value::Number{value: old_register_value + value_to_add});
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
}
